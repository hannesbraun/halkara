use std::io::Read;

use serde::Deserialize;

use super::{get_api, APP_NAME};

#[derive(Deserialize)]
pub struct Track {
    id: String,
    pub title: String,
    pub user: User,
    pub duration: u32,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
}

impl Track {
    pub fn get_stream(&self) -> Result<Vec<u8>, ureq::Error> {
        let api = get_api();

        // Get stream
        let stream_url = api + "tracks/" + &self.id + "/stream?app_name=" + APP_NAME;
        let resp = ureq::get(&stream_url).call()?;
        let mut bytes = Vec::with_capacity(self.duration as usize * 320 / 8);
        resp.into_reader().read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    pub fn get_duration(&self) -> String {
        format!("{}:{:0>2}", self.duration / 60, self.duration % 60)
    }
}
