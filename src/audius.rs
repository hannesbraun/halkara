use std::sync::RwLock;
use std::time::Instant;

use lazy_static::lazy_static;
use serde::Deserialize;

use track::Track;

pub mod track;

#[derive(Deserialize)]
struct ApiResponse {
    data: Vec<String>,
}

#[derive(Deserialize)]
struct TrendingResponse {
    data: Vec<Track>,
}

pub struct TrendingTrack {
    pub track: Track,
    pub rank: u8,
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
        let api_res = reqwest::blocking::get("https://api.audius.co").unwrap().json::<ApiResponse>().unwrap();
        url = String::from(api_res.data.first().unwrap()) + "/v1/";

        if let Ok(mut cache) = API_CACHE.write() {
            // Cache for next call
            cache.url = url.clone();
            cache.timestamp = Some(Instant::now());
        }
    }

    return url;
}

pub fn get_trending(genre: &str, time: &str) -> Vec<TrendingTrack> {
    // Select API endpoint
    let api = get_api();

    // Get trending tracks
    let genre_param = if genre.is_empty() { String::new() } else { String::from("&genre=") + genre };
    let time_param = if time.is_empty() { String::new() } else { String::from("&time=") + time };
    let trending_url = api + "tracks/trending?app_name=" + APP_NAME + &genre_param + &time_param;
    let trending_res = reqwest::blocking::get(trending_url).unwrap().json::<TrendingResponse>().unwrap();

    // Enrich with the track's rank
    let mut trending_tracks = Vec::new();
    let mut rank = 1u8;
    for track in trending_res.data {
        trending_tracks.push(TrendingTrack {
            track,
            rank,
        });
        rank += 1;
    }

    return trending_tracks;
}
