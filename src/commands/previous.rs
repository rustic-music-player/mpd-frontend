use failure::Error;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct PreviousCommand {
}

impl PreviousCommand {
    pub fn new() -> PreviousCommand {
        PreviousCommand {}
    }
}

impl MpdCommand<()> for PreviousCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let mut player = app.player.lock().unwrap();
        player.prev()
    }
}