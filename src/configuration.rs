#![cfg_attr(not(feature = "configuration"), allow(unused))]

#[cfg(feature = "configuration")]
use serde::Deserialize;

#[cfg(target_os = "linux")]
static DEFAULT_CFG_PATH: &'static str = ".config/echotune/echotune.toml";
#[cfg(target_os = "windows")]
static DEFAULT_CFG_PATH: &'static str = "AppData/Roaming/echotune/echotune.toml";
#[cfg(target_os = "macos")]
static DEFAULT_CFG_PATH: &'static str = "Library/Preferences/echotune/echotune.toml";

#[derive(Debug, Default)]
#[cfg_attr(feature = "configuration", derive(Deserialize))]
pub struct Config {
    pub main: TomlMain,
    pub playlist: TomlPlaylist,
    pub keybinds: Keybinds,
}

#[derive(Debug)]
#[cfg_attr(feature = "configuration", derive(Deserialize), serde(default))]
pub struct TomlMain {
    pub crash_on_execute: bool,
}
impl Default for TomlMain {
    fn default() -> Self {
        Self {
            crash_on_execute: false,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "configuration", derive(Deserialize), serde(default))]
pub struct TomlPlaylist {
    pub never_use: bool,
    pub highlighted_color: String,
}
impl Default for TomlPlaylist {
    fn default() -> Self {
        Self {
            never_use: false,
            highlighted_color: "f5c2e7".to_string(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "configuration", derive(Deserialize), serde(default))]
pub struct Keybinds {
    pub disable_default_arrow_keys: bool,
    pub disable_default_ctrlc_exit: bool,

    pub increase_volume: char,
    pub decrease_volume: char,
    pub seek_backward: char,
    pub seek_forward: char,
    pub toggle_loop: char,
    pub prev_song: char,
    pub next_song: char,
    pub pause: char,
    pub quit: char,
}
impl Default for Keybinds {
    fn default() -> Self {
        Self {
            disable_default_arrow_keys: false,
            disable_default_ctrlc_exit: false,

            increase_volume: 'K',
            decrease_volume: 'J',
            seek_backward: 'H',
            seek_forward: 'L',
            toggle_loop: 'r',
            prev_song: 'k',
            next_song: 'j',
            pause: ' ',
            quit: 'q',
        }
    }
}

impl Config {
    pub fn parse(to_parse: echotune::ConfigurationPath) -> Self {
    #[cfg(not(feature = "configuration"))] {
        return Config::default();
    }

#[cfg(feature = "configuration")] {
    use std::fs::read_to_string;

    let file = match to_parse {
        echotune::ConfigurationPath::Default => DEFAULT_CFG_PATH,
        echotune::ConfigurationPath::Custom(s) => s
    };
    #[allow(deprecated)]
    let file = format!("{}/{}", std::env::home_dir().unwrap().to_string_lossy().to_string(), file);

    let buf = read_to_string(file).unwrap();

    let parsed: Config = basic_toml::from_str(&buf).unwrap();
    dbg!(&parsed);

    parsed
}
    }
}

