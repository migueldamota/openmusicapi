use crate::models::track::{Track};
use async_trait::async_trait;

#[async_trait]
pub trait Service: Send + Sync {
	fn get_token(&self) -> &String;
	async fn fetch_token(&mut self) -> &String;
	async fn get_track_by_isrc(&self, isrc: &str) -> Track;

	// fn search(self, string: String) -> Vec<Track>;
}

#[derive(Hash, Eq, PartialEq)]
pub enum Services {
	Spotify,
	// Tidal,
}
