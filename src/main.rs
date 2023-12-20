mod models;
mod routes;
mod utils;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App,HttpServer};

    // load environment variables.
    dotenv().ok();

    println!("Started: http://localhost:8080");

    // Get access tokens.
    // let spotify_token = utils::tokens::spotify::get_token().await;

    HttpServer::new(||
        App::new()
            .service(routes::search::search)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
