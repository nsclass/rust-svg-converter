mod web_handler;

use std::io;

use actix_web::{middleware, App, HttpServer};

use crate::web_handler::{health, svg_conversion};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(health::health)
            .service(svg_conversion::svg_convert)
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
