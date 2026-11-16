# P5 review — 2026-11-16-errors-with-the-action · «Errors with the action»

**The plan:** `.andromeda/cache/docs/2026-11-16-errors-with-the-action/plan.md` (rev 4) — open it in an editor. No plan
stands in a chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-errors-with-the-action` is a diff of the
chunk's records.

## Goal
An acting call's answer (`click` · `type` · `press` · `advance` · `scroll`) gains `caused`, written only when the step
caused something. An item is `kind` (`panic` · `error` · `log`), `level`, `id`, `owner`, `event` and `count`: closed
words, ids and counts, never a payload, message or field value. That is F1 Option 1, the delegate's answer, PROVISIONAL
until the founder's word at the Epoch 5 boundary (inputs#I2).

Your technical answers, built as given:
- **Panics** are caught around the step and its settle, and the held instance is kept, with a check proving that later
  calls read right. The fallback is to end the session, never a new cause.
- **A handler `Err`** is read from dioxus-core's root default error boundary (inputs#I3).
- **Logs** are counted at every level on the app's and dioxus-core's targets by a capture in escher-telemetry beside
  the sink. The sink's output is unchanged.

## Top acceptance criteria
- matrix#v030-10 on a fixture page in both layout modes:
  - a panicking click, an `Err` click, a logging click, a logging `type` and a press Enter each carry `caused`, tied
    to the handler;
  - the same field crosses `serve_mcp` and a kept check, which replays green.
- After a caught panic the session still answers, and a following click acts and carries no `caused`.
- No answer holds a byte of a payload, an `Err` text or a log message. A refused call carries nothing.
- The `command` span keeps its eight fields, the sink prints what it printed, and `host_log` / `mcp_log` read 0 needles.
- An answer that caused nothing reads byte for byte as before: the committed Counter checks and the cross-OS digest
  table do not move.

## Touchpoints
- **New:** `escher-telemetry/src/capture.rs`, `dioxus-native-dom/src/boundary.rs`, `escher-driver/src/caused.rs`,
  `blitz-tests/tests/stand_act_errors.rs`.
- **Modified:** the driver's run path, schema, written form, MCP listing, explore and their tests;
  `escher-telemetry/src/lib.rs` (the filter moves onto the sink layer, the capture layer beside it); the reached
  record's per-step list; `fixtures/mod.rs`, `stand_explore.rs` and `stand_mcp_tools.rs`.

## Test commands (13 entries)
- **Unit:** escher-telemetry, dioxus-native-dom `reached::`/`boundary::`, escher-driver.
- **Integration:** `stand_act_errors` (new) and the six companions. The telemetry process checks and
  `host_log`/`mcp_log` sit beside them.
- **Legs:** `fast` · `stand-repeat` · `coverage`.
- **Operator pass:** hygiene, push, CI.
- **Baselines** (on the untouched tree): entries 1–3 and 5–7 green; 4 red on the target this chunk writes; 8–10 seen
  green at the last wrap.
- **CI:** `b1ce8ab89` (HEAD) re-read before this card: green, 17/17, wall 1997 s (CI#43805443417).

## Concretization preview — v030-10
- **Now:** «An action that causes a panic, an error or a handler log returns it in that action's answer, tied to the
  node and the handler, within the boundary the founder sets.»
- **Proposed:** «On a fixture page, in both layout modes:
  - a `click` whose handler panics, a `click` whose handler returns `Err`, a `click` whose handler logs at `info` and
    `warn`, a `type` whose `oninput` logs at `debug`, and a `press` Enter on a focused logging button each return a
    `caused` field in that call's answer;
  - the field holds one item per kind and level, each tied to the stable id, owner id and event of the handler the
    step reached, with closed words and counts alone (`kind` · `level` · `id` · `owner` · `event` · `count`) — the
    boundary the delegate set on F1, PROVISIONAL until the founder's own word;
  - the same field crosses `serve_mcp` in process and a kept check that replays green.»

  The outcome is not weakened: every kind the capability names is returned, tied to the node and the handler. What
  narrows is what an item may hold, which is the founder's boundary (Option 1).

## Leans for your read
- **No P2 re-run.** P4 added two new files inside surfaces the pinned scope named: `capture.rs` in escher-telemetry
  and `boundary.rs` in dioxus-native-dom. I read this as no new surface — the row read already picked those crates'
  telemetry, scrub, init and reached-record rows — so P2 is not run again. Rejecting this lean costs a whole row-read
  re-run (about 20 min).
- **Validation-1.** The scope said escher-telemetry is «read, unchanged». Your capture answer (inputs#I2) overturns
  that, so the scope was amended at val-1, rev 4, as intent-incomplete.
- **The driver installs no subscriber.** Your answer had «the driver sets a scoped default». The plan puts that scoped
  default inside escher-telemetry, so the rows saying the driver installs no subscriber stay true (obs ·
  decision/driver-library-installs-no-subscriber; arch · decision/driver-library-side-effect-free). obs
  `decision/headless-stand-no-sink` still moves: an Expected amendment.

## Beyond the plan
1. PROVISIONAL — the `caused` field's content rests on the delegate's F1 answer (inputs#I2). The founder's own word is
   owed at the Epoch 5 boundary, listed by `rul-3ndrhilc` with every standing mark. founder-forks.md F1
   (`.andromeda/runs/2026-11-16T04-47-11-phase/founder-forks.md`) carries his options whole.
2. Owner for a part left out: what 0.3.0's capability statement says of `caused` (`docs/capabilities.md`) → the
   existing entry «Capability statement».
3. Owner for a part left out: v030-10's witness row in `stand_requirement_sweep` → the existing entry «Stand
   requirement sweep».
