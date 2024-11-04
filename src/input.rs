use echotune::SongControl;
use getch_rs::{Getch, Key};

pub struct Input { getch: Getch }

impl Input {
    pub fn from_nothing_and_apply() -> Input {
        let getch = Getch::new();

        Input { getch }
    }

    pub fn blocking_wait_for_input(&self) -> SongControl {
        let ret: SongControl;
        // char but uwuified :3
        let chaw = self.getch.getch().expect("can't read");
        ret = match chaw {
            // TODO: arrow keys should be changed to respect hjkl
            Key::Up => SongControl::VolumeUp,
            Key::Down => SongControl::VolumeDown,
            Key::Left => SongControl::SeekBackward,
            Key::Right => SongControl::SeekForward,
            Key::Char('r') => SongControl::ToggleLoop,
            Key::Char('k') => SongControl::PrevSong,
            Key::Char('j') => SongControl::NextSong,
            Key::Char(' ') => SongControl::TogglePause,
            Key::Ctrl('c') | Key::Char('q') => SongControl::DestroyAndExit,
            _ => SongControl::No,
        };

        ret
    }
}

