# Fan-out results — 2026-10-07-settle-detection

Seven detectors, one per spec source, each given its document, the chunk report and its drift-base entries (15 detector entries over the seven prompts: architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 — the drift-base's own 15). Every return was entity-decoded and probed: 0 entities left in each. Dispositions are the orchestrator's (amendment-flow §Validate).

## Verdicts

- **architecture** — 12 proposals (D-arch-resources 12; D-arch-decisions no drift). Stripped: one comment line stating the D-arch-decisions verdict.
- **security-plan** — `proposals: []`. Stripped: per-detector bases and a note naming the plan's expected §API Security entry as outside its detectors. Raw twin: `.raw-fanout-security-plan.md`.
- **design-system** — `proposals: []`. Stripped: the basis and a note naming the expected §Motion entry and the moved harness-clock citation. Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: the basis and a note on the two `harness.rs` citations at line 44, the second imprecise before this chunk. Raw twin: `.raw-fanout-layout-templates.md`.
- **test-plan** — 23 proposals (D-tests-coverage 13 · D-tests-framework 10; D-tests-obs-harness no drift). Stripped: a header of per-detector verdicts, sites checked and left alone, and a statement that three coordinates were read from the tree.
- **obs-plan** — 2 proposals (D-obs-instrumentation 2; D-obs-stack and D-obs-pii no drift). Stripped: trailing comment lines with the two clean verdicts and the sites swept.
- **a11y-plan** — `proposals: []`. Stripped: per-detector bases and a note naming the expected §2 entry and the changed set's state at boot. Raw twin: `.raw-fanout-a11y-plan.md`.

## architecture — the return as it arrived (12 proposals)

```yaml
proposals:
  # D-arch-decisions: no drift. Report Dependencies = "none added, none bumped"; no new library, runtime, thread, feature or env read; §Established Decisions → [Driver session] is upheld (wire/host/client byte-identical, socket still answers hello/stop only, no tracing). Its one stale citation is carried below as a dependent of D-arch-resources.
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Test harness
    change: >-
      The export list gains `Busy`, `NotSettled`, `SETTLE_PASS_LIMIT` and `Settled`, and its citation moves `packages/blitz-test-harness/src/lib.rs:20-22` → `lib.rs:23-26`; add `Harness::settle(&mut self) -> Result<Settled, NotSettled>` (packages/blitz-test-harness/src/settle.rs:151-186): it repeats `pump`'s pass (`doc.poll(None)`, then `resolve` at the harness's own clock) until a pass is quiet (poll answered false, hover node unmoved, no load finished), at most `SETTLE_PASS_LIMIT = 64` passes; `Settled { passes: u32, animating: bool }`; `NotSettled { busy: Busy }` with `Busy { Render, Layout, Loads }` naming a class only, `Display` + `std::error::Error`, fixed messages; a load in flight (or `has_pending_critical_resources()`) reads `Loads` at once and is never waited on; settle moves no time (a timer not yet due or a running animation does not hold it open; the animation is reported through `animating`), reads no clock but the harness's, neither reads nor drains the changed set, logs nothing, starts no thread; a supplied `net_provider` whose `is_noop()` is false is wrapped in a crate-private counting `NetProvider` (`LoadCounter`, counts only), an offline or `wrap`ped harness carries none; `pump`, `tick` and every input helper are unchanged (none settles).
    sidecar: >-
      2026-10-07-settle-detection — Test harness contract registers `Harness::settle`, `Busy`, `Settled`, `NotSettled`, `SETTLE_PASS_LIMIT` and the counted net provider; re-export citation lib.rs:20-22 → 23-26.
    rationale: >-
      Report Changes → Symbols / APIs lists these as NEW public API of blitz-test-harness (re-exported at lib.rs:26) with the "What settle does" / "What settled means" bullets; architecture's Test harness contract names none of them (grep `settle` over architecture: 2 hits, `ResizeSettleCheck` and the Driver session "Not built" clause). Report's Expected amendments entry 2 names this site.
    basis: .andromeda/architecture.md:130 · packages/blitz-test-harness/src/lib.rs:23-26 · packages/blitz-test-harness/src/settle.rs:151-186
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Test harness
    change: >-
      The two harness.rs citations behind the `HarnessOptions` eight-field claim move — `packages/blitz-test-harness/src/harness.rs:12-26` → `harness.rs:14-28` (the struct) and `harness.rs:44-59` → `harness.rs:46-71` (`into_config`, which now returns `(DocumentConfig, Option<Arc<LoadCounter>>)`); the eight public fields and `Default` are unchanged.
    sidecar: >-
      2026-10-07-settle-detection — Test harness contract: harness.rs citations 12-26 → 14-28 and 44-59 → 46-71 (coordinates only; fields unchanged).
    rationale: >-
      Report Changes → Counts / qualifiers moved, "Citation coordinates moved" — `harness.rs:` architecture 2; `HarnessOptions` struct `12-26` → `14-28`, `into_config` `44-59` → `46-71`. Same contract line as the primary; the old ranges now point at the wrong lines.
    basis: .andromeda/architecture.md:130 · packages/blitz-test-harness/src/harness.rs:14-28 · packages/blitz-test-harness/src/harness.rs:46-71
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The re-export list gains `Busy` and `Settled` (from blitz-test-harness; `NotSettled` and `SETTLE_PASS_LIMIT` are not re-exported) and its citation moves `packages/escher-driver/src/lib.rs:20-32` → `lib.rs:21-34`; the session offers `label()`, `harness()`, `harness_mut()` and `act(&mut self, step: impl FnOnce(&mut Harness<DioxusDocument>)) -> Result<Settled, SessionError>` — runs the step on the held instance, settles it (`Harness::settle`) and maps an unsettled instance onto `SessionError::NotSettled(Busy)`; a step that leaves the instance unsettled is not rolled back; `act` is an in-process call and nothing it reads or returns crosses the socket; the session still holds a label and the harness only; citation `packages/escher-driver/src/session.rs:11-62` → `session.rs:11-79`.
    sidecar: >-
      2026-10-07-settle-detection — Driver session contract registers `Session::act` and the `Busy` / `Settled` re-exports; citations lib.rs:20-32 → 21-34, session.rs:11-62 → 11-79.
    rationale: >-
      Report Changes → Symbols / APIs: "NEW in `escher-driver`: `Session::act` …(session.rs:63-78)" and "NEW re-exports in the driver: `pub use blitz_test_harness::{Busy, Settled};` (lib.rs:30)"; architecture's Driver session contract lists the re-exports and the session's methods exhaustively and names neither. Report's Expected amendments entry 1 names this site.
    basis: .andromeda/architecture.md:133 · packages/escher-driver/src/session.rs:63-78 · packages/escher-driver/src/lib.rs:21-34
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The "`SessionError` is exactly …" list gains a twelfth variant after `Io(std::io::ErrorKind)` — `NotSettled(Busy)`; the message sentence reads: every message is a fixed string — the `Io` arm appends the kind's name and the `NotSettled` arm the class's name ("the instance did not go quiet after a step: {Render|Layout|Loads}") — and none carries a path, a label, an id or a value; citation `packages/escher-driver/src/error.rs:10-63` → `error.rs:12-70`.
    sidecar: >-
      2026-10-07-settle-detection — Driver session contract: `SessionError` gains the twelfth variant `NotSettled(Busy)`; citation error.rs:10-63 → 12-70.
    rationale: >-
      Report Changes → Symbols / APIs: "NEW twelfth variant `SessionError::NotSettled(Busy)` (error.rs:35-36), message `the instance did not go quiet after a step: {Render|Layout|Loads}`"; Counts: "`SessionError` variants: 11 → 12". Architecture states the enum "is exactly" eleven named variants and that only the `Io` arm appends a name.
    basis: .andromeda/architecture.md:133 · packages/escher-driver/src/error.rs:12-70
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Driver session (escher-driver)
    change: >-
      The closing clause drops `settle` from the not-built list and reads: Not built: a verb set, a settle verb or busy-source reply on `session.sock` (settle exists in process only, as `Session::act`), act by id, CLI JSON, an MCP tool, an idle expiry — a host whose client died without `stop` keeps running; a real driver session still has nobody delivering the Timer's ticks (time stays caller-driven) — with the measured-at pointer extended to escher-0.1.0/chunks/2026-10-07-settle-detection/report.md.
    sidecar: >-
      2026-10-07-settle-detection — Driver session contract: "Not built: … settle" superseded — settle is built in process (`Harness::settle`, `Session::act`); no settle verb crosses the socket.
    rationale: >-
      Report Changes → Spec claims disproved by measurement, third bullet: "A standing statement now SUPERSEDED … architecture §Standard Contracts → Driver session, 'Not built: a verb set, settle, act by id, CLI JSON, an MCP tool, an idle expiry' (architecture line 133) — settle is built"; Reverted / negative API facts: "no settle verb or busy-source reply on `session.sock`". This is the only `Not built` occurrence in architecture (1 hit).
    basis: .andromeda/architecture.md:133
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-test-harness
    change: >-
      The row reads: Modules harness, input, inspect, settle (packages/blitz-test-harness/src/lib.rs:18-21).
    sidecar: >-
      2026-10-07-settle-detection — Existing Scopes, blitz-test-harness row: fourth module `settle`; citation lib.rs:16-18 → 18-21.
    rationale: >-
      Report Changes → Crates / modules: "`blitz-test-harness` (a fourth module, `settle`, beside `harness`, `input`, `inspect` — `lib.rs:18-21`)"; citation move "the modules `16-18` → `18-21` (four now)". The row restates the harness's module set as three.
    basis: .andromeda/architecture.md:255 · packages/blitz-test-harness/src/lib.rs:18-21
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → escher-driver
    change: >-
      The row's session module reads "session (`Session`, the label rule, the step-then-settle `act`)", the error module "error (`SessionError`)" stays, "re-exported at the crate root" gains "beside blitz-test-harness's `Busy` and `Settled`", and the citation moves `packages/escher-driver/src/lib.rs:22-32` → `lib.rs:23-34`; the dependency sentence (blitz-test-harness and dioxus-native-dom only, no feature, no app) is unchanged.
    sidecar: >-
      2026-10-07-settle-detection — Existing Scopes, escher-driver row: `Session::act` and the `Busy` / `Settled` root re-exports; citation lib.rs:22-32 → 23-34.
    rationale: >-
      Report Changes → Symbols / APIs (driver: `Session::act`, re-exports at lib.rs:30) and citation moves "modules and re-exports … `22-32` → `23-34`". The row restates the crate-root surface the Driver session contract registers.
    basis: .andromeda/architecture.md:257 · packages/escher-driver/src/lib.rs:23-34
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Existing Scopes → blitz-tests
    change: >-
      The row gains, after the driver-session clause, the settle check — a step on a held instance returning with its delayed update present and no sleep, an idle settle changing nothing, a load in flight read as not settled, a running animation reported and not waited on, an instance that never goes quiet ending at the bound, in both layout modes (tests/blitz-tests/tests/stand_settle.rs) — and the closing clause reads: the module `tests/session_common/mod.rs` — no test target and no test in it, read through `mod session_common;` by five checks, four of the session checks (`stand_session_fresh`, `stand_session_ids`, `stand_session_lifecycle`, `stand_session_quiet`) and `stand_settle`; `mod common;` stays at eleven readers.
    sidecar: >-
      2026-10-07-settle-detection — Existing Scopes, blitz-tests row: new test target `stand_settle`; `mod session_common;` readers four → five.
    rationale: >-
      Report Changes → Crates / modules: "One new integration-test target, `stand_settle` in blitz-tests; it declares `mod session_common;` and not `mod common;`"; Counts: "Readers of `mod session_common;`: 4 → 5 … Stated as 'four' in … architecture line 263 ('read through `mod session_common;` by four of them')". Report's Expected amendments entry 3 names this row.
    basis: .andromeda/architecture.md:263 · tests/blitz-tests/tests/stand_settle.rs
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Tests
    change: >-
      The shared-module clause reads: … and the checks that hold a `Session` — the session checks and the settle check `stand_settle` — keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;` (tests/blitz-tests/tests/session_common/mod.rs:1-6).
    sidecar: >-
      2026-10-07-settle-detection — Conventions → Tests: `session_common` is no longer shared by the session checks only; `stand_settle` reads it too.
    rationale: >-
      Same retired claim as the blitz-tests row, worded differently: "the session checks keep what only they share in a second such module". Report Counts: the fifth reader is `stand_settle`, "which is not a `stand_session_*` file".
    basis: .andromeda/architecture.md:115
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Driver session]
    change: >-
      Citation only — `packages/escher-driver/src/lib.rs:12-18` → `lib.rs:13-19` (the crate doc's socket paragraphs); the decision's text is unchanged and upheld (lifecycle messages only, nothing of the screen on the wire, no env read, no `tracing`, no idle expiry).
    sidecar: >-
      2026-10-07-settle-detection — Established Decisions → [Driver session]: citation lib.rs:12-18 → 13-19 (coordinates only).
    rationale: >-
      Report Changes → Counts / qualifiers moved, citation moves: "`escher-driver/src/lib.rs` — the crate doc `1-18` → `1-19` (the socket paragraphs `12-18` → `13-19`; the new sentence is at `6-7`)"; one of the 5 architecture citations into the driver's three files.
    basis: .andromeda/architecture.md:101 · packages/escher-driver/src/lib.rs:13-19
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Design Philosophy → Headless, measurable, testable
    change: >-
      Citation only — the harness-runs-headless sub-bullet's `packages/blitz-test-harness/src/lib.rs:1-14` → `lib.rs:1-16` (the "No window, GPU, or compositor" sentence now sits at line 16, outside the old range).
    sidecar: >-
      2026-10-07-settle-detection — Design Philosophy, headless harness bullet: citation blitz-test-harness lib.rs:1-14 → 1-16 (coordinates only).
    rationale: >-
      Report Changes → citation moves: "`blitz-test-harness/src/lib.rs` — the crate doc `1-14` → `1-16` (… two new lines, `10-11`, are the settle bullet)"; one of the 4 architecture citations into that file.
    basis: .andromeda/architecture.md:35 · packages/blitz-test-harness/src/lib.rs:1-16
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Conventions → Documentation
    change: >-
      Citation only — `packages/blitz-test-harness/src/lib.rs:1-14` → `lib.rs:1-16` in the doc-comment citation list.
    sidecar: >-
      2026-10-07-settle-detection — Conventions → Documentation: citation blitz-test-harness lib.rs:1-14 → 1-16 (coordinates only).
    rationale: >-
      Same citation move as the Design Philosophy site (report: crate doc `1-14` → `1-16`); the second of the two `lib.rs:1-14` occurrences in architecture (lines 35 and 108).
    basis: .andromeda/architecture.md:108 · packages/blitz-test-harness/src/lib.rs:1-16
    dependent-of: D-arch-resources
```

### architecture — dispositions

1. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §Standard Contracts → Test harness)
2. apply — check 1 (playbook: Accurate this-chunk addition) · dependent of 1, applied with it
3. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §Standard Contracts → Driver session)
4. apply — check 1 (playbook: Accurate this-chunk addition) · dependent of 3, applied with it
5. apply — check 1 (playbook: Accurate this-chunk addition) · check 6 (the superseded "Not built: … settle" clause) · applied without the proposal's clause on the Timer's ticks, which the report carries under "Owed to the route" and P5 pins on the route
6. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
7. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
8. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §Existing Scopes, the blitz-tests row)
9. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied — a site the report's own sweep did not name; read by window at architecture.md:115@c3249
10. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied
11. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied
12. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied

## test-plan — the return as it arrived (23 proposals)

```yaml
# test-plan drift-detector — chunk 2026-10-07-settle-detection
# Detector verdicts (invariant level):
#   D-tests-coverage   — invariant HOLDS (every new surface carries unit + integration tests, report.md:77-78; tier 0). The proposals below are the doc's stale coverage inventory / counts, not a missing test.
#   D-tests-framework  — invariant HOLDS (cargo test, inline #[cfg(test)] units, one blitz-tests target, the Harness; report.md:124-139). The proposals below are the §3 Harness contract and its citations, which the chunk's checks now use through `settle` / `act`.
#   D-tests-obs-harness — NO DRIFT, no proposal: report.md:48 "Harness / gate surface: none" and report.md:31 "No wire shape, status shape, log field or scrub set changed"; obs-plan carries no `run stand` count and no settle text (0 hits). One cross-doc note: obs-plan §3 line 69 says "the five `stand_session_*` checks and ... their module session_common" — the reader-count edits below (test-plan lines 65, 100) need the obs-plan detector's matching edit (report.md:72 already expects it).
# Checked and left alone (no change in the report retires them): line 12 (`has_changes` false on a fresh blitz-dom document — a bare document, not a booted harness; report.md:56), line 90 (input helpers still pump, input.rs byte-identical), line 99 (`mod common;` readers stay eleven), lines 102/106/114/208 (host answers hello/stop only; two process checks), line 298 (`mock|Mock|stub|fake` over settle.rs: 0 hits).
# Coordinates not in the report were read from the tree for citation only: settle.rs test module 193-462, stand_settle.rs test 1 at 40-81 and ManualNetProvider at 184-224.
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-test-harness"
    change: >-
      The bullet should now also say the crate holds its first unit tests since 2026-10-07-settle-detection — 11 inline in `settle.rs`: 4 on the load counter, 6 on the settle loop over a scripted document and a wrapped HTML document, 1 on the `NotSettled` messages (packages/blitz-test-harness/src/settle.rs:193-462) — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md.
    sidecar: "2026-10-07-settle-detection: §1 blitz-test-harness — the crate's first unit tests, 11 in settle.rs (4 counter · 6 loop · 1 messages)."
    rationale: >-
      Report Counts: "blitz-test-harness unit tests: 0 → 11, the crate's first (cargo test -p blitz-test-harness --locked --lib settle, 11 passed)". The bullet names the crate with no test inventory, so the new tier-0 coverage is not carried.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:38"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (new bullet: blitz-test-harness)"
    change: >-
      Add a **blitz-test-harness** bullet: `settle.rs` pins the load counter (an answered request reaches the inner handler and leaves none in flight; a handler dropped unanswered leaves none; the finished count grows by one per handler and never falls; `is_noop` is the inner provider's), the settle loop (an idle document settles in one pass; work for k polls settles in k+1 passes; work for ever ends at `SETTLE_PASS_LIMIT` as `Render`; a request in flight after a quiet pass reads `Loads`; a handler finishing inside a pass costs one more pass; a wrapped HTML document is read through `has_pending_critical_resources()`) and that every `NotSettled` message is non-empty, distinct and holds no path — 11 tests (packages/blitz-test-harness/src/settle.rs:193-462).
    sidecar: "2026-10-07-settle-detection: §4 gains a blitz-test-harness bullet — 11 settle.rs unit tests."
    rationale: >-
      Report Counts (11 = 4 counter + 6 loop + 1 messages) and Deviations (the extra test `a_wrapped_document_is_read_through_its_render_blocking_resources`, the only one reaching the critical-resources arm). §4 lists unit coverage per crate and has no entry for this crate.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:38"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: >-
      The bullet should now also name settle detection over the stand — `stand_settle`, 8 checks, each in both layout modes, calling no sleep, clock, `pump` or `tick`: a Timer step through `Session::act` returns reading `Elapsed: 0.3s` having read `0.0s` inside the step; a settle on an idle lean task is `Settled { passes: 1, animating: false }` and changes nothing (same snapshot text, `has_changes` false after a drain, no focus, first Tab on `back-btn`); a write made in a `mounted` handler is present after the step; a load in flight reads `NotSettled(Loads)` until the check answers it, an `@import` included; a running animation reads `animating: true` and does not hold the step; an instance that never goes quiet ends at the bound as `Render`, a hover that never rests as `Layout`; a hover that moves with layout is resolved before the step returns (tests/blitz-tests/tests/stand_settle.rs:1) — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md.
    sidecar: "2026-10-07-settle-detection: §1 tests/blitz-tests — adds stand_settle (8 checks, both layout modes)."
    rationale: >-
      Report Files (new `tests/blitz-tests/tests/stand_settle.rs`), Crates/modules ("One new integration-test target, stand_settle") and "What the stand check reads" (eight tests). The bullet enumerates every stand file's coverage and stops at the five `stand_session_*` files.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:58"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session ↔ held instance"
    change: >-
      The bullet should now also say a step run through `Session::act` returns settled — the Timer's delayed update present, an idle step changing nothing, a `mounted` write present — or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable, in both layout modes, with no sleep and no clock read (tests/blitz-tests/tests/stand_settle.rs:40-81 and tests 2, 3, 6 of the same file).
    sidecar: "2026-10-07-settle-detection: §5 Session ↔ held instance — adds Session::act / settle coverage by stand_settle."
    rationale: >-
      Report Symbols/APIs (NEW `Session::act`, twelfth variant `SessionError::NotSettled(Busy)`) and Coverage of new surfaces ("stand_settle tests 1, 2, 3, 6"). The boundary bullet names only the three `stand_session_*` files; the report's Expected amendments list this site.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:78"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: >-
      Append one link to the chain: re-counted at 2026-10-07-settle-detection: `run stand` 80 `ok` stand events (+8 `stand_settle`, picked up by its `stand_` prefix with no script change; the selection lists 20 files), its `run.end` reading passed 80 · failed 0 · ignored 3 — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md.
    sidecar: "2026-10-07-settle-detection: §3 Proof — run stand 72 → 80 (+8 stand_settle), ignored 3 unchanged, 20 files."
    rationale: >-
      Report Counts: "`run stand`: 72 passed · 0 failed · 3 ignored → 80 · 0 · 3 (the run.end line of implement's gate run; the selection lists 20 files)"; it names test-plan line 115 as the stating site. The chain's last link still reads 72. The report measured `run.end`'s `passed`; "ok events" is the chain's own wording for the same count.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:35"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline (the `cargo test --workspace` chain)"
    change: >-
      Append one link to the chain: re-counted at 2026-10-07-settle-detection: 143 result lines, 598 passed · 0 failed · 8 ignored (+11 `blitz-test-harness` unit tests, the crate's first, adding no result line; +8 `stand_settle` stand checks in one new result line), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs at /implement and in its operator pass on the tree its pre-CI commit 0e4434ec carries (escher-0.1.0/chunks/2026-10-07-settle-detection/report.md).
    sidecar: "2026-10-07-settle-detection: §9 Local baseline — workspace 142 · 579 · 0 · 8 → 143 · 598 · 0 · 8."
    rationale: >-
      Report Counts: "Workspace test run: 142 result lines · 579 passed · 0 failed · 8 ignored → 143 · 598 · 0 · 8 ... The one new result line is stand_settle; the 19 new passes are 11 harness unit tests and 8 stand_settle checks", naming test-plan line 322 as the site. The chain's last link still reads 142 / 579.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:34"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: >-
      "read through `mod session_common;` by four `stand_session_*.rs` checks" should now read: by five checks — four `stand_session_*.rs` and `stand_settle.rs` (which declares `mod session_common;` and not `mod common;`); the `mod common;` count stays eleven.
    sidecar: "2026-10-07-settle-detection: §2 Directory pattern — session_common readers four → five (stand_settle)."
    rationale: >-
      Report Counts: "Readers of `mod session_common;`: 4 → 5 ... and now stand_settle, which is not a stand_session_* file. Stated as 'four' in ... test-plan line 65. Readers of `mod common;`: unchanged at eleven."
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:37"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: >-
      The reader list should now read: `stand_session_fresh`, `stand_session_ids`, `stand_session_lifecycle`, `stand_session_quiet` and, since 2026-10-07-settle-detection, `stand_settle` (no `stand_session_*` file) declare `mod session_common;`, and `stand_session_state` starts its sessions inline; the opening "what only the session checks share" should read "what the checks that hold a session share". The module itself is unchanged (citation `session_common/mod.rs:1-145` stays).
    sidecar: "2026-10-07-settle-detection: §3 session checks helper bullet — stand_settle is a fifth reader of session_common."
    rationale: >-
      Second statement of the four-reader claim retired at line 65: line 100 enumerates exactly four declaring files and scopes the module to "only the session checks". Report Counts (readers 4 → 5) and Crates/modules ("stand_settle ... declares `mod session_common;` and not `mod common;`"); both shared modules are under the preservation gate (report Files, untouched).
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:37"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (key file registries/contracts/test-plan/session-lifecycle.md), label session-proof"
    change: >-
      The proof row should now also carry `stand_settle` 8 (`Session::act` in its tests 1, 2, 3 and 6) — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md, with the fork's CI run 37610657363 on `0e4434ec` (16 of 16 jobs green, read by conclusion only) as the witness of the macOS, windows, iOS and android legs; escher-driver unit tests stay 13.
    sidecar: "2026-10-07-settle-detection: session-proof — adds stand_settle 8 and CI run 37610657363 on 0e4434ec; driver unit tests 13 unchanged."
    rationale: >-
      Report Outcome ("stand_settle passes with 8 passed"), Counts ("escher-driver unit tests: 13, unchanged") and Cross-project claims (CI#37610657363, sha 0e4434ec, checks 16/16, jobs read by conclusion only). The session contract gains `act` this chunk (session-start proposal below) and its proof row lists no check of it.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:51"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: >-
      Keep 13 inline unit tests (error.rs 3 · session.rs 2 · wire.rs 8); the citations should now read packages/escher-driver/src/error.rs:72-120 and packages/escher-driver/src/session.rs:81-103 (wire.rs:143-297 unmoved), and the error.rs clause should say the every-message check now covers the twelfth variant `NotSettled` in each of its three `Busy` classes.
    sidecar: "2026-10-07-settle-detection: §1 escher-driver — citations error.rs 65-110 → 72-120, session.rs 64-86 → 81-103; message check covers NotSettled(Busy)."
    rationale: >-
      Report citation map: session.rs "the unit tests 64-86 → 81-103"; error.rs "the unit tests 65-110 → 72-120". Report Counts: 13 unchanged, "the hand-kept list in every_message_is_non_empty_and_holds_no_path grew from 14 values to 17 (one per Busy class)".
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:45"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: >-
      The citations should now read packages/escher-driver/src/session.rs:81-103 and packages/escher-driver/src/error.rs:72-120 (wire.rs:143-297 unmoved), and the error.rs clause should add that the fixed-message pin includes `NotSettled` for `Render`, `Layout` and `Loads` — "the instance did not go quiet after a step: {class}", a class name and no path.
    sidecar: "2026-10-07-settle-detection: §4 escher-driver — same two citation moves; error.rs pin extends to the three NotSettled messages."
    rationale: >-
      Same two stale coordinates as the §1 escher-driver bullet (report lines 45-46), restated at test-plan line 141. Report Symbols/APIs: twelfth variant `SessionError::NotSettled(Busy)`, message `the instance did not go quiet after a step: {Render|Layout|Loads}`; Coverage: "fixed message plus a class name, no path — unit-tested".
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:46"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs → Network (`ManualNetProvider`)"
    change: >-
      The row should now also name `stand_settle`'s file-private `ManualNetProvider`, which keeps every request so the check answers a head stylesheet and then its `@import` when it chooses, on `.test` URLs with no real fetch (tests/blitz-tests/tests/stand_settle.rs:184-224), beside the existing render_blocking_stylesheet.rs source.
    sidecar: "2026-10-07-settle-detection: §8 Network — a second file-private ManualNetProvider, in stand_settle.rs."
    rationale: >-
      Report "What the stand check reads" test 4: "a file-private `ManualNetProvider` that keeps every request ... No real fetch is issued"; Outcome: "The loads leg is proven on a fixture whose answers the check delivers itself — met: test 4, ManualNetProvider". The row cites one file only; the report's Expected amendments list §8 Network.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:62"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs (new row: Document / network, blitz-test-harness unit tests)"
    change: >-
      Add a row: Document and network (settle unit tests) — a `Scripted` `Document` whose `poll` answers follow a script, and a `Held` `NetProvider` that keeps each handler until the test takes it, stand in for a real document and a fetching provider (packages/blitz-test-harness/src/settle.rs:209-234; packages/blitz-test-harness/src/settle.rs:319-373).
    sidecar: "2026-10-07-settle-detection: §8 gains a row for settle.rs's scripted Document and held-handler NetProvider stand-ins."
    rationale: >-
      Report Counts: the harness unit tests are "4 on the counter, 6 on the loop over a scripted document and a wrapped HTML document". The scripted document is a hand-written stand-in the §8 table does not list. The type names `Scripted` and `Held` and the line ranges were read from settle.rs, not from the report — the orchestrator should re-derive them; drop this row if the table is meant to hold only stand-ins the report names.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:38"
    dependent-of: D-tests-coverage
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (`Harness`) → Core"
    change: >-
      The list should now read: into_inner, base, base_mut, time, pump, tick, dispatch, dispatch_recorded, set_viewport_size (packages/blitz-test-harness/src/harness.rs:126-216), and `settle(&mut self) -> Result<Settled, NotSettled>` in the crate's fourth module (packages/blitz-test-harness/src/settle.rs:151-186), with `Busy`, `Settled`, `NotSettled` and `SETTLE_PASS_LIMIT` (64) re-exported (packages/blitz-test-harness/src/lib.rs:26).
    sidecar: "2026-10-07-settle-detection: §3 Core — adds Harness::settle and its outcome types; citation harness.rs 103-193 → 126-216."
    rationale: >-
      Report Symbols/APIs: NEW `Harness::settle(&mut self) -> Result<Settled, NotSettled>` (settle.rs:151-186), `SETTLE_PASS_LIMIT: u32 = 64`, `Busy`, `Settled`, `NotSettled`, re-exported at lib.rs:26; citation map "into_inner…set_viewport_size 103-193 → 126-216". `stand_settle` drives the instance through settle/act and calls no pump or tick, so the harness contract the chunk's checks use is not the one §3 lists.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:19"
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (`Harness`) → Pump semantics"
    change: >-
      The citation should now read harness.rs:145-170, and the bullet should add settle's semantics: one settle pass is pump's two calls in its order, keeping poll's answer; a pass is quiet when poll answered false, the hover node did not change and no load finished; after a quiet pass a request in flight or a pending critical resource returns `NotSettled { busy: Loads }` at once, otherwise `Settled { passes, animating }`; `SETTLE_PASS_LIMIT` passes with none quiet return `Render`, `Loads` or `Layout`; settle reads no clock but the harness's, advances no time, waits on no load, drains no changed set and logs nothing; `pump` and every input helper are unchanged (packages/blitz-test-harness/src/settle.rs:151-186).
    sidecar: "2026-10-07-settle-detection: §3 Pump semantics — citation 122-147 → 145-170; states what a settle pass is and what settled means."
    rationale: >-
      Report Symbols/APIs "What settle does" and "What 'settled' means for the four sources"; "UNCHANGED in the harness: pump, tick, dispatch, dispatch_recorded ... none [of pump's 44 callers] was changed to settle"; citation map "pump…dispatch_recorded 122-147 → 145-170". The report's Expected amendments name "test-plan §3 Core and Pump semantics".
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:20"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (`Harness`) → Construction"
    change: >-
      The first citation should now read packages/blitz-test-harness/src/harness.rs:83-124 (the constructors still pump once, `wrap` does not).
    sidecar: "2026-10-07-settle-detection: §3 Construction — citation harness.rs 68-101 → 83-124."
    rationale: >-
      Report citation map: "the three constructor impls 68-101 → 83-124 (from_html 85-87, from_html_with 89-96, from_component 101-103, from_vdom 105-113, wrap 117-124)". Old lines 68-101 now span the `into_config` tail and the `Harness` struct.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:42"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test levels observed → Harness-driven tests"
    change: >-
      The bullet should now read "... deterministic construction defaults, a pump/tick loop, `settle` (that pass repeated until no work is due, returning `Settled` or `NotSettled` naming the `Busy` class), inspection helpers and input synthesis (packages/blitz-test-harness/src/lib.rs:1-16)".
    sidecar: "2026-10-07-settle-detection: §2 Harness-driven tests — names settle; citation lib.rs 1-14 → 1-16."
    rationale: >-
      Report citation map: "blitz-test-harness/src/lib.rs — the crate doc 1-14 → 1-16 ... two new lines, 10-11, are the settle bullet". The bullet paraphrases that crate doc and omits the new capability.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:43"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Harness ↔ event pipeline"
    change: >-
      The citations should now read packages/blitz-test-harness/src/lib.rs:13-14 (the input-synthesis bullet) and packages/blitz-test-harness/src/harness.rs:155-193 (dispatch … RecordingHandler); the prose is unchanged.
    sidecar: "2026-10-07-settle-detection: §5 Harness ↔ event pipeline — citations lib.rs 11-12 → 13-14, harness.rs 132-170 → 155-193."
    rationale: >-
      Report citation map: "the input-synthesis bullet 11-12 → 13-14" and "dispatch…RecordingHandler 132-170 → 155-193". lib.rs:11-12 now holds the tail of the settle bullet and the inspection bullet.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:42"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§7 Test Data & Fixtures → Builders and options (the HarnessOptions bullet)"
    change: >-
      The citations should now read harness.rs:13-43 (fields and `Default`) and harness.rs:45-72 (`impl HarnessOptions`, HtmlProvider), and the bullet should add: a supplied `net_provider` whose `is_noop()` is false is wrapped in the crate-private load counter that `settle` reads (it stores counts only); a no-op or absent provider — the stand's — and a `wrap`ped document carry none. Fields and defaults are unchanged.
    sidecar: "2026-10-07-settle-detection: §7 HarnessOptions — citations 11-41 → 13-43 and 43-60 → 45-72; a fetching net_provider is wrapped in the load counter."
    rationale: >-
      Report Symbols/APIs CHANGED: "`HarnessOptions::into_config` (46-72) now returns (DocumentConfig, Option<Arc<LoadCounter>>) — a supplied provider whose is_noop() is false is wrapped in the counter ... a no-op provider and an absent one pass through unwrapped"; UNCHANGED: "HarnessOptions' fields and defaults"; citation map "doc+struct 11-26 → 13-28; Default 28-41 → 30-43; impl HarnessOptions 43-60 → 45-72".
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:23"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs → Events (`RecordingHandler`)"
    change: >-
      The source citation should now read packages/blitz-test-harness/src/harness.rs:176-201.
    sidecar: "2026-10-07-settle-detection: §8 RecordingHandler — citation harness.rs 153-178 → 176-201."
    rationale: >-
      Report citation map: "the recording handler 153-178 → 176-201" (every line from old 103 on is +23).
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:42"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Hand-written fakes and stubs → Time (seven_guis timer)"
    change: >-
      "applied on the next pump" should now read: applied on the next pass — a `pump`, an input helper's own pump, or a `Harness::settle` / `Session::act` pass — so a delivery made before an input helper is applied by that helper's pump and one made after it by nothing until a later pass; settle neither advances time nor waits on a timer not yet due; add tests/blitz-tests/tests/stand_settle.rs:40-81 to the sources.
    sidecar: "2026-10-07-settle-detection: §8 Time — a delivered tick is applied on the next pass (pump or settle); order of delivery and input helper decides."
    rationale: >-
      Report Decisions: "a delivery BEFORE the helper is applied by the helper's pump, a delivery AFTER it is applied by nothing until the next pass"; Spec claims disproved: a bare click then `deliver(3)` reads `0.0s` until `act` makes a pass (evidence/red-first.md readings 3, 3a, 3b); Symbols/APIs: "A source only time moves ... does not hold settle open and is not advanced by it". The report's Expected amendments name the `next pump` site at line 278; pump is no longer the only pass that applies a delivery.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:96"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Session lifecycle (key file registries/contracts/test-plan/session-lifecycle.md), label session-start"
    change: >-
      The row should now also say: a check runs one step and waits for quiet through `Session::act(step) -> Result<Settled, SessionError>`, which runs the closure on the held harness, settles it and maps a not-settled outcome onto `SessionError::NotSettled(Busy)` — a step that leaves the instance unsettled is not rolled back; `act` is in process and nothing it reads or returns crosses the socket (packages/escher-driver/src/session.rs:63-78); `harness()` / `harness_mut()` stay (packages/escher-driver/src/session.rs:35-61, unmoved).
    sidecar: "2026-10-07-settle-detection: session-start — adds Session::act and SessionError::NotSettled(Busy) beside harness() / harness_mut()."
    rationale: >-
      Report Symbols/APIs: NEW `Session::act(&mut self, step: impl FnOnce(&mut Harness<DioxusDocument>)) -> Result<Settled, SessionError>` (session.rs:63-78), "A step that leaves the instance unsettled is not rolled back. harness() / harness_mut() stay (53-61)"; "act is an in-process call; nothing it reads or returns crosses the socket". The key row states `harness()` / `harness_mut()` as the way a check reads the held harness; the report's Expected amendments name this row.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:25"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Session lifecycle (key file registries/contracts/test-plan/session-lifecycle.md), label session-host"
    change: >-
      The last citation should now read packages/escher-driver/src/error.rs:12-37; the listed lifecycle edges are unchanged (the enum's twelfth variant, `NotSettled(Busy)`, belongs to `act` under session-start, not to the host).
    sidecar: "2026-10-07-settle-detection: session-host — citation error.rs 10-33 → 12-37; edge list unchanged."
    rationale: >-
      Report citation map: "escher-driver/src/error.rs — every line from old 5 on is +2: the enum 10-33 → 12-37 (two more lines for the new variant)"; the report counts 2 citations into the driver's files in this key file. host.rs and client.rs are byte-identical, so the other two citations in the row stand.
    basis: "escher-0.1.0/chunks/2026-10-07-settle-detection/report.md:46"
    dependent-of: D-tests-framework
```

### test-plan — dispositions

1. reject as proposed — the re-derivation tell: `settle.rs:193-462` was read from the tree, not the report; the same amendment is raised by the orchestrator from the report (Counts: harness unit tests 0 → 11) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
2. reject as proposed — the re-derivation tell: `settle.rs:193-462` was read from the tree, not the report; the same amendment is raised by the orchestrator from the report (Counts; Deviations: the eleventh test) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
3. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
4. reject as proposed — the re-derivation tell: `stand_settle.rs:40-81` was read from the tree, not the report; the same amendment is raised by the orchestrator from the report (check 5, expected entry: §5 Session ↔ held instance) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
5. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §3 Proof)
6. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §9 Local baseline)
7. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
8. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied — the second statement of the four-reader claim, at test-plan.md:100
9. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied to the key file
10. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
11. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
12. reject as proposed — the re-derivation tell: `stand_settle.rs:184-224` was read from the tree, not the report; the same amendment is raised by the orchestrator from the report (check 5, expected entry: §8 Network) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
13. reject as proposed — the re-derivation tell: the type names `Scripted` and `Held` and both ranges were read from `settle.rs`; the detector says so itself; the same amendment is raised by the orchestrator from the report's "over a scripted document" and its own knowledge of the chunk's code — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
14. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §3 Core)
15. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §3 Pump semantics)
16. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied
17. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
18. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied
19. apply — check 1 (playbook: Accurate this-chunk addition) · dependent, applied
20. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied
21. reject as proposed — the re-derivation tell: `stand_settle.rs:40-81` was read from the tree, not the report; the same amendment is raised by the orchestrator from the report (check 5, expected entry: §8 Time; Decisions: the delivery-order fact) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
22. apply — check 1 (playbook: Accurate this-chunk addition) · check 5 (expected entry: §3 → Session lifecycle, `session-start`), applied to the key file
23. apply — check 1 (playbook: Accurate this-chunk addition) · citation only, applied to the key file

## obs-plan — the return as it arrived (2 proposals)

```yaml
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage (the `packages/escher-driver` observed-absent bullet, obs-plan.md:105)"
    change: "Extend the bullet's enumeration of the silent session surface from `Session::start`, `serve`, `start`, `attach`, `stop` and the lifecycle wire to also name `Session::act` (packages/escher-driver/src/session.rs:63-78) and the settle wait it runs, `Harness::settle` in `packages/blitz-test-harness` (packages/blitz-test-harness/src/settle.rs:151-186; up to `SETTLE_PASS_LIMIT` = 64 passes) — both observed absent of spans, events and a `tracing` dependency, logging nothing by design; the span over the settle wait stays owed by the route entry \"Driver command spans\" (as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md)."
    sidecar: "2026-10-07-settle-detection: §4 driver bullet now names `Session::act` and the harness settle wait `Harness::settle` as built and span-less; the span remains owed by \"Driver command spans\"."
    rationale: "Report Symbols / APIs adds two new operations — `Harness::settle` (settle.rs:151-186) and `Session::act` (session.rs:63-78) — and states \"Settle reads neither `has_changes` nor `take_changed_nodes`, logs nothing, starts no thread\"; Dependencies: \"`escher-driver` still names two dependencies, no feature and no `tracing`\"; Coverage of new surfaces marks both \"instrumentation n/a (it logs nothing by design; neither crate has a `tracing` dependency — the span over the settle wait is the later route entry 'Driver command spans')\"; Outcome (obs) criterion met. §4 requires no span today (it records the driver as silent with the span owed), so nothing is mis-built, but the bullet's enumerated surface omits the new operation and the body has 0 occurrences of `settle`, so the settle wait the owed span is for is not recorded as existing. Listed in the report's Expected amendments (obs-plan §4) as carried."
    basis: ".andromeda/obs-plan.md:105 (enumeration `Session::start`, `serve`, `start`, `attach`, `stop`); packages/escher-driver/src/session.rs:70 (`pub fn act`); packages/blitz-test-harness/src/settle.rs:151 (`pub fn settle`); grep of settle.rs, session.rs, error.rs, stand_settle.rs for `tracing|log::|println!|eprintln!|env::var|span!|instrument` = 0 hits"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract → Logging stack (subscriber installation) (the headless-stand / session-library bullet, obs-plan.md:69)"
    change: "In the no-subscriber census, where it reads \"the same holds for the session library `packages/escher-driver` (no `tracing` dependency, no subscriber, no env read, no print), for the five `stand_session_*` checks and for their module `tests/blitz-tests/tests/session_common/mod.rs`\", add that it also holds for the session step `Session::act`, for the settle loop `packages/blitz-test-harness/src/settle.rs` (`Harness::settle` — no `tracing` dependency in the crate, no subscriber, no env read, no print, no log) and for the check `stand_settle`, which is not a `stand_session_*` file and is the fifth reader of `mod session_common;` (with `stand_session_fresh`, `stand_session_ids`, `stand_session_lifecycle`, `stand_session_quiet`); the `stand_session_*` count stays five and the `mod common;` readers stay eleven; append `as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md` to the bullet's measured-at chain."
    sidecar: "2026-10-07-settle-detection: §3 Logging-stack census extended — `Session::act`, the harness settle loop and `stand_settle` (fifth `mod session_common;` reader, not a `stand_session_*` file) install no subscriber and log nothing."
    rationale: "Duplicate occurrence of the claim the primary retires (the closed membership of the silent session surface), restated in §3 as a census of who installs no sink. Report Counts / qualifiers moved: \"Readers of `mod session_common;`: 4 → 5 … and now `stand_settle`, which is not a `stand_session_*` file\"; Outcome (obs): \"The loop, the session step and the new check install no subscriber, read no env var, print or log nothing; `escher-driver` has no span, event or `tracing` dependency — met\". §3 calls `session_common` \"their module\" of the five `stand_session_*` checks; it is now also read by a check outside that set. Listed in the report's Expected amendments (obs-plan §3 Logging stack) as carried. Note for the orchestrator: D-obs-stack's own invariant HOLDS — Dependencies \"none added, none bumped\", no off-spec logger or OTel setup — so this is a census-completeness amendment, not a stack violation. obs-plan states no numeric count of `mod session_common;` readers, so no number is corrected; the other two `session_common` sites (line 69's citation `session_common/mod.rs:1-6`, line 325's `session_common/mod.rs:70-78`) cite a file under the preservation gate and need no change."
    basis: ".andromeda/obs-plan.md:69; `grep -l 'mod session_common;' tests/blitz-tests/tests/*.rs` = 5 files (stand_session_fresh, _ids, _lifecycle, _quiet, stand_settle); `ls tests/blitz-tests/tests/stand_session_*.rs` = 5; `grep -l 'mod common;'` = 11; packages/blitz-test-harness/Cargo.toml has 0 `tracing|log` hits"
    dependent-of: D-obs-instrumentation
# No proposal — D-obs-stack: report Dependencies "none added, none bumped"; no manifest or lockfile moved; no subscriber, logger or OTel setup introduced. Invariant holds against §3 and the keyed contract `Bootstrap phases` (otel-sdk-install: still no OTel crate; key file read, no edit needed).
# No proposal — D-obs-pii (escalate): the chunk adds no logging. `Busy` "names a class only: no node id, element id, name, URL or value"; `LoadCounter` "stores counts and nothing else — no URL, no request"; `SessionError::NotSettled` message is a fixed string plus a class name, no path (unit-tested, `every_message_is_non_empty_and_holds_no_path`); Schema / config: "No wire shape, status shape, log field or scrub set changed"; `escher-telemetry` and `session_host.rs` untouched, so §6 line 172 ("the `SessionError` message on a failed start or serve … fixed strings holding no path, label, id or value") and §8 line 290 ("no command can type into the held instance yet" — `act` is an in-process closure, nothing crosses the socket) both still hold. pii-scrubbing-wire key unaffected.
# Swept and unchanged: obs-plan cites none of the five files whose line coordinates moved (0 hits for `harness.rs:`, `blitz-test-harness/src/lib.rs:`, `escher-driver/src/{session,error,lib}.rs:`); its driver citations are `packages/escher-driver/Cargo.toml:13-15` and `packages/escher-driver/src/host.rs:35-73`, both byte-identical per the preservation gate. §9 Session state directory row unchanged — `stand_settle` holds sessions in process (`session_common::hold`, `Session::start`) and names no state directory. The leaf `.claude/rules/observability.md:36` ("settle wait … → 'Driver command spans' chunk") is a distillation, left to the cascade.
```

### obs-plan — dispositions

1. reject as proposed — the re-derivation tell: the basis cites the detector's own greps over source files, which the report does not carry; the same amendment is raised by the orchestrator from the report (check 5, expected entry: §4 Span / Trace Coverage) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator
2. reject as proposed — the re-derivation tell: the basis cites the detector's own greps and listings over the tree (`ls stand_session_*.rs`, the harness manifest); the same amendment is raised by the orchestrator from the report (check 5, expected entry: §3 Logging stack) — routine (playbook: Accurate this-chunk addition) → applied, its text and coordinates re-derived by the orchestrator

## Raised by the orchestrator (no detector owns them)

| # | Doc · section | What | Why raised | Disposition |
|---|---|---|---|---|
| O1 | security-plan §API Security | a settle call reports a load in flight at once and never waits on one; its outcome is a returned value naming a class only | check 5 — the plan's expected entry; no security detector covers it (the detector's return says so) | routine (Accurate this-chunk addition) → applied |
| O2 | design-system §Motion → Animation runtime | the settle rule's one answer for the keeps-animating set; the harness-clock citation re-pointed by measurement | check 5 — the plan's expected entry; the tokens detector does not cover it | routine → applied |
| O3 | a11y-plan §2 → Accessibility tree lifecycle | settle neither reads nor drains the changed set; a freshly booted harness document reads the flag true until something drains it | check 5 — the plan's expected entry · check 6 — the report's second disproved claim (the changed set at boot) | routine → applied |
| O4 | layout-templates §Surface: desktop-native → IA notes | the two `harness.rs` citations re-pointed by measurement (the defaults, and the viewport built from them) | the cascade's stale-citation fix; the detector's return names the site and that the second citation pointed at a closing brace before this chunk | routine → applied |
| O5 | test-plan §8 → Hand-written fakes and stubs | a row for the settle unit tests' scripted document and held-handler provider | test-plan proposal 13, rejected as proposed (its names and ranges were read from the tree) and re-derived here | routine → applied |

## Check 6 — the report's disproved claims, each disposed

| Claim | Disposition |
|---|---|
| the plan's prediction that the Timer step is not red before the fix | no master states it (`sites.py` for `[Ss]ettle`: 6 hits read). Its homes are the chunk's frozen documents and the ledger note on v010-10: routed to a dated premise-correction note on v010-10 (P7.3) and to the CARRY P5 pins for the cap's driver leg; the measured order fact lands in test-plan §8 Time (proposal 21) |
| the plan's assertion that `has_changes()` reads false after an idle `act` on a fresh session | stated in a11y-plan §2 by O3; routed to curation (P3) as a testing learning |
| architecture's "Not built: … settle" | architecture proposal 5, applied |

## Checks 2, 3, 4

- **Cross-contradiction (2):** none — architecture proposals 8 and 9 and test-plan proposals 7 and 8 edit the same claim (the readers of `mod session_common;`) in the same direction; obs-plan proposal 2 states it the same way.
- **Intent-consistency (3):** the report's deviations are each justified and none adds behaviour outside the working-route entry or the plan's acceptance criteria; the scope record holds no line (`gate.py scope` clean, 0 recorded). No escalation.
- **Absence needs evidence (4):** the absence claims applied are the report's, each with its search: `settle` 0 hits in security-plan, obs-plan's body and a11y-plan; no master states 19 stand files or a `SessionError` variant count. The cascade sweep re-reads them after the apply (`cascade-dispositions.md`).

## Escalations

None. No proposal matches the playbook's two escalate rules: nothing new crosses a boundary (`Session::act` is an in-process call and the socket's wire is byte-identical; the load counter forwards and stores counts only), and no provisional mark is removed.
