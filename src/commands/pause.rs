use failure::Error;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct PauseCommand {
}

impl PauseCommand {
    pub fn new() -> PauseCommand {
        PauseCommand {}
    }
}

impl MpdCommand<()> for PauseCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let mut player = app.player.lock().unwrap();
        player.pause()
    }
}