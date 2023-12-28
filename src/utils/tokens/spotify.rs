use async_trait::async_trait;
use chrono::NaiveDateTime;
use reqwest::{Method, RequestBuilder};
use serde::Deserialize;
use crate::models::track::{Track};
use crate::utils::tokens::service::Service;
use crate::utils::utils::{get_env, get_fetch_client};
use crate::models::spotify::Track as SpotifyTrack;

#[derive(Deserialize)]
struct SpotifyResponse {
    access_token: String,
    // token_type: String,
    // expires_in: u16,
}

pub(crate) struct Spotify {
    token: String,
}

impl Spotify {
    pub async fn new() -> Spotify {
        let mut spotify = Spotify {
            token: String::new()
        };
        spotify.fetch_token().await;
        spotify
    }

    fn api_builder(&self, url: &str, method: Method) -> RequestBuilder {
        let client = get_fetch_client();

        let api_url = &format!("https://api.spotify.com/v1{url}");

        client.request(method, api_url)
            .bearer_auth(self.get_token())
    }
}

#[async_trait]
impl Service for Spotify {

    fn get_token(&self) -> &String {
        &self.token
    }

    async fn fetch_token(&mut self) -> &String {
        log::info!("Fetching token...");

        let client_id = get_env("SPOTIFY_CLIENT_ID");
        let client_secret = get_env("SPOTIFY_CLIENT_SECRET");

        let client = get_fetch_client();
        let response = client.post("https://accounts.spotify.com/api/token")
            .basic_auth(client_id, Some(client_secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await;

        match response {
            Ok(res) => {
                let data: Result<SpotifyResponse, _> = res.json().await;

                self.token = data.unwrap().access_token.to_string();
                // println!("{}", self.token);
                &self.token
            },
            Err(error) => {
                log::error!("{}", error.to_string());

                // TODO: Handle invalid responses from Spotify
                panic!("invalid response (Spotify Token)")
            }
        }
    }

    async fn get_track_by_isrc(&self, isrc: &str) -> Track {

        #[derive(Deserialize)]
        struct APIResponse {
            tracks: APIResponseTracks
        }

        #[derive(Deserialize)]
        struct APIResponseTracks {
            items: Vec<SpotifyTrack>
        }

        let response = self.api_builder("/search", Method::GET)
            .query(&[("type", "track"), ("q", format!("isrc:{isrc}"))[..]]);

        match response.send().await {
            Ok(response) => {
                let track_data = response.json::<APIResponse>().await.unwrap();

                let track = &track_data.tracks.items[0];

                Track::from(Track {
                    title: String::from(&track.name),
                    duration_ms: track.duration_ms,
                    isrc: String::from(&track.external_ids.isrc),
                    last_fetched: NaiveDateTime::default(),
                    spotify_id: None,
                    tidal_id: None,
                    artists: Vec::from(&track.artists),
                })
            }
            Err(_) => {
                panic!("There was an error getting a track")
            }
        }
    }
}
