mod utils;
use std::net::{IpAddr, SocketAddr};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    http,
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use mongodb::Client;
use ziply::routes::{handle_get, shorten_url};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let default_domain = std::env::var("DEFAULT_DOMAIN").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    // Parse the IP address string into an IpAddr
    let ip_addr: IpAddr = default_domain.parse().unwrap();
    // Parse the port string into a u16
    let port_num: u16 = port.parse().unwrap();
    // Create a SocketAddr by combining the IP address and port
    let socket_addr = SocketAddr::new(ip_addr, port_num);

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    log::info!("starting HTTP server at http://zip.ly:8080");
    HttpServer::new(move || {
        //Adding Cors
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().starts_with(b"http://zip.ly"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(shorten_url)
            .route("/{urlId}", web::get().to(handle_get))
            // Add react static file
            .service(Files::new("/", "./src/client/build").index_file("index.html"))
            .app_data(web::Data::new(client.clone()))
            .wrap(Logger::default())
    })
    .bind(socket_addr)?
    .run()
    .await
}
