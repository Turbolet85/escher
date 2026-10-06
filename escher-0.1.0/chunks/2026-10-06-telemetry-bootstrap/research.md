# Codebase Research — 2026-10-06-telemetry-bootstrap

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 11 (+ 3 code-graph queries, 2 `cargo tree`, 4 `cargo info`, 3 crates.io dependency reads)
- **Harness rules consulted:** none — no live leg in this chunk (every proof is `cargo test` / a `ci-leg.sh` leg; no scenario run against a real process)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's only run was in progress, not red) and no CI-reading entry outside the operator leg
- **External inputs:** none. Every fact this chunk turns on is in this repository, or in crates.io metadata and the registry's published sources (`~/.cargo/registry/src/…/opentelemetry-otlp-0.33.0`). Those count as a tool's or dependency's behaviour, not an input.

## Files inspected
- `examples/seven_guis/src/main.rs` (full). The native `main` calls only `dioxus_native::launch(seven_guis::app::app)` at line 6. It installs no subscriber and no panic hook. The wasm `main` is empty.
- `examples/seven_guis/src/lib.rs` (1-23). The wasm entry `start()` installs `console_error_panic_hook::set_once()` (line 13). It is `cfg(target_arch = "wasm32")` only, and the native bootstrap must leave it untouched.
- `examples/seven_guis/Cargo.toml` (full). It depends on `dioxus-native` with `prelude` (+ `system-fonts` native, `woff` wasm) and `default = ["hybrid"]`. The workspace dep sets `default-features = false`, so **the stand does not turn on `dioxus-native/tracing`**: engine `tracing` call sites are compiled out of the stand by default (`cargo tree -p seven_guis -e features -i tracing` reaches `tracing` only through winit / winit-common, never through a blitz `tracing` feature).
- `Cargo.toml` (1-60, dependency lines). Members list (26, no escher crate). The workspace deps `tracing = "0.1.40"`, `tracing-subscriber = "0.3"` (line 184), `tokio = "1.42"`, `reqwest = { version = "0.13", default-features = false }` and `http = "1.1.0"`. Root dev-deps `env_logger = "0.11"` and `tracing-subscriber = "0.3"` (lines 285-286).
- `Cargo.lock` (by name). Locked versions: tracing 0.1.44, tracing-core 0.1.36, tracing-subscriber 0.3.23, tracing-log 0.2.0, log 0.4.34, reqwest 0.13.4, hyper 1.11.1, tokio 1.53.1, http 1.5.0, rustls 0.23.45. **No `opentelemetry*`, `tonic` or `prost` crate is locked** (re-derived: `awk` over `name = "…"` stanzas of Cargo.lock).
- `apps/browser/src/main.rs:74`, `apps/readme/src/main.rs:62`, `examples/todomvc/src/main.rs:17` — each calls `tracing_subscriber::fmt::init()` under its own `#[cfg(feature = "tracing")]`, with the default fmt writer (stdout). `wpt/runner/src/main.rs:458-459` calls `env_logger::init()` + `std::panic::set_hook(stash_panic_handler)` (re-derived: `grep -rn 'tracing_subscriber|set_global_default|env_logger::|LogTracer|set_hook|take_hook'` over apps packages examples wpt tests — 5 hits, no other install).
- `wpt/runner/src/panic_backtrace.rs` (1-40). This is the precedent hook. It captures message (from a `&str` or `String` payload), file, line, column and a forced backtrace into a thread-local. It **replaces** the hook rather than chaining it.
- `packages/blitz-net/Cargo.toml` (lines 14-38). blitz-net enables `reqwest` with `native-tls` (normal) and `native-tls-vendored` (target-specific). The stand does not pull blitz-net, since `dioxus-native/net` is off in the stand.
- `opentelemetry-otlp-0.33.0/src/exporter/http/mod.rs` (280-305, 780) and `src/exporter/mod.rs` (152-170, 299-301, 424-430). These are the exporter's implicit env reads, quoted under Patterns detected.

