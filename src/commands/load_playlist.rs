use failure::Error;
use commands::MpdCommand;
use rustic_core::Rustic;
use std::sync::Arc;

pub struct LoadPlaylistCommand {
    name: String
}

impl LoadPlaylistCommand {
    pub fn new(name: String) -> LoadPlaylistCommand {
        LoadPlaylistCommand {
            name
        }
    }
}

impl MpdCommand<()> for LoadPlaylistCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<(), Error> {
        let tracks = app
            .library
            .get_playlists()?
            .iter()
            .find(|playlist| playlist.title == self.name)
            .cloned()
            .unwrap()
            .tracks;
        app.player.queue_multiple(&tracks);
        Ok(())
    }
}