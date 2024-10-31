use tracing::level_filters::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_print::Print;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;
use yansi::Paint;

#[derive(Debug, Default)]
struct Dummy {
    _field: (),
}

fn main() {
    let layer = tracing_subscriber::fmt::layer()
        .event_format(Print::default())
        .with_ansi(false)
        .with_filter(LevelFilter::TRACE);

    let registry = Registry::default().with(layer);
    set_global_default(registry).unwrap();

    tracing::error!("error");
    tracing::warn!("warn");
    tracing::info!("info");
    tracing::debug!("debug");
    tracing::trace!("trace");

    // Functions as a newline
    tracing::info!("");

    // Other keys than message won't be displayed
    tracing::info!(key = "unseen", "only message will be seen");

    // Format macros work like normal
    tracing::info!(
        "newlines and formatting work\nsee? display={}, debug={:#?}",
        1337,
        Dummy::default()
    );

    // Extra ANSI colors can also be applied
    tracing::info!(
        "{} {} {}",
        "extra".bold(),
        "ansi".underline(),
        "formatting".italic()
    );
    tracing::error!("{}", "applies before level".strike())
}
