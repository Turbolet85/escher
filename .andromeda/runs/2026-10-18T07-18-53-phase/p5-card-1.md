# P5 review — 2026-10-18-one-virtual-clock («One virtual clock», v020-06)

Plan: `escher-0.2.0/chunks/2026-10-18-one-virtual-clock/plan.md` (304 lines, 14 steps, 17 gate entries, 9 criteria) · research: `research.md` · scope: `scope.md`.

## Goal
In a headless run, every engine time read comes from one clock that only the driver moves, so two shots of one state read the same pixels. The founder forks were answered by the operator as delegate, PROVISIONAL (inputs#I2):
- **F1 B:** `DocumentConfig` gains an optional clock, the wall clock by default, so windowed and embedder-built documents behave as today. The harness supplies a `VirtualClock` that is its own `time()`.
- **F2 A:** `advance --ms N` moves that clock by N on every session; `advanced_ms` reads N.
- **F2b A:** `time-unavailable` is retired, leaving eight causes.
- **F3 A:** the 500 ms double-click window reads the one clock.

## What changes (touchpoints)
- **blitz-dom:**
  - new `clock.rs` (`Clock`, `WallClock`, `VirtualClock`), re-exported from `lib.rs`;
  - `config.rs`: `DocumentConfig.clock`;
  - `document.rs`, `resolve.rs`: the fade stamp, its reads and its prune;
  - `events/pointer.rs`: the double-click window, pan and fling times;
  - `scrolling.rs`: the smooth-scroll and fling times;
  - `iframe.rs`: a sub-document inherits the clock.
  After the change, no `Instant::now()` or `SystemTime::now()` remains outside `clock.rs`.
- **blitz-test-harness `harness.rs`:** `time()` is the clock, `tick` moves it, and a new `advance_time(Duration)` moves it exactly.
- **escher-driver:**
  - `execute.rs`: `advance` moves the clock, then hands N to the app's step;
  - `session.rs`: the time step returns nothing;
  - `refusal.rs`: eight causes;
  - `schema.rs`, `schema_tests.rs`, `lib.rs`: the `screenshot` wall-time sentence retired, and `advance` and `click` help restated. `click`'s help now says that `type`'s own click counts toward a double click.
- **seven_guis `stand.rs`:** `timer_step` carries its sub-tick remainder (250 + 250 → 0.5 s).
- **New checks:**
  - `virtual_clock_engine.rs`: fade, double click and smooth scroll on a virtual clock, and the wall default;
  - `stand_virtual_clock.rs`: the CRUD list shot across one 800 ms wall sleep, then the fade moved by `advance`; and the double click by session time.
- **Restated checks:** `stand_act_timer`, `stand_act_spans`, `stand_mcp_tools`, `host_timer`, `cli_commands`, and the comments in `stand_screenshot_repeat` and `stand_non_lean_tasks`.

## Top acceptance criteria
1. `stand_virtual_clock`, both layout modes:
   - the post-scroll shot of `crud-list` is byte-identical after an 800 ms wall sleep with no `advance`;
   - `advance 600` and then `advance 100` each change it, and `advance 1000` leaves it as it was;
   - the post-scroll shot differs from the faded one (a thumb was painted);
   - the clock reads exactly 1.7 s.
2. The census entry prints 0: no wall-clock read in blitz-dom outside `clock.rs` (baseline red at 7; must-pass control green, `p5-controls.md`).
3. A document with no clock behaves as today: `virtual_clock_engine`'s wall case passes, and the `HtmlDocument` checks pass unchanged (`fragment_navigation`, `scroll_into_view_nested`, `scrollbars`, `scrollbar_drag`).
4. The double click: after a `type` and an `advance 500`, two `click`s select the word; `click`, `advance 500`, `click` leaves the caret. Deterministic across sessions.
5. Eight causes, every one returned by code. `advance --ms 0` / `60001` are still `malformed`. Spans keep their eight fields with no time or image. `host_log` and `mcp_log` show 0 needles.
6. `stand-repeat` 3 of 3, the a11y leg's counts unchanged, coverage 3 of 3, `fast` and `doc` green, and fork CI green on the pushed sha.

## Test commands (baselines on the untouched tree at f45bb570)
| # | entry | baseline |
|---|---|---|
| 1–3 | `cargo test -p blitz-dom --locked --lib` · `-p escher-driver` · `-p blitz-test-harness` | green |
| 4–5 | `--test virtual_clock_engine` · `--test stand_virtual_clock` | red — target absent (steps 11–12 write them) |
| 6 | eleven standing blitz-tests targets | green (20.35 s) |
| 7 | `cargo test -p seven_guis --locked` | green |
| 8 | wall-clock census (awk) | red, 7 hits = the reads steps 3–5 move; must-pass green |
| 9–10 | `ci-leg.sh fast` · `doc` | seen green at the last wrap |
| 11 | `ci-leg.sh stand-repeat` | green, 3 of 3 (282.67 s) |
| 12 | `ci-leg.sh a11y` | green |
| 13 | `ci-leg.sh coverage` | green, 3 of 3 floors |
| 14 | `ci-leg.sh audit` | green |
| 15–17 | operator: hygiene · pre-push gate + clean-tree push · `ci.py conclusion --sha HEAD --wait 1800` | leg operator |

The pre-CI commit subject: `chore(2026-10-18-one-virtual-clock): operator pre-CI commit, for the run this chunk's verdict reads`.

## Concretization preview — the capability this chunk claims
**v020-06 · One virtual clock** (`method: integration`)
- **Current acceptance:** "In a headless run that paints overlay scrollbars, two screenshots of the same state taken right after a scroll, with wall time passing between them and no driver advance, read the same pixels, and the fade moves only when the driver advances time."
- **Proposed concrete form** (never weaker):
  - Stand proof — `cargo test -p blitz-tests --locked --test stand_virtual_clock` passes in both layout modes. On a held CRUD session in the blitz-tests build, which paints overlay scrollbars, after twelve Creates and a driver `scroll` to the last row:
    - `crud-list`'s shot after an 800 ms wall-clock sleep with no `advance` is byte-identical to the shot right after the scroll;
    - `advance --ms 600` changes it, `advance --ms 100` changes it again, and `advance --ms 1000` leaves it unchanged;
    - the post-scroll shot differs from the faded one, so a thumb was painted;
    - the session clock reads exactly 1.7 s.
  - Engine census — the census entry prints 0: no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`.
  - Wall default — `cargo test -p blitz-tests --locked --test virtual_clock_engine` passes in both modes: the fade, double click and smooth scroll move only with the document's virtual clock, and a document with no clock still reads the wall clock.
- What it adds over today's form: the census makes "every time the engine reads" checkable by construction, and the double-click reading proves the requirement beyond the fade.

## Forks recorded PROVISIONAL (delegate operator, inputs#I2)
F1 B · F2 A · F2b A · F3 A, for the founder's epoch-boundary batch, beside the engine-debts and the cursor-fix batches. The wrap records them under «Provisional discharge». None widens a boundary.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | CARRY: the merge surface of this chunk's engine change (`packages/blitz-dom/src/{config,clock,document,resolve,iframe,scrolling}.rs`, `events/pointer.rs`), flagged upstreamable, no upstream pull request | the existing entry «Upstream sync ahead of visual lint and real-app readiness» |

Apply? (yes / review / cancel)
