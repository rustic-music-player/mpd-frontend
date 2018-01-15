use error::MpdError;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct SetVolumeCommand {
    pub volume: u32
}

impl SetVolumeCommand {
    pub fn new(volume: u32) -> SetVolumeCommand {
        SetVolumeCommand {
            volume
        }
    }
}

impl MpdCommand<()> for SetVolumeCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), MpdError> {
        let mut player = app.player.lock().unwrap();
        player.set_volume(self.volume);
        Ok(())
    }
}