# Report — 2026-10-16-file-splits

**Chunk:** escher's document.rs test modules out, mcp.rs and schema.rs split along their seams
**Date:** 2026-10-16T17:32Z
**Commits:** since `last_wrap` 2026-10-16T15:13:00Z:
- `4f3939914 refactor(2026-10-16-handler-splits-and-readme): …` — the previous chunk's own wrap commit, this chunk's
  base;
- `54d4ea21f chore(2026-10-16-file-splits): operator pre-CI commit, for the run this chunk verdict reads` — this
  chunk's tree, staged whole.

Basis: `git log --format='%h %s' 4f393991..HEAD` and `git log --since 2026-10-16T15:13:00Z`.

## Changes (structured — detectors read this)

- **Files:** `gate.py scope` reads `scope: clean — changed 11 · listed 11 · recorded 0` over base `4f393991`, the
  parent of the oldest pre-CI commit `54d4ea21`.
  - Modified (3):
    - `packages/blitz-dom/src/document.rs` — the two inline modules are replaced by `#[path]` declarations. Lines
      1–3461 are byte-identical to the base (gate `cmp <(git show 4f39…:…document.rs | head -n 3461) <(head -n 3461 …)`
      green).
    - `packages/escher-driver/src/mcp.rs` — keeps the module doc, `START_APP`, `serve_mcp`, `io_error`,
      `tool_call_span`, `Held` and `Server`. It declares `mod framing; mod listing; mod result;` and the
      `#[path = "mcp_tests.rs"] mod tests;`.
    - `packages/escher-driver/src/schema.rs` — keeps the bounds, `KEY_NAMES`, `BUSY_CLASSES`, `NodeFieldSpec` +
      `NODE_FIELDS`, the field and verb consts, `VERBS`, `ExitStatus` + `EXIT_STATUSES` and `verb()`. It declares
      `mod spec;` and `pub use spec::{ArgKind, ArgSpec, FieldKind, FieldSpec, Level, VerbSpec};` (schema.rs:4-7), and
      `#[path = "schema_tests.rs"] mod tests;` (schema.rs:477-479).
  - New (8): `packages/blitz-dom/src/document_changed_set_tests.rs`, `packages/blitz-dom/src/document_font_reading_tests.rs`,
    `packages/escher-driver/src/mcp/framing.rs`, `packages/escher-driver/src/mcp/listing.rs`,
    `packages/escher-driver/src/mcp/result.rs`, `packages/escher-driver/src/mcp_tests.rs`,
    `packages/escher-driver/src/schema/spec.rs`, `packages/escher-driver/src/schema_tests.rs`.
  - Every coordinate of new text below is from `## New text, by line`.
  - Chunk evidence: `escher-0.2.0/chunks/2026-10-16-file-splits/evidence/` holds the step-1 records
    (`mcp-requests.jsonl`, `help-before.json`, `mcp-before.jsonl`, `driver-tests-before.txt`, `dom-tests-before.txt`),
    `sizes-after.md` and `operator-pass.md`.

