
extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod web_handler;

use actix_web::{http, server, App, HttpRequest, Path, Json, State, Responder};
use actix_web::middleware::Logger;

use web_handler::*;

fn create_app() -> App<()> {
    App::new()
        .middleware(Logger::default())
        .resource("/health", |r| r.f(health))

        .resource("/svg/conversion", |r| {
            r.method(http::Method::PUT).with_config(svg_convert, |(cfg,)| {
                cfg.limit(4096);
            })
        })
}

fn main() {
    server::new(|| create_app())
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
