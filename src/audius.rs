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

const APP_NAME: &str = "Halkara";

fn get_api() -> String {
    let api_res = reqwest::blocking::get("https://api.audius.co").unwrap().json::<ApiResponse>().unwrap();
    return String::from(api_res.data.first().unwrap()) + "/v1/";
}

pub fn get_trending(genre: &str, time: &str) -> Vec<Track> {
    // Select API endpoint
    let api = get_api();

    // Get trending tracks
    let genre_param = if genre.is_empty() { String::new() } else { String::from("&genre=") + genre };
    let time_param = if time.is_empty() { String::new() } else { String::from("&time=") + time };
    let trending_url = api + "tracks/trending?app_name=" + APP_NAME + &genre_param + &time_param;
    let trending_res = reqwest::blocking::get(trending_url).unwrap().json::<TrendingResponse>().unwrap();

    return trending_res.data;
}
