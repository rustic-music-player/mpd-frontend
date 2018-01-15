use error::MpdError;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct StopCommand {
}

impl StopCommand {
    pub fn new() -> StopCommand {
        StopCommand {}
    }
}

impl MpdCommand<()> for StopCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.stop();
        Ok(())
    }
}