- **Symbols / APIs:** no public item is added, removed or renamed. The crate-root `pub use` list and every
  `crate::schema::NAME` path resolve unchanged, through `schema.rs`'s re-export (the build and 73 unit tests green).
  `serve_mcp` stays the one `mcp` item reached from outside `mcp` (`cli.rs`, `lib.rs`, `stand_mcp_masked.rs`,
  `stand_mcp_tools.rs`, as research's graph query read).
  - **Moved inside `mcp`, widened to `pub(super)` and no further** (gate `grep -c -E '^\s*pub(\(crate\))? '` over the
    three submodules reads 0):
    - `mcp/framing.rs`: `MAX_MCP_REQUEST_BYTES`, `MAX_ID_BYTES`, `Request`, `read_request`, `Id`, `ProtocolError`,
      `Envelope`, `envelope`, `result_reply`, `error_reply`, `write_line`, `list`;
    - `mcp/listing.rs`: `initialized`, `description`, `tool`, `tool_list`;
    - `mcp/result.rs`: `call_of`, `Ending` and its `word`, `ToolResult` with its `ending` field and its methods
      `accepted`, `shot`, `refused`, `failed`, `bounded`, `written`.

    Private still: `reply`, `strings`, `LATEST_VERSION` · `PROTOCOL_VERSIONS` · `SERVER_NAME`, `Id::admitted` ·
    `Id::written`, `ProtocolError::code` · `::message`, and `ToolResult`'s `text` and `image`. `MAX_*_BYTES`,
    `description` and `tool` are reached only by `mcp::tests`.
  - **Moved to `schema/spec.rs`, kept `pub`:** `ArgKind` and its `name` · `bound`, `ArgSpec`, `FieldKind` and its
    `name`, `FieldSpec`, `Level` and its `name`, and `VerbSpec`.
  - **The one hand-built span**, `tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending =
    Empty)`, stays in `mcp.rs`, now at `packages/escher-driver/src/mcp.rs:116-119` (at the base `mcp.rs:537-540`). The
    `command` span at `execute.rs:86` is untouched. Gates: both span sites byte-identical, `info_span!` 2 sites in the
    crate.
  - **The line map, base → now**, for the citation sweep. A range that moved file changes its path, not only its
    digits. Measured by `$TMPDIR/{scratchpad}/line_map.py`, which matches each cited range's first and last lines in
    the new files, with each end read at the tree: `sed -n` on `mcp.rs` 110 · 116-119 · 138 · 145 · 177 · 211 · 213 ·
    273 · 285-287, `mcp_tests.rs` 590-593 · 623-624, `result.rs` 42, and `schema.rs` 112 · 469 · 477-479.
    - `mcp.rs` stays in `mcp.rs`:
      - `:51-117` (`serve_mcp` with its doc) → `mcp.rs:44-110`;
      - `:537-540` (`tool_call_span`) → `mcp.rs:116-119`;
      - `:559-566` (`expire`) → `mcp.rs:138-145`;
      - `:598-632` (`tool_call`) → `mcp.rs:177-211`;
      - `:634-694` (`run`, `start`) → `mcp.rs:213-273`.
    - `mcp.rs` moved to `mcp/framing.rs`:
      - `:134-174` (`read_request`) → `framing.rs:27-67`;
      - `:184-205` (`impl Id`) → `framing.rs:77-98`;
      - `:207-236` (`ProtocolError` and its impl) → `framing.rs:100-129`;
      - `:253-283` (`envelope`) → `framing.rs:146-176`.
    - `mcp.rs` moved to `mcp/listing.rs`:
      - `:325-345` (`initialized`) → `listing.rs:28-48`, its `env!("CARGO_PKG_VERSION")` line (`:339`) at
        `listing.rs:42`;
      - `:347-356` (`description`) → `listing.rs:50-59`;
      - `:358-407` (`tool`) → `listing.rs:61-110`.
    - `mcp.rs` moved to `mcp/result.rs`:
      - `:416-440` (`call_of`) → `result.rs:15-42` (rustfmt wraps its signature after `pub(super)`, 3 lines longer);
      - `:460-535` (`ToolResult` and its impl) → `result.rs:62-137`.
    - The `mcp.rs` tests:
      - `:706-1324` (`mod tests`) → `mcp_tests.rs:1-624`, declared at `mcp.rs:285-287`;
      - `:1291-1322` → `mcp_tests.rs:592-623`.
    - `schema.rs` (the base lines 4–109 move by +4 and 260–622 by −146):
      - `:32-108` → `schema.rs:36-112`;
      - `:235-242` (`Level::name`) → `schema/spec.rs:129-136`;
      - `:294-297` → `schema.rs:148-151`;
      - `:324-328` → `schema.rs:178-182`;
      - `:415-448` → `schema.rs:269-302`;
      - `:513-569` → `schema.rs:367-423`;
      - `:577-615` → `schema.rs:431-469`;
      - `:623-962` (`mod tests`) → `schema_tests.rs:1-342`, declared at `schema.rs:477-479`.
    - `document.rs`:
      - `:3462-3693` (`changed_set_tests`) → `document_changed_set_tests.rs:1-235`, declared at `document.rs:3462-3464`;
      - `:3695-3719` (`font_reading_tests`) → `document_font_reading_tests.rs:1-26`, declared at `document.rs:3466-3468`;
      - lines 1–3461 hold their numbers (`document.rs:3403-3408` and `:3424-3439`, cited at test-plan.md:45 and :240,
        are unmoved).

- **Crates / modules:** no crate added or removed. `lib.rs` is unchanged, and escher-driver's fourteen private
  modules stay fourteen.
  - The `mcp` module gained three private child modules — `framing`, `listing`, `result` — each a file under
    `packages/escher-driver/src/mcp/`.
  - The `schema` module gained one private child, `spec`, at `packages/escher-driver/src/schema/spec.rs`, its six types
    re-exported.
  - Four new `cfg(test)` children are held in files of their own through `#[path]`: `document::changed_set_tests`,
    `document::font_reading_tests`, `mcp::tests` and `schema::tests`. The workspace's out-of-line `#[path]` test
    modules go from one (`dioxus_document::bridge_tests`) to five.
  - The test paths are unchanged: both `cargo test … --lib -- --list` outputs `cmp`-equal their step-1 records
    (escher-driver 73 tests, blitz-dom 97).
- **Dependencies:** none — no manifest or lockfile changed (gate `git diff --name-only 4f39… -- packages examples tests
  apps wpt Cargo.toml Cargo.lock | grep -c -v -x …` reads 0).
- **Schema / config:** none — no config key, wire field, MCP field, refusal cause or span field.
  - `escher-session help` is byte-identical to `evidence/help-before.json`.
  - `escher-session mcp` over the seven requests of `evidence/mcp-requests.jsonl` is byte-identical to
    `evidence/mcp-before.jsonl` (6 reply lines; two runs on the base cmp-equal).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** test and span counts are unchanged:
  - escher-driver 73 unit tests (mcp 11, schema 6), blitz-dom `document::` 18 (changed_set 7, font_reading 2), and
    `info_span!` sites 2. Gates: `contains test result: ok. 73 passed`, `contains test result: ok. 18 passed`, `last
    line 2`.
  - What moved is the FILE each count stands in. 9 of `document.rs`'s 18 tests now stand in its two sibling files, as
    `document::` tests. `mcp.rs`'s 11 and `schema.rs`'s 6 now stand in `mcp_tests.rs` and `schema_tests.rs`, as
    `mcp::tests` and `schema::tests`.
  - Sites stating such a count by file:
    - test-plan.md:11 «18 in document.rs»;
    - test-plan.md:21 «schema.rs 6», «mcp.rs 11»;
    - test-plan.md:143 «schema.rs pins the verb table», «mcp.rs that `help` is answered»;
    - the registry key `registries/contracts/test-plan/session-lifecycle.md:11` «mcp.rs 11 · … · schema.rs 6»
      (pattern `bare-file`, below).
  - Sizes (tokei 14.0.0, code lines; `evidence/sizes-after.md`), base → now: `document.rs` 2,793 → 2,573 · `mcp.rs`
    1,140 → 217 · `schema.rs` 840 → 421. The largest split file is `mcp_tests.rs` at 579. `framing.rs` 167 ·
    `listing.rs` 91 · `result.rs` 115 · `spec.rs` 97 · `schema_tests.rs` 324 · `document_changed_set_tests.rs` 201 ·
    `document_font_reading_tests.rs` 19.
- **Dev-tool versions:** none — tokei re-read at 14.0.0, rust-code-analysis-cli used as at the previous chunk.
- **Harness / gate surface:** none — no agent-run script, CI step, leg or verdict shape changed. The gate entries are
  this plan's.
- **Cross-project / external claims:**
  - `inputs.py verify`: `inputs: 2 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 2 · uncited 2 · unparsed 0`. It was fired before this report, so the uncited count is that
    moment's.
  - inputs#I1 · message: the operator, typed in this session, 2026-10-16 17:08Z · copy message · n/a — the directive for
    the operator pass: fire 24 as written, the pre-CI commit message, then 25 through `--operator 25` and 26 as written,
    write `evidence/operator-pass.md`, stop on a red.
  - inputs#I2 · message: the operator, typed in this session, 2026-10-16 17:27Z · copy message · n/a — the word at this
    wrap's invocation, «run P1 only and stop» (the two-window form).
  - The P5 review card `.andromeda/runs/2026-10-16T15-51-04-phase/p5-card-1.md` is named by no inputs entry. It is cited
    by the matrix's v020-01 notes and was read whole here. Its `## Beyond the plan` reads `none`.
  - CI read: run CI#38937698317 on sha `54d4ea21ff66` (the pre-CI commit, this chunk's whole tree), `verdict: green ·
    checks 17/17 · wall 959 s`, every check succeeded, `Stand checks repeated` and `Coverage report` among them
    (`evidence/operator-pass.md`).
- **Reverted / negative API facts:** imports added to `spec.rs` and `mcp.rs` only to carry intra-doc links were written
  and removed. rustc reads such an import as `unused_imports`, which `-D warnings` would fail. The links name their
  targets by path instead — see Deviations 1.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** escher-driver's unit-test count in the registry key file
  `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` reads «escher-driver's 72 unit tests (cli.rs 5 ·
  client.rs 2 · …)».
  - Measured: 73 at HEAD, before and after the split. Gate `cargo test -p escher-driver --locked` reads `test result:
    ok. 73 passed`, and `evidence/driver-tests-before.txt` ends `73 tests, 0 benchmarks`. The plan's note re-derived
    `client 3` by `grep -c #[test]` per file.
  - test-plan.md:21 already states the 73 (`the count of 73 and client.rs's third at
    escher-0.1.0/chunks/2026-10-14-quality-gates/report.md`). The key file kept 72 since that chunk.
  - The same 72 stands in the leaf `.claude/docs/services/escher-driver.md:60` («72 unit tests (… `client.rs` 2 …)»).
  - This chunk did not move the figure; it measured it.
- **Expected amendments (from plan):** the patterns are in `.andromeda/runs/2026-10-16T17-27-46-wrap/report-patterns.toml`,
  fired as `cascade.py sweep --pre-pass` (trail `cascade-2026-10-16-file-splits-pre-pass.json`). Counts are each
  pattern's `standing` figure.
  - **architecture §Conventions → Tests**, from one out-of-line `#[path]` test module to five — carried (Crates /
    modules). Pattern `path-module`: standing 7 in total.
    - architecture.md:115 (Conventions → Tests, the one module named) → changed.
    - architecture.md:254 (the dioxus-native-dom scope row) → no-change: a true claim of that module.
  - **architecture §Existing Scopes → blitz-dom and → escher-driver**, `mcp`'s three children and `schema`'s `spec` —
    carried (Crates / modules).
    - Pattern `module-count`: standing 2. architecture.md:257 (the escher-driver row's «fourteen private modules» and its
      `mcp (…)` · `schema (…)` entries) → changed: the children named, the count stays fourteen. obs-plan.md:70 («the
      fourteenth module, `image`») → no-change, true.
    - The blitz-dom row, architecture.md:235, lists lib.rs's modules (`packages/blitz-dom/src/lib.rs:37-84`); this
      chunk added none there. It is matched by no pattern; the fact for it is `document`'s two `cfg(test)` children in
      files of their own (Crates / modules).
  - **test-plan §2 Directory pattern and §4 Test file location**, «bar one `#[path]` module» grows to five — carried
    (Crates / modules). Pattern `path-module`, its test-plan rows:
    - test-plan.md:66 (§2, «bar one `cfg(test)` module held in a file of its own through `#[path]`») → changed;
    - test-plan.md:137 (§4, «one unit-test module sits out of line») → changed;
    - test-plan.md:24, :99 (the bridge module's own tests and why it is a child) → no-change, true;
    - test-plan.md:331 (a dated count history) → no-change, history.
  - **Every `file:line` citation into `mcp.rs`, `schema.rs`, and `document.rs` past line 3461**, re-pointed by the line
    map, with no claim text changed — carried (Symbols / APIs, the line map). The P2 citation sweep re-points the digits.
    A range whose text moved file needs its path changed by hand.
    - Pattern `mcp-cite`: standing 18 rows. architecture.md 111 · 133 ×14 · 150 ×3 · 154; security-plan.md 27 · 57 · 82
      ×4 · 84 ×11 · 353 ×2 · 399; design-system.md 397; layout-templates.md 74 ×4; test-plan.md 21 ×2 · 103; obs-plan.md
      30 · 107 ×2 · 181; a11y-plan.md 281.
    - Pattern `schema-cite`: standing 6 rows. architecture.md 133 ×7; security-plan.md 79 · 353 ×2; design-system.md
      397; test-plan.md 21 · 143.
    - Pattern `doc-tail-cite`: standing 2 rows, test-plan.md 11 · 12 ×2.
    - Every row is a citation to re-point by the map → changed (digits, and the path where the range moved file).
  - **The leaves follow through the cascade** — carried. The sweep's leaf rows:
    - `path-module`: `.claude/rules/testing.md:14` («one module sits in a file of its own»), `.claude/docs/services/dioxus-native-dom.md:41`
      and `.claude/docs/tests-summary.md:30` (the last two name the bridge module's own tests, true);
    - `mcp-cite`: `CLAUDE.md:85` («`packages/escher-driver/src/mcp.rs` (unit tests inline)»);
    - `bare-file`: `.claude/docs/services/escher-driver.md:60` and `.claude/docs/tests-summary.md:24` (per-file test
      counts);
    - `doc-modules`: `.claude/docs/tests-summary.md:30`.
  - The by-file test-count sites (Counts / qualifiers moved) are not in the plan's list. They stand under patterns
    `bare-file` (standing 4: test-plan.md 21 ×4 · 143 ×2 · 331; the key `session-lifecycle.md:11` ×2) and `doc-modules`
    (standing 3: test-plan.md 11 · 55 · 331).
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element. The moved code keeps its marks:
  - `mcp/framing.rs` (request reader, id, protocol errors) → validation bound and fixed errors✓ (unchanged; the mcp
    record byte-identical) · instrumentation n/a (the span stays in `mcp.rs`) · PII n/a · tests unit (`mcp::tests`
    11)✓ · a11y n/a · tokens n/a
  - `mcp/listing.rs` · `mcp/result.rs` · `schema/spec.rs` → validation n/a (no new input) · instrumentation n/a ·
    PII n/a · tests unit (`mcp::tests`, `schema::tests`)✓ · a11y n/a · tokens n/a

## Deviations from intent
1. **Seven intra-doc links name their targets by path.**
   - The changes: `serve_mcp`'s doc «[`VERBS`]» → «[`VERBS`](schema::VERBS)» (`packages/escher-driver/src/mcp.rs:53`).
     In `schema/spec.rs`, «[`MAX_ID_BYTES`]», «[`MAX_TEXT_BYTES`]», «[`KEY_NAMES`]», «[`MAX_MILLISECONDS`]»,
     «[`BUSY_CLASSES`]» and «[`NODE_FIELDS`]» → «[`NAME`](super::NAME)».
   - Why: after the move their targets are no longer imported in those files. rustc reads an import used only by a doc
     link as `unused_imports`, measured in this chunk's own diagnostics, so clippy `-D warnings` would fail. Without it,
     rustdoc `-D warnings` would fail on the broken link.
   - The rendered text is unchanged, and the `doc` leg is green. The plan says items move «unchanged»; this is the one
     textual change inside moved items, a link target only.
2. **`mcp_tests.rs` has use lines beyond moved items.**
   - Lines naming moved items, as step 4 allows: `super::framing::{MAX_ID_BYTES, MAX_MCP_REQUEST_BYTES}` and
     `super::listing::{description, tool}`.
   - Lines naming items the parent no longer imports: `crate::command::ArgValue`, `crate::schema::{ArgKind, ArgSpec,
     KEY_NAMES, MAX_MILLISECONDS, VERBS}` (the original `ArgSpec` line merged in) and `crate::wire::MAX_ANSWER_BYTES`.
   - Why: they reached the tests through `use super::*` only while `mcp.rs` imported them for the code that moved. This
     is the mechanism the plan's implementation note names, met for non-moved names.
   - No test, helper or assertion changed.
3. **rustfmt re-wraps.**
   - `call_of`'s signature, after `pub(super)`.
   - One assertion line joins after the dedent (`… == bounded,`, `mcp_tests.rs`).
   - `mcp.rs`'s `use framing::{…}` block (`mcp.rs:30-33`).
   - A base-vs-now line diff of every moved span (`$TMPDIR/{scratchpad}/verify_move.py`) read identical apart from
     these, deviation 1's links and the use lines.
4. **Span edges.**
   - The shape types moved from `schema.rs:110-258` at the base (with `ArgKind`'s doc line and derive). The plan cites
     `:112-259`.
   - The document module bodies are `document.rs:3464-3692` and `:3697-3718`; the plan cites the modules at
     `:3462-3694` and `:3695-3719`.
   - The cut is the plan's, after line 3461.
5. **`schema_tests.rs`'s `//!` was corrected before the gates.** The first draft said the tests read private consts.
   They reach only `pub` and `pub(crate)` names (read by grep over the file), so it now says the module keeps the
   `schema::tests` paths and reads the table through the module's own names.

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 11 · listed 11 · recorded 0 (companion 0
· mechanical 0 · in-intent 0 · widening 0)`).

## Decisions & corrections
- The operator's directive for the operator pass, inputs#I1: «Run the operator pass for 2026-10-16-file-splits now, in
  plan order: entry 24 (gate.py hygiene) as written, the commit "chore(2026-10-16-file-splits): operator pre-CI commit,
  for the run this chunk verdict reads", entry 25 through gate.py run --operator 25, then entry 26 (ci.py conclusion
  --wait) as written. Record it in evidence/operator-pass.md. Stop and tell me if any entry is red.» It was carried out
  in that order, every entry green (`evidence/operator-pass.md`).
- The operator's word at this wrap's invocation, inputs#I2: «run P1 only and stop». P1 ends at this report. P2–P7 run
  at a 2a resume in this run dir, `.andromeda/runs/2026-10-16T17-27-46-wrap/`.
- A measured fact, a curation candidate: rustc counts an import used only by intra-doc links as unused (the editor
  diagnostics after the split: «unused import: `VERBS`», «unused imports: `BUSY_CLASSES`, … `NODE_FIELDS`»). An item
  moved away from its doc links' targets names them by path (`[`X`](super::X)`), or the import keeps a code use.
- Plan step 4's one allowed edit («`use` lines naming a moved item by its new module») was narrower than the move
  needed. A non-moved name the parent stops importing needs a test use line too (Deviations 2). This was recorded as
  friction `input.plan-step-ambiguous` at implement's code checkpoint.
- A guard refusal: the Bash guard refused `cd ~/.claude/skills/andromeda-implement/references && …` («moves the
  session cwd»). It was re-issued once with absolute paths and ran.
- A recurrence of a standing learning (2026-10-10, extended 2026-10-15): the per-package `cargo test -p escher-driver`
  warns blitz-paint's `variant Intrinsic is never constructed`. It is outside the change; the fast leg is green.
- A sweep hazard and a recurrence of host-linux's Processes rule: a process census by `ps -eo pid,comm,args | grep -c
  -E 'escher-sessio|seven_guis_nati'` counted 2, both the probing shell matching its own command line. Re-taken by
  `awk '$3 ~ /^(…)$/'` on the `comm` column, it read 0.
- Precedent kept: the pre-CI commit stages the whole tree with `git add -A`, as `c71f7ed14` did for the previous chunk.
- A pre-existing drift, found by this chunk's measurement, is raised for P2 Validate (Spec claims disproved): the key
  file and the leaf hold escher-driver's 72 unit tests, while HEAD reads 73.

## Outcome
- **v020-01**: met on the diff, all four parts.
  - (1) The complexity entry's last line is `over-ceiling: handle_click handle_key_or_input_event handle_pointerdown
    handle_pointermove touch_action_pan_axes`.
  - (2) `grep -c -E '^mod (changed_set_tests|font_reading_tests) \{' …document.rs` reads 0, and lines 1–3461 are
    `cmp`-equal to the base.
  - (3) All eight driver files are ≤ 800 tokei code lines, the largest `mcp_tests.rs` at 579.
  - (4) The fast, doc and a11y legs are green, and CI#38937698317 on `54d4ea21ff66` reads `verdict: green · checks
    17/17`.
  - The matrix entry reads `implemented`, its ref naming the gate entries; P7.3 flips it.
- **tests:** the driver reads 73 and `document::` 18; both `-- --list` outputs equal their records. Met.
- **security:** the mcp record is byte-identical. The six production files read 0 panic, print, env, subscriber and
  feature-gate hits (the handshake's `env!("CARGO_PKG_VERSION")` line excluded, now `listing.rs:42`) and 0 serializer
  hits. Nothing widens past `pub(super)`. Met.
- **design · layouts:** `help` is byte-identical, and so is the `tools/list` reply inside the mcp record. Met.
- **obs:** both `info_span!` sites read as at the base, 2 in the crate. `stand_mcp_tools`, `stand_act_spans`, `mcp_log`
  and `host_log` pass, per package and workspace-built in the fast leg. Met.
- **a11y:** `ci-leg.sh a11y` reads 6 · 6 · 4 · 8 · 5 · 14, and `stand_accessibility_ids` reads 8 in both modes. Met.
- **arch:** no tracked file outside the three sources changed, the only new files are the eight, and fast and doc pass.
  Met.
- **Gates** — implement's block over `.andromeda/runs/2026-10-16T16-56-12-implement/gate-2026-10-16-file-splits.json`,
  first full firing, `entries 26 · green 23 · red 0 · recorded 0 · timeout 0 · not-run 3`:
  - `cargo test -p escher-driver --locked` — green (exit 0, `73 passed`)
  - `cargo test -p blitz-dom --locked --lib document::` — green (exit 0, `18 passed`)
  - `cargo test -p escher-driver --locked --lib -- --list | cmp - …driver-tests-before.txt` — green (exit 0)
  - `cargo test -p blitz-dom --locked --lib -- --list | cmp - …dom-tests-before.txt` — green (exit 0)
  - `cargo build --locked -p seven_guis --bin escher-session` — green
  - `target/debug/escher-session help | cmp - …help-before.json` — green (exit 0)
  - `target/debug/escher-session mcp < …mcp-requests.jsonl | cmp - …mcp-before.jsonl` — green (exit 0)
  - `cargo test -p blitz-tests --locked --test stand_mcp_tools --test stand_mcp_masked --test stand_act_spans --test
    stand_accessibility_ids` — green (8 · 2 + 2 ignored · 1 · 5)
  - `cargo test -p seven_guis --locked --test mcp_server --test mcp_log --test host_log --test cli_commands` — green
    (cli_commands 12 · host_log 1 · mcp_log 1 · mcp_server 5)
  - `grep -c -E '^mod (changed_set_tests|font_reading_tests) \{' …document.rs` — green (exit 1, last line 0)
  - `cmp <(git show 4f39…:…document.rs | head -n 3461) <(head -n 3461 …)` — green (exit 0)
  - the tokei loop over the eight driver files — green (exit 0)
  - `git diff --name-only 4f39… -- packages examples tests apps wpt Cargo.toml Cargo.lock | grep -c -v -x …` — green
    (last line 0)
  - `git ls-files --others --exclude-standard -- packages examples tests apps wpt | grep -c -v -x …` — green (last line 0)
  - the panic/print/env grep over the six production files — green (last line 0)
  - the serializer/terminal grep — green (last line 0)
  - the `pub` grep over the three submodules — green (last line 0)
  - the two span-site `grep -r -h -F … | wc -l` — green (last line 2)
  - `grep -r -h -F 'info_span!' packages/escher-driver/src | wc -l` — green (last line 2)
  - the complexity reader — green (last line `over-ceiling: …`)
  - `bash .github/scripts/ci-leg.sh a11y` — green
  - `bash .github/scripts/ci-leg.sh fast` — green (149 s; 186 `test result: ok`, 0 FAILED; CI scripts `Ran 100 tests
    … OK`)
  - `bash .github/scripts/ci-leg.sh doc` — green
  - the three `leg = 'operator'` entries — not run by plain `run`, fired at the operator pass (below)
- **Operator pass** — recorded in `evidence/operator-pass.md`:
  - `gate.py hygiene` — green (`hygiene: clean — read 78 …`);
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
    build/escher-0.2.0` — green, 80 s, `history moved: refs/remotes/origin/build/escher-0.2.0 4f393991→54d4ea21`, the
    intended push;
  - `ci.py conclusion --sha HEAD --wait 1800` — green, CI#38937698317 on `54d4ea21ff66`, `verdict: green · checks
    17/17 · wall 959 s`.
- **Smoke:** skipped — no boot path or UI surface changed. The plan states no boot smoke, and the touched `mcp` role
  was run as the built binary by the help, mcp and `seven_guis` entries.
- **Deferrals:** none. Rust source changed, so every gate ran.
- **Watches:** none — the plan folds no `watch:`.
- **Outcome basis:**
  - The verdicts rest on the operator pass's final state: the commit list `54d4ea21f chore(…): operator pre-CI commit`
    (`git log 4f393991..HEAD`) and the final HEAD's CI run CI#38937698317, recorded in `evidence/operator-pass.md`.
  - Implement's P4 report is held whole in this window and persisted at
    `.andromeda/runs/2026-10-16T16-56-12-implement/outcome-2026-10-16-file-splits.md`. It is the only `*-implement` dir
    holding `outcome-2026-10-16-file-splits.md`, and nothing supersedes it.
  - Post-implement artifacts: `evidence/operator-pass.md`.
- **Process hygiene:** implement's census (outcome file) reads every process it started terminated: the step-1 binary
  runs, cargo, and gate entries 1–23. The operator pass started entry 25's fast leg and push, through gate.py, and the
  `ci.py` poller; both exited on their own. Re-measured at 2026-10-16T17:31:51Z with `ps -eo pid,ppid,comm,args` and
  awk on `comm` ∈ {escher-sessio, seven_guis_nati, cargo, rustc, cargo-clippy, clippy-driver}: none running.
## Addendum — resumed 2026-10-16
- inputs#I3 · message: the operator, typed in this session, 2026-10-16 17:34Z · copy message · n/a — the operator's
  directive at this resume: the per-file test counts in the masters and the registry files follow the moved tests to
  their new sibling files; the escher-driver unit-test count reading 72 (`session-lifecycle.md:11`,
  `services/escher-driver.md:60`) is amended to the measured 73.
  - bears on **Counts / qualifiers moved**: adds to it — the by-file count sites listed there move with their tests to
    the sibling files (`document_changed_set_tests.rs` · `document_font_reading_tests.rs`, `mcp_tests.rs`,
    `schema_tests.rs`), on the operator's word.
  - bears on **Spec claims disproved by measurement**: adds to it — the disposition of the 72 is the operator's: amended
    to the measured 73 in the key file and in the leaf.
- inputs.py verify, re-fired: `inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 3 · uncited 1 · unparsed 0`; moved since the report: uncited 2 → 1 (I1 and I2 cited by the
  report; I3, snapped at this resume, is cited by this addendum).

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 4f393991 (the parent of the oldest pre-CI commit 54d4ea21) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### packages/blitz-dom/src/document.rs — added 4 line(s) in 2 range(s)
added: 3463-3464 · 3467-3468
### packages/blitz-dom/src/document_changed_set_tests.rs — new file · 235 line(s)
- 12-38 @16 «fn make_doc(»
  - 20-24 «let mut doc = BaseDocument::new(DocumentConfig {»
- 40-42 «fn one_div(mutator: &mut DocumentMutator<'_>) -> Vec<NodeId> {»
- 44-52 @45 «fn changed_set_fresh_document_reads_no_change() {»
  - 48-51 «assert!(»
- 54-84 @55 «fn changed_set_attribute_write_marks_its_node_until_drained() {»
  - 58-61 «assert!(»
  - 63-64 «doc.mutate()»
  - 66-69 «assert!(»
  - 71-74 «assert!(»
  - 76-77 «doc.mutate()»
  - 79-83 «assert_eq!(»
- 86-109 @87 «fn changed_set_write_outside_the_document_marks_nothing() {»
  - 99-102 «assert!(»
  - 104-107 «assert!(»
- 111-148 @112 «fn changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing() {»
  - 113-118 «let (mut doc, ids) = make_doc(true, |mutator| {»
  - 120-123 «assert!(»
  - 128-132 «assert_eq!(»
  - 136-139 «assert!(»
  - 143-147 «assert_eq!(»
- 150-180 @151 «fn changed_set_checked_change_marks_its_node() {»
  - 152-158 «let (mut doc, ids) = make_doc(true, |mutator| {»
  - 160-164 «let checked = |doc: &BaseDocument| {»
  - 165-169 «assert_eq!(»
  - 172-176 «doc.snapshot_node_and(checkbox, ElementState::CHECKED, |node| {»
- 182-210 @185 «fn changed_set_idle_resolve_marks_nothing_in_either_layout_mode() {»
  - 186-209 «for incremental in [false, true] {»
- 212-235 @213 «fn changed_set_drained_id_of_a_dropped_node_reads_none() {»
  - 224-227 «assert!(»
  - 228-234 «for id in taken {»
### packages/blitz-dom/src/document_font_reading_tests.rs — new file · 26 line(s)
- 8-13 «fn doc_with(font_ctx: FontContext) -> BaseDocument {»
  - 9-12 «BaseDocument::new(DocumentConfig {»
- 15-19 @16 «fn a_context_holding_a_decodable_font_can_paint_text() {»
- 21-26 @22 «fn a_context_holding_no_family_cannot_paint_text() {»
### packages/escher-driver/src/mcp.rs — added 15 line(s) in 6 range(s)
added: 10-13 · 22 · 28 · 30-35 · 53 · 286-287
- 30-33 «use framing::{»
### packages/escher-driver/src/mcp/framing.rs — new file · 207 line(s)
- 16-25 @18 «pub(super) enum Request {»
- 27-67 @29 «pub(super) fn read_request(input: &mut impl BufRead) -> io::Result<Request> {»
  - 32-66 «loop {»
- 69-75 @71 «pub(super) enum Id {»
- 77-98 «impl Id {»
  - 78-85 @79 «fn admitted(value: &Value) -> Option<Id> {»
  - 87-97 «fn written(&self) -> String {»
- 100-108 @102 «pub(super) enum ProtocolError {»
- 110-129 «impl ProtocolError {»
  - 111-118 «const fn code(self) -> i32 {»
  - 120-128 «const fn message(self) -> &'static str {»
- 131-144 @133 «pub(super) enum Envelope {»
  - 138-143 @139 «Request {»
- 146-176 @148 «pub(super) fn envelope(line: &[u8]) -> Envelope {»
  - 149-151 «let Ok(text) = std::str::from_utf8(line) else {»
  - 152-154 «let Ok(value) = serde_json::from_str::<Value>(text) else {»
  - 155-157 «let Value::Object(mut request) = value else {»
  - 160-166 «let method = match (spoken, request.remove("method")) {»
  - 167-175 «match id {»
- 178-184 «fn reply(id: &Id, member: &str, value: &str) -> String {»
- 186-188 «pub(super) fn result_reply(id: &Id, result: &str) -> String {»
- 190-195 «pub(super) fn error_reply(id: &Id, error: ProtocolError) -> String {»
- 197-201 «pub(super) fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> {»
- 203-207 @204 «pub(super) fn list(items: impl IntoIterator<Item = String>) -> String {»
### packages/escher-driver/src/mcp/listing.rs — new file · 117 line(s)
- 19-26 @20 «fn strings<'a>(items: impl IntoIterator<Item = &'a str>) -> String {»
  - 21-25 «list(items.into_iter().map(|item| {»
- 28-48 @30 «pub(super) fn initialized(params: Option<&Value>) -> String {»
  - 31-33 «let asked = params»
  - 34-37 «let version = PROTOCOL_VERSIONS»
- 50-59 @52 «pub(super) fn description(verb: &VerbSpec) -> String {»
  - 53-57 «let fields: Vec<String> = verb»
- 61-110 @63 «pub(super) fn tool(verb: &VerbSpec, apps: &[&str]) -> String {»
  - 65-91 «for arg in verb.args {»
  - 95-103 «input_schema.json(»
- 112-117 @113 «pub(super) fn tool_list(apps: &[&str]) -> String {»
### packages/escher-driver/src/mcp/result.rs — new file · 137 line(s)
- 15-42 @17 «pub(super) fn call_of(»
  - 22-37 «for (name, value) in arguments {»
  - 38-41 «Ok(Call {»
- 44-50 @46 «pub(super) enum Ending {»
- 52-60 «impl Ending {»
  - 53-59 «pub(super) const fn word(self) -> &'static str {»
- 62-69 @64 «pub(super) struct ToolResult {»
- 71-137 «impl ToolResult {»
  - 72-78 «pub(super) fn accepted(text: String) -> ToolResult {»
  - 80-87 @81 «pub(super) fn shot(width: u32, height: u32, png: &[u8]) -> ToolResult {»
  - 89-95 «pub(super) fn refused(refusal: Refusal) -> ToolResult {»
  - 97-103 «pub(super) fn failed(error: SessionError) -> ToolResult {»
  - 105-117 @107 «pub(super) fn bounded(self) -> ToolResult {»
  - 119-136 «pub(super) fn written(&self) -> String {»
### packages/escher-driver/src/mcp_tests.rs — new file · 624 line(s)
- 20-22 «fn never(_: &str) -> Result<Session, SessionError> {»
- 24-27 @25 «fn empty(app: &str) -> Result<Session, SessionError> {»
- 29-40 @30 «fn exchange(»
- 42-44 «fn request(id: &str, method: &str, params: &str) -> String {»
- 46-52 «fn tool_call(id: u32, name: &str, arguments: &str) -> String {»
  - 47-51 «request(»
- 54-56 «fn error(id: &str, code: i32, message: &str) -> String {»
- 58-60 «fn tool_reply(id: u32, result: &ToolResult) -> String {»
- 62-66 «fn lines(requests: &[String]) -> Vec<u8> {»
- 68-125 @69 «fn each_line_is_read_by_its_rules() {»
  - 70-74 «let at_bound = format!(»
  - 80-117 «let rows: Vec<(Vec<u8>, Vec<String>)> = vec![»
  - 119-124 «for (row, (line, replies)) in rows.into_iter().enumerate() {»
- 127-165 @128 «fn a_line_is_read_whole_across_reads_and_an_unended_last_line_is_one() {»
  - 132-137 «let rows: [(usize, &str, usize); 4] = [»
  - 139-151 «for (row, (capacity, input, count)) in rows.into_iter().enumerate() {»
  - 157-164 «assert_eq!(»
- 167-199 @168 «fn an_id_is_echoed_only_in_its_two_forms() {»
  - 172-189 «let rows: Vec<(String, Option<String>)> = vec![»
  - 191-198 «for (row, (id, echoed)) in rows.into_iter().enumerate() {»
- 201-277 @202 «fn each_method_gets_its_reply() {»
  - 203-220 «let version = |asked: &str, answered: &str| {»
  - 223-271 «let rows: Vec<(String, String)> = vec![»
  - 273-276 «for (row, (sent, answered)) in rows.into_iter().enumerate() {»
- 279-344 @280 «fn each_argument_value_becomes_its_kind_or_refuses_the_call() {»
  - 283-291 «let call = |args: Vec<(&str, ArgValue)>| {»
  - 292-323 «let rows: Vec<(&str, Result<Call, Refusal>)> = vec![»
  - 325-330 «for (row, (arguments, expected)) in rows.into_iter().enumerate() {»
  - 331-336 @332 «let read = exchange(»
  - 337-343 «assert_eq!(»
- 346-365 @347 «fn listed(verb: &VerbSpec) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {»
  - 349-354 «let required = verb»
  - 355-363 «let types = verb»
- 367-443 @368 «fn the_listing_is_the_eleven_verbs_of_the_table() {»
  - 369-371 «let Ok(Value::Object(result)) = serde_json::from_str::<Value>(&tool_list(&APPS)) else {»
  - 372-374 «let Some(Value::Array(tools)) = result.get("tools") else {»
  - 378-381 «assert_eq!(»
  - 382-442 «for (row, (listed_tool, verb)) in tools.iter().zip(VERBS).enumerate() {»
- 445-531 @446 «fn the_session_verbs_are_answered_by_the_server() {»
  - 447-452 «let hello = |served: u64| Hello {»
  - 455-482 «let rows: Vec<(Vec<String>, Vec<ToolResult>)> = vec![»
  - 484-492 «for (row, (sent, results)) in rows.into_iter().enumerate() {»
  - 494-505 @496 «let sent = [»
  - 508-517 «let expected = [»
  - 519-530 «for (index, result) in expected.iter().enumerate() {»
- 533-554 @534 «fn help_is_answered_by_the_server_with_or_without_a_session() {»
  - 536-541 «let sent = [»
  - 546-549 «assert_eq!(»
- 556-572 @557 «fn a_session_idle_for_its_expiry_is_dropped_when_a_request_arrives() {»
  - 558-561 «let sent = [»
  - 565-568 «assert_eq!(»
- 574-603 @575 «fn a_result_over_the_answer_bound_is_replaced_by_the_session_error() {»
  - 576-579 «let rows = [»
  - 581-584 «for (row, (length, ending)) in rows.into_iter().enumerate() {»
  - 585-588 «assert_eq!(»
  - 595-598 «assert_eq!(»
  - 599-602 «assert_eq!(»
- 605-624 @606 «fn a_shot_is_written_as_its_image_block_then_its_text_block() {»
  - 608-611 «assert_eq!(»
  - 612-614 «let Ok(Value::Object(result)) = serde_json::from_str::<Value>(&written) else {»
  - 615-617 «let Some(Value::Array(content)) = result.get("content") else {»
### packages/escher-driver/src/schema.rs — added 6 line(s) in 2 range(s)
added: 4-7 · 478-479
### packages/escher-driver/src/schema/spec.rs — new file · 152 line(s)
- 4-19 @6 «pub enum ArgKind {»
- 21-45 «impl ArgKind {»
  - 22-32 @23 «pub const fn name(self) -> &'static str {»
  - 34-44 @35 «pub const fn bound(self) -> &'static str {»
- 47-58 @49 «pub struct ArgSpec {»
- 60-83 @62 «pub enum FieldKind {»
- 85-101 «impl FieldKind {»
  - 86-100 @87 «pub const fn name(self) -> &'static str {»
- 103-115 @105 «pub struct FieldSpec {»
- 117-126 @119 «pub enum Level {»
- 128-137 «impl Level {»
  - 129-136 @130 «pub const fn name(self) -> &'static str {»
- 139-152 @141 «pub struct VerbSpec {»
### packages/escher-driver/src/schema_tests.rs — new file · 342 line(s)
- 10-22 «const VERB_NAMES: [&str; 11] = [»
- 24-46 @25 «fn the_eleven_verbs_are_named_in_order_each_at_its_level() {»
  - 30-34 «let stated = [»
  - 37-41 «let level_names = [»
  - 43-45 «for (row, (level, name)) in level_names.into_iter().enumerate() {»
- 48-113 @49 «fn every_verb_states_its_help_its_arguments_and_a_result() {»
  - 51-56 «let field_help = |verb: &VerbSpec, name: &str| {»
  - 57-86 @58 «let amended = [»
  - 88-90 «for (row, (stated, text)) in amended.into_iter().enumerate() {»
  - 91-93 «for acting in [&CLICK, &TYPE, &PRESS, &ADVANCE, &SCROLL] {»
  - 94-112 «for verb in VERBS {»
- 115-126 @116 «fn a_verb_is_found_by_its_exact_name_only() {»
  - 117-119 «for name in VERB_NAMES {»
  - 120-122 «let near = [»
  - 123-125 «for name in near {»
- 128-294 @129 «fn each_verb_has_its_stated_shapes_and_the_key_list_its_twelve_names() {»
  - 132-184 «let rows: [(&str, Args, Vec<&str>); 11] = [»
  - 186-199 «for (verb, (name, args, fields)) in VERBS.iter().zip(rows) {»
  - 201-225 «let field_kinds = [»
  - 227-232 «for verb in VERBS {»
  - 234-250 «assert_eq!(»
  - 253-256 «assert_eq!(»
  - 261-272 «let arg_kinds = [»
  - 274-277 «for (kind, name, bound) in arg_kinds {»
  - 278-289 «let field_kind_names = [»
  - 291-293 «for (kind, name) in field_kind_names {»
- 296-325 @297 «fn the_thirteen_node_keys_are_named_in_order_each_with_its_help() {»
  - 298-301 «let names = [»
  - 304-307 «for (row, key) in NODE_FIELDS.iter().enumerate() {»
  - 308-313 «let help = |name: &str| {»
  - 318-323 «for name in ["role", "name"] {»
- 327-342 @328 «fn the_four_exit_statuses_are_stated_in_status_order() {»
  - 329-334 «let rows = [»
  - 337-341 «for (row, (ending, (status, name))) in EXIT_STATUSES.iter().zip(rows).enumerate() {»
