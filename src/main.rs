mod song;
mod input;
mod tui;
mod file_format;
mod configuration;

#[macro_use]
mod macros;

use std::sync::{atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::Relaxed}, RwLock, mpsc::channel, Arc};
use std::{io::{BufReader, BufRead}, fs::File};

lazy_static::lazy_static!{
    static ref PLAYLIST: RwLock<Vec<String>> = Default::default();
    static ref CFG_IS_LOOPED: AtomicBool = AtomicBool::new(false);
    static ref SONG_INDEX: AtomicUsize = AtomicUsize::new(0);
    static ref SONG_TOTAL_LEN: AtomicU64 = AtomicU64::new(0);
    static ref SONG_CURRENT_LEN: AtomicU64 = AtomicU64::new(0);
    static ref VOLUME_LEVEL: encore::AtomicF32 = encore::AtomicF32::new(0.0);
}

fn parse_playlist(file: BufReader<File>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = PLAYLIST.write().unwrap();
    let home = std::env::var("HOME").unwrap_or_else(|_| String::new());
    for line in file.lines() {
        let mut line = match line {
            Ok(k) => k,
            Err(err) => return Err(format!("argv[1] should be a media file or Encore-compatable playlist.\n{err}").into()),
        };
        if line.starts_with("//") {
            continue; // its a comment; skip
        }
        line = line.replacen('~', &home, 1);
        if File::open(&line).is_ok() {
            lines.push(line); // file exists, therefore, push it onto the playlist
        }
    }
    lines.shrink_to_fit();

    Ok(())
}

fn quit_with(e: &str, s: &str) -> Result<std::convert::Infallible, Box<dyn std::error::Error>> {
    eprintln!("{e}");
    Err(s.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::thread::spawn;
    use std::time::Duration;
    use encore::SongControl::*;

    let cfg = configuration::Config::parse(&encore::ConfigurationPath::Default);
    if cfg.main.crash_on_execute {
        panic!("nya~");
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        quit_with("argv[1] should be a media file or Encore-compatable playlist.", "argv[1] not supplied")?;
    }

    let file = &args[1];
    let mut reader = BufReader::new(File::open(file)?);
    let fmt = file_format::check_file(&mut reader)?;
    let mut render_requested_mode = encore::RenderMode::Full;

    match fmt {
        encore::FileFormat::Other => {
            parse_playlist(reader)?;
            if PLAYLIST.read().unwrap().len() == 0 {
                quit_with("no songs in playlist array; are all of the paths valid?", "playlist file has zero length")?;
            }
        },
        encore::FileFormat::Audio => {
            let mut lines = PLAYLIST.write().unwrap();
            if args.len() > 2 {
                for s in args.iter()
                    .skip(1) // the first index is encore itself
                    .by_ref() // then actually iterate through it
                {
                    lines.push(s.to_owned());
                }
            } else {
                render_requested_mode = encore::RenderMode::Safe; // only one song, so do minimal
                lines.push(file.to_string());
            }
        },
    };

    let (mtx, mrx) = channel();
    let mtx = Arc::new(mtx);
    let audio_over_mtx = mtx.clone();
    let ctrlc_mtx = mtx.clone();

    let (rtx, rrx) = channel();
    let rtx = Arc::new(rtx);
    let main_rtx = rtx.clone();
    let render = spawn(move || {
        let mut tui = tui::Tui::init()
            .with_rendering_mode(render_requested_mode);
        tui.enter_alt_buffer().unwrap();
        loop {
            tui.tick();
            let receive = rrx.recv_timeout(Duration::from_secs(1));
            if let Ok(k) = receive {
                match k {
                    DestroyAndExit => break, // the destructor will exit the alt buffer
                    ToggleLoop => CFG_IS_LOOPED.store(!CFG_IS_LOOPED.load(Relaxed), Relaxed),
                    _na => {
                        #[cfg(debug_assertions)]
                        eprintln!("the operation {_na:?} is not applicable for rendering");
                    }
                };
            }
        }
    });

    let _input = spawn(move || {
        let input = input::Input::from_nothing_and_apply();
        loop {
            let i = input.blocking_wait_for_input();
            match i {
                DestroyAndExit => {
                    send_control_errorless!(DestroyAndExit, ctrlc_mtx);
                    break;
                },
                NextSong => {
                    if PLAYLIST.read().unwrap().len() != 1 {
                        let i = SONG_INDEX.load(Relaxed);
                        SONG_INDEX.store(i + 1, Relaxed);
                        send_control_errorless!(NextSong, rtx, mtx);
                    }
                }
                PrevSong => {
                    let sub = match SONG_INDEX.load(Relaxed).checked_sub(1) {
                        Some(n) => n,
                        None => continue,
                    };
                    SONG_INDEX.store(sub, Relaxed);
                    send_control_errorless!(PrevSong, rtx, mtx);
                }
                No => (), // there is nothing
                signal => {
                    send_control_errorless!(signal, rtx, mtx);
                }
            }
        }
    });

    let mut audio = song::Song::new();
    audio.play();
    loop {
        let receive = mrx.recv_timeout(Duration::from_secs(1));
        if let Ok(k) = receive {
            match k {
                DestroyAndExit => {
                    send_control!(DestroyAndExit, main_rtx);

                    // wait for the threads to finish
                    // FIXME: input doesnt seem to work. it hangs.
                    __exit_await_thread!(render);

                    break;
                }
                PrevSong | NextSong => audio.rejitter_song(),
                TogglePause => if audio.sink.is_paused() {audio.play()} else {audio.pause()} // why no ternary operator in rust
                VolumeUp => {
                    let prev_vol = audio.sink.volume();
                    audio.sink.set_volume(prev_vol + 0.1);
                },
                VolumeDown => {
                    let prev_vol = audio.sink.volume();
                    let request_vol = prev_vol - 0.1;
                    // no .saturating_sub for f32 cause primitive type, so we do this:
                    let normalized_vol = if request_vol < 0.0 { 0.0 } else { request_vol };
                    audio.sink.set_volume(normalized_vol);
                },
                // seeking may fail. if so, then silently fail, because who cares??
                SeekForward => {
                    let _ = audio.sink.try_seek(audio.sink.get_pos() + std::time::Duration::from_secs(5));
                }
                SeekBackward => {
                    let _ = audio.sink.try_seek(audio.sink.get_pos().saturating_sub(std::time::Duration::from_secs(5)));
                }
                _na => {
                    #[cfg(debug_assertions)]
                    eprintln!("the operation {_na:?} is not applicable for audio");
                }
            }
        }

        if audio.sink.empty() {
            let song_index = SONG_INDEX.load(Relaxed);
            if song_index >= PLAYLIST.read().unwrap().len() - 1 { // playlist len always + 1 because math
                send_control_errorless!(DestroyAndExit, audio_over_mtx);
            } else if !CFG_IS_LOOPED.load(Relaxed) {
                SONG_INDEX.store(song_index + 1, Relaxed);
            }
            audio.rejitter_song();
        } else {
            // task: synchronise global variables based on what we have.

            // there is a bug here: sometimes, this returns None.
            // some mp3s work, but others don't. i dont know why precisely.
            let total_dur = match audio.total_duration {
                Some(n) => n.as_secs(),
                None => 0,
            };
            SONG_CURRENT_LEN.store(audio.sink.get_pos().as_secs(), Relaxed);
            SONG_TOTAL_LEN.store(total_dur, Relaxed);

            VOLUME_LEVEL.store(audio.sink.volume(), Relaxed);
        }
    }

    Ok(())
}

