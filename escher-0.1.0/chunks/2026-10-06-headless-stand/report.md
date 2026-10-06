# Report — 2026-10-06-headless-stand

**Chunk:** Headless stand — seven_guis lean four in TaskShell, no display, fixed viewport, bundled fonts, no network, fresh per check
**Date:** 2026-10-06T02:50Z
**Commits:** `1503df2e chore(2026-10-06-headless-stand): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit of this chunk; `d57a6c69` after `last_wrap` is the previous chunk's wrap commit) — basis `git log --since=2026-10-06T01:50:39Z`

## Changes (structured — detectors read this)
- **Files:** basis `git diff --stat d57a6c69` (16 source files, 512+ / 30−):
  - new: `examples/seven_guis/src/stand.rs` · `tests/blitz-tests/tests/stand_boot.rs` · `stand_counter.rs` · `stand_flight_booker.rs` · `stand_timer.rs` · `stand_crud.rs` · `tests/blitz-tests/tests/dioxus_falsy_disabled.rs` (out-of-list, widening)
  - modified: `packages/blitz-test-harness/src/harness.rs` · `examples/seven_guis/src/{app.rs,lib.rs,tasks/timer.rs}` · `examples/seven_guis/Cargo.toml` · `Cargo.toml` · `tests/blitz-tests/Cargo.toml` · `Cargo.lock` · `packages/dioxus-native-dom/src/mutation_writer.rs` (out-of-list, widening)
  - chunk folder: `plan.md` · `research.md` · `scope.md` · `scope-record.md` · `evidence/{disabled-false-probe,operator-14-hygiene,operator-15-push,operator-16-ci}.txt` · this report
  - `examples/seven_guis/src/main.rs` untouched (gate `git diff --quiet d57a6c69 -- examples/seven_guis/src/main.rs` exit 0)
- **Symbols / APIs:**
  - `blitz_test_harness::HarnessOptions` gains two public fields: `font_ctx: Option<FontContext>` → `DocumentConfig.font_ctx`, and `incremental: Option<bool>` → `DocumentConfig.incremental`; `Default` keeps both `None` (packages/blitz-test-harness/src/harness.rs:12-26, `into_config` at :44-59). The 3 existing struct-literal sites (`dir_attribute.rs`, `pointer_events.rs`, `touch_events.rs`) end in `..Default::default()` and are unchanged; the stand's `options()` is a 4th, full literal.
  - `seven_guis::app::Task` is now `pub` (was private), seven variants unchanged, derives `Debug` added; new `pub fn task_in_shell(task: Task, on_back: EventHandler<()>) -> Element` renders a task inside the (still private) `TaskShell`. `app()` calls it for the windowed flow — `app()` keeps its callers `main.rs` (launch) and the wasm entry. Element tree, ids, classes and CSS constants unchanged.
  - `seven_guis::DEJAVU_SANS: &[u8]` — new crate-root `pub const` (`include_bytes!("../assets/DejaVuSans.woff2")`), now compiled on native AND wasm; the wasm entry uses it (`super::DEJAVU_SANS`), no second `include_bytes!`.
  - `seven_guis::tasks::timer::TimerTicks` — new `pub` type (`Clone + Default`, `Rc<RefCell<{pending, waker}>>`), method `deliver(n: u64)`; the `Timer` component reads it with `try_use_context::<TimerTicks>()`: absent → the unchanged `futures_timer::Delay` 100 ms loop; present → one `+0.1`-capped step per delivered tick, no `Delay`.
  - `seven_guis::stand` (native-only, `#[cfg(not(target_arch = "wasm32"))]`): `enum LeanTask { Counter, FlightBooker, Timer, Crud }` + `LeanTask::ALL` + `LeanTask::task()`; consts `VIEWPORT_WIDTH = 800`, `VIEWPORT_HEIGHT = 600`, `HIDPI_SCALE = 1.0`, `COLOR_SCHEME = ColorScheme::Light`; `fn font_ctx() -> FontContext` (`build_single_font_ctx(DEJAVU_SANS)`, system fonts off); `fn options(incremental: bool) -> HarnessOptions` (pinned viewport + bundled font, `net_provider: None`, `base_url: None`); `fn boot(LeanTask, HarnessOptions) -> Harness<DioxusDocument>`; `fn boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`. Every boot builds a fresh `VirtualDom::new_with_props` over a private root that provides a `TimerTicks` in context (for every task — an undelivered source keeps the timer at 0.0 s) and renders `task_in_shell` with a no-op back handler. The module installs no telemetry, reads no env var, names no `blitz_net` (census gate).
  - `dioxus-native-dom` `mutation_writer::set_attribute_inner` (private fn): a falsy value (`AttributeValue::Bool(false)`, Text `"false"`, Int 0, Float 0.0, None) of `disabled` now CLEARS the attribute, as `checked` already did (`matches!(local_name, "checked" | "disabled") && is_falsy`, mutation_writer.rs:406-407). Before, `disabled: false` was written as the literal `disabled="false"`. Callers: every Dioxus-native app (the browser, seven_guis, todomvc, examples) through the `WriteMutations` path — not a sole-caller change.
  - No port, socket, listener, env var or IPC surface added.
