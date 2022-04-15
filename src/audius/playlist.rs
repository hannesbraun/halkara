use serde::Deserialize;

use super::track::Track;
use super::{get_api, APP_NAME};
use super::{OrderedTrack, TrackGroup};

#[derive(Deserialize)]
struct Playlist {
    id: String,
    pub playlist_name: String,
}

#[derive(Deserialize)]
pub struct PlaylistResponse {
    data: Vec<Playlist>,
}

#[derive(Deserialize)]
struct PlaylistTracksResponse {
    data: Vec<Track>,
}

impl PlaylistResponse {
    pub fn track_groups(self) -> Vec<TrackGroup> {
        let api = get_api();
        let mut track_groups = Vec::new();
        for playlist in self.data.into_iter() {
            let playlist_tracks_url = format!("{}playlists/{}/tracks", api, playlist.id);
            let playlist_tracks_response: PlaylistTracksResponse = ureq::get(&playlist_tracks_url)
                .query("app_name", APP_NAME)
                .call()
                .unwrap_or_else(|_| {
                    panic!(
                        "Unable to execute GET request for playlist {}",
                        playlist.playlist_name
                    )
                })
                .into_json()
                .unwrap_or_else(|_| {
                    panic!(
                        "Unable to deserialize the tracks for playlist {}",
                        playlist.playlist_name
                    )
                });
            track_groups.push(TrackGroup {
                tracks: playlist_tracks_response
                    .data
                    .into_iter()
                    .enumerate()
                    .map(|(i, track)| OrderedTrack {
                        index: i + 1,
                        track,
                    })
                    .collect(),
                name: playlist.playlist_name,
            });
        }

        track_groups
    }
}
