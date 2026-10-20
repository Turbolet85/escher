# arch extract

## Relevance
relevant — the chunk extends the snapshot model's text form (dioxus-native-dom) and the driver's verb table, executor and carriers (escher-driver), both held by arch's Standard Contracts.

## Constraints
- `Snapshot::to_text` is required to stay the snapshot's one text form, a pure function of the `Snapshot` (no document access, no failure path, no log, no print), one line per node in `Snapshot::nodes()` pre-order with a child two spaces deeper, and `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) a ceiling a caller compares against, never enforced by `to_text` — so a part-cut either sits beside `to_text` or reuses its per-line form unchanged; where the cut lives (a new pure function in `snapshot_text`, or the executor) is P4's, and whether anything pages today is research's question (per architecture §Standard Contracts → Dioxus DOM bridge).
- The snapshot, its text form and its diff are crate-private modules of dioxus-native-dom compiled only under its `accessibility` feature, which escher-driver names unconditionally; new snapshot-side code is required to sit under the same gate (per architecture §Cross-cutting Patterns → Config management).
- `Session::run` is required to stay the one entry point for a call — `validate` first, nothing public running a `Command` that did not come from `validate` — and `VERBS` holds twelve verbs in three levels, each a `'static` `VerbSpec { name, level, help, args, fields }`; a walk reached by an agent goes through this table and executor, whatever form P4 picks (per architecture §Standard Contracts → Driver session).
- A new verb, argument, result field, exit or carrier is a widening of the validated verb set and of the carriers' content — each prior one ratified by the founder — while a new field of a closed value set inside a ratified carrier is not; a part's text is screen content, so the walk's shape is a founder's fork, its most conservative option named (per architecture §Established Decisions → [Driver session]).
- Every answer is bounded at 1,048,576 bytes on the socket (`oversize`), the command line and an MCP result (`AnswerTooLarge`), and an MCP request line at 65,536 bytes; the chunk is required to keep these bounds unchanged (per architecture §Standard Contracts → Driver session).
- The session's record of ids is fed from every snapshot a call takes once `validate` admits it (and from `Session::lint`'s); a part read is a snapshot taken, so whether a part feeds the record whole or only its own ids is a design point P4 states (per architecture §Standard Contracts → Driver session).
- A kept check keeps only calls a record can spell — a verb among the eight instance verbs, arguments its schema names — and `Check::from_bytes` admits `tool` one of those eight; a new verb or argument either joins that spelling or is not kept, and replay of the two committed Counter checks must still hold (per architecture §Standard Contracts → Driver session).

## Patterns to follow
- A snapshot-derived product as a pure inherent method in a crate-private module of dioxus-native-dom (`snapshot_text`, `snapshot_diff`): no document access, no failure path, no log, values from the `Snapshot` only, no `NodeId` in any field, re-exported from the crate (per architecture §Standard Contracts → Dioxus DOM bridge).
- The driver schema as one `'static` table: a verb's args and result fields each carry their help, `describe()` and MCP `tools/list` serve them from that table (`help` last), and `Outcome::to_json` writes an answer by hand keyed by the verb's result fields in table order, in the crate-private `json` module (per architecture §Standard Contracts → Driver session).
- Fixture proof: a check is one test-target file directly under `tests/blitz-tests/tests/`, auto-discovered (no `[[test]]` table), running `for incremental in [false, true]`; a fixture page with one reader stays in its check's file, a page more than one check reads goes in `tests/fixtures/mod.rs` (per architecture §Conventions → Tests).
- Session-holding checks read the driver's call builders and outcome readers from `tests/session_common/mod.rs` (per architecture §Existing Scopes → blitz-tests).

## Anti-patterns to avoid
- No log, event or span field carries the snapshot's text, a part of it, its length, or (by extension) a part count or index: the `command` span holds exactly its eight fixed-word or count fields, and a `snapshot` records `verb` alone (per architecture §Standard Contracts → Driver session).
- No serialization crate writes an answer and `serde_json` stays a reader only; no new dependency, env var, listener or workspace crate lands silently (per architecture §Established Decisions → [Driver session]; architecture §Occupied Resources).
- escher-driver depends on no app: the fixture and any boot for it come from the caller, not the session library (per architecture §Established Decisions → [Driver session]).

## Contract bindings
- arch ↔ security: the carriers (socket `call`, command line stdout, MCP stdio, kept check at rest) and their bounds — a part's text crossing them is the same content class as `snapshot`'s text; any new verb/argument/field is a widening the security plan records.
- arch ↔ obs: the `command` and `tool_call` span field sets are fixed; a new verb records `verb` and, refused, `cause` only.
- arch ↔ tests: `stand_requirement_sweep` asserts each lean task's screen inside `SNAPSHOT_TEXT_BUDGET`; `stand_snapshot_text` pins the four lean tasks' sizes; `stand_replay` replays the committed `tests/blitz-tests/checks/counter-{cli,mcp}.json`, whose kept answers include `snapshot` text — an under-budget `snapshot` answer that moves breaks them.
- arch ↔ tests (describe): `help`'s self-description and MCP `tools/list` are written from `VERBS`, so a new verb or field moves their pinned listings and counts.

## Acceptance criteria contributions
- (arch) On every stand screen inside the budget, `snapshot`'s answer is byte-identical to before on every carrier, and the two committed Counter checks still replay green (per architecture §Standard Contracts → Driver session).
- (arch) The walk is reached only through `Session::run` after `validate` (or `validate_session`); whatever verb, argument or field it adds appears in `VERBS` with its help and is listed by `help` and MCP `tools/list`, and a refused call changes nothing (per architecture §Standard Contracts → Driver session).
- (arch) The `command` span still carries at most its eight fields, none holding a part's text, size, count or index (per architecture §Standard Contracts → Driver session).
- (arch) No new workspace crate, dependency, env var or listener; `Cargo.lock` unchanged unless an `§Occupied Resources` registration lands first (per architecture §Occupied Resources).
