use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web};
use crate::routes;
use crate::utils::tokens::service::{Service, Services};
use crate::utils::tokens::spotify::Spotify;

pub struct Project {
    config: Config,
}

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub struct AppData {
    pub services: Mutex<HashMap<Services, Box<dyn Service>>>
}

impl Project {

    pub fn new(config: Config) -> Self {

        Project {
            config,
        }
    }

    async fn register_services() -> HashMap<Services, Box<dyn Service>> {
        let mut services: HashMap<Services, Box<dyn Service>> = HashMap::new();

        services.insert(Services::Spotify, Box::new(Spotify::new().await));

        services
    }

    pub async fn listen(self) -> std::io::Result<()> {
        use actix_web::{App, HttpServer};

        let config = self.config.clone();

        let services = Project::register_services().await;
        let data = web::Data::new(AppData {
            services: Mutex::new(services)
        });

        HttpServer::new(move || {
            App::new()
                // .wrap(
                //     ErrorHandlers::new()
                //         .default_handler(Project::error_handler)
                // )
                .app_data(data.clone())
                .service(routes::tracks::get_track)
        })
            .bind((config.host, config.port))?
            .run()
            .await
    }

    // fn error_handler<B>(_res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    //     todo!()
    //     let (req, res) = res.into_parts();
    //
    //     let res = ServiceResponse::new(req, res)
    //         .map_into_boxed_body()
    //         .map_into_right_body();
    //
    //     Ok(ErrorHandlerResponse::Response(res))
    // }
}
