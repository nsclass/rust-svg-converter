// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use color_eyre::Result;
use svg_web_service::run;

#[actix_rt::main]
async fn main() -> Result<()> {
    let (server, _) = run()?;
    let _ = server.await;

    Ok(())
}
