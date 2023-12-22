use std::collections::HashMap;
use crate::routes;
use crate::utils::tokens::service::{Services};
use crate::utils::tokens::spotify::Spotify;

pub struct Project {
    config: Config,
    services: HashMap<Services, Spotify>,
}

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub struct AppState {
    pub project: Project,
}

impl Project {

    pub fn new(config: Config) -> Self {

        Project {
            config,
            services: HashMap::with_capacity(4),
        }
    }

    pub async fn register_services(&mut self) {
        // Spotify
        let mut spotify = Spotify::new();
        spotify.fetch_token().await;
        self.services.insert(Services::SPOTIFY, spotify);

        // Tidal

        // Apple Music
        // services.insert(Services::TIDAL, Tidal::new());
    }

    pub fn get_service(&self, service: &Services) -> &Spotify {
        if !self.services.contains_key(service) {
            panic!("Service does not exist")
        }

        self.services.get(service).unwrap()
    }

    pub fn get_config(self) -> Config {
        self.config
    }

    pub async fn listen(self) -> std::io::Result<()> {
        use actix_web::{App, HttpServer, web};

        let config = self.config.clone();

        HttpServer::new(||
            App::new()
                .service(routes::search::search)
        )
            .bind((config.host, config.port))?
            .run()
            .await
    }
}
