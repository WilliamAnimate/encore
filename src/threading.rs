use std::thread::{spawn, JoinHandle};
use std::ops::Deref;

pub struct ThreadAbstraction (Option<JoinHandle<()>>);

impl ThreadAbstraction {
    pub fn spawn<F: std::marker::Send + FnOnce() + 'static>(f: F) -> ThreadAbstraction {
        ThreadAbstraction(Some(spawn(f)))
    }

    pub fn spawn_if<F: std::marker::Send + FnOnce() + 'static>(f: F, condition: bool) -> ThreadAbstraction {
        if condition {
            ThreadAbstraction(Some(spawn(f)))
        } else {
            ThreadAbstraction(None)
        }
    }

    pub fn join(self) {
        if self.0.is_some() {
            if cfg!(debug_assertions) {
                // in debug, crash if thread panicked
                self.0.unwrap().join().unwrap();
                return;
            }
            let _ = self.0.unwrap().join();
        }
    }
}

impl Deref for ThreadAbstraction {
    type Target = Option<JoinHandle<()>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
