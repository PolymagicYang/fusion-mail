use tracing::{Subscriber, subscriber::set_global_default, dispatcher::SetGlobalDefaultError};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formartting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formartting_layer)
}

pub fn init_tracing() -> Result<(), SetGlobalDefaultError> {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(get_subscriber(
        "fusion-mail".to_string(),
        "info".to_string(),
    ))
}