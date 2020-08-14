mod health;
mod svg_conversion;

use health::*;
use svg_conversion::*;

use actix_web::web::ServiceConfig;

pub fn app_config(config: &mut ServiceConfig) {
    config.service(health).service(svg_convert);
}
