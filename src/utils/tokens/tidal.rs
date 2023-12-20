use base64::{engine::{general_purpose}, Engine as _};
use std::env;
use serde::Deserialize;
use crate::utils::utils::get_env;

#[derive(Deserialize)]
struct TidalResponse {
    access_token: String,
    // token_type: String,
    // expires_in: u16,
}

pub async fn get_token() -> String {
    let client_id = get_env("SPOTIFY_CLIENT_ID");
    let client_secret = get_env("SPOTIFY_CLIENT_SECRET");

    let auth = general_purpose::STANDARD.encode(&format!("{client_id}:{client_secret}"));

    let data = [("grant_type", "client_credentials")];

    let client = crate::utils::utils::get_fetch_client();
    let response = client.post("https://auth.tidal.com/v1/oauth2/token")
        .header("Authorization", format!("Basic {auth}"))
        .form(&data)
        .send()
        .await;

    match response {
        Ok(res) => {

            let data: Result<SpotifyResponse, _> = res.json().await;

            data.unwrap().access_token
        },
        Err(_) => {
            panic!("Invalid response. (Spotify token)")
        }
    }
}
