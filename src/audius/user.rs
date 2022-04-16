use super::track::TracksResponse;
use super::{get_api, OrderedTrack, TrackGroup, APP_NAME};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UserResponse {
    pub data: User,
}

impl UserResponse {
    pub fn track_group(self) -> TrackGroup {
        let api = get_api();
        let tracks_url = format!("{}users/{}/tracks", api, self.data.id);
        let tracks_response: TracksResponse = ureq::get(&tracks_url)
            .query("app_name", APP_NAME)
            .call()
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to execute GET request for user {}",
                    self.data.name
                )
            })
            .into_json()
            .unwrap_or_else(|_| {
                panic!(
                    "Unable to deserialize the tracks for user {}",
                    self.data.name
                )
            });
        TrackGroup {
            tracks: tracks_response
                .data
                .into_iter()
                .enumerate()
                .map(|(i, track)| OrderedTrack {
                    index: i + 1,
                    track,
                })
                .collect(),
            name: self.data.name,
        }
    }
}
