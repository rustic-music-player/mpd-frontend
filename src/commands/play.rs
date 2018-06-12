use failure::Error;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct PlayCommand {
}

impl PlayCommand {
    pub fn new() -> PlayCommand {
        PlayCommand {}
    }
}

impl MpdCommand<()> for PlayCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let mut player = app.player.lock().unwrap();
        player.play()
    }
}