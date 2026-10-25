# Codebase Research — 2026-10-25-capability-statement

## Scope
- **Depth:** moderate (a document chunk; the reads verify every count and limit the statement will state) · **Reads:** 14 · **Globs/Greps:** 15
- **Harness rules consulted:** `.claude/rules/testing.md` — `## Session Additions` read (lines 50-60), one addition applied: 2026-10-10 «search `tests/` and `examples/*/tests/` for the table's name» (the `VERBS` readers below); no live leg in this chunk — its gates are the standing `ci-leg.sh fast` and `doc` legs
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — Viola's 0.2.0 planning assembles `intent.md` from its gitignored incubator and routes it; the incubator's `research/` is where findings are dropped («Drop results here as they come»); no escher statement is named there

## Files inspected
- `README.md` (full, 85 lines) — line 48 reads «gets the eleven driver commands as tools» (CARRY 1, re-verified); line 71 already names «a statement of what Escher can do and its limits» as Epoch 5 work; `## Status` is one line per epoch (lines 52-73)
- `packages/escher-driver/src/schema.rs` (lines 8-18, 51, 80-101, 156-201, 486-515) — `VERBS` holds twelve in three levels (`SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, START, STATUS, STOP, HELP`, schema.rs:490-492); `NODE_FIELDS: [NodeFieldSpec; 13]` (schema.rs:51); `EXIT_STATUSES: [ExitStatus; 4]` (schema.rs:506); `MAX_PART = 1000` (schema.rs:18); the boolean model's two limits stated in node-key help (schema.rs:80, 96, 101, 156-157, 201); `snapshot`'s paging help (schema.rs:179-182)
- `packages/escher-driver/src/refusal.rs` (lines 1-40) — `Cause` holds eight (`CAUSES: [Cause; 8]`, refusal.rs:35); `Covered`'s meaning states its two measured-off readings (refusal.rs:20-25) and `OffScreen`'s the transformed scrolling box (refusal.rs:26-29) — the three measured-off readings are the tool's own text
- `packages/escher-driver/src/error.rs` (lines 80, 173, 296) — `SESSION_ERRORS: [ErrorSpec; 15]`, pinned by its own unit test (error.rs:296)
- `packages/escher-driver/src/session.rs` (lines 19, 370-420) — `LOAD_WAIT = 5 s` (session.rs:19); recording is `#[cfg(unix)]`, `Unsupported` off unix (session.rs:378-395)
- `packages/escher-driver/src/host.rs` (lines 11, 44, 217), `client.rs` (lines 125, 311) — `IDLE_EXPIRY = 1800 s`; the host and the client are unix only (`cfg(unix)` / `cfg(not(unix))` arms)
- `packages/escher-driver/src/lib.rs:21` — «a request is at most 16,384 bytes and an answer at most 1,048,576»
- `.andromeda/security-plan-detail/threat-model-summary.md` (lines 30-37, `boundary-mcp-stdio`) — the MCP stdio row lists ids, names, masked values and snapshot text, then the shot's `image` block; a lint's findings are not named there, and diffs only inside the cold-agent sentence — drift for the wrap (Open questions)
- `.andromeda/residuals.md` (lines 1-8, 14, 15, 23, 32, 33) — the AT-witness line reads `re-carried:0.2.0` (residuals.md:14), not `target: next`; the focus-fixup line (residuals.md:32) and native checkedness (residuals.md:33) `open`; «reacts to a click» `re-carried:0.2.0` (residuals.md:23); the CRUD list's current option `absorbed:v020-03` (residuals.md:15)
- `escher-0.2.0/requirements.md` (lines 8, 42-43) — the AT witness is a carried residual «owed at an active assistive technology, which this version does not reach»
- `escher-0.2.0/intent.md` (full) — §Principles «escher leads Viola», §6 «escher is ready before that planning starts»
- `.andromeda/architecture.md` (lines 225-243, Directory structure) — the tree names no `docs/`; `docs/` holds three upstream files (`architecture.mmd`, `licensing.md`, `parley.md`, each last changed by an upstream commit: `git log --oneline -3 -- docs`)
- `.github/scripts/ci-leg.sh` (lines 9, 27) — `ci-scripts` runs `python3 -m unittest discover -s .github/scripts`, one of the four fast legs
- `/home/turbolet/dev/projects/viola/viola-0.2.0-incubator/README.md` — read once at take-up, snapped as `inputs#I1`

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-25T02-58-45-phase/tree-query-2026-10-25-capability-statement.json`)
- **VERBS** — 36 reference sites over 14 files (the trace's rows, summed), among them `examples/seven_guis/tests/cli_commands.rs`, `examples/seven_guis/tests/mcp_server.rs`, `packages/escher-driver/src/mcp/listing.rs`, `tests/blitz-tests/tests/stand_act_scroll.rs`, `tests/blitz-tests/tests/stand_act_spans.rs` — every count the program states is read from the table; README.md is no reader, which is how «eleven» drifted
- **CAUSES** · **NODE_FIELDS** · **EXIT_STATUSES** — read by `describe.rs` (3 sites each) and `json.rs`; the self-description is the one place the program writes all four tables out
- The chunk changes no symbol: no caller is touched; 0 cross-crate blast radius

## Patterns detected
- **The tool states its own limits** (refusal.rs:20-29; schema.rs:80-101): the measured-off readings and the boolean model's limits are `'static` help text, served by `help` — the statement points at `help` as the current, self-updating source and quotes those texts rather than re-deriving them.
- **Counts drift where prose copies a table** (README.md:48 against schema.rs:490): the README's count was hand-copied at 2026-10-13 and missed `lint`'s addition; no check reads README.md (`git grep -n -l README -- .github/ scripts/ tests/ examples/ packages/`: 1 hit, `examples/assets/servo-new.html`, an unrelated HTML asset).
- **A stated reach carries its witness and a gap its owner** (residuals.md:14, :32, :33; a11y-plan §9): the masters name a check, a CI run or a residuals line beside every claim.

