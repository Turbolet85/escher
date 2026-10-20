# Codebase Research — 2026-10-19-visual-lint

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (a `timeout`-bounded boot smoke; no such entry planned here)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's directive: a fork that widens a boundary or turns the project's direction goes, with every other such fork, into one `founder-forks.md` in the run dir, then is asked by dialog

## Files inspected
- `packages/escher-driver/src/execute.rs` (full) — `Outcome` (five variants), `Session::run` → `execute`, `command_span!` with eight fields, `record_result` (a `Screen` or `Shot` records `verb` alone), `locate` / `resolve` / `in_view` / `crop` / `covered`. `screenshot` (`execute.rs:226-250`) is the read-only arm to copy: no step, no settle, `fonts-unavailable` read from the harness's `ScreenshotError`.
- `packages/escher-driver/src/schema.rs` (300-484) — `VERBS` at `schema.rs:432-434` (eleven, the seven instance verbs first, `screenshot` seventh), `EXIT_STATUSES`, `verb()`. `HELP`'s fields use `FieldKind::Entries` for keyed lists.
- `packages/escher-driver/src/schema/spec.rs` (60-152) — `FieldKind` has ten kinds. `Entries` ("a list of entries, each keyed as its field's help names") already carries a keyed list, so a findings field needs no new kind.
- `packages/escher-driver/src/refusal.rs` (60-120) — the `FontsUnavailable` meaning names no verb ("the app this session holds has no font it can paint text with"), so another verb can return it with no text change.
- `packages/blitz-test-harness/src/contrast.rs` (1-150) — `Harness::text_contrast(NodeId) -> Option<TextContrast>`, read-only. It returns `None` for no text, a missing style, or an `opacity` below 1, a filter or a background image on the walk. `required_ratio` is 3.0 for large text, else 4.5. escher-driver already depends on blitz-test-harness (`execute.rs:5-7`), so the reader is reachable with no new dependency.
- `tests/blitz-tests/tests/stand_contrast.rs` (grep, 661 lines) — `read()` (`stand_contrast.rs:171-188`) classifies `ExemptInactive` where the snapshot's `enabled` is `Some(false)`, `Pass`/`Fail` by `ratio >= required_ratio()`, and `NoText` on `None`.
- `packages/dioxus-native-dom/src/snapshot.rs` (1-230) — `SnapshotNode { id, role, name, state, bounds, children }`. `bounds` comes from `get_client_bounding_rect`. `Snapshot::nodes()` walks in pre-order, and `get(id)` looks one up.
- `packages/blitz-dom/src/scrolling.rs` (618-640, 744-780) — `visible_region(node_id)` is the viewport narrowed by each box in `containing_block_chain(node_id)` whose `overflow_x` / `overflow_y` is not `visible`. The chain starts at the element's containing block, so it never includes the element's own clip. It reads `overflow` alone, not `contain: paint`.
- `packages/blitz-dom/src/layout/inline.rs` (870-905) — an inline root's laid-out text extent is `inline_layout.layout.width()` / `height()` at the layout's scale. `content_width = max(width, layout.width())` is where unbreakable text runs past the width its lines were broken at.
- `packages/blitz-dom/src/node/node.rs` (1165-1185, 1675-1700) — `inline_layout_data` lives on the inline root's `ElementData`, and `text_offset_at_point` reads `layout.scale()` and `block_offset`. These are the parts a read-only text-extent reader composes.
- `packages/escher-driver/src/recording.rs` (89-92, 425) — the recorded set is `Level::Instance` read from `VERBS` (`schema::verb(..).filter(level == Instance)`), not a list. A new instance verb therefore joins recording and `Check::from_bytes` with no code change there.
- `tests/blitz-tests/tests/fixtures/mod.rs` (grep) — `boot`, `hold`, and the pages `password_labelled`, `password_named` and `covered` (`fx-root` / `fx-under` / `fx-cover`).
- `tests/blitz-tests/tests/session_common/mod.rs` (grep) — one builder per verb (`snapshot()` … `screenshot_of(id)`) and the readers `act`, `shot`, `refused`, `refused_unchanged`.
- `scripts/cold-agent.sh:48` and `.github/scripts/test_cold_agent.py:23` — a hard-coded `VERBS` tuple of the eleven verb names. The pipe's `isolated` reading requires exactly those tools, so a twelfth verb moves both.
- `examples/seven_guis/src/session_host.rs:13-18` — the task slugs `counter`, `flight-booker`, `timer`, `crud`.

