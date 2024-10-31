# tracing-print

![Crates.io Version](https://img.shields.io/crates/v/tracing-print)
![docs.rs](https://img.shields.io/docsrs/tracing-print)

A simple `println`-like format for `tracing-subscriber`.

Prints the message field of an event and no others. Formats the message
according to log level:

- `ERROR`: red
- `WARN`: yellow
- `INFO`: no formatting
- `DEBUG`: blue
- `TRACE`: dim

### Example

Run this example: `cargo run --example simple`

```rust
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
    "formatting".underline()
);
tracing::error!("{}", "applies before level".underline())
```

Looks like this:

![tracing-print example output](https://raw.githubusercontent.com/pbar1/tracing-print/refs/heads/main/.github/assets/example.png)
