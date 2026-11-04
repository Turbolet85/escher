@carried n=1 label=security-plan/logging-monitoring fact=fct-4hqe6pzr
why: the probe's test or example code runs under escher's own sink and adds no log field or span, so the sink's allowlist rule has to hold unchanged: an escher target's content-named fields redacted, any other target dropped whole.
quote: an event whose target starts with `escher_` prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted; an event from any other target is not printed at all — dropped whole, zero bytes, at every level, WARN and ERROR included, and whatever `RUST_LOG` names
@carried n=2 label=security-plan/logging-monitoring fact=fct-4hqe6pzr
why: the MountedData panic the probe re-measures reaches stderr through the unscrubbed panic hook, outside the sink, so no probe-read value (a component name, a handler, a signal's value) may be put into a panic message or a probe assertion text.
quote: Its reach is that sink only: the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed, the chained std panic hook still prints the raw panic message to stderr
@carried n=3 label=security-plan/logging-monitoring fact=fct-ksl3fslx
why: the probe adds no span or span field, so these counts must still read the same at the chunk's end.
quote: `packages/escher-driver/src` holds two `info_span!`, `stand_act_spans` reads eight span fields
@carried n=4 label=security-plan/logging-monitoring fact=fct-uh4dhzjw
why: a probe check lives in `tests/blitz-tests/`, where tests print diagnostics to stdout, so the probe's test must print, log or put in a field nothing it reads; it states only verdicts, counts and closed words.
quote: tests print diagnostics with `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/text_selection_anonymous_block.rs:107)
@carried n=5 label=security-plan fact=fct-wkixw6vk
why: an «exposed with a cost» change inside `dioxus-native-dom` falls under the engine-crate tracing gate, so it must add no `tracing` site, field or span there.
quote: dioxus-native's `tracing` feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net (packages/dioxus-native/Cargo.toml:69; packages/dioxus-native-dom/Cargo.toml:19)
@binding n=1
**escher's sink ↔ the scope's «Nothing crosses»**: the scope bars any new log field or span and any printing or logging of what the probe reads. The sink's rule and the span counts are the values that must stay as they are (security-plan/logging-monitoring fct-4hqe6pzr) (security-plan/logging-monitoring fct-ksl3fslx).
@binding n=2
**the panic hook's reach ↔ Error Handling's dioxus-native-dom panic paths**: the MountedData `RefCell already borrowed` panic is printed raw by the panic hook (security-plan/logging-monitoring fct-4hqe6pzr). Error Handling records `try_doc`'s `None` on a mutable borrow beside `NodeHandle`'s panics (security-plan fct-c4rr3jfv) (security-plan fct-f4v2lnec). The reading names the panic's mechanism and coordinates only, and leaves the fix's options to the founder.
@binding n=3
**engine-crate tracing ↔ the scope's no-engine-change boundary**: any escher-side change in `dioxus-native-dom` adds no tracing site behind the feature gate (security-plan fct-wkixw6vk) (security-plan fct-u3c7wcoc).
