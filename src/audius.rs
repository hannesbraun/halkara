use std::sync::RwLock;
use std::time::Instant;

use lazy_static::lazy_static;
use serde::Deserialize;

use playlist::PlaylistResponse;
use track::{Track, TrackResponse, TracksResponse};
use user::UserResponse;

mod playlist;
pub mod track;
pub mod trending;
mod user;

#[derive(Deserialize)]
struct ApiResponse {
    data: Vec<String>,
}

pub struct OrderedTrack {
    pub track: Track,
    pub index: usize,
}

pub struct TrackGroup {
    pub tracks: Vec<OrderedTrack>,
    pub name: String,
}

struct ApiCache {
    url: String,
    timestamp: Option<Instant>,
}

lazy_static! {
    static ref API_CACHE: RwLock<ApiCache> = RwLock::new(ApiCache {
        url: String::new(),
        timestamp: None,
    });
}

const APP_NAME: &str = "Halkara";

fn get_api() -> String {
    let mut update_cache = true;
    let mut url = String::new();
    if let Ok(cache) = API_CACHE.read() {
        if let Some(timestamp) = cache.timestamp {
            if Instant::now().duration_since(timestamp).as_secs() < 3600 {
                update_cache = false;
                url = cache.url.clone();
            }
        }
    }

    if update_cache {
        let api_res: ApiResponse = ureq::get("https://api.audius.co")
            .call()
            .unwrap()
            .into_json()
            .unwrap();
        let mut endpoints = api_res.data.into_iter();

        let mut works = false;
        while !works {
            // Select endpoint
            url = endpoints.next().expect("Selecting API endpoint") + "/v1/";

            // Test endpoint
            let resp_result = ureq::get(format!("{}tracks/QxamW", url).as_str())
                .query("app_name", APP_NAME)
                .call();
            if let Ok(resp) = resp_result {
                let track: Result<TrackResponse, _> = resp.into_json();
                if track.is_ok() {
                    works = true;
                }
            }
        }

        if let Ok(mut cache) = API_CACHE.write() {
            // Cache for next call
            cache.url = url.clone();
            cache.timestamp = Some(Instant::now());
        }
    }

    url
}

pub fn resolve(url: &str) -> Result<Vec<TrackGroup>, String> {
    let api = get_api();
    let resp = ureq::get(format!("{}resolve", api).as_str())
        .query("app_name", APP_NAME)
        .query("url", url)
        .call()
        .unwrap_or_else(|_| panic!("Unable to execute GET request for {}", url))
        .into_string()
        .expect("Extracting string from resolve response");

    if let Ok(playlist_response) = ureq::serde_json::from_str::<PlaylistResponse>(&resp) {
        Ok(playlist_response.track_groups())
    } else if let Ok(tracks_response) = ureq::serde_json::from_str::<TracksResponse>(&resp) {
        Ok(vec![tracks_response.track_group()])
    } else if let Ok(user_response) = ureq::serde_json::from_str::<UserResponse>(&resp) {
        Ok(vec![user_response.track_group()])
    } else {
        Err(format!("Unable to resolve {}", url))
    }
}
