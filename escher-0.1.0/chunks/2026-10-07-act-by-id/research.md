# Codebase Research — 2026-10-07-act-by-id

## Scope
- **Depth:** deep · **Reads:** 27 (13 whole or sectioned files through Read, 14 line ranges) · **Globs/Greps:** 14 · **Graph queries:** 6 (trail `tree-query-2026-10-07-act-by-id.json` in the run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its one Session Addition included (a `timeout N` boot smoke reads as the gate's own bound; not applicable — no entry here bounds a stay-up process); `.claude/rules/testing.md` — its three Session Additions, the second applied (no check is driven by a deleting key unless it is the check of that key's own platform behaviour)
- **Platform issues consulted:** none — no runner-only bullet was folded from a CI verdict (Setup 5a's one run was in progress, not red), and no entry reads CI outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `packages/escher-driver/src/session.rs` (full) — `Session { label, harness }` (`:23-26`); `start(label, boot)` (`:35-46`); `harness` / `harness_mut` (`:54-61`); `act(step) -> Result<Settled, SessionError>` maps a not-quiet instance to `SessionError::NotSettled` and does not roll back (`:70-78`). No field for a time step, no id memory.
- `packages/escher-driver/src/command.rs` (`:1-232`) — `Command` is a public enum with public fields (`:103-130`), so a `Command` can be built without `validate`; `validate(call) -> Result<Command, Refusal>` (`:161-202`); `Key` has 12 variants, `backspace` and `delete` among them (`:30-55`).
- `packages/escher-driver/src/schema.rs` (`:1-300`) — the acting result is `settled` · `busy` (present only when not settled) · `added` · `removed` · `changed` (`:147-183`), `advance` adds `advanced_ms` "never more than `ms`" (`:185-190`, `:256-267`); `NODE_FIELDS` is nine names (`:33-35`); `press` "presses one key on the focused element" (`:236-254`); `snapshot` returns one field `text` (`:192-202`).
- `packages/escher-driver/src/refusal.rs` (the cause texts) — `not-found` means "no element on the screen reads the id" (`:60`); `stale` needs the session to have read the id earlier (`:61-63`); `time-unavailable` means "the app this session holds gives it no way to move its time" with the remedy "start a session whose host supplies a time step" (`:70`, `:96-99`).
- `packages/escher-driver/src/error.rs` (`:8-40`) — twelve `SessionError` variants, all fixed strings; `NotSettled(Busy)` is the twelfth (`:36`).
- `packages/escher-driver/src/host.rs` (`:30-129`) — `serve(state_dir, session: Session)` takes the session by value (`:35`) and passes only `session.label()` to `answer_until_stop` (`:48`), which loops on `accept` (`:107-123`): the held instance is unreachable while the host serves, and no request can touch it.
- `packages/escher-driver/src/wire.rs` (its constants and grammar) — `MAX_REQUEST_BYTES = 64` (`:16`), `MAX_REPLY_BYTES = 128` (`:20`), `IO_BOUND` 2 s (`:13`), requests `hello v1` and `stop v1` only (`:25-28`, `:158-159`).
- `packages/escher-driver/src/lib.rs` (full) — eight private modules, the re-export list (`:39-49`), crate doc ending "no command runs through a session yet" (`:24-25`).
- `packages/escher-driver/Cargo.toml` (full) — two dependencies, `blitz-test-harness` and `dioxus-native-dom`, no feature named, no `[features]` table.
- `examples/seven_guis/src/session_host.rs` (full) — argv check before anything boots (`:24-32`); the Timer's handle bound at `:35`, stored at `:39`, dropped at `:45` after `serve` returns; nothing delivers a tick.
- `examples/seven_guis/src/stand.rs` (`:56-103`) — `options`, `boot`, `boot_timer` returning `(Harness, TimerTicks)`; the module is where the stand's boot and the tick handle meet in library code.
- `examples/seven_guis/src/tasks/timer.rs` (`:10-65`) — `TimerTicks::deliver(n)` queues ticks and wakes (`:20-26`); one tick adds `0.1` to `elapsed`, capped at the duration (`:57-60`).
- `examples/seven_guis/Cargo.toml` (`:15-50`) — `escher-driver` and `blitz-test-harness` are native-only dependencies (`:29-35`); the `escher-session` binary (`:45-47`).
- `packages/blitz-test-harness/src/harness.rs` (`:75-76`, `:140-155`) — `Harness.doc` is a public field, so a session reaches `DioxusDocument::snapshot` as `harness.doc.snapshot()`; `time()` reads the animation clock, `tick(dt)` adds to it and pumps.
- `packages/blitz-test-harness/src/input.rs` (`:1-28`, `:95-232`) — `click(selector)` is `center_of(selector)` then `click_at(x, y)` (`:97-100`); `click_at` dispatches pointer down and up, then pumps (`:103-108`); `press(key)` / `press_with(key, modifiers)` take `keyboard_types::Key` and `Modifiers` (`:201-218`); `type_text(text)` presses one character key per `char` (`:221-225`).
- `packages/blitz-test-harness/src/inspect.rs` (`:50-76`) — `layout_rect_of(node_id)` reads `absolute_position` and `final_layout` and **panics** on a node that does not resolve (`expect("node does not exist")`, `:59`).
- `packages/blitz-test-harness/src/lib.rs` (full) — re-exports `Harness`, `HarnessOptions`, the four event builders, `Rect`, and the settle types; it does not re-export `keyboard_types::Key`, `Modifiers` or `UiEvent`.
- `packages/blitz-test-harness/src/settle.rs` (its types) — `Busy { Render, Layout, Loads }` (`:21-28`), `Settled { passes, animating }` (`:32-38`), `settle()` (`:151`).
- `packages/dioxus-native-dom/src/snapshot.rs` (`:1-135`) — `Snapshot { roots }`, `SnapshotNode { id, role, name, state, bounds, children }`, `NodeState { enabled, checked, value, focused }`, `Snapshot::get(id)` (`:74-76`); a node's id is its accessibility node's `author_id` (`:124-129`), read through `get_node`.
- `packages/dioxus-native-dom/src/snapshot_diff.rs` (`:1-110`) — `SnapshotDiff { added, removed, changed }`, `is_empty()` (`:27-29`), `DiffNode { id, parent, role, name, state, bounds }` (`:34-47`) — with `NodeState`'s four fields that is the schema's nine `NODE_FIELDS`.
- `packages/dioxus-native-dom/src/lib.rs` (its re-exports) — `NodeId` is re-exported (`:41`); the snapshot items stand behind `#[cfg(feature = "accessibility")]`.
- `packages/dioxus-native-dom/Cargo.toml` (`:12-24`) — `accessibility = ["blitz-dom/accessibility", "dep:accesskit"]`; it forwards no `tracing` feature.
- `packages/blitz-dom/src/events/pointer.rs` (`:538-549`, `:600-709`) — a pointer event on a text input focuses the hit node (`:547`); `handle_click` stops at an element carrying `disabled` by presence (`:635-638`), toggles a checkbox or radio and focuses it (`:645-695`).
- `packages/blitz-dom/src/node/text.rs` (`:296-336`, `:360-372`, `:428-436`) — the `Key::Backspace` arm sits under `#[cfg(not(target_os = "macos"))]` (`:329-331`); `Key::Delete` is not gated (`:320`); `apply_apple_standard_keybinding` is compiled on every platform (`:368`) and its `"deleteBackward:"` arm calls `backdelete` (`:430-433`).
- `packages/blitz-dom/src/layout/construct.rs` (`:436-462`) — special element data is built for `textarea`, the text-entry input types, `checkbox` and `radio`, and for no `range`.
- `packages/blitz-dom/src/document.rs` (`:1665-1720`) — `set_focus_to(node_id)` and `clear_focus()` are public.
- `tests/blitz-tests/tests/session_common/mod.rs` (full) — `hold(task, incremental) -> (Session, TimerTicks)` (`:33-43`); `change` drives the harness by selector (`:46-57`); the process-side helpers (`state_dir`, `host_command`, `Host`).
- `tests/blitz-tests/tests/stand_settle.rs` (`:36-70`) — the Timer step: `timer.act(|_| ticks.deliver(151))` reads `Elapsed: 15.0s` on return (`:50-55`); the Reset step clicks and delivers inside one closure (`:66-70`).
- `tests/blitz-tests/tests/stand_session_state.rs` (`:60-92`) — the standing proof that a click on a text input followed by `type_text` sets its value (`click("#crud-name")`, `type_text("Ada")`, value reads `Ada`, `:83-88`).
- `examples/seven_guis/tests/host_log.rs` (its spawn and needle lines) — `RUST_LOG=trace` at `:79`, needles searched at `:146-155`.

## Graph impact (from the code-graph query; plane `rust`)
- **`Session::act`** — 7 call sites, all in `tests/blitz-tests/tests/stand_settle.rs` (`:51`, `:58`, `:66`, `:94`, `:116`, `:173`, `:329`), none in a library or a binary. A new entry point beside it leaves them untouched; a change to `act`'s return type edits all seven.
- **`Session::start`** — 10 reference sites outside the crate in six files (query 5's row for the definition in `session.rs`; the crate's other `start`, the client's, has 9 in three) (`examples/seven_guis/src/session_host.rs:36`, `tests/blitz-tests/tests/session_common/mod.rs:41`, `stand_session_lifecycle.rs:101`, `stand_session_quiet.rs:93`, `stand_session_state.rs:61` · `:78` · `:105` · `:137`, `stand_settle.rs:171` · `:326`) and one unit test (`session.rs:96`). A time step added as a new parameter of `start` threads through all of them; an additive constructor or setter threads through none.
- **`Session::harness` / `harness_mut`** — 51 call sites in six test files; the checks drive the held harness by selector today. Nothing in this chunk needs them to change.
- **`validate`** — 7 call sites, all unit tests in `command.rs`, plus the re-export: nothing outside the crate calls it, and nothing calls it before running anything.
- **`serve`** — called by `session_host.rs:44` and two ignored host children (`stand_session_lifecycle.rs:103`, `stand_session_quiet.rs:101`).
- **Crate edges** — `escher-driver → blitz-test-harness`, `escher-driver → dioxus-native-dom`; inbound `seven_guis → escher-driver`, `blitz-tests → escher-driver`. No other crate reaches it.
- **External surface of escher-driver** (query 5) — `Session`, `harness_mut`, `harness`, `stop`, `attach`, `start` (both), `SessionError` with four of its variants, `act`, `serve`, and the `Hello` / `Started` fields. No schema item (`Command`, `Call`, `VERBS`, `validate`, `Refusal`, `Cause`) is referenced outside the crate: the executor is their first consumer.

## Patterns detected
- **A click by node is the selector click's own arithmetic** (`input.rs:97-108`, `inspect.rs:57-74`): `click(selector)` resolves a node, takes the centre of its border box and calls `click_at`. Resolving the node from a stable id instead of a selector and taking the same centre gives the same pointer events — the caller supplies an id and no coordinate. The centre must be read through `get_node`, not `layout_rect_of`, which panics on an unresolved node.
- **A click on a text input focuses it, and typing follows focus** (`pointer.rs:547`; `stand_session_state.rs:83-88`): `type` by id is a click on the element, then `type_text`. The standing check proves it for `crud-name` and `crud-surname`.
- **An id resolves through `element_ids()`** (`dioxus_document.rs:215`): it returns `(NodeId, String)` pairs in document pre-order, computed on demand; there is no stored index. The executor's lookup is a linear search for the first pair whose string equals the id; an id matching none is the `not-found` case, whose stated meaning is exactly that (`refusal.rs:60`).
- **The diff is two snapshots and one call** (`snapshot.rs:83`, `snapshot_diff.rs:60`): `harness.doc.snapshot()` before the step, the step through `Session::act`, `snapshot()` after, `before.diff(&after)`. `DiffNode` and `NodeState` together carry the schema's nine node fields with nothing to map.
- **The time step cannot live in the binary** (`host.rs:35`, `:48`, `:107-123`): `serve` owns the session and blocks, so whatever moves time is handed to the session before `serve`. The stand's mapping of milliseconds onto ticks therefore sits where both the host binary and an in-process check can reach it — `seven_guis::stand`, which already returns the tick handle (`stand.rs:79-83`).
- **The crate cannot name a key today** (`blitz-test-harness/src/lib.rs`, `input.rs:13`): `Harness::press_with` takes `keyboard_types::Key` and `Modifiers`, which neither of escher-driver's two dependencies re-exports. `click_at(f32, f32)` and `type_text(&str)` need no such type. Running `press` needs either a re-export from blitz-test-harness or a third dependency on the driver's manifest (`keyboard-types` is already a workspace dependency, so the lockfile gains no package either way).
- **Backspace has a platform-neutral path the harness does not use** (`text.rs:329-331`, `:368`, `:430-433`; `blitz-traits/src/events.rs:71`; `blitz-dom/src/events/driver.rs:197`, `:257`): on macOS the key arm is compiled out, while `UiEvent::AppleStandardKeybinding("deleteBackward:")` reaches `backdelete` on every platform (`target_os` occurs 0 times under `packages/blitz-dom/src/events/` and `packages/blitz-traits/src/` — `grep -rn target_os`). A `press backspace` that dispatches that binding on macOS deletes there with no edit to the engine's editor. The equality "the binding deletes one character on macOS" is not measurable on this host; its witness is the fork's macOS CI leg.
- **No range interaction model** (`construct.rs:441-459`; `grep -rn -i '"range"' packages/blitz-dom/src packages/dioxus-native-dom/src` → 2 hits: the Slider role at `accessibility.rs:272` and a snapshot unit-test fixture): a range input gets a role and a value reading and no input handling. Read from the code; no range input was driven.
- **The feature adds no package** (`cargo tree -p seven_guis -e normal --locked -i accesskit`; `-i accesskit_xplat`; `-i accesskit_winit`): `accesskit 0.25.0` is already in the graphs of `escher-driver` and `seven_guis` through `blitz-dom`, because blitz-test-harness names `blitz-dom/accessibility` (`packages/blitz-test-harness/Cargo.toml:15`). Naming `dioxus-native-dom/accessibility` in escher-driver turns on that crate's own `dep:accesskit` and compiles its snapshot modules into both seven_guis binaries; no platform adapter is in the stand's graph (`accesskit_xplat` and `accesskit_winit` "did not match any packages") and the feature forwards none — the adapter comes only through dioxus-native's and blitz-shell's own `accessibility` features, which seven_guis does not name (`cargo tree -p seven_guis -e features --locked -i dioxus-native-dom` lists `prelude`, `system-fonts`, `vello-hybrid`, `woff`).
- **Counts this chunk extends** — escher-driver unit tests 25 in six files (`grep -c '#\[test\]' packages/escher-driver/src/*.rs`: command 5 · error 3 · refusal 3 · schema 4 · session 2 · wire 8); `stand_*.rs` files 20 (`ls tests/blitz-tests/tests/stand_*.rs | wc -l`).

## Conventions to follow
- **Private module, re-exported at the root, every public item documented** (`lib.rs:27-49`, `#![deny(missing_docs)]`).
- **Fixed strings in every error and refusal; nothing a call supplied in any of them** (`error.rs:10`, `refusal.rs:46-99`); no `{:?}` of a `Command`, `Call` or `ArgValue` anywhere outside a test's own row index.
- **Unit tests inline at the end of the source file, each table with its row count asserted and a row index as the only assertion message** (`schema.rs:277-300`).
- **One behaviour per `stand_*.rs` file, `for incremental in [false, true]`, booted through `seven_guis::stand`, shared helpers from `session_common` and `common`** (`stand_settle.rs:40-43`).
- **The host binary checks its argv before anything boots and writes nothing to stdout** (`session_host.rs:24-32`).

## New files to create
- `packages/escher-driver/src/execute.rs` — the executor: a validated command run on a session, its typed outcome, the id lookup
- `tests/blitz-tests/tests/stand_act*.rs` — the driver-action checks on the stand, one behaviour per file

## Files to modify
- `packages/escher-driver/Cargo.toml` — the `dioxus-native-dom` feature named
- `packages/escher-driver/src/lib.rs` — the new module, its re-exports, the crate doc
- `packages/escher-driver/src/session.rs` — the time step a caller hands the session
- `packages/blitz-test-harness/src/lib.rs` — the key types re-exported for the driver
- `packages/blitz-test-harness/src/input.rs` — a helper that dispatches the delete binding
- `examples/seven_guis/src/stand.rs` — the stand's mapping of milliseconds onto Timer ticks
- `examples/seven_guis/src/session_host.rs` — the host hands that step to its session
- `tests/blitz-tests/tests/session_common/mod.rs` — a held session that carries the time step

  The fork was answered "in process" (the operator, 2026-10-07, at the P4 forks), so the list was narrowed:
  the seven socket-branch files it first carried — escher-driver's wire, host, client and error sources,
  seven_guis' `host_log` check and its shared module, and `stand_session_lifecycle` — are read and left
  as they are. `Cargo.lock` is expected unchanged (no package enters the graph); a `--locked` build is its witness.
  Caller threading: no existing signature changes if the time step is additive — `Session::start`'s 10
  outside reference sites and `Session::act`'s 7 are left as they are (Graph impact).

## Open questions
- Where a command runs — in process on a `Session`, or across the session socket (scope's fork; the wire's 64- and 128-byte bounds hold neither an id of 1024 bytes nor a diff) → blocks: plan-decision. **Answered (the operator, 2026-10-07, at the P4 forks): in process.**
- What `advance` does with a remainder below one tick (100 ms): dropped per call, or carried to the next call → blocks: plan-decision. **Closed at P4 as a decisive lean, not asked:** the schema states `advanced_ms` is "never more than `ms`" (`schema.rs:189`), and a carried remainder makes a later call move more than it asked — so whole ticks per call, the remainder dropped.
- What this chunk does about a range input, which no input can change (CARRY 2, second half): measured and re-homed, or built → blocks: plan-decision. **Answered (the operator, 2026-10-07, at the P4 forks): measured through the driver as an agent would drive it, the reading recorded with its fixture; no owner named in the plan — the owner is decided at the wrap.**
