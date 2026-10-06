# tests extract

## Relevance
relevant. The chunk adds headless harness checks for the stand. Its surfaces are the harness construction and options, the input and inspection helpers, the integration-test layout and the workspace test leg, all of which test-plan §2/§3/§5/§7/§8/§9 cover. §1 tier justification, the §3 5-command contract, §10 gates, §11 anti-patterns and §12 decisions read NOT YET MEASURED / NO RECORDED INTENT and are not cited.

## Constraints
- The checks must run as Rust built-in-harness `#[test]` functions under the workspace test leg (`cargo test --workspace --locked`, `ci-leg.sh test`), and `ci-leg.sh fast` (fmt → clippy → test → ci-scripts) is the local pre-push gate they must pass (per test-plan §1 Coverage scope "Workspace (CI)", §3 Runners "Rust tests", §9 "Local pre-push gate").
- Placement: test-plan §2 Directory pattern and §5 "tests/blitz-tests crate" require one integration file per behaviour under `tests/blitz-tests/tests/`, depending only on dev-dependencies. Choosing between that and a `seven_guis` test target, and any new dev-dependency edge (blitz-tests → seven_guis), is research's question and an arch crate-graph fact.
- Boot through the `Harness` Dioxus constructors (`from_component` / `from_vdom(vdom, options)`). Constructors pump once and `wrap` does not. Input helpers pump after dispatch, while `dispatch`/`dispatch_recorded` do not (per test-plan §3 blitz-test-harness "Construction", "Pump semantics", "Input helpers").
- Do not drive Dioxus-backed tasks with `dispatch_recorded`. It runs against the underlying BaseDocument and bypasses Dioxus VirtualDom forwarding (per test-plan §5 "Harness ↔ event pipeline"). Interactions go through the input helpers (click, type_text, press…), which route through the real event-dispatch pipeline without a window.
- Pinned viewport: the existing seed is fixed sizes (800x600 / 400x300, scale 1.0, Light). `HarnessOptions` carries width, height, scale, color_scheme, base_url and net_provider, and has no font-context field (per test-plan §7 Seed strategies "Viewports", Builders "HarnessOptions"). Adding a font or net option is a public harness API change, so it is an arch §Standard Contracts amendment.
- Fonts: font-dependent assertions today rely on `system-fonts`, which is on by default across the workspace, and tests skip with `eprintln!` when text measures 0x0 (per test-plan §9 "Fonts", §2 "Font-dependent tests", §8 "Real dependencies kept"). The chunk's bundled-font mandate replaces that dependence. Whether the harness can take a bundled font context today is research's question.
- Timer: `pump` polls with no waker and resolves at the harness time (per test-plan §3 "Pump semantics"). Virtual-time precedents exist: WPT timers on virtual time without the timer thread (§8 "Time (WPT)"), and blitz-vibey-script's virtual time / `without_timer_thread` for embedders (§3 Crate-local helpers). Whether the timer task's `futures_timer::Delay` can be advanced headlessly without real sleeps is research's question.

## Patterns to follow
- Dioxus counter through the harness: `harness_smoke.rs` clicks a Dioxus counter and asserts its rendered text. This is the nearest template for a per-task proof check (per test-plan §6 "Headless harness smoke").
- Inspection helpers for reading state: `query`, `text_content`, `attr`, `layout_rect`, `center_of`, `focused`, `dom_string`. `dom_string` is a stable one-node-per-line serialization suitable for snapshot-style assertions (per test-plan §3 "Inspection helpers", §2 "Harness-driven tests").
- Network stand-ins: no-op providers (`DummyNetProvider` family), or a recording provider (`RecordingNetProvider` records requested URLs instead of fetching), so a check can assert zero requests (per test-plan §8 Hand-written fakes "Network", "Net, navigation, shell").
- Differential oracle: run the same fixture and steps in an incremental and a non-incremental document and compare layouts, for any stand check that asserts layout (per test-plan §2 "Differential oracle").
- Naming: name each check for the behaviour it guards (per test-plan §2 "Test function naming").

## Anti-patterns to avoid
- Real-time sleeps before polling, as in blitz-vibey-script's timer tests (per test-plan §2 "Script tests"). The chunk requires the timer check to be deterministic without wall-clock waits.
- Conditional assertions that pass without asserting, and font-skip early returns. Text-input tests assert only inside an `if` on measured size (per test-plan §4 "Conditional assertions"), and font tests skip on 0x0 (§2 "Font-dependent tests"). With bundled fonts, a stand check must assert unconditionally.
- Shared state across checks, such as shared `Rc<RefCell<…>>` app props (per test-plan §7 Builders, keyed-nodes test). Each check builds its own harness and VirtualDom ("fresh per check"). test-plan §11 records no bans of its own.

## Contract bindings
- tests ↔ arch: a public `HarnessOptions` font/net addition, or a blitz-tests → seven_guis dev-dependency edge, is harness API or crate-graph surface (per test-plan §7 Builders "HarnessOptions", §5 "tests/blitz-tests crate"). Each is an arch §Standard Contracts / §Occupied Resources amendment at wrap.
- tests ↔ security: the no-live-network check uses the §8 network stand-ins (no-op or recording provider). This keeps `blitz-net` (no timeout or size cap) off the stand's check path, per the security rules §Untrusted input.
- tests ↔ obs: the stand's stderr log line and its four `telemetry_*` integration files (per test-plan §3 "Stand log format") stay with the windowed binary. The headless boot owns no boot/run/status/cleanup/logs command. That contract belongs to the next entry, "Stand test contract", and test-plan §3's 5-command contract reads NOT YET MEASURED.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` exits 0, and the workspace test count rises from the 416 passed · 0 failed · 4 ignored baseline by the new stand checks, with 0 failed and no new ignored (per test-plan §9 "Local pre-push gate", "Local baseline").
- Each of counter, flight booker, timer and CRUD has at least one check. It boots the task in TaskShell through a `Harness` Dioxus constructor at one shared pinned `HarnessOptions` viewport, reads its initial state through the inspection helpers, drives one interaction through an input helper (not `dispatch_recorded`), and asserts the resulting state (per test-plan §3 blitz-test-harness, §5 "Harness ↔ event pipeline").
- The stand checks assert unconditionally, with no font-skip path and no real sleep. A recording net provider (or a refusing one) sees zero requests across a boot and interaction (per test-plan §2 "Font-dependent tests", §8 "Network").
- Any stand check that asserts layout passes for `incremental in [false, true]` with identical results (per test-plan §2 "Differential oracle").
