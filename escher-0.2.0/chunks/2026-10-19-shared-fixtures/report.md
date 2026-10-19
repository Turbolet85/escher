# Report — 2026-10-19-shared-fixtures

**Chunk:** Shared fixtures — fixture pages and one held network stand-in stated once in blitz-tests' shared modules, run through agent-run
**Date:** 2026-10-19
**Commits:** since `last_wrap` (2026-10-19T21:49:58Z): `0fbdda022 chore(2026-10-19-shared-fixtures): operator pre-CI commit, for the run this chunk's verdict reads` (basis: `git log --format='%h %s' 5174f92a..HEAD`)

## Changes (structured — detectors read this)
- **Files:** test code only, 20 files under `tests/blitz-tests/tests/` (basis: `git diff --stat 5174f92a -- tests`: 20 files changed, 229 insertions, 384 deletions):
  - new: `fixtures/mod.rs` (107 lines; a shared module in a subdirectory, no test target);
  - modified (19): `stand_settle.rs`, `font_face_sniffed_source.rs`, `stand_boot.rs`, `stand_act_diff.rs`, `stand_diff.rs`,
    `stand_snapshot_text.rs`, `stand_mcp_masked.rs`, `stand_session_recording.rs`, `stand_act_spans.rs`, `stand_mcp_tools.rs`,
    `stand_act_filled.rs`, `stand_act_screenshot.rs`, `stand_act_obstructed.rs`, `stand_act_scroll.rs`,
    `stand_act_transformed.rs`, `stand_act_timer.rs`, `stand_replay.rs`, `stand_keyboard.rs`, `stand_snapshot_state.rs`.
  - Unchanged, by the plan's constraints: `common/mod.rs`, `session_common/mod.rs`, `stand_contrast.rs`, upstream's
    `render_blocking_stylesheet.rs` and `link_rel_attribute.rs`, every file under `examples/`, `packages/`, `scripts/`,
    `.github/`, `Cargo.toml`, `Cargo.lock`, `deny.toml` (basis: the gate entry `git diff --name-only 5174f92a… -- packages
    examples tests apps wpt .github scripts Cargo.toml Cargo.lock deny.toml | grep -c -v -x -E '…(the nineteen)…'` green,
    last line 0).