- **Crates / modules:** new module `seven_guis::stand`; no crate added or removed.
- **Dependencies:** new workspace-internal edges, all path crates already in `Cargo.lock` (no package stanza added — gate `git diff d57a6c69 -- Cargo.lock | grep -c '^+name = '` → 0):
  - root `[workspace.dependencies]` gains `seven_guis = { path = "./examples/seven_guis" }` (default features — `dioxus-native` refuses to compile with no renderer)
  - `blitz-tests` `[dev-dependencies]` gains `seven_guis = { workspace = true }`
  - `seven_guis` native target (`cfg(not(target_arch = "wasm32"))`) gains `blitz-test-harness = { workspace = true }` (normal dep) AND `blitz-traits = { workspace = true }` (normal dep — NOT in the plan; needed to name `ColorScheme`, see Deviations)
  - `seven_guis` native `dioxus-native` features `["system-fonts"]` → `["system-fonts", "woff"]` (woff2 decoder `wuff`, already locked, now in the `-p blitz-tests` resolve — gate `cargo tree -p blitz-tests -e features -i wuff --locked | grep -c '^wuff v'` → 1)
  - `Cargo.lock`: 3 added lines — `"seven_guis"` in blitz-tests' list; `"blitz-test-harness"`, `"blitz-traits"` in seven_guis' list (basis `git diff d57a6c69 -- Cargo.lock`)
  - external registry crates: none added or bumped; `cargo-deny` audit leg green
- **Schema / config:** none — no config key, schema, scrub or redaction shape.
- **Spec-master edits:** none (no master edited by implement).
- **Counts / qualifiers moved:**
  - workspace tests: 416 passed · 0 failed · 4 ignored → **430 passed · 0 failed · 4 ignored**, 114 → **120** result lines (basis: `grep '^test result:' target/ci-logs/test.log` summed, the `ci-leg.sh fast` run of /implement's second gate block, log written 2026-10-06T02:22:30Z; the same count again at the operator pass's `fast`). +14 = 13 stand checks (stand_boot 6 · stand_counter 1 · stand_flight_booker 1 · stand_timer 3 · stand_crud 2) + 1 regression test (`dioxus_falsy_disabled` 1). Stated at test-plan.md:285 (§9 Local baseline).
  - a11y CI leg: unchanged at 3 test files (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 3) — basis `target/ci-logs/a11y.log`.
  - HarnessOptions field count 6 → 8 (test-plan.md:216 lists the six; architecture.md:129 names `HarnessOptions { width, height }` with Default).
  - fork CI pipeline wall: 561 s (run 37395425505) / 642 s (run 37401351553) → **1104 s** (run 37404017734 on `1503df2e`) — basis `ci.py conclusion` line in `evidence/operator-16-ci.txt`; cause not measured (this push changed `Cargo.lock`).
