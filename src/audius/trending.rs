use super::track::Track;
use super::{get_api, OrderedTrack, TrackGroup, APP_NAME};
use serde::Deserialize;

#[derive(Deserialize)]
struct TrendingResponse {
    data: Vec<Track>,
}

pub fn get_trending(genre: &str, time: &str) -> TrackGroup {
    // Select API endpoint
    let api = get_api();

    // Get trending tracks
    let trending_url = format!("{}tracks/trending", api);
    let mut request = ureq::get(&trending_url).query("app_name", APP_NAME);
    if !genre.is_empty() {
        request = request.query("genre", genre);
    }
    if !time.is_empty() {
        request = request.query("time", time);
    }
    let trending_res: TrendingResponse = request
        .call()
        .expect("Unable to execute GET request for list of trending tracks")
        .into_json()
        .expect("Unable to deserialize the list of trending tracks");

    // Enrich with the track's rank
    let mut trending_tracks = Vec::with_capacity(100);
    let mut rank = 1;
    for track in trending_res.data {
        trending_tracks.push(OrderedTrack { track, index: rank });
        rank += 1;
    }

    let name = match time {
        "month" => String::from("Trending tracks of this month"),
        "allTime" => String::from("Trending tracks of all time"),
        _ => String::from("Trending tracks of this week"),
    };

    TrackGroup {
        tracks: trending_tracks,
        name,
    }
}
