use bytes::Bytes;
use serde::Deserialize;

use super::{get_api, APP_NAME};

#[derive(Deserialize)]
pub struct Track {
    id: String,
    pub title: String,
    pub user: User,
    duration: u32,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

impl Track {
    pub fn get_stream(&self) -> Option<Bytes> {
        let api = get_api();

        // Get stream
        let stream_url = api + "tracks/" + &self.id + "/stream?app_name=" + APP_NAME;
        reqwest::blocking::get(stream_url)
            .unwrap()
            .bytes()
            .map_or_else(
                |err| {
                    eprintln!("Error: {}", err.to_string());
                    None
                },
                |bytes| Some(bytes),
            )
    }

    pub fn get_duration(&self) -> String {
        format!("{}:{:0>2}", self.duration / 60, self.duration % 60)
    }
}
