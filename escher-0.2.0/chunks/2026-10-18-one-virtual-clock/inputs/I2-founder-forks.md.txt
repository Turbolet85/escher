# Founder forks — 2026-10-18-one-virtual-clock («One virtual clock», v020-06)

The founder is away. The operator answers these in his stead (inputs#I1), and each answer is recorded as a delegate's, PROVISIONAL, for the founder's epoch-boundary batch (`.andromeda/playbook.md`, «Provisional discharge»). Each fork below is his because it changes what the engine does for every document, or what an agent meets. None of the options widens a boundary: no new carrier, verb, argument, field, log field or exit. The evidence for each claim is in `escher-0.2.0/chunks/2026-10-18-one-virtual-clock/research.md` and `scope.md`.

Answer with one letter per fork (e.g. `F1 B · F2 A · F2b A · F3 A`). A free-text answer is fine too.

---

## F1 — Where a document's time comes from

**The fork.** The engine reads the wall clock directly in five places: the overlay scrollbar fade (`packages/blitz-dom/src/document.rs:1870,1879,1887`, `resolve.rs:71`), double-click detection (`events/pointer.rs:394-407`), the smooth-scroll start and frame (`scrolling.rs:515,896`), the fling frame (`scrolling.rs:871`) and pan timing (`pointer.rs:267,717`). The requirement is that in a headless run all of them read one clock that only the driver moves. CSS animations and transitions already read the time `resolve(time)` is handed (`stylo.rs:135`); the harness hands its own `time`, and the windowed shell hands wall seconds (`blitz-shell/src/window.rs:280-302`).

- **A — One document clock for every document: `resolve`'s time.** The document keeps the time its last `resolve` was handed, and every read above takes it. Activity is stamped at the next `resolve`, so the fade starts from the next frame rather than the event's instant. Windowed documents change too: the fade measures from frame times, and double-click compares the frame times of two clicks. An idle window that has not resolved between two clicks can read them as nearer together than they were.
  - *Engine change for every document:* yes, windowed included.
  - *Merge surface:* `document.rs`, `resolve.rs`, `events/pointer.rs`, `scrolling.rs`. A CARRY on «Upstream sync ahead of visual lint and real-app readiness».
  - *Risk:* windowed double-click and fade timing move off the event instant. That needs a windowed check this chunk does not have (the stand's windowed binary is measured only for the log needles).
- **B — A clock the embedder supplies, the wall clock by default (Recommended).** `DocumentConfig` gains one optional clock provider, on the same `Option<Arc<dyn …>>` shape as its net, navigation and shell providers (`config.rs:31-60`). With none supplied it reads `web_time::Instant`, exactly what every read does today. Every read above goes through it. The harness supplies a virtual clock whose time is the harness's own `time`, so `resolve(time)` and the engine's reads are one clock, moved by `tick` and the driver's `advance`.
  - *Engine change for every document:* an API addition. With no clock supplied, behaviour is unchanged: the same reads, now taken through the default.
  - *Merge surface:* as A, plus `config.rs` and a new `clock.rs`. A CARRY on the same upstream-sync entry.
  - *Why recommended:* it meets the requirement headlessly, which is the requirement's scope ("in a headless run"). It leaves the windowed engine as it is, and it follows the embedder-services pattern the architecture records ("Embedder services are injected, with no-op defaults").
- **C — The fade alone, through a harness override.** The harness pins the fade's clock, and every other read stays on the wall clock.
  - *Engine change for every document:* none beyond a test hook.
  - *What it fails:* "every time the engine reads". Double-click stays on the wall clock, and that does move driver readings (F3), so v020-06 could not be claimed whole.

**Dependences:** F3's options read "the one clock" as F1 defines it. Under F1 C, F3 can only be C. Under B, which technical reads move with the fade (all five, or the fade and double-click) is a technical fork for the operator. Under A, every moved read is a windowed change and so the founder's.

---

## F2 — What `advance` moves

**The fork.** Today `advance --ms N` hands N to the session's time step alone — only the Timer's session gets one. It reports `advanced_ms` as the time delivered to the app (`execute.rs:175-187`; whole 100 ms Timer ticks with the remainder dropped, so 250 reads 200 and 99 reads 0). The harness's clock stays where it was (`stand.rs:90-93`), and any other session refuses `advance` as `time-unavailable`. With nothing moving the instance's clock, a scrollbar thumb shown by a driver `scroll` would never fade, which fails v020-06's second clause ("the fade moves only when the driver advances time").

- **A — `advance` moves the held instance's one clock on every session (Recommended).** The clock moves by exactly N. The app's time step, where the session's caller handed one, then receives N. `advanced_ms` reads the clock's move, always N.
  - The stand's Timer step carries the sub-tick remainder, so the Timer reads the one clock: 250 then 250 shows 0.5 s, where today it shows 0.4 s. That is a stand change: `timer_step`, with its unit-test rows moving.
  - `advance` then answers on counter, flight booker and CRUD too, and `time-unavailable` is never returned (F2b).
  - *What an agent meets:* `advance` works on every task, and `advanced_ms` always equals the milliseconds asked.
  - *Checks that move:* `host_timer` (250 → `advanced_ms` 250), `cli_commands`, `stand_act_timer` (its "the animation clock is where it was" assertions become "the clock moved by N"), `stand_act_spans` and `stand_mcp_tools` (a refused `advance` becomes accepted, its `tool_call` `ending` `accepted`).
- **B — `advance` moves the clock only on a session handed a time step.** The stand host and the checks' `hold` hand every lean task a step (a clock-only step for the three that have no app time). A library session given none still refuses `time-unavailable`, so "all nine causes are returned by code" stays true. `advanced_ms` keeps its meaning, the time delivered to the app: the Timer keeps whole ticks with the remainder dropped, and the other tasks read N.
  - *What an agent meets:* the same `advance` on every stand task, the Timer's readings unchanged.
  - *Cost:* the Timer's app time and the engine's clock drift apart by the dropped remainder (250 moves the clock 250 and the Timer 200). The app's time is then not strictly the one clock.
- **C — Leave `advance` as it is.** In a driver session nothing moves the instance's clock: a thumb shown by a scroll stays at full opacity until the session ends. The repeat-shot half of v020-06 holds, but "the fade moves only when the driver advances time" cannot, so the cap could not be claimed whole.

**Dependences:** F2b exists only under A. Under B or C, `time-unavailable` stays as it is.

### F2b — What becomes of `time-unavailable` (under F2 A)
- **A — Retire it (Recommended).** The refusal set closes at eight causes. `help`, the self-description and the MCP listing stop naming it, and "every cause is returned by code" stays true at eight.
  - *What an agent meets:* one fewer refusal, and every one it reads in `help` can actually occur.
  - *Ripple (code):* `refusal.rs`, `execute.rs`, the schema's cause table and its pins, `stand_act_spans` (its `cause="time-unavailable"` line), `stand_act_timer`, `stand_mcp_tools`, `cli_commands`.
  - *Ripple (docs, at the wrap):* the count "nine" in CLAUDE.md, the architecture, the security, obs and test plans, and the leaves.
- **B — Keep it in the table, never returned.** The cause set stays nine, and the recorded fact becomes "eight of nine returned by code". An agent reading `help` meets a cause it can never be given, against the tool's rule that it states its own limits truly (2026-10-10-driver-cli).

---

## F3 — Two driver clicks on one point, on the one clock

**The fork.** The engine counts a second mousedown within 500 ms and 2 px as click 2 (`pointer.rs:394-407`). Click 2 selects the word in a text field (`pointer.rs:548-556`), selects all in a password field (`pointer.rs:647-651`) and sends the app a `DoubleClick` (`pointer.rs:968-974`); the stand's Circle Drawer opens its dialog on one (`examples/seven_guis/src/tasks/circle_drawer.rs:72`). Today wall time decides, so two driver `click`s on a text field read differently by how fast the agent sends them. On a clock only the driver moves, two clicks with no `advance` between them are 0 ms apart.

- **A — The double-click window reads the one clock (Recommended).** A second `click` on the same point within 500 ms of the session's time is a double click, whatever wall time passes. Two single clicks need an `advance` of at least 500 ms between them. `click`'s help says so, in the program's own fixed text inside the ratified carrier.
  - *What an agent meets:* deterministic clicks. Clicking a button twice still runs its click handler twice (the counter increments twice), now plus a `DoubleClick` the app may ignore. A second click on a text field selects the word, and Circle Drawer's dialog opens by two clicks as it does by hand today.
  - *Checks:* `stand_non_lean_tasks.rs:82` and `password_caret_click.rs:107` become deterministic rather than wall-window-dependent; their comments are restated.
- **B — Every driver `click` lands as a first click.** The session resets the count before each click. Deterministic, but a double click is unreachable through the driver: Circle Drawer's dialog cannot be opened by an agent, and a second click on a text field never selects a word.
- **C — Leave the double-click window on the wall clock.** Two driver clicks still read by how fast they arrive. This fails "every time the engine reads", so v020-06's requirement is met for the fade alone. The limit is stated in `click`'s help.

**Dependences:** under F1 B, the window reads the document's clock: virtual in a harness, the wall clock in a window, so windowed double-click is unchanged. Under F1 A, it reads frame times in every document. Under F1 C, only C is open.

---

**Recommendation in one line:** `F1 B · F2 A · F2b A · F3 A`. The headless engine reads one clock, the driver's `advance` moves it on every task, the refusal set stays truthful at eight, and two clicks are a double click by the session's time. Windowed behaviour is untouched.

---

## Answer (the operator, as the founder's delegate — PROVISIONAL, 2026-10-18T07:36Z)

Verbatim:

> Founder forks, answered in his stead (delegate operator, PROVISIONAL): F1 B · F2 A · F2b A · F3 A, each the recommended option for the reason the file argues (B keeps the windowed engine unchanged; the refusal set stays truthful at eight; two clicks read by the session time). Record them in founder-forks.md and go on with P4 and P5.

| Fork | Answer | Authority | Status |
|---|---|---|---|
| F1 — where a document's time comes from | B — a clock the embedder supplies through `DocumentConfig`, wall by default; the harness supplies a virtual clock | delegate operator | PROVISIONAL, for the founder's epoch batch |
| F2 — what `advance` moves | A — the held instance's one clock on every session, by exactly the milliseconds asked; `advanced_ms` reads that move; the Timer's step carries its remainder | delegate operator | PROVISIONAL |
| F2b — `time-unavailable` | A — retired; the refusal set closes at eight causes | delegate operator | PROVISIONAL |
| F3 — two driver clicks on one point | A — the double-click window reads the one clock | delegate operator | PROVISIONAL |

The technical forks, answered by the operator in the P4 dialog (2026-10-18): one 800 ms sleep between the two shots, the no-sleep rule's single recorded exception; the proof on the stand's CRUD list, in `tests/blitz-tests/tests/stand_virtual_clock.rs`; all five engine reads (fade, double click, smooth-scroll start and frame, fling frame, pan times) through the document's clock.