- **Symbols / APIs:** the third blitz-tests shared module `tests/blitz-tests/tests/fixtures/mod.rs`, `#![allow(dead_code)]`,
  its `//!` doc in `common/mod.rs`'s shape, naming no other shared module (coordinates from the new-text listing below):
  - `boot(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument>` (`fixtures/mod.rs:17-20`) —
    `Harness::from_vdom(VirtualDom::new(app), stand::options(incremental))`, the stand's pinned options;
  - `hold(label: &str, app: fn() -> Element, incremental: bool) -> Session` (`:22-25`) — `Session::start(label, || boot(app,
    incremental))`, unwrapped `.expect("the label is one of the closed set")`;
  - three pages: `password_labelled` (`:27-35`; `fx-root` holding `<label>` `Passphrase` for the password input
    `fx-secret`), `password_named` (`:37-44`; `fx-root` holding `fx-secret`, a password input named `Secret` by
    `aria-label`), `covered` (`:46-62`; `fx-under` under the `z-index: 5` overlay `fx-cover`);
  - `HeldNet` (`:64-68`, `impl` `:70-98`, `impl NetProvider for HeldNet` `:100-107`), the held network stand-in:
    `#[derive(Default)]`, a `Mutex<Vec<(String, Box<dyn NetHandler>)>>` filled by `fetch` (url and handler kept);
    readers `held() -> usize` (`:71-74`) and `urls() -> Vec<String>` (`:76-84`, request order); its one writer
    `answer(&self, body: &'static [u8]) -> String` (`:86-97`, `#[track_caller]`: pops the LAST held request, answers it
    through `NetHandler::bytes`, returns its url). It does not override `is_noop`, so the harness wraps it in its load
    counter (`stand_settle` reads `NotSettled { busy: Loads }` through it). No print, `tracing` call, env read, file,
    socket, listener or `blitz_net`.
  - Removed file-private symbols (each private to one target, no caller crosses a target): `stand_settle`'s `fixture` and
    `ManualNetProvider`; `font_face_sniffed_source`'s `HeldNetProvider`; `stand_boot`'s escher-authored
    `RecordingNetProvider`; `password_fixture` in `stand_act_diff`, `stand_diff`, `stand_snapshot_text`,
    `stand_mcp_masked`, `stand_session_recording`; `covered_fixture` in `stand_act_spans`, `stand_mcp_tools`;
    `boot_fixture` in `stand_snapshot_text`, `stand_keyboard`, `stand_snapshot_state`; `hold_fixture` in `stand_act_scroll`,
    `stand_session_recording`; `hold` in `stand_act_obstructed`, `stand_act_transformed`; `hold_variant` in `stand_replay`.
  - Readers — 19 files declare `mod fixtures;` (the nineteen modified files; basis: gate entry `grep -l '^mod fixtures;'
    tests/blitz-tests/tests/*.rs | wc -l` → 19). Remaining callers kept: `stand_diff`'s `Screen::fixture` keeps its name
    and signature (body `Self::of(fixtures::boot(fixture, incremental))`); `stand_mcp_masked`'s and `stand_mcp_tools`'
    MCP boot closures keep `Session::start(app, …)` and its `Result` (the label is the MCP caller's), building the harness
    with `fixtures::boot`; `stand_boot`'s `recording_options` takes `&Arc<HeldNet>`. Pages with one reader, or whose ids
    differ, stay in their files (`stand_act_obstructed`'s richer covered page, `stand_act_filled`'s and
    `stand_act_screenshot`'s password pages, both `restless_fixture`s, and every other single-reader page).
  - `Harness::from_vdom` over `tests/blitz-tests/tests/*.rs`: 27 → 7 lines, the seven non-fixture sites
    (`dioxus_falsy_boolean_attrs`, `stale_node_mapping`, `stand_actionable_keys`, `stand_id_persistence`,
    `stand_non_lean_tasks`, `stand_id_edits` ×2) (basis: gate entry `cat tests/blitz-tests/tests/*.rs | grep -c
    'Harness::from_vdom'` → 7; a tree count, no master site).
  - `NetProvider for` over blitz-tests: 5 files → 3 (`fixtures/mod.rs`, upstream's `link_rel_attribute.rs` and
    `render_blocking_stylesheet.rs`; basis: gate entry `grep -l 'NetProvider for' …`).
  - No port, socket, listener, env var, IPC method, endpoint or export added.
- **Crates / modules:** no crate added or changed; one test-only module added (`fixtures`, no target — the
  `BlitzTestsTargetsTest` pin and the 139-target listing unchanged).
- **Dependencies:** none added or bumped (`Cargo.toml` / `Cargo.lock` untouched).
- **Schema / config:** none.
- **Spec-master edits:** none.
- **Retired:** none — the three escher stand-in copies and the file-private helpers are moved into the shared module,
  not retired (no census row).
- **Counts / qualifiers moved:**
  - blitz-tests shared modules that are no target: 2 → 3 (`common`, `session_common`, `fixtures`) — pattern
    `shared-mods`: standing 26 over the masters + 1 registry row. Dispositions per class: **stale (the third module
    missing)** — `architecture.md:137` (@c3496–4233, the convention: «a second such module»), `architecture.md:350`
    (@c7031, @c20777, the shared-module statements), `test-plan.md:27` (@c116–466, «two shared modules that are no
    target»), `test-plan.md:68` (@c913–2325, «two shared modules that are no target» with their reader counts),
    `test-plan.md:102`/`:103` (§3 Crate-local test helpers: `common` and `session_common` described, no `fixtures`
    entry), `obs-plan.md:70` (@c585 «their shared modules» — the census statement that gains `fixtures/mod.rs`); **no-change
    (a true claim sharing the token: a citation of an unchanged `common/mod.rs`, `session_common/mod.rs` or seven_guis'
    `examples/seven_guis/tests/common/mod.rs`)** — `architecture.md:221`, `:230`, `:234`, `:351`, `security-plan.md:323`,
    `test-plan.md:10`, `:19`, `:50`, `:51`, `:105`, `:336`, `obs-plan.md:74`, `:75`, `:78`, `:342`, `a11y-plan.md:217`,
    `:284`, `:330`, `registries/contracts/test-plan/session-lifecycle.md:8`.
  - Readers of the new module: 0 → 19 — pattern `readers`: standing 2 (`test-plan.md:103` @c2926, the 25 readers of
    `session_common`, unchanged → no-change; `obs-plan.md:70` @c14202, «the 17 readers of the stand checks' shared module»
    at an earlier chunk, historical → no-change). The 19 is a new count with no standing site; it lands where the third
    module is stated (above). `common` stays at 21 readers and `session_common` at 25 (no file gained or lost either
    declaration; basis: the 19 diffs add `mod fixtures;` alone).
  - Files that declare no shared module: `stand_mcp_masked`, `stand_boot` and `font_face_sniffed_source` now declare
    `fixtures` — pattern `declnone`: standing 4. `test-plan.md:68` @c1974 (`stand_mcp_masked.rs` and
    `password_painted_masked.rs` «declare none» of `mod common;`, measured at 2026-10-12-headless-screenshot — about
    `common`, still true) → no-change; `test-plan.md:102` @c2044 (`password_caret_click` declares none of `common`) →
    no-change; `test-plan.md:103` @c3202 (`stand_mcp_masked`, `stand_non_lean_tasks`, `password_caret_click` declare no
    `session_common`, still true) → no-change; `obs-plan.md:70` @c6011 (`accessibility_states` declares no shared module)
    → no-change. No standing site says `stand_boot` or `font_face_sniffed_source` declares none; the obs-plan §3 census
    statement that lists them is the Expected-amendment entry below.
  - escher's held network stand-ins: 3 file-private copies → 1 shared (`HeldNet`); upstream's two unchanged — pattern
    `heldnet`: standing 3. `test-plan.md:12` («every request held by a hand `HeldNetProvider`») → stale (the name);
    `test-plan.md:278` (the §8 Network row: «two file-private ones: `render_blocking_stylesheet`'s, and `stand_settle`'s»,
    citing `stand_settle.rs:182-214`, lines that no longer hold the stand-in) → stale; `test-plan.md:277` (upstream's
    `RecordingNetProvider` row, citing `link_rel_attribute.rs:14-24` alone) → no-change (upstream's copy stands;
    `stand_boot`'s escher copy was never on that row).
  - The test-data bootstrap of the agent-run contract: NOT YET MEASURED → measured — pattern `bootstrap`: standing 1
    (`test-plan.md:131` «NOT YET MEASURED — the test-data bootstrap mechanism of the 5-command contract») → stale. The
    reading: fixtures are compiled into each target by `agent-run.sh boot` (`cargo test --no-run`), nothing is seeded, and
    `run stand` runs every fixture-holding `stand_*` check, `run {file}` (e.g. `run font_face_sniffed_source`) and `run all`
    the others (basis: gate entries `bash scripts/agent-run.sh boot` / `run stand` / `run font_face_sniffed_source`, below).
  - File-private fixture boots and holds left: 9 files → 0 — pattern `helpers` (`password_fixture|covered_fixture|
    hold_fixture|boot_fixture|hold_variant`): no master site — its control never fired over the seven masters at
    `0fbdda02` (the sweep's refusal line: «control: helpers, title never fired over the seven masters' pre-pass text
    (0fbdda02)»); the tree count is the gate entry `cat tests/blitz-tests/tests/*.rs | grep -c -E '^fn
    (boot_fixture|hold_fixture|hold|hold_variant)\(|^fn fixture\(app:'` → exit 1, 0.
  - The silence census over the files touched — pattern `census`: standing 3 (`obs-plan.md:70` @c9091, `:77`, `:78`,
    each a per-chunk silence statement of an earlier chunk) → no-change; this chunk's reading is new: 0 print, sink, env,
    file or socket sites in `fixtures/mod.rs` (gate entry `grep -c -E 'println!|…|blitz_net' …fixtures/mod.rs` → exit 1,
    0) and 4 print or sink sites over the 19 files with the module, the 4 standing at the base (`stand_act_spans`' re-run
    children's census exceptions), none added (gate entry `cat …{the nineteen}.rs …fixtures/mod.rs | grep -c -E
    'println!|eprintln!|print!|dbg!|tracing_subscriber|escher_telemetry|set_global_default'` → 4); the blitz-tests env-read
    set unchanged (gate entry → `1 env!("CARGO_MANIFEST_DIR");1 env!("CARGO_PKG_NAME");1 env!("CARGO_TARGET_TMPDIR");6
    std::env::current_exe;1 std::env::var;`).
  - Citations into the 19 changed files: their cited lines moved (imports and removed helpers shift each file) — pattern
    `cites19`: standing 46 rows. Disposition per class: **re-point** (digits, P2's citation sweep) for every row whose
    claim still holds at the moved lines — the `stand_keyboard.rs`, `stand_snapshot_state.rs`, `stand_snapshot_text.rs`,
    `stand_diff.rs`, `stand_act_*.rs`, `stand_mcp_*.rs`, `stand_replay.rs`, `stand_session_recording.rs`,
    `stand_boot.rs`, `stand_settle.rs` and `font_face_sniffed_source.rs` test-range citations in `architecture.md:63`,
    `:98`, `:160`, `:190`, `:201`, `:230`, `:350`, `security-plan.md:101`, `:103`, `:121`, `:125`, `:133`, `:362`,
    `design-system.md:290`, `layout-templates.md:10`, `:45`, `test-plan.md:12`, `:27`, `:52`, `:57`, `:89`, `:93`,
    `:101`, `:105`, `:184`, `:186`, `:230`, `:289`, `obs-plan.md:70`, `:117`, `:118`, `a11y-plan.md:37`, `:78`, `:147`,
    `:151`, `:161`, `:170`, `:190`, `:200`, `:216`, `:217`, `:285`, `:287`, `:330`; **stale (the cited lines hold
    nothing of the claim)** — `test-plan.md:278`'s `stand_settle.rs:182-214` (the removed `ManualNetProvider`, carried with
    the §8 row above); **no-change** — `design-system.md:291`'s `stand_act_timer.rs:1-12`, the `//!` doc, which did not
    move (the file's first added line is 19, new-text listing). A `…:1-N` header citation of a `//!` doc keeps its lines
    where the doc did not move.
  - Test counts: **none moved — verified.** blitz-tests 139 result lines · 586 passed · 0 failed · 12 ignored, and the
    listing byte-identical (139 targets, 598 tests) to `evidence/blitz-tests-before.txt` written on the untouched tree;
    workspace `lines 199 passed 1045 failed 0 ignored 12`; agent-run `run stand` 178 · 0 · 7, `run
    font_face_sniffed_source` 2 · 0 · 0; a11y leg 43 (6 · 6 · 4 · 8 · 5 · 14); CI per-OS tallies equal to CI#39324662409
    (basis: `evidence/counts.md`, `evidence/ci.md`).
  - Sweep reach: 8 patterns · 8 moved facts, 8 with a pattern (the report's own count: shared modules, readers, declare
    none, stand-ins, bootstrap, helpers, census, citations; `helpers` control unfired) · owner pattern control unfired
    (the refusal line above names `title`) · by line 24 lines · 196 offsets.
  - Long-line (line, fact) dispositions — every line over 2 000 chars the `by line` block names: `architecture.md:137`
    shared-mods stale · `:190` cites19 re-point · `:201` cites19 re-point · `:221` shared-mods no-change · `:230`
    shared-mods no-change, cites19 re-point · `:350` shared-mods stale, cites19 re-point · `security-plan.md:121` cites19
    re-point · `:125` cites19 re-point, owner no-change · `:133` cites19 re-point · `test-plan.md:10` shared-mods
    no-change · `:27` shared-mods stale, cites19 re-point · `:50` shared-mods no-change · `:68` shared-mods stale,
    declnone no-change · `:102` shared-mods stale, declnone no-change · `:103` shared-mods stale, readers no-change,
    declnone no-change · `:105` shared-mods no-change, cites19 re-point · `:184` cites19 re-point · `:336` shared-mods
    no-change · `obs-plan.md:70` shared-mods stale, cites19 re-point, declnone no-change, census no-change, readers
    no-change · `:118` cites19 re-point · `a11y-plan.md:170` cites19 re-point · `:217` cites19 re-point, shared-mods
    no-change · `:284` shared-mods no-change · `:285` cites19 re-point.
  - Owner pattern `owner` (`owned by a (route entry|CARRY)`): standing 2 (`architecture.md:90` the Upstream-sync CARRY,
    `security-plan.md:125` @c9272 an id's exit) → no-change, neither about this chunk; base 2 (`playbook.md:58`, `:66`) →
    no-change.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none — `scripts/agent-run.sh` and `.github/scripts/test_agent_run.py` unchanged (no `run
  fixtures` selection: the operator's answer at P4, «Existing selections (Recommended)»). The agent-run selections run
  every reader: `run stand` the 18 `stand_*` readers among the 19, `run font_face_sniffed_source` the one non-stand
  reader, which holds a load.
- **Cross-project / external claims:**
  - `inputs: absent — no external input snapshotted` (`inputs.py verify`, this wrap).
  - CI#39336988025 on `0fbdda02242a` (the pre-CI commit; the wrap's own commit then adds bookkeeping on top): `verdict:
    green · checks 17/17 · wall 1017 s`, per-OS tallies equal to CI#39324662409 (`evidence/ci.md`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none.
- **Expected amendments (from plan):**
  - test-plan §1 — blitz-tests holds three shared modules that are no target, `fixtures` joining `common` and
    `session_common`, with its nineteen readers → carried: Counts «shared modules 2 → 3» and «Readers 0 → 19»; search
    `shared-mods` (test-plan 10 rows: `:27` stale, `:68` stale, the rest no-change) + `readers` (test-plan 1, no-change).
  - test-plan §3 Crate-local test helpers — `fixtures/mod.rs` and what it states; the «NOT YET MEASURED — the test-data
    bootstrap mechanism» line replaced by the measured reading → carried: Symbols / APIs, Counts «bootstrap»; search
    `shared-mods` (test-plan `:102`, `:103` stale) + `bootstrap` (test-plan 1: `:131` stale).
  - test-plan §7 Test Data & Fixtures and §8 Hand-written fakes and stubs (Network rows) — the shared pages, and one
    `HeldNet` in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider` and
    `stand_boot`'s `RecordingNetProvider`, upstream's two rows unchanged → carried: Counts «held network stand-ins»;
    search `heldnet` (test-plan 3: `:12` stale, `:278` stale, `:277` no-change; 0 in every other master).
  - architecture §Conventions → Tests — the third shared module and the reader counts, re-measured at the wrap →
    carried: Counts «shared modules» (common 21, session_common 25, fixtures 19); search `shared-mods` (architecture 6:
    `:137` stale, `:350` stale, `:221`/`:230`/`:234`/`:351` no-change).
  - obs-plan §3 Logging stack — `fixtures/mod.rs` in the silence census; `stand_mcp_masked`, `stand_boot` and
    `font_face_sniffed_source` now declare a shared module → carried: Counts «silence census» and «declare none»; search
    `shared-mods` (obs-plan 5: `:70` stale, the rest no-change) + `census` (obs-plan 3, no-change) + `declnone` (obs-plan
    1, no-change).
  - security-plan §Input Validation (`@font-face` source row, Fetched fonts row) and §API Security (Settle and loads in
    flight row) — `file:line` citations re-pointed to the moved stand-in; coordinates only → carried: Counts «Citations»;
    search `cites19` (security-plan 6: `:101`, `:103`, `:121`, `:125`, `:133`, `:362`, all re-point). The «every request
    held by a hand provider» wording at `:103` stays true of `HeldNet`.
  - design-system §Motion — the `stand_settle.rs:279-307` citation re-pointed after the file shrinks; coordinates only →
    carried: Counts «Citations»; search `cites19` (design-system 2: `:290` re-point, `:291` no-change).
- **Coverage of new surfaces:**
  - `fixtures::HeldNet` (test-only network stand-in, in process) → validation n/a (no input crosses; its bodies are
    `'static` test bytes) · instrumentation n/a (no log site, by the silence census) · PII n/a · tests integ
    (`stand_settle::a_load_in_flight_reads_not_settled_until_it_is_answered`, `font_face_sniffed_source`'s two,
    `stand_boot`'s two, both layout modes) · a11y n/a · tokens n/a.
  - `fixtures::{boot, hold, password_labelled, password_named, covered}` (test-only) → validation n/a ·
    instrumentation n/a · PII n/a (the password pages' typed text read as `MASKED_VALUE` by the moved checks) · tests
    integ (19 readers, both layout modes) · a11y n/a (no stand control or colour moved; `stand_keyboard` reads the moved
    boot, a11y leg 43 green) · tokens n/a (no CSS moved; the `covered` page's inline `#cccccc` overlay is test markup
    moved byte for byte).

## Deviations from intent
1. **`stand_mcp_tools`' covered site (plan step 6).** The step lists it under `fixtures::hold` with «its label at
   `:532`», but that site is `serve_mcp`'s boot closure: the label is the MCP caller's `app` and the closure returns the
   `Result` the server handles, which a `hold` that unwraps cannot serve. Written as step 5 writes `stand_mcp_masked`'s:
   `Session::start(app, || fixtures::boot(fixtures::covered, incremental))`. Every probe the plan counts reads as
   predicted (`Harness::from_vdom` 7; the covered page stated once).
2. **`stand_replay`'s label** is written `slug(LeanTask::Counter)`, the expression the removed `hold_variant` carried, in
   place of the literal `"counter"` step 7 names — the same value; the helper's doc moved to a one-line comment at its one
   call site.
3. **`stand_session_recording`'s hold** now panics, on a refused label only, under the shared message («the label is one
   of the closed set») where its own helper read «the fixture's label is a label» — panic-path text, no assertion.
4. **`stand_snapshot_text`'s page** gained the root id `fx-root` (the plan's implementation note). No reading moved: its
   two password checks green in both modes, so the file's own page was not kept.
5. `font_face_sniffed_source` discards `answer`'s returned url as a plain statement (no `#[must_use]`; clippy clean).

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 20 · listed 20 · recorded 0 (companion 0
· mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 49`, this wrap's P1 read).

## Decisions & corrections
- Plan-step ambiguity met at implement: step 6 named an MCP boot closure as a `fixtures::hold` site; resolved by step 5's
  form and surfaced in the outcome (friction record `2026-10-19T22:31:26Z-b`, `input.plan-step-ambiguous`). A P4 plan
  naming a hold site should check whether the site's label is the caller's (a `serve_mcp` boot closure returns
  `Result<Session, SessionError>`).
- Guard refusal: the Bash guard (`bash-guard.sh`, cd arm) refused a `cd` into `tests/blitz-tests/tests` at implement P1;
  re-issued once as a subshell `( cd … && … )` — a recurrence of the standing learning host-linux §Paths («`cd` only
  inside a subshell»), which the previous wrap's handoff also listed as a recurrence.
- Hook refusal: the stamp-ahead PostToolUse hook refused an estimated end time written into `evidence/counts.md`
  («Stamp ahead of the clock: 22:41Z (+2 min) … while the clock reads 22:38:10Z»); re-written to cite the agent-run boot
  event's own stamp (2026-10-19T22:33:21Z). An evidence time is read off a recorded event or `date -u`, never estimated.
- Sweep hazard: the token `HeldNet` is a prefix of the removed `HeldNetProvider`, so a bare `HeldNet` grep counts the
  old name's sites as the new one's — this wrap's pattern used `HeldNet\b` beside the old names.
- Operator direction (2026-10-19, this session, in the operator's words): «Run the operator pass for
  2026-10-19-shared-fixtures now, each entry in plan order: 21 hygiene; the commit with the subject exactly
  "chore(2026-10-19-shared-fixtures): operator pre-CI commit, for the run this chunk's verdict reads"; 22 the fast leg
  with the clean-tree guard and the push; 23 ci.py conclusion; 24 the per-OS tallies. Record it in evidence/ci.md. Stop
  and tell me if any entry is red.» — the agent fired the pass on that word: the commit staged the whole tree (as the
  previous pre-CI commit `62403b643` did), entry 22 through `gate.py run --operator 22` (its `history moved` the intended
  push), entry 23 as written, entry 24 through `--operator 24 --id 39336988025`; none red.
- Operator direction at this wrap's invocation: «run P1 only and stop» — the two-window form; P2–P7 resume from this run
  dir (`.andromeda/runs/2026-10-19T23-02-04-wrap/`).
- P4 forks answered by the operator (recorded in the plan's Provenance, 2026-10-19, dialog; technical): «Third module
  fixtures/ (Recommended)» · «Existing selections (Recommended)».

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) blitz-tests exits 0 with no `FAILED`; the listing equals `evidence/blitz-tests-before.txt` byte for byte; every
  moved check keeps its test name and asserts what it did in both layout modes — **met** (entries 1, 2).
- (tests) `fixtures/mod.rs` exists, is no target, and is the one statement of the boot, hold, three pages and `HeldNet`;
  no file-private fixture boot or hold left; `Harness::from_vdom` at the seven non-fixture sites; nineteen readers —
  **met** (entries 6, 7, 8; the two MCP boot closures build through `fixtures::boot`, Deviation 1).
- (tests) one escher network stand-in, upstream's two files byte-unchanged — **met** (entries 4, 12).
- (security) `stand_settle` reads `NotSettled { busy: Loads }` while a load is held and settles once answered;
  `font_face_sniffed_source` asks once, registers the `wOF2` body and nothing for the unsniffable one; the module holds no
  env read, file, socket, listener or `blitz_net`; the env-read set unchanged — **met** (entries 1, 9, 11).
- (security) every moved password check still reads `MASKED_VALUE` and 0 occurrences of its typed text — **met** (entry 1:
  `stand_act_diff`, `stand_diff`, `stand_snapshot_text`, `stand_mcp_masked`, `stand_session_recording` green).
- (obs) silence census 0 in `fixtures/mod.rs`, 4 over the files touched, none added — **met** (entries 9, 10).
- (tests) agent-run boot `ready`; `run stand` 178 · 0 · 7; `run font_face_sniffed_source` 2 — **met** (entries 14–16).
- (a11y) a11y leg passes, 43 tests; `common/mod.rs` and `stand_contrast.rs` untouched — **met** (entry 17, entry 12).
- (layouts · design) no `examples/` change; obstructed, scroll, transformed and covered checks read what they read —
  **met** (entries 12, 1).
- (tests) fast and doc legs pass; workspace 199 · 1045 · 0 · 12; the off-unix check compiles; CI green with stand-repeat
  three rounds green — **met** (entries 18–20, 3; CI#39336988025).

Gates (the final state rests on the operator pass: `0fbdda022`, its CI run CI#39336988025 recorded in `evidence/ci.md`;
implement's run `.andromeda/runs/2026-10-19T22-26-15-implement/`, trail `gate-2026-10-19-shared-fixtures.json`, one
firing, `entries 24 · green 20 · red 0 · recorded 0 · timeout 0 · not-run 4`):
- `cargo test -p blitz-tests --locked 2>&1` — green · exit 0 · `lacks FAILED` held.
- `cargo test -p blitz-tests --locked -- --list 2>/dev/null | cmp - …/evidence/blitz-tests-before.txt` — green · exit 0.
- `cargo check --tests --locked --target x86_64-pc-windows-gnu -p blitz-tests` — green · exit 0.
- `grep -l 'NetProvider for' … | paste -sd' '` — green · last line the three files.
- `grep -l -E '^fn (password_fixture|covered_fixture)\(' … | paste -sd' '` — green · last line `stand_act_filled`,
  `stand_act_obstructed`, `stand_act_screenshot`.
- `cat … | grep -c -E '^fn (boot_fixture|hold_fixture|hold|hold_variant)\(|^fn fixture\(app:'` — green · exit 1 · 0.
- `cat … | grep -c 'Harness::from_vdom'` — green · 7.
- `grep -l '^mod fixtures;' … | wc -l` — green · 19.
- `grep -c -E 'println!|…|blitz_net' …/fixtures/mod.rs` — green · exit 1 · 0.
- `cat …{nineteen}.rs …/fixtures/mod.rs | grep -c -E 'println!|…|set_global_default'` — green · 4.
- `cat … | grep -oE 'env!…' | sort | uniq -c | awk …` — green · the env-read line unchanged.
- `git diff --name-only 5174f92a… -- packages examples tests apps wpt .github scripts Cargo.toml Cargo.lock deny.toml |
  grep -c -v -x -E '…'` — green · exit 1 · 0.
- `git ls-files --others --exclude-standard -- … | grep -c -v -x 'tests/blitz-tests/tests/fixtures/mod.rs'` — green ·
  exit 1 · 0.
- `bash scripts/agent-run.sh boot` — green · `"outcome": "ready"`.
- `bash scripts/agent-run.sh run stand` — green · `"selection": "stand", "passed": 178, "failed": 0, "ignored": 7`.
- `bash scripts/agent-run.sh run font_face_sniffed_source` — green · passed 2, failed 0, ignored 0.
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0 (6 · 6 · 4 · 8 · 5 · 14).
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 · artifact `target/ci-logs/test.log` fresh.
- `grep -E '^test result:' target/ci-logs/test.log | awk …` — green · `lines 199 passed 1045 failed 0 ignored 12`.
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — leg operator: fired as written before the
  pre-CI commit, `hygiene: clean — read 41 …` (`evidence/ci.md`).
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
  build/escher-0.2.0` — leg operator: green · exit 0 · `history moved: refs/remotes/origin/build/escher-0.2.0
  5174f92a→0fbdda02` (the intended push) (`evidence/ci.md`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — leg operator:
  `0fbdda02242a verdict: green · checks 17/17 · wall 1017 s · runs CI#39336988025 completed/success` (`evidence/ci.md`).
- `gh run view <id> -R Turbolet85/escher --log | … awk …` — leg operator, `recorded` (expect = [], exit 0) with
  `<id>` = 39336988025: Test [default features] 199 · 1045 · 0 · 12; Test (macos) 180 · 1042 · 0 · 12; Coverage report
  180 · 1042 · 0 · 12; Test (windows) 180 · 1000 · 0 · 9; Accessibility (a11y) tests 6 · 43 · 0 · 0; Stand checks repeated
  156 · 624 · 0 · 21 — every job equal to CI#39324662409, the outcome green and this chunk's (`evidence/ci.md`).
- Smoke: skipped — no boot-path or UI-surface change (test code only); no `smoke` or `self-verify` entry.

Watches: none folded.

Outcome basis: the operator pass (`0fbdda022`, the one commit since the base; CI#39336988025 on its sha, recorded in
`evidence/ci.md`), implement's P4 report as given in this window and persisted at
`.andromeda/runs/2026-10-19T22-26-15-implement/outcome-2026-10-19-shared-fixtures.md` (outcome green), and
`evidence/counts.md`. No directive between implement and this report changed the tree.

Process hygiene: implement P4's census — cargo / rustc started by the gate entries, `terminated` (`census: none
running`); the session hosts of `stand_session_lifecycle` / `stand_session_quiet` started by a gate entry's test run,
`terminated`. The operator pass's entries 22–24 ran in this session and left nothing running. Re-measured at this wrap:
`gate.py census --comm escher-session --comm cargo --comm rustc --args agent-run.sh` → `census: none running`.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 5174f92a (the parent of the oldest pre-CI commit 0fbdda02) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### tests/blitz-tests/tests/fixtures/mod.rs — new file · 107 line(s)
- 17-20 @18 «pub fn boot(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {»
- 22-25 @23 «pub fn hold(label: &str, app: fn() -> Element, incremental: bool) -> Session {»
- 27-35 @28 «pub fn password_labelled() -> Element {»
  - 29-34 «rsx! {»
- 37-44 @38 «pub fn password_named() -> Element {»
  - 39-43 «rsx! {»
- 46-62 @48 «pub fn covered() -> Element {»
  - 49-61 «rsx! {»
- 64-68 @66 «pub struct HeldNet {»
- 70-98 «impl HeldNet {»
  - 71-74 @72 «pub fn held(&self) -> usize {»
  - 76-84 @77 «pub fn urls(&self) -> Vec<String> {»
  - 86-97 @88 «pub fn answer(&self, body: &'static [u8]) -> String {»
- 100-107 «impl NetProvider for HeldNet {»
  - 101-106 «fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {»
### tests/blitz-tests/tests/font_face_sniffed_source.rs — added 7 line(s) in 4 range(s)
added: 7 · 10 · 13-15 · 33-34
### tests/blitz-tests/tests/stand_act_diff.rs — added 3 line(s) in 3 range(s)
added: 17 · 20 · 273
### tests/blitz-tests/tests/stand_act_filled.rs — added 3 line(s) in 3 range(s)
added: 15 · 18 · 203
### tests/blitz-tests/tests/stand_act_obstructed.rs — added 7 line(s) in 7 range(s)
added: 27 · 250 · 309 · 378 · 517 · 532 · 567
### tests/blitz-tests/tests/stand_act_screenshot.rs — added 3 line(s) in 3 range(s)
added: 16 · 21 · 244
### tests/blitz-tests/tests/stand_act_scroll.rs — added 4 line(s) in 4 range(s)
added: 20 · 264 · 318 · 429
### tests/blitz-tests/tests/stand_act_spans.rs — added 4 line(s) in 4 range(s)
added: 29 · 38 · 653 · 680
### tests/blitz-tests/tests/stand_act_timer.rs — added 11 line(s) in 4 range(s)
added: 19 · 21 · 170-177 · 211
### tests/blitz-tests/tests/stand_act_transformed.rs — added 3 line(s) in 3 range(s)
added: 14 · 80 · 105
### tests/blitz-tests/tests/stand_boot.rs — added 8 line(s) in 7 range(s)
added: 5 · 10-11 · 22 · 89 · 96 · 102 · 107
### tests/blitz-tests/tests/stand_diff.rs — added 3 line(s) in 3 range(s)
added: 26 · 93 · 689
### tests/blitz-tests/tests/stand_keyboard.rs — added 10 line(s) in 10 range(s)
added: 34 · 37 · 444 · 462 · 479 · 496 · 520 · 547 · 618 · 756
### tests/blitz-tests/tests/stand_mcp_masked.rs — added 3 line(s) in 2 range(s)
added: 13-14 · 25
### tests/blitz-tests/tests/stand_mcp_tools.rs — added 4 line(s) in 3 range(s)
added: 30 · 37 · 514-515
### tests/blitz-tests/tests/stand_replay.rs — added 9 line(s) in 4 range(s)
added: 16 · 18 · 20 · 203-208
### tests/blitz-tests/tests/stand_session_recording.rs — added 9 line(s) in 5 range(s)
added: 24 · 26 · 28 · 201 · 253-257
### tests/blitz-tests/tests/stand_settle.rs — added 19 line(s) in 11 range(s)
added: 5 · 11 · 19 · 21 · 140 · 166 · 191 · 211-215 · 230-234 · 294 · 337
### tests/blitz-tests/tests/stand_snapshot_state.rs — added 7 line(s) in 7 range(s)
added: 18 · 21 · 211 · 515 · 553 · 619 · 670
### tests/blitz-tests/tests/stand_snapshot_text.rs — added 5 line(s) in 4 range(s)
added: 11-12 · 15 · 349 · 380
