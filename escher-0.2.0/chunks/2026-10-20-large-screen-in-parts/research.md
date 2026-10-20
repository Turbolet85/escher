# Codebase Research — 2026-10-20-large-screen-in-parts

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 9 · graph queries 2 (rust plane)
- **Harness rules consulted:** none — no live leg in this chunk (the proof is `cargo test` targets and CI legs run
  bare; `.claude/rules/testing.md` auto-loaded and read whole, 9 Session Additions applied as reading rules)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (the
  take-up's CI#39397887372 was in progress, not red)
- **External inputs:** `inputs#I1` — the operator's fork routing: a boundary-widening or direction fork goes to the
  operator by dialog, marked as the founder's with its most conservative option named; technical forks by dialog

## Files inspected
- `packages/dioxus-native-dom/src/snapshot_text.rs` (full) — `SNAPSHOT_TEXT_BUDGET` = 10,000 (:16), documented as "a
  ceiling a caller compares against, not a behaviour" and "never truncates or pages" (:9-16); `to_text` writes one
  line per node in `nodes()` pre-order with the indent absolute to the root (:41-50, `write_line` :53-94), so any run
  of consecutive lines is byte-equal to the same lines of the whole text. Seven unit tests over hand-built snapshots.
- `packages/escher-driver/src/schema.rs` (full) — `SNAPSHOT` takes no argument and has one field `text` (:172-187);
  `VERBS` twelve (:460-462); `FieldKind::Count` already exists for counts (`width`, `pid`).
- `packages/escher-driver/src/schema/spec.rs` (full) — six `ArgKind`s (:6-19), each with `name()` and `bound()`
  (:21-45); none is "a whole number counted from 1" — `Milliseconds` is bound 1..=60000 and named for time.
- `packages/escher-driver/src/execute.rs` (:1-200) — `Outcome::Screen { text }` (:26-29); the `snapshot` arm reads
  `read_screen(harness, seen).to_text()` (:170-172): the whole text, nothing paged; `read_screen` records every id
  of the snapshot (:331-341). The `command` span's eight fields (:99).
- `packages/escher-driver/src/json.rs` (:305-365) — `Outcome::Screen` writes `{"text":…}` (:325-329).
- `packages/escher-driver/src/mcp.rs` (:205-256) — an accepted `snapshot` answers the bare text as its MCP text block
  (:230); every other accepted outcome its JSON line (:234); `screenshot` its own two-block form (:231-233).
- `packages/escher-driver/src/refusal.rs` (:1-130) — eight causes (:35-44), each a fixed name, meaning and remedy;
  none means "no such part" (`malformed` is an argument outside its schema or bound, `not-found` an id).
- `packages/escher-driver/src/session.rs` (:55-175) — `SeenIds` holds at most `MAX_SEEN_IDS` 4096 ids, the least
  recently read forgotten past the bound (:66-117); its doc: "the ids the session's screens have read" (:69).
- `packages/escher-driver/src/command.rs` (via grep :140, :183, :247, :333-360) — `("snapshot", [])` is the one shape
  `validate` admits (:247); the per-kind admission match (:333-360) has an arm per `ArgKind`, `ArgValue::Number`
  already carried for `Milliseconds` (:351).
- `packages/escher-driver/src/cli.rs` (via grep :248-253, :350) — a flag's value is read as a number only for
  `ArgKind::Milliseconds` (:253); the usage line per kind (:350).
- `packages/escher-driver/src/mcp/listing.rs` (via grep :68-80) — the input schema per `ArgKind`.
- `packages/escher-driver/src/recording.rs` (via grep :120) — arguments kept by kind; a number kept as written.
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs` (:55-133) — Cells is reached from Home's `task-card-cells`, not
  booted by `stand`; 676 reacting cells (:95-103).
- `tests/blitz-tests/tests/fixtures/mod.rs` (via grep) — `boot` (:18), `hold` (:23), shared pages `password_labelled`
  (:28), `password_named` (:38), `covered` (:48), `HeldNet`.
- `tests/blitz-tests/tests/stand_mcp_tools.rs` (via grep :33-90, :517-550) — runs `serve_mcp` in process on a
  session it boots, a fixture included (`fixtures::covered`, :517): an MCP walk on a fixture is provable in process.
- `scripts/cold-agent.sh` (via grep :196-201) — `valid_step` admits `tool` by name and `arguments` as any object; no
  argument name is checked, so an argument on `snapshot` moves nothing there, a new verb would.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-20T09-07-29-phase/tree-query-2026-10-20-large-screen-in-parts.json`)
