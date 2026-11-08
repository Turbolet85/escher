# P5 review — 2026-11-08-what-can-i-do-here «What can I do here»

The plan to read in an editor: `.andromeda/cache/docs/2026-11-08-what-can-i-do-here/plan.md` (rev 3, 448 lines, about
290 lines of prose outside the gate fence). No plan stands in a chunk's folder; writing in that file changes no record
until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-08-what-can-i-do-here`
shows the chunk's records as a diff.

## Goal
A new verb, `actions`, the tenth instance verb, lists each actionable node of the current screen. For each node it gives
the stable id, the owner (where an owner element's handler serves it), `keyed` (author key or positional path) and the
verbs a call can name it by. For each verb it says whether the call would be accepted now or which refusal cause it
would return. It runs no step and changes nothing.

- **Node model:** the UI graph's (agent-visible node, optional owner).
- **Verbs:** the table's instance verbs with an `id` argument, so the read's code holds no list of verb names.
- **Causes:** each one comes from the per-verb target check the executor already applies before acting, factored out of
  `Session::execute` and shared, so a listed cause equals a real call's by construction. `VerbSpec` is unchanged.
- **Carriers:** the socket's `call`, the command line, MCP and a kept check (the founder's word, inputs#I4).
- **Actionable set:** one rule shared with `unkeyed_actionable`. Its listener leg moves from the mark to the exact
  current listeners.

## Top acceptance criteria
1. (matrix#v030-06) On the six served screens, both layout modes, every listed node's per-verb answer equals what
   `Session::run` returns for that call. This holds on the lean tasks at boot in full and on stated Cells and load-page
   subsets. It includes the three `disabled` controls, the flight booker's options `covered` with owner `flight-type`,
   and `crud-person-14` `off-screen` until a `scroll`.
2. The executor refuses exactly as before: the 68/9 click census, and the disabled, covered, off-screen, select, popup,
   shot and transform cases (entry 5).
3. The read changes nothing; its answer holds ids, owner ids, verb words, cause words and booleans only (a typed
   sentinel and `MASKED_VALUE` 0 times, no `NodeId`).
4. The `command` span records `verb` alone (`cause` when refused); `host_log` and `mcp_log` read 0 needles at `trace`;
   escher-driver keeps 2 `info_span!`.
5. `VERBS` fourteen, describe `[14, 14, 12, 3, 8, 4, 15]`, `tools/list` fourteen, `docs/capabilities.md` and README
   fourteen.

There are ten criteria in all.

## Touchpoints
- 2 new files: `packages/escher-driver/src/actions.rs` and `tests/blitz-tests/tests/stand_actions.rs`.
- 30 files to modify: `dioxus-native-dom` (`actionable.rs`, `lib.rs`); escher-driver (execute, schema, spec, command,
  json, describe, lib, mcp, and the test children); the blitz-tests shared modules and four checks; the seven_guis
  process checks; the cold-agent pipe; `docs/capabilities.md`; README.
- No engine, harness, manifest, lock or committed-check change (entry 9 guards it).

## Test commands (19 entries)
- **Unit:** dioxus-native-dom whole and the `actionable` filter; escher-driver.
- **Integration:** `stand_actions` (new); the executor witnesses (9 targets); the actionable, graph and probe witnesses;
  the span, MCP and replay checks; `cargo test -p seven_guis`.
- **Probes:** the no-engine guard; the silence census; the 2 `info_span!` count.
- **Stand and legs:** agent-run boot and `run stand`; `fast`, `doc`, `audit`.
- **Operator:** hygiene, push, `ci.py` green.
- **Baselines** (on the untouched tree):
  - 2 green (4 passed);
  - 4 red (no target yet);
  - 5 green (50 passed);
  - 6 green (22);
  - 7 green (12);
  - 9 green, its control red on an edited `Cargo.toml`;
  - 10 green vacuous, its controls silent green and `println!` red (`.andromeda/runs/2026-11-08T13-02-17-phase/controls.md`);
  - the other nine are seen green in the previous chunk's wrap.

## Concretization preview — v030-06 (the one cap claimed)
- **Now:** «On the stand, one command lists each actionable node on the current screen with its verbs and, for each that
  cannot be acted on now, why (disabled, covered, off-screen), agreeing with the refusal an actual call returns.»
- **Proposed:** «On the stand's six served screens, in both layout modes, the `actions` verb lists each actionable node
  (focusable, an interactive role, a current handler, or served by an owner) with the instance verbs of `VERBS` that take
  an `id`, and for each verb that cannot be taken now its closed cause word; for every listed node and verb on the four
  lean tasks at boot, and on the stated Cells and load-page subsets, the listed acceptance or cause equals what
  `Session::run` returns for that call — the three `disabled` controls, the flight booker's two options `covered` with
  owner `flight-type`, and after twelve Creates `crud-person-14`'s `click` `off-screen` until a `scroll` among them
  (`cargo test -p blitz-tests --locked --test stand_actions`).»
- This narrows nothing. «disabled, covered, off-screen» stay, with `fonts-unavailable` and `malformed` named only where
  a real call returns them. «agreeing with the refusal an actual call returns» becomes a per-node, per-verb equality
  against `Session::run`.

## Whose word stands behind it
- **The verb on every carrier:** the founder's word «Новый глагол actions (Рекомендую)», relayed verbatim by the overseer
  (inputs#I4). It is the boundary widening pattern `rul-izni57qv`, so the wrap halts on it once, as its answer card.
- **The shared executor check, the `keyed` fold-in and the exact-listener node rule:** the overseer's calls, delegate the
  overseer, 2026-11-08 (inputs#I4). The `keyed` field on the carriers is PROVISIONAL for the founder's own word in his
  batch at the epoch boundary. The other two are technical, with no boundary or contract change.

## Validation
- Mechanical checks clean: the fence parses (19 entries), planlint 0 hits, baseline `NOT SEEN` 0, research lists parsed
  (32 items), snapshot citations 0 unparsed.
- I1 reads drifted: the overseer ticked its «What can I do here» row ☐ → ☑ after the snap. The direction is unchanged;
  the wrap re-verifies it.
- Validation-1: aligned with the scope (rev 3).
- `sampler: off`.

## Beyond the plan
- The unpaged `actions` answer is a stated limit beside «An unpaged graph». Its owner is the same residual,
  `res-7cc5bi4k` (target: next), widened to name the `actions` answer, and the wrap records it there.

Apply? (yes / review / cancel)
