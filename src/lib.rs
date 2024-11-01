#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(clippy::pedantic)]

use std::fmt;
use std::marker::PhantomData;

use tracing::field::Visit;
use tracing::Event;
use tracing::Level;
use tracing_subscriber::fmt::format::FormatEvent;
use tracing_subscriber::fmt::format::FormatFields;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::registry::LookupSpan;

/// Simple println format for `tracing-subscriber`. Prints the message field of
/// an event and no others.
///
/// Formats the message according to log level:
/// - `ERROR`: red
/// - `WARN`: yellow
/// - `INFO`: no formatting
/// - `DEBUG`: blue
/// - `TRACE`: dim
#[derive(Default)]
pub struct Print {
    inner: PhantomData<()>,
}

impl<S, N> FormatEvent<S, N> for Print
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut visitor = MessageVisitor { message: None };
        event.record(&mut visitor);
        let message = visitor.message.ok_or(fmt::Error)?;

        #[cfg(feature = "ansi")]
        if writer.has_ansi_escapes() {
            let message = apply_color(*event.metadata().level(), &message);
            return writeln!(writer, "{message}");
        }

        writeln!(writer, "{message}")
    }
}

#[cfg(feature = "ansi")]
fn apply_color(level: Level, message: &str) -> yansi::Painted<&str> {
    use yansi::Paint;

    match level {
        Level::ERROR => message.red(),
        Level::WARN => message.yellow(),
        Level::INFO => yansi::Painted::new(message),
        Level::DEBUG => message.blue(),
        Level::TRACE => message.dim(),
    }
}

/// A visitor to capture only the `message` field.
pub struct MessageVisitor {
    message: Option<String>,
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{value:?}"));
        }
    }
}
