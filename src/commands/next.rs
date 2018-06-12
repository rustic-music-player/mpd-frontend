use failure::Error;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct NextCommand {
}

impl NextCommand {
    pub fn new() -> NextCommand {
        NextCommand {}
    }
}

impl MpdCommand<()> for NextCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let mut player = app.player.lock().unwrap();
        player.next()
    }
}