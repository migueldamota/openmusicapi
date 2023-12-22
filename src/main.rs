mod models;
mod routes;
mod utils;

use dotenv::dotenv;
use crate::utils::project::{Config, Project};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load environment variables.
    dotenv().ok();

    let project = Project::new(Config {
        host: String::from("127.0.0.1"),
        port: 9000,
    });

    // Register music services
    // project.register_services().await;

    project.listen().await
}