- **`Snapshot::to_text`** — 28 call sites (query 1, `callee_file LIKE '%snapshot_text.rs'`): one production caller,
  `Session::execute` @ `packages/escher-driver/src/execute.rs:171`; the rest are tests (dioxus-native-dom's unit
  tests, `cli_commands.rs:262`, `stand_snapshot_text`, `stand_settle`, `stand_session_fresh`, `stand_keyboard`,
  `stand_act_refused`). An additive reader beside `to_text` leaves every one unchanged.
- **`Outcome::Screen`** — 14 references (refs on `Screen` in escher-driver): `execute.rs:170, :276`, `json.rs:325,
  :706`, `mcp.rs:230`, `recording_tests.rs:78, :81`, `cli_commands.rs:261`, `session_common/mod.rs:199`,
  `stand_act_spans.rs:193, :344`, `stand_load_wait.rs:79`, `stand_mcp_tools.rs:278`, `stand_requirement_sweep.rs:69`.
  A field added to the variant moves every pattern that destructures `{ text }` — each of these files is a boundary
  member if the answer gains fields on the variant.

## Patterns detected
- **A pure snapshot-derived product beside `to_text`** (`snapshot_text.rs:41`, `snapshot_diff.rs`): an inherent
  method on `Snapshot`, no document access, no failure path, re-exported from the crate root.
