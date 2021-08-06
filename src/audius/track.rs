use bytes::{Buf, Bytes};
use bytes::buf::Reader;
use serde::Deserialize;

use super::{APP_NAME, get_api};

#[derive(Deserialize)]
pub struct Track {
    id: String,
    repost_count: u32,
    favorite_count: u32,
    pub title: String,
    pub user: User,
    duration: u32,
    play_count: u32,
}

#[derive(Deserialize)]
pub struct User {
    album_count: u32,
    followee_count: u32,
    follower_count: u32,
    handle: String,
    id: String,
    is_verified: bool,
    pub name: String,
    playlist_count: u32,
    repost_count: u32,
    track_count: u32,
}

impl Track {
    pub fn get_stream(self) -> Reader<Bytes> {
        let api = get_api();

        // Get stream
        let stream_url = api + "tracks/" + &self.id + "/stream?app_name=" + APP_NAME;
        reqwest::blocking::get(stream_url).unwrap().bytes().unwrap().reader()
    }

    pub fn get_duration(&self) -> String {
        format!("{}:{:0>2}", self.duration / 60, self.duration % 60)
    }
}
