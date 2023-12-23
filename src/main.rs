mod models;
mod routes;
mod utils;

use dotenv::dotenv;
use crate::utils::project::{Config, Project};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // load environment variables.
    dotenv().ok();

    let project = Project::new(Config {
        host: String::from("127.0.0.1"),
        port: 9000,
    });

    project.listen().await
}
