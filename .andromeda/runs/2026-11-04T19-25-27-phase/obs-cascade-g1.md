@carried n=1 label=obs-plan/telemetry-strategy fact=fct-46rgllvo
why: The probe must give the same verdicts under a workspace build, which turns blitz-dom/tracing on, and under `cargo test -p blitz-tests`, which leaves it off, so no verdict may rest on an engine tracing event.
quote: By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events: one `INFO blitz_dom::document … message=[redacted]` beside a driver `type`
@carried n=1 run=2 label=obs-plan fact=fct-upbrcclf
quote: By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on, while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events (test-plan §9 → Engine features by runner).
@carried n=1 run=3 label=obs-plan fact=fct-6oskq4op
quote: dioxus-native's `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:69)
@carried n=2 label=obs-plan/telemetry-strategy fact=fct-weudw6wt
why: The reading's statement of the MountedData hazard for the windowed path, and for any app that meets it, must hold that escher-telemetry's panic event is compiled into both seven_guis binaries, and the probe itself must add no capture of its own.
quote: escher-telemetry depends on `tracing` with no feature and always compiles its startup and panic events into the two native binaries of the seven_guis package, `seven_guis_native` and `escher-session`, whose engine `tracing` features stay off in a package-alone build (`-p seven_guis`; a workspace-wide build turns blitz-dom's on — Feature wiring, below)
@carried n=2 run=2 label=obs-plan fact=fct-fmm6lpcc
quote: escher-telemetry and escher-driver take `tracing` with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, `seven_guis_native` and `escher-session`, whose engine features stay off in a package-alone build.
@carried n=2 run=3 label=obs-plan fact=fct-cg2a6gz6
quote: `seven_guis_native` (the windowed stand binary) installs `escher_telemetry::init(escher_telemetry::service_identity!())` in the native `main` before `dioxus_native::launch`, reporting an `Err` with `eprintln!` and continuing
@carried n=3 label=obs-plan/telemetry-strategy fact=fct-tasmmgp5
why: The probe check lands in the test crate slice under tests/blitz-tests, and the scope's «Nothing crosses» boundary requires that slice to keep no telemetry or logging crate, span or trace.
quote: observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
@carried n=3 run=2 label=obs-plan/span-trace-coverage fact=fct-j47hr7dn
quote: observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
@carried n=3 run=3 label=obs-plan fact=fct-nvda2d25
quote: telemetry, tracing or logging frameworks over the reading's examples and test crate slices
@carried n=4 label=obs-plan/telemetry-strategy fact=fct-atc72w5q
why: Any probe code the plan keeps under examples/seven_guis must keep the examples slice free of a tracing framework and of spans, as the scope's «Nothing crosses» boundary requires.
quote: observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files (examples slice)
@carried n=4 run=2 label=obs-plan/span-trace-coverage fact=fct-sicgzqzx
quote: observed absent — spans · searched: `span!|instrument` over the 32 slice files (examples slice)
@carried n=4 run=3 label=obs-plan fact=fct-dfgrfrux
quote: Spans, instrumented functions and traces observed absent over the reading's apps, examples, s05, s06, layout, s09, blitz-vibey-script / blitz, s11, test crate and wpt runner slices
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: The scope's boundary adds no span or field, so escher-driver's command and tool_call spans must stay as they are; if the probe drives a stand task through Session::run, the span is disabled because no subscriber is installed, and nothing the probe reads may be fielded.
quote: never fielded — `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its byte size or its `width` / `height`, a lint's findings or their count, the session's label, the record of ids, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`: an argument fielded by its schema name, or a `Command` or an `Outcome` fielded with `Debug`, would print an id, typed text, an accessible name or a control's value (of the argument names only `text` is in the sink's scrub set, §8); the driver emits no event and installs no subscriber, so with none installed the span is disabled, under escher's sink it is one line per call at `info` or below (§6) and at the default `warn` it is filtered out before it is created
@carried n=5 run=2 label=obs-plan/span-trace-coverage fact=fct-mp6exhtv
quote: no field added, the span keeps its two
@carried n=5 run=3 label=obs-plan fact=fct-yqpbkmod
quote: its eight fields (`verb` · `settled` · `busy` · `passes` · `added` · `removed` · `changed` · `cause`) closed words, bools and counts recorded only where they apply; never fielded — the screen text, any argument, or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the driver emits no event and installs no subscriber.
@carried n=5 run=4 label=obs-plan fact=fct-wa4xq2qh
quote: two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`
@carried n=6 label=obs-plan fact=fct-icom2cq7
why: A headless stand booted in process has no escher sink, so each verdict's evidence must come from the probe's assertions, and the probe must install no sink or subscriber of its own.
quote: a headless boot made in process has no escher sink, those two children's aside;
@carried n=7 label=obs-plan fact=fct-rvanvmh2
why: The probe reads dioxus-native-dom as it is, so the crate's optional tracing and log-times features must stay unchanged and no probe reading may depend on them.
quote: **dioxus-native, dioxus-native-dom** — optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:60-69; packages/dioxus-native-dom/Cargo.toml:19)
@binding n=1
**telemetry-strategy ↔ span-trace-coverage**: the probe's verdicts must be the same under the workspace build, where blitz-dom/tracing is on, and under `cargo test -p blitz-tests`, where it is off (obs-plan/telemetry-strategy fct-46rgllvo); with no subscriber installed in process, any command span a Session::run call opens is disabled (obs-plan/span-trace-coverage fct-zuu6oxcq), so neither engine events nor driver spans may serve as a verdict's evidence.
@binding n=2
**telemetry-strategy ↔ the body (§3 Logging stack)**: the reading's hazard statement for the windowed path must hold that escher-telemetry's panic event is compiled into both seven_guis binaries (obs-plan/telemetry-strategy fct-weudw6wt) and that `seven_guis_native` installs the sink before launch (obs-plan fct-cg2a6gz6), while the in-process headless probe has no sink at all (obs-plan fct-icom2cq7).
@binding n=3
**telemetry-strategy ↔ span-trace-coverage (absences)**: the probe's test file under tests/blitz-tests and any probe code kept under examples/seven_guis must keep both slices free of a telemetry crate (obs-plan/telemetry-strategy fct-tasmmgp5) (obs-plan/telemetry-strategy fct-atc72w5q) and free of a span (obs-plan/span-trace-coverage fct-j47hr7dn) (obs-plan/span-trace-coverage fct-sicgzqzx).
@binding n=4
**telemetry-strategy ↔ the body (§1)**: dioxus-native-dom must stay a read surface: its optional tracing and log-times features stay unchanged (obs-plan fct-rvanvmh2), and dioxus-native's tracing feature, which reaches it (obs-plan fct-6oskq4op), must not be a path a probe reading depends on.
