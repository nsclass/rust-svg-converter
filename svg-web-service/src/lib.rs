pub mod config;
mod web_handler;

use crate::config::Config;
use actix_web::dev::Server;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;
use web_handler::app_config;

pub fn run(conf: Config) -> Result<Server, std::io::Error> {
    info!("starting server at http://{}:{}", conf.host, conf.port);

    let server = HttpServer::new(move || App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", conf.host, conf.port))?
        .run();
    Ok(server)
}
