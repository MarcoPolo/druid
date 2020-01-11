//! android implementation of runloop.

pub struct RunLoop;

impl RunLoop {
    pub fn new() -> RunLoop {
        RunLoop
    }

    pub fn run(&mut self) {
        // No op, we've already started the run loop by starting the app.
    }
}
