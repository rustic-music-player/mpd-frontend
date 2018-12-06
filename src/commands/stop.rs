use commands::MpdCommand;
use failure::Error;
use rustic_core::{player::PlayerState, Rustic};
use std::sync::Arc;

pub struct StopCommand {}

impl StopCommand {
    pub fn new() -> StopCommand {
        StopCommand {}
    }
}

impl MpdCommand<()> for StopCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        app.player.set_state(PlayerState::Stop)
    }
}
