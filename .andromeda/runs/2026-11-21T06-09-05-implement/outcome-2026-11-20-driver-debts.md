# Outcome — 2026-11-20-driver-debts · implement run 2026-11-21T06-09-05-implement

**Outcome: green** — the eight non-operator gates green; six one-shot controls hold. Next → the OPERATOR PASS:
entries 9 (`gate.py hygiene`), 10 (the push) and 11 (`ci.py conclusion`, the macOS/windows `--ctrl` witness), then
`/andromeda-wrap-session`.

## Files changed (22)
- driver: `packages/escher-driver/src/{schema,schema_tests,command,execute,json,json_tests,cli_tests,mcp_tests,fixture_tests,recording_tests,coverage}.rs`
- reached record: `packages/dioxus-native-dom/src/{reached,dioxus_document}.rs`
- checks: `tests/blitz-tests/tests/{stand_graph,stand_act_select,stand_handler_coverage,stand_mcp_tools,stand_replay,stand_act_spans}.rs`, `tests/blitz-tests/tests/session_common/mod.rs`, `examples/seven_guis/tests/cli_commands.rs`
- docs: `docs/capabilities.md`

## Deviations (with justification)
- The reached record's write after the dispatch is held by a drop guard (`PendingWrite`): a handler's panic unwinds
  out of `Runtime::handle_event`, so a plain write after it never ran and `stand_act_error_text` lost the panicking
  handler's `caused` tie. On unwind the dispatch is written as stopped — the innermost listener alone. Unit test
  `a_dispatch_a_handlers_panic_unwinds_records_the_innermost_alone`. Research had not read the unwind path.
- Plan step 7 (`stand_act_select`): "the same calls with no ctrl choose row 3 alone" — measured, the plain arrows
  choose row 3 alone and the closing `press space` then flips it out (empty set). The check pins both readings: after
  the arrows `[fx-three]`, after the space `[]`.
- Plan step 7 (coverage): "click a cell, then press enter" leaves the editor unfocused (a click opens it with
  `keyed_open` false), so the key never reaches it. The checks add a `click` on `cell-input` before the Enter, in
  process and through the binary.
- The MCP non-adjacent set: the matrix acceptance names `stand_mcp_tools` for it, so an MCP case on a `multiple`
  select fixture was added (`press_tool_calls_with_ctrl_build_a_set_of_rows_that_are_not_adjacent`), controlled by a
  sixth control (`control-2.toml`).
- A kept check read back gives its arguments in name order (serde_json's map), not schema order; the file still
  writes schema order. `recording_tests` compares the read-back arguments as sets by name and pins the file text's
  order (`key`, `ctrl`).
- `stand_act_spans` pins 41 calls (was 40): the added `graph --part 1` drive on CRUD.
- No `describe.rs`, `dioxus_document_tests.rs` or `test_capability_statement.py` edit was needed (no count moved).
- Scope record (`gate.py scope`: clean — changed 22 · listed 19 · recorded 3):
  - companion · `tests/blitz-tests/tests/session_common/mod.rs` · serves step 4 · self (exhaustive `Outcome` match; `graph_part`, `graph_of_part`, `press_ctrl` builders; `graphed` reads a paged graph whole)
  - companion · `tests/blitz-tests/tests/stand_act_spans.rs` · serves step 4 · self (exhaustive match; the graph-part span)
  - in-intent · `tests/blitz-tests/tests/stand_replay.rs` · serves step 7 · self

## Gates (run 2026-11-21T06-09-05-implement)
- firing 1: red 3 (`cargo test -p blitz-tests … --test stand_act_spans …` red · exit 0 — the spans call pin and the
  panic-unwind loss; `bash .github/scripts/ci-leg.sh fast` red · exit 0 — rustfmt; `bash .github/scripts/ci-leg.sh
  stand-repeat` red · last line — `stand_act_error_text`), the rest green.
- firing 2 (after the fix): `entries 11 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 3` — entries 1–8 green
  (fast 640.67 s, doc 33.86 s, stand-repeat 867.05 s).
- firing 3 (`--only 2,6`, after the MCP set test was added): both green (fast 581.32 s).
- `stand-repeat` was not re-run after the MCP set test was added (the operator's word: re-run entries 2 and 6); the
  new test ran green in entry 2 and in `fast`'s workspace tests.
- Operator entries 9 · 10 · 11: `not run — leg operator` — the operator pass's.
- No guard entry; no defer; no deferral.

## Controls (`evidence/control-1.txt`, `evidence/control-2.txt`)
`control: 5 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (ctrl-nothing, graph-uncut,
stop-unit, stop-stand, stop-binary) and `control: 1 holds · … · clone removed` (ctrl-nothing-mcp).

## Smoke
skipped — no boot-path change; `host_log`, `mcp_log`, `mcp_server`, `cli_commands` drove both binary roles as entry 4.

## Matrix
v040-05 · planned → implemented · ref the eight witness tests (matrix trail in the run dir).

## Process hygiene
| process | started by | final state |
|---|---|---|
| none | — | `census: none running` (`gate.py census --comm escher-session --comm seven_guis_nati --comm cargo --args escher-session`) |

## Decisions & corrections
- The operator paused the run mid-implement for another project's quiet host ("start NOTHING after it … wait for my
  word") and resumed with "Re-run entries 2 and 6, then the matrix refs, the report and the land" — the operator,
  2026-11-21.
- Bash guard refusals, each re-issued once: a `cd` moving the session cwd; a `cat` heredoc with a file target.
- A handler panic unwinds out of dioxus-core's dispatch: any "write after the dispatch" needs a drop guard.
