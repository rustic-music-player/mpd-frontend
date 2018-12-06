use commands::MpdCommand;
use failure::Error;
use rustic_core::{player::PlayerState, Rustic};
use std::sync::Arc;

pub struct PauseCommand {}

impl PauseCommand {
    pub fn new() -> PauseCommand {
        PauseCommand {}
    }
}

impl MpdCommand<()> for PauseCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        app.player.set_state(PlayerState::Pause)
    }
}
