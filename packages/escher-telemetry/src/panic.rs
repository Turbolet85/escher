use std::panic::{self, PanicHookInfo};

/// Sets a panic hook that logs the panic as one error event, then runs the hook it replaced.
pub(crate) fn install() {
    let previous = panic::take_hook();
    panic::set_hook(Box::new(move |info: &PanicHookInfo<'_>| {
        let (file, line, column) = info
            .location()
            .map_or(("<unknown>", 0, 0), |l| (l.file(), l.line(), l.column()));
        let payload = info.payload_as_str().unwrap_or("<non-string payload>");
        tracing::event!(
            target: "escher_telemetry::panic",
            tracing::Level::ERROR,
            {
                panic.file = file,
                panic.line = line,
                panic.column = column,
                panic.payload = payload,
            },
            "panic"
        );
        previous(info);
    }));
}
