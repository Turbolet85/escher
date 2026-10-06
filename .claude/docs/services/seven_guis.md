# seven_guis (the stand)

_Crate notes. Primary sources: `.andromeda/architecture.md` §Existing Scopes, `.andromeda/layout-templates.md` desktop-native, the active `escher-X.Y.Z/intent.md`._

## Responsibility
The 7GUIs benchmark app ("Seven benchmark tasks for GUI frameworks") in Dioxus — tasks counter, temp_converter, flight_booker, timer, crud, circle_drawer, cells. escher's internal stand: every 0.1.0 capability (stable ids, snapshot, driver, screenshots, a11y) is proven on it headlessly.

## Key integrations

### Consumes from
- dioxus-native (native binary `seven_guis_native`) or a wasm32 cdylib (`--no-default-features --features hybrid`, `console_error_panic_hook`).
- escher-telemetry (native target only): `main` calls `escher_telemetry::init(escher_telemetry::service_identity!())` before `launch` — stderr log lines carry `service.name=seven_guis`; an init `Err` is `eprintln!`ed and the stand still launches. The engine `tracing` features stay off.

- On the native target only, blitz-test-harness and blitz-traits (the headless stand), with `dioxus-native` features `system-fonts` + `woff` — `woff` decodes the bundled DejaVu woff2 in every build that includes the stand. seven_guis is a `[workspace.dependencies]` path entry with default features (dioxus-native refuses to compile with no renderer); blitz-tests consumes it as a dev-dependency.

### Publishes to
- Screens: Home (centered 640px column of task cards with description and tag) and TaskShell (header with back button, title, spacer over a scrolling body).
- The headless stand `seven_guis::stand` (native only): `LeanTask { Counter, FlightBooker, Timer, Crud }`; pinned `VIEWPORT_WIDTH 800` · `VIEWPORT_HEIGHT 600` · `HIDPI_SCALE 1.0` · `COLOR_SCHEME Light`; `font_ctx()` (bundled DejaVu Sans, system fonts off); `options(incremental) -> HarnessOptions` (offline: `net_provider: None`); `boot(LeanTask, options) -> Harness<DioxusDocument>` and `boot_timer(options) -> (Harness, TimerTicks)` — each a fresh `VirtualDom` mounting the task in TaskShell (no Home) with a no-op Back. It installs no telemetry, reads no env var, names no `blitz_net`.
- `app::Task` (pub) and `app::task_in_shell(Task, EventHandler<()>)` — the windowed `app()` and the stand render through the same mapping; `TaskShell` stays private. `DEJAVU_SANS` — the crate-root woff2 bytes, shared by the wasm entry and the stand.
- `tasks::timer::TimerTicks` — the timer's tick seam: absent from context, the timer runs its 100 ms `futures_timer::Delay`; present, each `deliver(n)` tick is one 0.1 s step capped at the duration, applied on the next pump. Every stand boot provides one, so an undriven timer holds 0.0 s.

## Internal conventions
- Example CSS in `const CSS: &str` raw strings; accent `#4a6cf7` on `#f5f5f5`, text `#1a1a1a`; invalid `#e53e3e` family, success `#ebf8ee` family.
- Inputs replace the focus outline with a border colour or box-shadow on focus.
- `idna_adapter = "=1.0.0"` pin disables unicode URL support.

## Stand-relevant behaviour
- Flight booker validates `dd.mm.yyyy` with month range and leap years; return not before start; invalid dates get an `invalid` class and disable Book.
- Cells: tokenizer rejects unknown characters, reference cycles read `#CYCLE`, division by zero is an error; commits on Enter or blur.
- Circle drawer clamps diameter 5–100.

## Entry points for modification
- `examples/seven_guis/src/app.rs` (Home, TaskShell, `task_in_shell`) · `src/stand.rs` (headless stand) · `src/tasks/*.rs` · `src/main.rs` · `src/lib.rs` (`DEJAVU_SANS`, wasm entry)

## Testing this crate
- Stand checks live in `tests/blitz-tests/tests/stand_{boot,counter,flight_booker,timer,crud}.rs`, each building its own harness through `stand::boot` and driving it with the harness input helpers: `cargo test -p blitz-tests --test stand_boot` (etc.). They assert unconditionally under the bundled font — no font skip, no sleep, no `dispatch_recorded`. Select task elements by class (no task element has an id yet); CRUD's unlabeled inputs and buttons by structural selectors under `.crud-root`.
- `type_text` into a Dioxus controlled input reaches its `oninput` (the flight-booker and CRUD checks witness it).
- Run windowed: `just seven_guis` (`RUST_LOG=info` shows the telemetry lines on stderr).
- Boot smoke (windowed, needs `WAYLAND_DISPLAY`): `RUST_LOG=info timeout 10 target/debug/seven_guis_native` — exit 124 (still up when stopped) and `service.name=seven_guis` in the log.

## References
- `.claude/rules/a11y.md` (the stand is in its paths) · `.claude/docs/design-summary.md`
