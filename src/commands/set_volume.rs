use failure::Error;
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
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let volume = (self.volume as f32) / 100f32;

        app.player.set_volume(volume)
    }
}