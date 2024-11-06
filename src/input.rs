use echotune::SongControl;
use getch_rs::{Getch, Key};

pub struct Input(Getch);

impl Input {
    pub fn from_nothing_and_apply() -> Input {
        Input(Getch::new())
    }

    pub fn blocking_wait_for_input(&self) -> SongControl {
        let keybinds = &crate::CONFIG.read().unwrap().keybinds;
        let increase_vol = keybinds.increase_volume;
        let decrease_vol = keybinds.decrease_volume;
        let seek_backward = keybinds.seek_backward;
        let seek_forward = keybinds.seek_forward;
        let toggle_loop = keybinds.toggle_loop;
        let prev_song = keybinds.prev_song;
        let next_song = keybinds.next_song;
        let pause = keybinds.pause;
        let quit = keybinds.quit;

        let ret: SongControl;
        // char but uwuified :3
        let chaw = self.0.getch().expect("can't read");
        ret = match chaw {
            Key::Up => SongControl::VolumeUp,
            Key::Down => SongControl::VolumeDown,
            Key::Left => SongControl::SeekBackward,
            Key::Right => SongControl::SeekForward,
            Key::Char(c) => {
                /*
                 * FIXME: you can't match a variable with another variable, like this:
                 * match chaw {
                 *     Key::Up => SongControl::VolumeUp, // this matches correctly
                 *     Key::Char(prev_song) => SongControl::PrevSong, // this is interpreted as an catch-all
                 *     Key::Char(next_song) => SongControl::NextSong, // this is unreachable because the above is a catch-all
                 *     _ => todo!("add the others"), // this is unreachable because the above above is a catch-all
                 * }
                 * and its not just a simple erroneous compiler warning: its actually treated as an
                 * catch-all.
                 *
                 * ...aaaaaaaaaaaaaand i have NO IDEA why.
                 *
                 * this could be fixed if you configure keybindings at compile time instead, but i don't
                 * want to do that because we aren't suckless.org or gentoo users. (i use arch btw)
                 *
                 * as for perf, it doesn't really matter. this code only runs on input and
                 * computers are REALLY fast. the bottleneck would be the underlying OS.
                 */
                if c == increase_vol {
                    SongControl::VolumeUp
                } else if c == decrease_vol {
                    SongControl::VolumeDown
                } else if c == seek_backward {
                    SongControl::SeekBackward
                } else if c == seek_forward {
                    SongControl::SeekForward
                } else if c == toggle_loop {
                    SongControl::ToggleLoop
                } else if c == prev_song {
                    SongControl::PrevSong
                } else if c == next_song {
                    SongControl::NextSong
                } else if c == pause {
                    SongControl::TogglePause
                } else if c == quit {
                    SongControl::DestroyAndExit
                }
                else {
                    SongControl::No
                }
            }
            Key::Ctrl('c') => SongControl::DestroyAndExit,
            _ => SongControl::No,
        };

        ret
    }
}

