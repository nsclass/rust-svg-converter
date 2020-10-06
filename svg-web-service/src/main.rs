// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use color_eyre::Result;
use svg_web_service::app_run;
use svg_web_service::config::Config;

#[actix_rt::main]
async fn main() -> Result<()> {
    let conf = Config::from_env()?;
    let _ = app_run(conf)?.await;

    Ok(())
}
