use failure::Error;
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
    fn handle(&self, app: &Arc<Rustic>) -> Result<Option<MpdSong>, Error> {
        let player = app.player.lock().unwrap();
        let track = player.queue
            .current()
            .map(|track| MpdSong::from(track.clone()));
        Ok(track)
    }
}