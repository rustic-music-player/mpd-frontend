use commands::MpdCommand;
use failure::Error;
use rustic_core::Rustic;
use song::MpdSong;
use std::sync::Arc;

pub struct CurrentSongCommand {}

impl CurrentSongCommand {
    pub fn new() -> CurrentSongCommand {
        CurrentSongCommand {}
    }
}

impl MpdCommand<Option<MpdSong>> for CurrentSongCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Option<MpdSong>, Error> {
        let track = app.player.current().map(|track| MpdSong::from(track));
        Ok(track)
    }
}
