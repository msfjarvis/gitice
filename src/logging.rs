use tracing::dispatcher::SetGlobalDefaultError;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, registry, Layer};

fn configure_tracing(filter: Targets) -> Result<(), SetGlobalDefaultError> {
    let registry = registry();
    let stdout_log = fmt::layer().compact();
    let subscriber = registry.with(stdout_log.with_filter(filter));
    set_global_default(subscriber)
}

pub(crate) fn init() -> Result<(), SetGlobalDefaultError> {
    let tracing_filter = Targets::new().with_target("gitice", Level::TRACE);
    configure_tracing(tracing_filter)
}