## Conventions to follow
- **Front page content set** (arch §Project Intent → Front page): README.md says what Escher is, its status one line per epoch, how the command line and MCP surface run; «Escher» capitalised in prose, crate, binary and path names verbatim (README.md:1-9, 52-73).
- **Measured claims cite where they were measured** («as measured at {chunk}/report.md», or a CI run id), as every master body does.

## New files to create
- `docs/capabilities.md` — the capability statement: what Escher 0.2.0 can do, its stated limits, its a11y reach and what an MCP client carries onward (fork F1's recommended branch; under F1's README-only branch no file is created)
- `.github/scripts/test_capability_statement.py` — the drift pin: every `VERBS` name and the table counts read from `schema.rs`, `refusal.rs` and `error.rs` match the statement and README's MCP sentence (fork F2's recommended branch; absent under its manual-only branch)

## Files to modify
- `README.md` — line 48's «eleven» to twelve (CARRY 1); a pointer to the statement (F1)

## Sweeps
- `eleven` over tracked text (`git grep -n -w eleven -- README.md docs/ packages/escher-driver/ examples/seven_guis/src/`): README.md:48 — 1 hit · 1 changed · 0 no-change
- `README` as a read path over `.github/ scripts/ tests/ examples/ packages/` (`git grep -n -l README`): 1 hit · 0 changed · 1 no-change (`examples/assets/servo-new.html`, an upstream HTML asset that names no escher file)

## Open questions
- Where does Viola's planning read the statement — a tracked escher document with a pointer dropped in Viola's incubator `research/`, README.md alone, or a copy in Viola's incubator? → blocks: plan-decision (P4 fork F1; the file lists above are provisional on it)
- Does the chunk add a CI-script pin that keeps the statement's counts equal to the schema's tables, or stay document-only with the operator's read as the witness? → blocks: plan-decision (P4 fork F2)
- security-plan's `boundary-mcp-stdio` row names no lint findings and diffs only inside its cold-agent sentence, while §Input Validation and arch name both → note, not a question: an `Expected amendments (wrap)` entry, never a phase edit
