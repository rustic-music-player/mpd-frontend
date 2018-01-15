use error::MpdError;
use commands::MpdCommand;
use song::MpdSong;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct CurrentSongCommand {
}

impl CurrentSongCommand {
    pub fn new() -> CurrentSongCommand {
        CurrentSongCommand {}
    }
}

impl MpdCommand<Option<MpdSong>> for CurrentSongCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Option<MpdSong>, MpdError> {
        let player = app.player.lock().unwrap();
        let track = match player.queue.current() {
            Some(track) => Some(MpdSong::from(track.clone())),
            None => None
        };
        Ok(track)
    }
}