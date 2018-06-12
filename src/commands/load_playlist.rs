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
            .playlists
            .read()
            .unwrap()
            .iter()
            .find(|playlist| playlist.title == self.name)
            .cloned()
            .unwrap()
            .tracks;
        let mut player = app.player.lock().unwrap();
        player.queue.add_multiple(&tracks);
        Ok(())
    }
}