use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;
use failure::Error;

pub struct ChangeVolumeCommand {
    pub volume: i32
}

impl ChangeVolumeCommand {
    pub fn new(volume: i32) -> ChangeVolumeCommand {
        ChangeVolumeCommand {
            volume
        }
    }
}

impl MpdCommand<()> for ChangeVolumeCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let mut player = app.player.lock().unwrap();
        let volume = (player.volume() as i32 + self.volume).min(100).max(0);
        player.set_volume(volume as u32)
    }
}