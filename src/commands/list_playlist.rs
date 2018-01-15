use error::MpdError;
use commands::MpdCommand;
use rustic_core::{Rustic, Track};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct PlaylistItem {
    file: String
}

impl From<Track> for PlaylistItem {
    fn from(track: Track) -> PlaylistItem {
        PlaylistItem {
            file: track.uri
        }
    }
}

pub struct ListPlaylistCommand {
    name: String
}

impl ListPlaylistCommand {
    pub fn new(name: String) -> ListPlaylistCommand {
        ListPlaylistCommand {
            name
        }
    }
}

impl MpdCommand<Vec<PlaylistItem>> for ListPlaylistCommand {
    fn handle(&self, app: &Arc<Rustic>) -> Result<Vec<PlaylistItem>, MpdError> {
        let playlists = app
            .library
            .playlists
            .read()
            .unwrap();
        let playlist = playlists
            .iter()
            .find(|playlist| playlist.title == self.name);
        match playlist {
            Some(playlist) => {
                let tracks = playlist.tracks
                    .iter()
                    .cloned()
                    .map(PlaylistItem::from)
                    .collect();
                Ok(tracks)
            },
            None => Ok(vec![])
        }
    }
}