- **Dev-tool versions:** none — no host tool installed or changed.
- **Harness / gate surface:** the in-process `Harness` construction surface widened (the two `HarnessOptions` fields); a new stand boot surface `seven_guis::stand::{boot, boot_timer, options}` is the drive surface later stand checks use. No `agent-run.*` script, CI step, status shape or log format changed (`scripts/agent-run.*` still owned by "Stand test contract"). No CI workflow edit.
- **Cross-project / external claims:**
  - CI run `CI#37404017734` (Turbolet85/escher, push of `build/escher-0.1.0`) measured sha `1503df2e1a4f` → **verdict: green**, checks 16/16, wall 1104 s, all 16 jobs success incl. "Build wasm examples" and "MSRV Build [Rust 1.91]" (basis: `evidence/operator-16-ci.txt`, `gh run view 37404017734 -R Turbolet85/escher --json jobs`). The verdict was taken on `1503df2e`; this wrap's commit adds only bookkeeping, spec and evidence files on top.
  - Upstream: the falsy-`disabled` fix in dioxus-native-dom is upstreamable to DioxusLabs/blitz (the defect is upstream code; basis: the source read of `mutation_writer.rs` at d57a6c69, unmodified from upstream by any escher chunk).
  - inputs: none — no external input snapshotted (`inputs.py verify` → `inputs: absent`).
