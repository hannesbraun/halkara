use std::io::Read;

use super::OrderedTrack;
use serde::Deserialize;

use super::{get_api, TrackGroup, APP_NAME};

#[derive(Deserialize)]
pub struct Track {
    id: String,
    pub title: String,
    pub user: User,
    pub duration: u32,
}

#[derive(Deserialize)]
pub struct TracksResponse {
    data: Vec<Track>,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

impl TracksResponse {
    pub fn track_group(self) -> TrackGroup {
        TrackGroup {
            tracks: self
                .data
                .into_iter()
                .enumerate()
                .map(|(i, track)| OrderedTrack {
                    index: i + 1,
                    track,
                })
                .collect(),
            name: "Single track".to_string(),
        }
    }
}

impl Track {
    pub fn get_stream(&self) -> Result<Vec<u8>, ureq::Error> {
        let api = get_api();

        // Get stream
        let stream_url = api + "tracks/" + &self.id + "/stream";
        let resp = ureq::get(&stream_url).query("app_name", APP_NAME).call()?;
        let mut bytes = Vec::with_capacity(self.duration as usize * 320 / 8);
        resp.into_reader().read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    pub fn get_duration(&self) -> String {
        format!("{}:{:0>2}", self.duration / 60, self.duration % 60)
    }
}
