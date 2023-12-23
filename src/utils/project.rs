use std::collections::HashMap;
use crate::routes;
use crate::utils::tokens::service::{Service, Services};
use crate::utils::tokens::spotify::Spotify;

pub struct Project {
    config: Config,
    services: HashMap<Services, Box<dyn Service>>,
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
            services: HashMap::new(),
        }
    }

    async fn register_services() -> HashMap<Services, Box<dyn Service>> {
        let mut services = HashMap::new();

        services.insert(Services::SPOTIFY, Box::new(Spotify::new().await));

        services
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
        use actix_web::{App, HttpServer};

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