## Graph impact (from the code-graph query; `.andromeda/runs/2026-10-19T23-47-25-phase/tree-query-2026-10-19-visual-lint.json`)
- **`VERBS`** — referenced from 14 files (`refs` on `callee_name = 'VERBS'`): escher-driver `cli.rs`, `cli_tests.rs`, `command.rs`, `describe.rs`, `execute.rs`, `lib.rs`, `mcp/listing.rs`, `mcp_tests.rs`, `schema.rs`, `schema_tests.rs`; `examples/seven_guis/tests/cli_commands.rs`, `mcp_server.rs`; `tests/blitz-tests/tests/stand_act_scroll.rs`, `stand_act_spans.rs`. A new row reaches every table-driven reader. The tests that pin counts move.
- **`Outcome`** (execute.rs) — referenced from 14 files: escher-driver `execute.rs`, `json.rs`, `lib.rs`, `mcp.rs`, `recording.rs`, `recording_tests.rs`, `replay.rs`; `examples/seven_guis/tests/cli_commands.rs`; blitz-tests `session_common/mod.rs`, `stand_act_diff.rs`, `stand_act_spans.rs`, `stand_mcp_tools.rs`, `stand_requirement_sweep.rs`, `stand_session_recording.rs`. A new variant reaches each exhaustive `match`: `json.rs`, `mcp.rs` (or `mcp/result.rs`), `record_result`, and the `session_common` readers.
- **`text_contrast`** — 5 callers, all tests (`contrast.rs` units, `stand_contrast.rs:180,524`). The lint is its first non-test caller.
- **`visible_region`** — 6 callers: `execute.rs:403` (`in_view`), `execute.rs:414` (`crop`), and four blitz-tests checks. The lint reuses it unchanged.

## Patterns detected
- **The read-only instance verb** (`execute.rs:226-250`): a `screenshot` arm returns its own `Outcome` variant, runs no settle, and refuses `fonts-unavailable` before any work. `record_result` returns early for it (`execute.rs:263`), so its span holds `verb` and, when refused, `cause`.
- **Geometry, never a hit, for visibility** (`execute.rs:397-408`): `in_view` reads `visible_region` and never a hit test.
- **One shared contrast classifier** (`stand_contrast.rs:171-188`): exempt only where `enabled == Some(false)`, no text means not measured, and pass or fail by `required_ratio()`.
- **The verb table drives every surface** (`recording.rs:92`, `cli.rs:171,228,350`, `mcp/listing.rs`): the level and the arguments are read from `VerbSpec`, never from a second list.

## Conventions to follow
- **The written forms are hand-written JSON keyed by the schema's words**: `json.rs` writes one form per `Outcome` variant. `serde_json` only reads (`execute.rs` and `json.rs`; arch §Established Decisions → [Driver session]).
- **Every table is pinned with its row count** (`schema_tests.rs`), and an assertion message carries a row index or a layout mode, never an id (`execute.rs:539-554` pattern).
- **Both layout modes** in every stand or fixture check: `for incremental in [false, true]` (test-plan §3 `session-proof`).

## Measured at the take-up (stand at boot, through `escher-session` at e97a7f41b's source)
- Re-derived: `python3 -I $TMPDIR/{scratchpad}/stand_probe.py counter flight-booker timer crud`. It starts a session per task, takes one `snapshot`, parses each node's `@x,y wxh`, and pairs every two nodes neither of which is an ancestor of the other.
  - **overlap**: 0 pairs whose boxes intersect with positive area, over 13, 18, 19 and 27 nodes.
  - **zero size**: 5 nodes. `task-header-spacer` is 0 high in all four tasks (603 × 0, 564 × 0, 620 × 0, 619 × 0). `timer-progress` is 0 × 20 at boot: its fill is `width: {pct}%` at 0 % elapsed, `examples/seven_guis/src/tasks/timer.rs:74`.
  - **outside the viewport**: 0 nodes.
- CRUD at boot: the three rows (`crud-person-0..2`, each 367 × 29 from y 113.8 to 200.8) lie inside `crud-list` (24, 112.796875, 369 × 419.203125). No row is clipped at boot.
- Not measured here: text extent on the stand (no reader exists yet) and the contrast of snapshot nodes outside `stand_contrast`'s rows. Both are implement's first reading.

