# obs extract

## Relevance
partial — the chunk adds no instrumentation (no source crate touched); obs applies only to how the baseline run's timing and output are taken and recorded (obs tier 0, per obs-plan §1).

## Constraints
- Obs tier is 0 (per obs-plan §1 Obs Scope Summary): the chunk owes no instrumentation depth. The wall-clock figure is a host measurement recorded as evidence. It is not a new metric, because the plan records no metric, span or log-field naming convention to emit it under (§2's naming block is NOT YET MEASURED, so nothing is cited from it).
- Telemetry is compiled in only behind each crate's `tracing` cargo feature, with a no-op path when the feature is off (per obs-plan §2 Telemetry Strategy, "Telemetry mechanism"). The baseline run must not change that gating, and it must state which features the build compiled. obs-plan §2 "Feature wiring" names `tracing` as a default feature of `blitz`, forwarding to blitz-shell, blitz-html and blitz-net, so a default `--workspace` build compiles some tracing call sites in. Which features `cargo build --workspace` actually unifies on this host is research's question.
- Timing output is opt-in through `log-times` / `log-frame-times` / `log-phase-times`. blitz-dom's `log-phase-times` turns on `debug_timer/enable` (per obs-plan §2 "Feature wiring"; obs-plan §5 Metric Coverage, rows "Frame / phase timing" and "debug_timer"). The baseline wall-clock is only comparable if these stay off, or if any that are on are named beside the figure.
- `tests/blitz-tests` sends its diagnostics to stdout/stderr through `println!`/`eprintln!`. A `blitz-dom/log-phase-times` feature there prints per-phase resolve timings (per obs-plan §6 Log Coverage, "tests/blitz-tests"). The `paint_tree_bench` test prints host-dependent medians as a markdown table (per obs-plan §5, rows "`paint_tree_bench` test" / "external page"). That output is a reading of this host, not a pass/fail signal.
- The plan observes no tracing subscriber or logging crate in the test-crate slice (per obs-plan §2 "Absent", test crate slice; obs-plan §3 Observability Harness Contract, "Logging stack"). Do not expect `RUST_LOG` or a subscriber to add structured output to the blitz-tests run. Whether any test installs one is research's question.
- Log values are emitted unscrubbed: URLs, attribute values, outer HTML and CSS property values (per obs-plan §8 PII Scrubbing & Compliance, "Values logged as-is"), and no scrubbing exists (§8 "Scrubbing (absent)"). Any captured run output placed in the chunk's evidence folder therefore carries raw values.

## Patterns to follow
- Record timing as a plain figure with its basis stated, the same way the workspace's existing timing surfaces print durations: WPT runner per-run total duration, `screenshot` example step timings (per obs-plan §5 Metric Coverage, current-truth table). Build no new pipeline for the baseline figure.
- Keep timing features opt-in and named. When phase timings are wanted, the existing route is the `log-phase-times` feature forwarding to `debug_timer/enable` (per obs-plan §2 "Feature wiring"; obs-plan §5 rows "blitz-dom `resolve`" / "debug_timer"). It is not a new print.
- Leave the feature-gated call-site shape untouched: `#[cfg(feature = "tracing")]` with a no-op fallback (per obs-plan §2 "Telemetry mechanism"). The chunk adds no call site.

## Anti-patterns to avoid
- Do not add an unconditional print or log call to any crate or test to time the baseline. That breaks the feature-gated, no-op-when-off mechanism (per obs-plan §2 "Telemetry mechanism"). obs-plan §11 Obs Anti-Patterns has no recorded intent, so this ban derives from §2 alone.
- Do not commit raw captured test or build logs without review. They carry unscrubbed URLs, attribute values and outer HTML (per obs-plan §8 "Values logged as-is").

## Contract bindings
- obs ↔ tests: the blitz-tests stdout diagnostics, the `paint_tree_bench` printed medians and the `blitz-dom/log-phase-times` timing feature are obs surfaces inside the test suite this chunk runs (per obs-plan §5; obs-plan §6 "tests/blitz-tests"). The baseline records them as readings. The tests side owns pass/fail.
- obs ↔ security: captured output in evidence is unscrubbed user-content-bearing log data (per obs-plan §8). This binds to security-plan logging/scrubbing, which the plan observes as absent.
- obs-plan §3 keyed contract "Bootstrap phases" (labels otel-sdk-install, pii-scrubbing-wire) does not bind here. The chunk installs no tooling (scope §Boundaries).

## Acceptance criteria contributions
- The chunk's diff adds no `tracing`/`log`/`println!` call site under `packages/`, `tests/` or `examples/`. Source gating is unchanged (per obs-plan §2 Telemetry Strategy, "Telemetry mechanism").
- The recorded wall-clock states the cargo feature set of the measured build and test run. `log-times` / `log-frame-times` / `log-phase-times` are off, or named if on (per obs-plan §2 "Feature wiring"; obs-plan §5 Metric Coverage).
- Any `paint_tree_bench` medians kept in evidence are labelled as this host's readings and not used as thresholds (per obs-plan §5 Metric Coverage, "`paint_tree_bench` test").
- Evidence committed to the chunk folder holds summary figures and exit statuses. Any raw log excerpt has been reviewed for URLs, attribute values and outer HTML (per obs-plan §8 PII Scrubbing & Compliance).
