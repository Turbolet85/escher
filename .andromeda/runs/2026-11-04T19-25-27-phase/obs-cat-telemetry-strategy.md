@carried n=1 label=obs-plan/telemetry-strategy fact=fct-weudw6wt
why: The probe must leave the telemetry mechanism as it stands: no span beyond escher-driver's two, and any engine or bridge tracing kept behind the `tracing` cargo feature, so a probe or an escher-side bridge change adds no span macro or ungated event.
quote: Telemetry is the `tracing` crate's event macros and, in escher-driver, two span macros (`info_span!`, `command` and, since 2026-10-10-mcp-surface, `tool_call`, §4), in the engine and upstream crates compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28)
@carried n=2 label=obs-plan/telemetry-strategy fact=fct-46rgllvo
why: A probe check run on the stand under a workspace build gets engine `tracing` turned on, so a probe check must install no sink of its own; if it did, engine events would reach it, and its readings must not depend on which build ran it.
quote: By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events
@carried n=3 label=obs-plan/telemetry-strategy fact=fct-tasmmgp5
why: A probe check placed in `tests/blitz-tests/` must keep the test crate slice free of telemetry and logging crates: no `tracing`, `log::`, `env_logger` or metrics token, and no subscriber, env read or print.
quote: observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
@carried n=4 label=obs-plan/telemetry-strategy fact=fct-atc72w5q
why: Probe code placed in `examples/seven_guis/` must keep the examples slice free of tracing and telemetry calls.
quote: observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files (examples slice)
@carried n=5 label=obs-plan/telemetry-strategy fact=fct-qollkyhe
why: The probe adds no workspace crate or dependency, so no telemetry backend may enter the build files.
quote: observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
@carried n=6 label=obs-plan fact=fct-6oskq4op
why: The bridge the probe reads, `dioxus-native-dom`, gets its tracing only through dioxus-native's `tracing` feature; any escher-side change in it priced as "exposed with a cost" must stay behind that feature and add no log site.
quote: dioxus-native's `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:69)
@binding n=1
**Telemetry mechanism ↔ the scope's "Nothing crosses" boundary**: the probe adds no span, log field or log site, so escher-driver keeps its two span macros and the engine's tracing stays feature-gated (obs-plan/telemetry-strategy fct-weudw6wt); the scope's ban on any added span or log field, and on logging a component name, handler or signal value, is the same line from the chunk's side.
@binding n=2
**Feature unification ↔ test-plan §9 → Engine features by runner**: a probe check in `tests/blitz-tests/` runs with `blitz-dom/tracing` on under the workspace build and off under `-p blitz-tests`, so it installs no subscriber and its verdicts hold under both builds (obs-plan/telemetry-strategy fct-46rgllvo); the test plan's runner table says which build each runner makes.
@binding n=3
**Slice absences ↔ the probe's surfaces (`tests/blitz-tests/`, `examples/seven_guis/`)**: a new probe file must keep the slice searches reading 0, with no telemetry token, subscriber, env read or print (obs-plan/telemetry-strategy fct-tasmmgp5) (obs-plan/telemetry-strategy fct-atc72w5q).
@binding n=4
**dioxus-native-dom's feature-gated tracing ↔ arch (the bridge's "exposed with a cost" changes)**: an escher-side change inside `dioxus-native-dom` adds no tracing site outside dioxus-native's `tracing` feature and no log site at all (obs-plan fct-6oskq4op); the arch plan states the change's boundary.