## Mechanism equalities the plan needs (verified at HEAD)
- `visible_region(n)` for a CRUD row equals `crud-list`'s padding box cut by the viewport. It excludes the row's own clip because the chain starts at its containing block (`scrolling.rs:618-633`). A `clipped` reading built on it therefore misses a `contain: paint` clip and an element's own `overflow` (the latter matters only for truncated text, where the clip is the element's own).
- `text_contrast(n)` returns `None` for an element holding no text that is not whitespace (`contrast.rs:93-118`). The stand's `timer-duration` range input is `NoText` (`stand_contrast.rs:96`), so a lint that skips `None` reads no contrast finding there.

## New files to create
- `packages/escher-driver/src/lint.rs` — the lint: the six readings over a snapshot's nodes, `Finding`, `Defect`, `Session::lint`
- `packages/blitz-dom/src/text_extent.rs` — a read-only reader of an inline root's laid-out text box against its content box
- `tests/blitz-tests/tests/text_extent.rs` — the reader's check, both layout modes
- `tests/blitz-tests/tests/stand_lint.rs` — the planted-defect fixture check, the stand-at-boot check and the CRUD twelve-Creates agreement, both layout modes

## Files to modify
- `packages/escher-driver/src/lib.rs` — the `lint` module and the re-exports of `Finding` and `Defect`
- `packages/blitz-dom/src/lib.rs` — the `text_extent` module and its re-export
- `packages/escher-driver/src/schema.rs` — gated (F1 carriage): the `lint` row in `VERBS` and its result fields
- `packages/escher-driver/src/schema_tests.rs` — gated (F1 carriage): the verb-table pins and their counts
- `packages/escher-driver/src/command.rs` — gated (F1 carriage): `Command::Lint`, its validation and `call_of`
- `packages/escher-driver/src/execute.rs` — gated (F1 carriage): `Outcome::Linted` and the `lint` arm
- `packages/escher-driver/src/json.rs` — gated (F1 carriage): the lint answer's written form
- `packages/escher-driver/src/describe.rs` — gated (F1 carriage): the self-description's count pins
- `packages/escher-driver/src/mcp.rs` — gated (F1 carriage): the lint tool result
- `packages/escher-driver/src/mcp_tests.rs` — gated (F1 carriage): the tool listing and the result
- `packages/escher-driver/src/cli_tests.rs` — gated (F1 carriage): the lint command line
- `packages/escher-driver/src/recording_tests.rs` — gated (F1 carriage): a recorded lint step
- `tests/blitz-tests/tests/session_common/mod.rs` — gated (F1 carriage): `lint()` and its reader
- `tests/blitz-tests/tests/stand_act_spans.rs` — gated (F1 carriage): the lint row
- `tests/blitz-tests/tests/stand_mcp_tools.rs` — gated (F1 carriage): the lint tool call
- `examples/seven_guis/tests/cli_commands.rs` — gated (F1 carriage): the lint command row
- `examples/seven_guis/tests/mcp_server.rs` — gated (F1 carriage): `tools/list` and the lint result
- `examples/seven_guis/tests/host_log.rs` — gated (F1 carriage): a lint call on the host's flow
- `examples/seven_guis/tests/mcp_log.rs` — gated (F1 carriage): a lint call on the role's flow
- `scripts/cold-agent.sh` — gated (F1 carriage): the twelve verb names
- `.github/scripts/test_cold_agent.py` — gated (F1 carriage): the twelve verb names

## Sweeps
- `"screenshot"` over `scripts`, `.github`, `packages`, `examples`, `tests` (`*.py`, `*.sh`, `*.rs`, `*.json`): 14 hits · 14 files. Each is a site a verb list or a per-verb row lives in. The verb-list sites are on the modify list. `host_log.rs` and `mcp_log.rs` are on it for their flows. `command.rs`, `replay.rs`, `schema.rs` and `recording_tests.rs` are on it or are table-driven (`replay.rs`: no change, its `screenshot` is a test record).
- `VERBS` via the graph: 14 files, listed above. `stand_act_scroll.rs` reads the table and pins no count: no change.

## Open questions
- none — both resolved at P4 (inputs#I2): F1 in process only, delegate operator, PROVISIONAL, with the carriage the founder's word decides as v020-05's gated criterion; T1 zero size only where the element holds content, T2 text cut from view, T3 the lint walks `contain: paint` itself. Engine readers confirmed: `Node::is_focussable` (`node/node.rs:553`), `Node::is_inline_root` (`node/node.rs:65`), `ElementData::text_input_data` (`node/element.rs:574`), `BaseDocument::get_client_bounding_rect` (`document.rs:2441`), `BaseDocument::can_paint_text` (`document.rs:1071`), `Session::harness` (`session.rs:180`).
