// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument, Subscriber};
use tracing_subscriber::{EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;

/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name.into(),
        std::io::stdout
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) -> Result<()> {
    LogTracer::init().context("set a logger")?;
    set_global_default(subscriber).context("set a global subscriber")?;
    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let subscriber = get_subscriber("svg_converter".into(), "info".into());
        init_subscriber(subscriber)?;

        info!("loading configuration");
        let mut cfg = config::Config::new();

        cfg.merge(config::Environment::default())?;
        cfg.try_into()
            .context("loading configuration from environment")
    }
}
