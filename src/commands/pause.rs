use failure::Error;
use commands::MpdCommand;
use rustic_core::{Rustic, player::PlayerState};
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
        app.player.set_state(PlayerState::Pause)
    }
}