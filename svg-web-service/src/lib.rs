pub mod config;
mod web_handler;

use crate::config::Config;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use color_eyre::Result;
use tracing::info;
use tracing_actix_web::TracingLogger;
use web_handler::app_config;

pub fn app_run(conf: Config) -> Result<Server, std::io::Error> {
    info!("starting server at http://{}:{}", conf.host, conf.port);

    let server = HttpServer::new(move || App::new().wrap(TracingLogger).configure(app_config))
        .bind(format!("{}:{}", conf.host, conf.port))?
        .run();
    Ok(server)
}
