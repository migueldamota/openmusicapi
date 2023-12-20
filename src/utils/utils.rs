use reqwest::Client;

pub fn get_fetch_client() -> Client {
    Client::new()
}

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap()
}