- **Reverted / negative API facts:**
  - a throwaway probe test `tests/blitz-tests/tests/zz_probe_disabled.rs` was written, run once and deleted (it measured the disabled defect; its readings are in `evidence/disabled-false-probe.txt`); never committed.
  - `app::TaskShell` stays private (the plan allowed it).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **Plan step 6 / acceptance (a11y) premise** — "initially `.flight-btn` carries no `disabled` attribute" (`chunks/2026-10-06-headless-stand/plan.md` step 6, `stand_flight_booker.rs` bullet). Measured FALSE on the tree at d57a6c69 + this chunk's stand: `attr(".flight-btn","disabled")` read `Some("false")`, and the element matched `:disabled` (gate 1 red in /implement's first block; `evidence/disabled-false-probe.txt`). Cause: dioxus-native-dom wrote a falsy `disabled` literally. **Disposition: resolved in the impl** by the engine fix (widening, operator's delegate word) — the claim holds after the fix (gate 1 green; regression test red-before-green).
  2. **scope.md §Surfaces "This chunk changes no markup the windowed app renders"** (`chunks/2026-10-06-headless-stand/scope.md` a11y-surface bullet). The stand's SOURCE markup is unchanged, but the engine fix changes the DOM the windowed app builds: an enabled Book / Update / Delete button no longer carries `disabled="false"` and no longer matches `:disabled` (so it is no longer styled with the `:disabled` rules and its element state reads ENABLED). Focusability did NOT change: `flush_is_focussable` parses the value as a bool (`element.rs:629`), so a `disabled="false"` button was focusable before the fix and stays focusable after it. Disposition needed: scope.md is a chunk artifact, not a master — the fact is carried here for the a11y / design detectors.
  3. **Implicit engine fact, no master states it** — blitz-dom keys disabled-ness TWO ways: on attribute PRESENCE for the `DISABLED`/`ENABLED` element state and hence Stylo `:disabled` (`element.rs:446-451` `has_attr(disabled)`) and for the pointer click target (`pointer.rs:330`, `:457`); on the PARSED BOOL value for focusability (`element.rs:629` `attr_parsed::<bool>(disabled).unwrap_or(false)` — so `disabled="false"` and a bare `disabled=""` both leave a control focusable, as `focusability_updates.rs:60-62`'s note records). CORRECTION of this report's first draft, which stated focusability was presence-keyed — raised by the a11y-plan detector, re-read at the source by the orchestrator. a11y-plan.md:141-144 describes focusability "if it is not disabled" and the `DISABLED`/`ENABLED` states without saying how either is keyed, and no master states how Dioxus boolean attributes reach the DOM. Pointer clicks on a `disabled="false"` button still reached the Dioxus handler (probe: booked-msg present) — measured, not explained. Carried for the a11y-plan / arch detectors to disposition.
- **Expected amendments (from plan):**
  - arch §Standard Contracts → Test harness — `HarnessOptions` gains `font_ctx` and `incremental`: **carried** (Symbols bullet 1). Sites: `grep -n HarnessOptions .andromeda/architecture.md` → 1 hit (line 129, the Test harness contract); test-plan.md → 2 hits (84, 216).
  - arch §Occupied Resources → Names / crate edges — `blitz-tests → seven_guis` (dev), `seven_guis → blitz-test-harness` (native normal dep), seven_guis' native `dioxus-native` gains `woff`, the `seven_guis` stand module in §Existing Scopes: **carried** (Dependencies + Crates/modules bullets), plus the unplanned `seven_guis → blitz-traits` edge. Sites: architecture.md line 64 (§Stack Testing row lists blitz-tests dev-dependencies — `grep -n 'blitz-tests dev-dependencies'` 1 hit), line 164 (crate diagram), §Existing Scopes table row `| seven_guis | examples/seven_guis | Tasks … |` (`grep -n '^| seven_guis'` 1 hit); `grep -c 'blitz-tests → \|blitz-tests ->'` over the 7 masters → 0 hits (no edge-arrow notation exists to extend).
  - test-plan §3 "Construction" and §7 Builders "HarnessOptions" — the two fields and the stand boot: **carried** (Symbols + Harness/gate surface). Sites: test-plan.md:84 (§3 Construction), :216 (§7 HarnessOptions).
  - test-plan §2 "Font-dependent tests" — stand checks assert unconditionally under the bundled font: **carried** (Symbols — `stand::font_ctx`; Outcome — `task_title_measures_non_zero_under_the_bundled_font` has no skip path, census gate finds no `eprintln!`). Site: test-plan.md:52 (`grep -n Font-dependent` 1 hit).
  - test-plan §9 "Local baseline" — re-counted workspace tests: **carried** (Counts bullet 1). Site: test-plan.md:285 (`grep -n '416'` → 1 hit in test-plan.md, 6 in its sidecar).
  - design-system §Typography Loading — the headless stand registers bundled DejaVu Sans for every generic with system fonts off: **carried** (Symbols — `stand::font_ctx`, `DEJAVU_SANS`). Sites: design-system.md:127-128 and :409 (`grep -n DejaVu` 2 hits + `font_ctx|build_single_font_ctx` at 128).
  - layout-templates §Surface: desktop-native IA notes — the headless stand mounts a named task in TaskShell under `main#main` at the pinned 800 × 600: **carried** (Symbols — `stand`, Outcome — `each_lean_task_mounts_inside_task_shell`). Site: layout-templates.md:10 (`grep -n TaskShell` 1 hit; it cites `app.rs:114-164` / `202-209` / `274-319`, line numbers this chunk shifted: now `app.rs` `app` :69, `task_in_shell` :82, `TaskShell` :153-172, `HOME_CSS` :174, `SHELL_CSS` :273-328).
- **Coverage of new surfaces:**
  - `seven_guis::stand::{boot, boot_timer, options, font_ctx}` → validation n/a (no external input; checks use literal selectors only) · instrumentation n/a (test-time boot; telemetry install forbidden by plan constraint) · PII n/a · tests integ (stand_boot/counter/flight_booker/timer/crud.rs) · a11y n/a (no new UI element; stand markup unchanged) · tokens n/a
  - `HarnessOptions.font_ctx` / `.incremental` → validation n/a · instrumentation n/a · PII n/a · tests integ (stand_boot.rs — non-zero title under the bundled font; incremental false/true `dom_string` identity) · a11y n/a · tokens n/a
  - `TimerTicks` (timer tick seam) → validation n/a · instrumentation n/a · PII n/a · tests integ (stand_timer.rs, 3 tests) · a11y n/a (timer markup unchanged) · tokens n/a
  - `app::Task` (pub) + `app::task_in_shell` → validation n/a · instrumentation n/a · PII n/a · tests integ (stand_boot.rs mount/order) · a11y n/a (tree unchanged) · tokens n/a (CSS constants unchanged)
  - dioxus-native-dom falsy-`disabled` clearing → validation n/a · instrumentation n/a · PII n/a · tests integ (dioxus_falsy_disabled.rs: attribute absent / `:disabled` unmatched when false, present when true, toggled both ways; stand_flight_booker.rs) · a11y attribute + `:disabled`✓, focusability / Tab order NOT asserted ✗ (the fix leaves focusability unchanged — it was already parsed-bool keyed, `element.rs:629`; no test asserts a Dioxus control's focus/Tab order) · tokens n/a

## Deviations from intent
- **`seven_guis → blitz-traits` dependency (not in plan step 5).** Plan step 4 asks for a public `ColorScheme::Light` constant; no crate in seven_guis' direct graph re-exports `ColorScheme` (checked: dioxus-native, dioxus-native-dom, blitz-dom, blitz-test-harness re-exports). Added `blitz-traits = { workspace = true }` to the listed seven_guis manifest; already locked and already a transitive dep, so no lock stanza.
- **`stand::boot` installs a tick source for every task, including the timer.** The plan's "for the timer it provides the tick source" read narrowly would leave `boot(LeanTask::Timer)` on the real 100 ms `Delay`, which can fire between pumps and break the two-boot `dom_string` identity. An undelivered `TimerTicks` keeps elapsed at 0.0 s; `boot_timer` returns the handle.
- **Engine fix outside research's lists (widening)** — see the scope record below; it made plan step 6's flight-booker premise true rather than rewording the check.
- **Regression test outside research's lists (widening)** — `dioxus_falsy_disabled.rs`; red with the one-line fix reverted (`left: Some("false")`), green with it.
- Plan step 6's checks were written as specified; the counter, flight-booker, CRUD-create and timer-tick checks also loop `for incremental in [false, true]` (testing rules §Determinism).
- Scope record (`gate.py scope` → `scope: clean — changed 16 · listed 14 · recorded 2 (companion 0 · mechanical 0 · in-intent 0 · widening 2) · absorbed 0 · excluded 43`):
  - widening · `packages/dioxus-native-dom/src/mutation_writer.rs` · serves step 6 · word: "Widen: fix the engine — overseer under the founder's standing delegation: this is a real engine defect (Dioxus web semantics clear a falsy disabled), and v010-05's snapshot ('a disabled control reads disabled') rests on the engine reporting disabled truthfully — infrastructure ahead of the queue. Record the widening with this authority, pin it with the regression test, and note it in the report as an upstreamable fix." — delegate overseer, 2026-10-06 (PROVISIONAL — a boundary widening is ratified only by the founder's own word)
  - widening · `tests/blitz-tests/tests/dioxus_falsy_disabled.rs` · serves step 6 · word: "pin it with the regression test" — delegate overseer, 2026-10-06 (PROVISIONAL)

## Decisions & corrections
- **Decision (delegate overseer, 2026-10-06, PROVISIONAL):** widen to fix the engine's falsy-`disabled` write rather than surface-and-stop or amend the check's premise; record it as upstreamable.
- **Resolved open question:** `type_text` into a Dioxus controlled `input` reaches `oninput` (keyboard.rs `DomEventData::Input` → dioxus_document `NativeFormData`) — witnessed by stand_crud create (green) and the flight booker (`value "01.01.2026x"` → class `invalid`).
- **Sweep hazard:** in a Dioxus-native document, `attr(sel, "disabled").is_some()` was true for an ENABLED control before this chunk (`disabled="false"`); any check, a11y walker or snapshot reading disabled-ness by attribute presence over a Dioxus app mis-read every enabled control. After the fix the presence read is truthful for `disabled` and `checked` only — other Dioxus boolean attributes (`readonly`, `required`, `hidden`, `multiple`, `selected`, `open`, `autofocus`) still write `"false"` literally (`set_attribute_inner`, unchanged for them; not measured whether any stand control uses one).
- **Plan defect:** a plan step that names a type as a public constant must check the crate can name it — `ColorScheme` reached seven_guis only transitively.
- **Process (recurrence of a Tier 3 learning):** the project's Bash guard refused a `cat > file <<EOF` heredoc for the throwaway probe test (the learning "The project's Bash guards refuse a heredoc written to a file and a leading cd", 2026-10-06, Tier 3); the probe was written with the Write tool instead. A leading `cd` into the project root itself passed the guard (it allows a cd into the cwd).
- **Upstream doc drift seen, not this chunk's:** `config.rs:54` says `style_threading` "Defaults to Parallel" while the `#[default]` is `Sequential` (research §Patterns).
- The fork's CI wall nearly doubled (1104 s vs 561/642 s) on the push that changed `Cargo.lock`; the Actions-cache budget note (10.72 GB > 10 GB) is carried, not re-measured.

## Outcome
Acceptance criteria re-asserted against the diff:
- (tests) gate 1 `cargo test -p blitz-tests --locked --test stand_boot --test stand_counter --test stand_flight_booker --test stand_timer --test stand_crud` exits 0; each lean task has a check that boots it in TaskShell, reads initial state, drives one input-helper interaction and asserts — **met**.
- (layouts) `stand_boot.rs::each_lean_task_mounts_inside_task_shell` asserts `main#main > #task-shell`, `#task-header > #back-btn + #task-title`, `#task-header + #task-body` and the title text for all four — **met**.
- (design) `task_title_measures_non_zero_under_the_bundled_font` asserts width > 0 and height > 0 with no skip path in the package-alone build; the context is `build_single_font_ctx` (system fonts off) — **met**.
- (design / tests) two fresh boots and incremental false/true boots give equal `dom_string()` for all four — **met**.
- (tests) `stand_timer.rs`: 3 ticks → `Elapsed: 0.3s` + `width: 2.0%;`, Reset → `Elapsed: 0.0s`, 151 ticks cap at `Elapsed: 15.0s` with duration `15.0s`; census finds no `sleep|Delay|dispatch_recorded|eprintln!` in the checks — **met**.
- (a11y) `stand_flight_booker.rs` passes only when the bad date sets `invalid` and `.flight-btn` carries `disabled` (and carried none before), no colour asserted — **met after the engine fix** (red before it; see Spec claims disproved 1). blitz-dom `accessibility` stays on in the stand checks build (gate `-i accesskit` count 2) and `wuff` is in the resolve (gate `-i wuff` last line 1) — **met**.
- (security / arch) zero net requests across boot + click for all four tasks, with a fixture control proving the injected provider records (`stand_options_reach_the_injected_net_provider`); census finds no `blitz_net|reqwest|env::var|escher_telemetry::init|println!`; `Cargo.lock` gains no package stanza; audit leg green — **met**. The diff adds no net client, env read, port or package.
- (obs) `examples/seven_guis/src/main.rs` unchanged since d57a6c69; the boot smoke read `service.name=seven_guis` from a stand up for 10 s — **met**.
- (tests) `ci-leg.sh fast` exit 0; workspace 430 · 0 · 4 (predicted ≥ +8 passed: measured +14) — **met**.
- (tests / a11y) `ci-leg.sh doc` and `ci-leg.sh a11y` green, a11y list unchanged at three files — **met**.
- (arch) the operator's CI read on the pushed sha `1503df2e` reads `verdict: green`, its wasm job ("Build wasm examples") success — **met**.
- No capability claimed — `matrix.py show --chunk 2026-10-06-headless-stand` → `claimed by …: 0` — **met**.

Gates (/implement's second block, run dir `.andromeda/runs/2026-10-06T02-13-34-implement`, after the operator pass re-verified by the final HEAD's CI):
- `cargo test -p blitz-tests --locked --test stand_boot --test stand_counter --test stand_flight_booker --test stand_timer --test stand_crud` → green (exit 0 · lacks `running 0 tests` · lacks `FAILED`); first block red · `lacks FAILED` (the disabled premise) — fixed
- `cargo tree -p blitz-tests -e features -i wuff --locked | grep -c '^wuff v'` → green (exit 0 · last line 1)
- `cargo tree -p blitz-tests -e features -i accesskit --locked | grep -c 'blitz-dom feature "accessibility"'` → green (exit 0)
- `test -f examples/seven_guis/src/stand.rs && cat … | grep -c -E 'blitz_net|reqwest|env::var|escher_telemetry::init|println!'` → green (exit 1 · last line 0)
- `ls tests/blitz-tests/tests/stand_*.rs > /dev/null && cat … | grep -c -E 'sleep|Delay|dispatch_recorded|eprintln!'` → green (exit 1 · last line 0)
- `git diff d57a6c69… -- Cargo.lock | grep -c '^+name = '` → green (exit 1 · last line 0)
- `git diff --quiet d57a6c69… -- examples/seven_guis/src/main.rs` → green (exit 0)
- `cargo build -p seven_guis --bin seven_guis_native --locked` → green (exit 0)
- `RUST_LOG=info timeout 10 target/debug/seven_guis_native; test $? -eq 124` → green (exit 0 · contains `service.name=seven_guis`) — the boot smoke, WAYLAND_DISPLAY set
- `bash .github/scripts/ci-leg.sh fast` → green (exit 0); first block red · exit 1 (rustfmt diff in timer.rs) — fixed by `cargo fmt`
- `bash .github/scripts/ci-leg.sh doc` → green
- `bash .github/scripts/ci-leg.sh a11y` → green
- `bash .github/scripts/ci-leg.sh audit` → green
- `python -X utf8 …/gate.py hygiene` (leg operator) → hand-driven at the operator pass: exit 0 · `hygiene: clean` — `evidence/operator-14-hygiene.txt`
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (leg operator) → exit 0, pushed `d57a6c69..1503df2e` — `evidence/operator-15-push.txt`
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` (leg operator) → exit 0 · `verdict: green` · checks 16/16 · CI#37404017734 on `1503df2e` — `evidence/operator-16-ci.txt`
- Smoke: boot-path changed (app.rs / lib.rs / timer.rs); the plan's smoke entry ran as a P2 gate in both blocks, green.
- No `defer`, no `--skip`, no `round` / `live` legs.

Watches: none folded on this entry.

Outcome basis: the operator pass ran — Setup 4's commit list `1503df2e chore(2026-10-06-headless-stand): operator pre-CI commit, …` (the only commit from parent `d57a6c69`), and the final HEAD's CI run CI#37404017734 green on `1503df2e` (`evidence/operator-16-ci.txt`); /implement's P4 report (this conversation) for what only it holds; operator directive between implement and this report: "go — the operator pass, entries 14-16 in order, then stop for the wrap" (changed nothing in source). Post-implement artifacts: `evidence/operator-{14,15,16}-*.txt`.

Process hygiene (implement P4 census + the operator pass, re-measured with `pgrep -af 'seven_guis_native|zz_probe|stand_'` at /implement P4 → none running):
| process | started by | final state |
|---|---|---|
| `seven_guis_native` (gate 9, ×2) | /implement run | terminated (`timeout` at 10 s) |
| test binaries + the throwaway probe | /implement run | terminated (probe file deleted) |
| `ci-leg.sh fast` + `git push` (entry 15) | the operator pass | terminated (exit 0) |
| `ci.py conclusion` poller (entry 16) | the operator pass | terminated (exit 0, 37 polls over 1114 s) |
| `code-graph.py refresh` | this wrap (Setup 7) | terminated (exit 0) |