## Graph impact (from the code-graph query)
- **launch** (`dioxus-native/src/lib.rs`) has 19 callers. The one this chunk touches is `seven_guis main()` @ `examples/seven_guis/src/main.rs:6`. The bootstrap call goes before it, so no signature changes and no caller threading is needed.
- **crate_edges for seven_guis**: outbound to `blitz-dom` and `dioxus-native`, no inbound. The stand is a leaf, so adding a dependency on the new crate gives zero cross-crate blast radius.
- **Name collision** (`symbol WHERE name LIKE 'telemetry%' OR LIKE 'escher%' OR = 'scrub' OR LIKE 'init_tracing%'`): 0 rows on the rust plane. The rust plane built (db_state in the trace), and the names searched are new, so the rows count is a genuine "free" (trace: `.andromeda/runs/2026-10-06T00-14-27-phase/tree-query-2026-10-06-telemetry-bootstrap.json`).

## Patterns detected
- **Feature-gated subscriber install** (`apps/browser/src/main.rs:73-74`; `examples/todomvc/src/main.rs:16-17`): the app's own `tracing` feature gates `tracing_subscriber::fmt::init()`. `fmt::init()` writes to **stdout** by default (`fmt::Layer` writer is `io::stdout`). It also calls `try_init`, which installs `tracing_log::LogTracer` when the resolved `tracing-log` feature is on, and it is on in this workspace (`cargo tree --workspace -e features -i tracing-subscriber` lists `tracing-log`, `env-filter`, `fmt`, `ansi`, `registry`). So a `log`-facade record such as `js_console` reaches the subscriber.
- **User content travels in two shapes at engine call sites** (re-derived: `grep -rn -A3 '(trace|debug|info|warn|error)!('` over packages/{blitz-dom,blitz-net,dioxus-native-dom,dioxus-native,blitz-shell,blitz-html,blitz,blitz-vibey-script}/src):
  - **named fields:** `url` (blitz-net lib.rs:212, 239, 242, 260, 263, 276, 302, 306; blitz-dom document.rs:1262), `error` (blitz-net lib.rs:239, 260, 306; document.rs:1264, 1271; construct.rs:484), `html` (construct.rs:483), `value` (node/element.rs:707), `path` (blitz-net lib.rs:58);
  - **interpolated into the message string:** at least 19 sites:
    - blitz-dom mutator.rs:1147, 1159, 1166, 1216 (image src / iframe src);
    - blitz-dom layout/damage.rs:492, 501, 507 (image URL);
    - blitz-dom events/pointer.rs:753 (href + document URL), 758 (element attrs);
    - dioxus-native-dom mutation_writer.rs:150 (text), 202 (text), 388 (attribute value);
    - dioxus-native assets.rs:48, 53, 60 (request, Debug);
    - dioxus-native link_handler.rs:15;
    - blitz lib.rs:49 (`Launching {url}`);
    - blitz-vibey-script runtime.rs:1102 and document.rs:266 (JS error text);
    - and `log::debug!(target: "js_console", "{msg}")` at runtime.rs:1252, via the log bridge.
- **The OTLP exporter reads env vars implicitly**, and that cannot be switched off through its API. `opentelemetry-otlp 0.33.0` `exporter/http/mod.rs:293-297` reads `OTEL_EXPORTER_OTLP_{TRACES_}HEADERS` **unconditionally** at `build()`, after any explicit headers, and merges them in. `:780` reads `OTEL_EXPORTER_OTLP_ENDPOINT` when no endpoint is set explicitly. `exporter/mod.rs:299-301, 424-430` read `…_INSECURE` and `…_TIMEOUT`. The headers variable is the standard place an OTLP auth token travels, so turning export on opens a **credential path**.
- **Process-global installs**: a global default subscriber can be set once per process (`set_global_default` errors on the second call), and `std::panic::set_hook` replaces whatever was there. Idempotence, and chaining via `take_hook`, are the bootstrap's to provide. Neither exists in-repo.

