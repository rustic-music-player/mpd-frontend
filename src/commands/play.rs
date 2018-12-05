use failure::Error;
use commands::MpdCommand;
use rustic_core::{Rustic, player::PlayerState};
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
        app.player.set_state(PlayerState::Play)
    }
}