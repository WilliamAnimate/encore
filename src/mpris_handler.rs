use encore::SongControl;
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use std::sync::atomic::Ordering;

type Tx = Arc<mpsc::Sender<encore::SongControl>>;

pub struct MediaInfo {
    pub controls: MediaControls,
    // tx: Arc<mpsc::Sender<encore::SongControl>>,
}

impl MediaInfo {
    // pub fn from_tx(tx: Tx) -> MediaInfo {
    pub fn from_tx() -> MediaInfo {
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
            // tx,
        }
    }

    pub fn update(&mut self) {
        eprintln!("update commence");
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

        self.controls
            // man wtf why'd you have to create a wrapper struct for a single Duration
            .set_playback(MediaPlayback::Playing { progress: Some(MediaPosition(song_len)) }).unwrap();

        self.controls.set_metadata(metadata).unwrap();
        // TODO: played length
        // self.controls.set_playback(MediaPlayback::Playing { progress: song_len })
        eprintln!("update complete");
    }

}

pub fn on_media_event(ev: MediaControlEvent, tx: Tx) {
    eprintln!("media event: {:?}", ev);
    let r = match ev {
        MediaControlEvent::Pause | MediaControlEvent::Play | // TODO: don't toggle.
            MediaControlEvent::Toggle => SongControl::TogglePause,
        MediaControlEvent::Next => SongControl::NextSong,
        MediaControlEvent::Previous => SongControl::PrevSong,
        x => unimplemented!("got event {:?}. how'd you get here?", x),
    };

    eprintln!("responding with: {:?}", r);

    tx.send(r).expect("wtf");
}

