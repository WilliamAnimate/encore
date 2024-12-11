#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Copy)]
/// don't Box<SongControl> this value, or you're going to have a very hard time with .clone()
/// because it will panic.
/// :troll:
pub enum SongControl {
    VolumeUp,
    VolumeDown,
    SeekForward,
    SeekBackward,

    ToggleLoop,
    PrevSong,
    NextSong,
    TogglePause,

    No, // skull

    DestroyAndExit,

    Unset,
}

impl Clone for SongControl {
    fn clone(&self) -> Self {
        panic!("why are we on the heap???");
    }
}

#[derive(PartialEq, Default)]
pub enum RenderMode {
    Safe, // if term is too small, or if under resource constraints, or user specified, or
    #[default]
    Full, // the entire TUI
    NoSpace,
}

#[derive(PartialEq)]
pub enum FileFormat {
    Audio,

    // and if no match
    Other
}

pub enum ConfigurationPath<'a> {
    Default,
    Custom(&'a str)
}

pub struct AtomicF32(AtomicUsize);

/// no hardware support bruh
impl AtomicF32 {
    #[inline] pub fn new(v: f32) -> Self {
        AtomicF32(AtomicUsize::new(v.to_bits().try_into().unwrap()))
    }

    #[inline] pub fn load(&self, order: Ordering) -> f32 {
        f32::from_bits(self.0.load(order).try_into().unwrap())
    }

    #[inline] pub fn store(&self, val: f32, order: Ordering) {
        self.0.store(val.to_bits().try_into().unwrap(), order);
    }
}

pub fn to_vec<R: std::io::BufRead>(reader: R) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut v = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = if let Some(l) = normalize_line(&line) { l } else { continue };
        v.push(line); // fast code
    }

    Ok(v)
}

pub fn normalize<R: Iterator<Item = String>>(i: R) -> Vec<String> {
    let mut vec = Vec::new();
    for s in i {
        if let Some(s) = normalize_line(&s) { vec.push(s) } else { continue }
    }

    vec
}

pub fn normalize_line(s: &str) -> Option<String> {
    use std::env;

    let home = if cfg!(unix) { env::var("HOME") } else { env::var("USERPROFILE") }
        .expect("can't find home dir");

    if s.is_empty() { return None };
    Some(s.replacen('~', &home, 1))
}