- **An optional argument on an instance verb** (`schema.rs:276-308`, `screenshot`'s `id`, `required: false`):
  `validate` admits the verb with and without it.
- **A carrier-specific MCP form** (`mcp.rs:230-234`): `snapshot` the bare text, `screenshot` two blocks, the rest the
  JSON line — the MCP form follows the outcome, not the verb.
- **A field present only with a reading** (`schema.rs:124-130`, `BUSY` `always: false`; `json.rs:316-320` leaves it
  out with no reading) — the shape under which a whole screen's `snapshot` answer stays byte-identical while a paged
  one gains fields.
- **A non-acting read feeds the record of ids** (`execute.rs:331-341`; `Session::lint` likewise, since
  2026-10-19-visual-lint).

## Conventions to follow
- **Fixed text**: a cause, an argument's help and a field's help are `'static` table text (`schema.rs`,
  `refusal.rs:61-117`); no refusal holds a call's value (`refusal.rs:126-130`).
- **Fixture placement**: a page one check reads stays in that check's file (`.claude/rules/testing.md` §Fixtures).
- **Both layout modes**, assertion messages naming a mode and an index only (testing.md §Determinism; test-plan §5).

## Measured
- Today a screen over the budget is answered whole on every carrier: `execute.rs:170-172` reads the whole
  `to_text`, `json.rs:325-329` writes it, `mcp.rs:230` sends it as the text block; only the 1,048,576-byte answer and
  result bounds hold it (derived from those three sites at HEAD c1a2e5b85).
- Every stand screen is inside the budget: the largest boot text is Crud at 2,049 B and the largest hosted line
  2,202 B (test-plan amendment 2026-10-10-snapshot-states, re-read in tests-history.md), so no stand task can prove
  a walk; the proof is a fixture page (and Cells is not booted by `stand`, `stand_non_lean_tasks.rs:65-69`).
- A run of consecutive lines of `to_text` is byte-equal to those lines of the whole text, because the indent is the
  node's depth from its root, written per line (`snapshot_text.rs:45-47, :54-56`): parts cut at line boundaries and
  concatenated in order ARE the whole text, each node's line exactly once.
- A `snapshot` read changes nothing and moves no time (`execute.rs:170-172` reads `harness.doc.snapshot()` alone, no
  pump, no settle), so two reads with no acting call between read the same text — consecutive part reads on an
  unchanged instance cut one text.

## New files to create
- `tests/blitz-tests/tests/stand_snapshot_parts.rs` — the fixture check: an over-budget page (in this file, one
  reader), the walk through `Session::run` and through `serve_mcp` in process, both layout modes

## Files to modify
- `packages/dioxus-native-dom/src/snapshot_text.rs` — the part cut beside `to_text`, with its unit tests
- `packages/dioxus-native-dom/src/lib.rs` — re-export of what the cut adds
- `packages/escher-driver/src/schema.rs` — `snapshot`'s argument and result fields, their help
- `packages/escher-driver/src/schema/spec.rs` — a new `ArgKind` for a part number, its name and bound
- `packages/escher-driver/src/schema_tests.rs` — the schema pins (kinds, `snapshot`'s row)
- `packages/escher-driver/src/command.rs` — `snapshot`'s admitted shapes and `Command::Snapshot`'s argument
- `packages/escher-driver/src/lib.rs` — the crate's re-exports, if the part bound is public
- `packages/escher-driver/src/execute.rs` — the part read, its outcome variant, the record of ids fed from the part
- `packages/escher-driver/src/json.rs` — the paged answer's JSON line and its tests
- `packages/escher-driver/src/mcp/listing.rs` — the new kind's input schema
- `packages/escher-driver/src/mcp_tests.rs` — listing and result pins
- `packages/escher-driver/src/cli.rs` — the new kind's flag value and usage spelling
- `packages/escher-driver/src/cli_tests.rs` — argv and usage-line pins for `snapshot --part`
- `examples/seven_guis/tests/cli_commands.rs` — `snapshot --part` on a lean task through the binary
- `examples/seven_guis/tests/host_log.rs` — one part read driven on the host at `trace`
- `examples/seven_guis/tests/mcp_log.rs` — one part read driven through the `mcp` role at `trace`
- `tests/blitz-tests/tests/session_common/mod.rs` — a `snapshot_part` builder and a part reader
- `tests/blitz-tests/tests/stand_act_spans.rs` — a part read's `command` span line, in both layout modes

## Sweeps
- `Outcome::Screen` over the rust plane's refs (code-graph): 14 hits · 0 changed · 14 no-change (a paged answer is a
  new `Outcome` variant, so every `{ text }` pattern stands; the two exhaustive matches, `execute.rs:276` and
  `json.rs:325`, gain the new variant's arm in the files above) — re-derived: graph query 2 in the trace.
- `ArgValue::Number` handling over `wire.rs:131, :152`, `recording.rs:124` and `command.rs`'s `call_of` (git grep):
  generic over every number argument — no change; `describe.rs:145`'s count row `[12, 13, 12, 3, 8, 4, 15]` counts
  verbs, node keys, key names, busy classes, causes, exits and session errors, none of which moves — no change.
- `to_text` callers over the rust plane (code-graph): 28 hits · 1 changed (`execute.rs:171`) · 27 no-change (tests
  of the unchanged whole text) — re-derived: graph query 1 in the trace.
- `ArgKind::` over `packages/escher-driver/src/` (git grep, `_tests.rs` excluded): match sites in `cli.rs`,
  `command.rs`, `mcp/listing.rs`, `recording.rs`, `schema/spec.rs` — each listed above for a new kind.

## Open questions
- How an agent reaches a part — an argument on `snapshot`, a new verb, an opt-in or a default paging over the
  budget, or an in-process reader only — and what a part past the last answers (an existing cause cannot say it; a
  new cause widens) → blocks: plan-decision (the founder's forks, inputs#I1). Files above are listed for the
  argument shape; an in-process-only answer drops every escher-driver carrier file but `execute.rs`/`session.rs`.
- Whether a part read feeds the record of ids with the part's ids or the whole screen's → blocks: plan-decision
  (leaned in P4: the whole screen's, as `snapshot` and `lint` do — `resolve` records the whole screen before any
  acting call resolves its id, `execute.rs:378-382`, so a part-only record would move no reading).
