// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod health;
mod index;
mod svg_conversion;

use health::*;
use index::*;
use svg_conversion::*;

use actix_files::Files;
use actix_web::web::{self, ServiceConfig};

pub fn app_config(config: &mut ServiceConfig) {
    config
        .route("/svg/conversion", web::put().to(svg_convert))
        .route("/health", web::get().to(health))
        .route("/", web::get().to(single_page_app))
        .service(Files::new("/", "./react-ui/build").index_file("index.html"));
}
