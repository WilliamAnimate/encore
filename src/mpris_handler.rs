use encore::SongControl;
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use std::sync::atomic::Ordering;

pub struct MediaInfo {
    pub controls: MediaControls,
}

impl MediaInfo {
    pub fn new() -> MediaInfo {
        #[cfg(target_os = "windows")]
        todo!("implement hWnd bullshit. use linux in the meantime.");

        #[cfg(not(target_os = "windows"))]
        let hwnd = None;

        let config = PlatformConfig {
            dbus_name: "encore",
            display_name: "Encore",
            hwnd,
        };

        let controls = MediaControls::new(config).expect("hmm");

        MediaInfo {
            controls,
        }
    }

    pub fn update(&mut self) {
        let song_len = Duration::from_secs(crate::SONG_CURRENT_LEN.load(Ordering::Relaxed));
        let total_song_len = Duration::from_secs(crate::SONG_TOTAL_LEN.load(Ordering::Relaxed));

        let song_index = crate::SONG_INDEX.load(Ordering::Relaxed);
        let title = crate::PLAYLIST.read().unwrap();

        let metadata = MediaMetadata {
            title: Some(encore::trim_path(&title[song_index])),
            album: None, // TODO: playlist name
            artist: None, // TODO: implement artist readout
            duration: Some(total_song_len),
            cover_url: None,
        };

        let playback = {
            if crate::PAUSED.load(Ordering::Relaxed) {
                MediaPlayback::Paused { progress: Some(MediaPosition(song_len)) }
            } else {
                MediaPlayback::Playing { progress: Some(MediaPosition(song_len)) }
            }
        };
        self.controls
            .set_playback(playback).unwrap();

        self.controls.set_metadata(metadata).unwrap();
    }

}

pub fn on_media_event(ev: MediaControlEvent, tx: Arc<mpsc::Sender<encore::SongControl>>) {
    let r = match ev {
        MediaControlEvent::Pause => SongControl::Pause,
        MediaControlEvent::Play => SongControl::Resume,
        MediaControlEvent::Toggle => SongControl::TogglePause,
        MediaControlEvent::Next => SongControl::NextSong,
        MediaControlEvent::Previous => SongControl::PrevSong,
        MediaControlEvent::Stop => SongControl::DestroyAndExit,
        x => unimplemented!("got event {:?}. how'd you get here?", x),
    };

    tx.send(r).expect("wtf");
}

