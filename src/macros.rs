macro_rules! send_control_errorless {
    ($signal:expr, $($tx:expr),*) => {
        $({
            let _ = $tx.send($signal);
        })*
    }
}

macro_rules! send_control {
    ($signal:expr, $($tx:expr),*) => {
        $({
            $tx.send($signal)?
        })*
    }
}

macro_rules! __exit_await_thread {
    ($($thread:expr),*) => {
        $(
            $thread.join().unwrap();
        )*
    }
}

macro_rules! not_enough_space {
    ($tooey:expr) => {{
        $tooey.render_set_mode(RenderMode::NoSpace);
        // forgive me for this unfortunate error message.
        return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "s-stop!!~ there's not enough room... mmmfph"));
    }}
}

