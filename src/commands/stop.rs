use failure::Error;
use commands::MpdCommand;
use rustic_core::{Rustic, player::PlayerState};
use std::sync::Arc;

pub struct StopCommand {
}

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