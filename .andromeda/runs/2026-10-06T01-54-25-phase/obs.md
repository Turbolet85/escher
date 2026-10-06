# obs extract

## Relevance
partial — the chunk adds no telemetry surface (JSON-line logs belong to "Stand test contract"), but it refactors the stand's entry point that carries escher's only installed subscriber and boots engine crates headlessly whose log sites carry content.

## Constraints
- The windowed stand's `main` must keep installing `escher_telemetry::init(escher_telemetry::service_identity!())` before `dioxus_native::launch`, reporting an `Err` and continuing, when `app.rs`/`lib.rs` gain a headless named-task entry (per obs-plan §3 Observability Harness Contract → Logging stack). Whether the headless entry itself should install a subscriber is P4's call. Research's question: does the refactor touch `examples/seven_guis/src/main.rs` at all?
- `init` installs a process-global `Registry`, so it succeeds only once per process (per obs-plan §3 → Logging stack). That conflicts with "fresh per check": if any headless check installs it, a second call in the same test binary must be tolerated, never unwrapped. Research's question: does any check or harness path call `init`?
- Any new log call in the harness, the stand or the engine stays behind `#[cfg(feature = "tracing")]` with a no-op fallback (per obs-plan §2 Telemetry Strategy → Telemetry mechanism). A font-context or net option on `HarnessOptions` adds no unconditional event.
- Escher's own sink writes to stderr only, one line per event, with `service.name`/`service.version` on every line and levels from `RUST_LOG` defaulting to `warn` (per obs-plan §3 → Service identity and line format; §6 Log Coverage → Log format). The stand checks must not depend on or assert those lines; that contract belongs to the next entry.
- With engine `tracing` features on, dioxus-native-dom's mutation writer logs text-node contents and attribute values, and blitz-net logs full request URLs, all unscrubbed outside escher's sink (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is; §6 → dioxus-native, blitz-net). Research's question: does `blitz-test-harness` or the stand's test target turn any engine `tracing` feature on?
- dioxus-native logs fetches made without a net provider at warn, and blitz-dom logs resource-load failures at warn with an `error` field (per obs-plan §6 → dioxus-native; §7 Error Capture & Reporting → Error classes captured). A no-net or refusing provider, or a bundled font, may surface as these warn events rather than as errors. Research's question: does the bundled-font path go through a fetch at all?

## Patterns to follow
- Feature-gated `tracing` events with a no-op path when the feature is off, as across blitz-dom (per obs-plan §2 → Telemetry mechanism).
- Test diagnostics go to stdout/stderr through `println!`/`eprintln!` in `tests/blitz-tests` and install no subscriber (per obs-plan §6 → tests/blitz-tests; §2 → Absent, test crate slice).
- escher-telemetry's own integration tests run one per process. Any check that touches the global subscriber follows that isolation (per obs-plan §3 → Logging stack).

## Anti-patterns to avoid
- Adding a log field that carries user or page content (task input text, attribute values, a font or asset path or URL) on a new harness or stand path. §8 lists exactly these values as logged as-is wherever escher's allowlist sink is not installed (per obs-plan §8 → Values logged as-is / Scrubbing).
- Writing telemetry to stdout from the stand, or adding an unconditional `println!` in an engine or harness crate (per obs-plan §3 → Logging stack; §2 → Telemetry mechanism).

## Contract bindings
- obs ↔ tests §3 harness: the headless boot is the surface that the next entry, "Stand test contract", will attach JSON-line logs and status to. This chunk has to leave a single named entry that a subscriber can be installed around (per obs-plan §3 Observability Harness Contract). The JSON schema and heartbeat rows are NOT YET MEASURED there, so they bind nothing now.
- obs ↔ security §Untrusted input: the no-live-network configuration also keeps blitz-net's unscrubbed `url` fields from being emitted by any stand check (per obs-plan §8 → Values logged as-is).
- Bootstrap phases: `pii-scrubbing-wire` is discharged for escher's sink. `otel-sdk-install` stays deferred to "Driver command spans". This chunk advances neither (per obs-plan §3 → Bootstrap phases).

## Acceptance criteria contributions
- After the entry-point refactor, `seven_guis_native` still calls `escher_telemetry::init` before `launch`, and its smoke run still prints `service.name=seven_guis` on stderr (per obs-plan §3 Observability Harness Contract → Logging stack / Service identity).
- No new `tracing`/`log` call site in `examples/seven_guis`, `packages/blitz-test-harness` or the engine crates is outside `#[cfg(feature = "tracing")]`, and none adds a field carrying text, attribute, path or URL values (per obs-plan §2 Telemetry Strategy; §8 PII Scrubbing & Compliance).
- The stand proof checks pass in any order inside one test binary without panicking on a second global-subscriber install. Either no check calls `init`, or a repeated `Err` is tolerated (per obs-plan §3 → Logging stack).