## Conventions to follow
- **Workspace-inherited manifests**: in-repo crates are declared in `[workspace.dependencies]` with a `path` and consumed with `{ workspace = true }` (Cargo.toml:43-58). Package metadata comes from `[workspace.package]` (Cargo.toml:32-39; e.g. `examples/seven_guis/Cargo.toml:5`).
- **Optional capability = Cargo feature** (`packages/dioxus-native/Cargo.toml:67`; `packages/blitz-dom/Cargo.toml:23`). OTel export goes behind a non-default feature of the new crate, and the stand's default build must not resolve it.
- **Engine `tracing` gating stays**: call sites are `#[cfg(feature = "tracing")]` with a no-op fallback (`packages/blitz-dom/src/layout/mod.rs:131-139`). Any call site this chunk edits keeps its gate.
- **Integration tests** are one file per behaviour in `tests/blitz-tests/tests/` (59 files today, re-derived: `ls tests/blitz-tests/tests | wc -l`), each its own test binary. That gives process isolation for global-subscriber and panic-hook tests.
- **CI legs** go through `.github/scripts/ci-leg.sh` (test leg `cargo test --workspace --locked` at line 21, default features; msrv leg `cargo +1.91 build --workspace --locked` at line 24). The leg list is pinned by `.github/scripts/test_ci_workflows.py`.

## Version fit (OTel family, as published)
- `opentelemetry` / `opentelemetry_sdk` / `opentelemetry-otlp` 0.33.0, `tracing-opentelemetry` 0.34.0 — rust-version 1.75.0 each, ≤ the workspace's 1.91 (`cargo info`).
- `opentelemetry-otlp` requires `reqwest ^0.13.1` (locked 0.13.4 ✓), `http ^1.1` (1.5.0 ✓), `prost ^0.14`, `tokio ^1` (optional). `tracing-opentelemetry` requires `tracing-subscriber ^0.3.22` (0.3.23 ✓), `tracing-log ^0.2.0` (✓) (crates.io `/api/v1/crates/{c}/{v}/dependencies`). Newly locked crates: `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-otlp`, `opentelemetry-proto`, `opentelemetry-http`, `tracing-opentelemetry`, `prost` (+ derive), and their small transitive helpers. **None touches a coupled pin** (html5ever family, skrifa, svgtypes, taffy/parley, winit).
- Transport: `http-proto` + `reqwest-blocking-client` posts over reqwest's blocking client, which runs its own internal runtime thread, so no ambient tokio runtime is needed before `launch`. The workspace's `reqwest` is `default-features = false`, so **with no TLS feature added, export reaches `http://` endpoints only**. `https` needs a reqwest TLS feature (blitz-net's choice is `native-tls`).

## New files to create
- `packages/escher-telemetry/Cargo.toml` — the bootstrap crate (lib), `otel` feature off by default
- `packages/escher-telemetry/src/lib.rs` — the init entry point, service identity, sink choice, typed init error
- `packages/escher-telemetry/src/scrub.rs` — the scrub layer
- `packages/escher-telemetry/src/panic.rs` — the chaining panic hook
- `packages/escher-telemetry/src/format.rs` — the event formatter: identity prefix + the allowlist scrub applied per field
- `tests/blitz-tests/tests/telemetry_stdout_silent.rs` — an emitted event reaches stderr, its sentinel never stdout
- `tests/blitz-tests/tests/telemetry_scrub.rs` — sentinel values in engine-target messages and fields, and in content fields of any target, never reach the sink
- `tests/blitz-tests/tests/telemetry_panic_hook.rs` — a panic logs one error event and the previous hook still runs
- `tests/blitz-tests/tests/telemetry_init_idempotent.rs` — a second init is harmless

## Files to modify
- `Cargo.toml` — workspace member + the new crate's `[workspace.dependencies]` entry
- `Cargo.lock` — the new path crate recorded (no registry crate added)
- `examples/seven_guis/Cargo.toml` — depend on the bootstrap crate (native target)
- `examples/seven_guis/src/main.rs` — call the bootstrap before `launch`
- `tests/blitz-tests/Cargo.toml` — dev-dependency on the bootstrap crate

## Open questions
- none. All three were decided at P4 (overseer, under the founder's standing delegation of technical forks, 2026-10-06). Scrub reach is the allowlist at the subscriber (no engine edits). OTel export is DEFERRED, so the transport and headers questions pass to the CARRY's owner, "Driver command spans", for the founder.
