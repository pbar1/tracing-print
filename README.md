# tracing-print

[![Crates.io Version](https://img.shields.io/crates/v/tracing-print)](https://crates.io/crates/tracing-print)
[![docs.rs](https://img.shields.io/docsrs/tracing-print)](https://docs.rs/tracing-print)
[![CI](https://github.com/pbar1/tracing-print/actions/workflows/ci.yml/badge.svg)](https://github.com/pbar1/tracing-print/actions/workflows/ci.yml)

A simple `println`-like format for `tracing-subscriber`.

Prints the message field of an event and no others. Formats the message
according to log level:

- `ERROR`: red
- `WARN`: yellow
- `INFO`: no formatting
- `DEBUG`: blue
- `TRACE`: dim

### Usage

```rust
let layer = tracing_subscriber::fmt::layer()
    .event_format(tracing_print::Print::default());
```

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
    "formatting".italic()
);
tracing::error!("{}", "applies before level".underline())
```

Looks like this:

<pre>
<font color="#9D0006">error</font>
<font color="#81550E">warn</font>
info
<font color="#076678">debug</font>
<font color="#938974">trace</font>

only message will be seen
newlines and formatting work
see? display=1337, debug=Dummy {
    _field: (),
}
<b>extra</b> <u>ansi</u> <i>formatting</i>
<font color="#9D0006"><s>applies before level</s></font>
</pre>
