proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → tests/blitz-tests
    change: >-
      After the `accessibility_states` entry, add: since 2026-10-17-upstream-sync-engine-debts's upstream merge (upstream #859 · #1013, at `23b9b26a9`), two checks of the builder's merged ARIA reads on a parsed page — `accessibility_label` 10 (`aria-label` as the label; `aria-labelledby` as `labelled_by` for one and for two nodes; an id the DOM does not hold ignored; the targets' text as the label; `aria-labelledby` outranking `aria-label` and content; a self-reference with nested text; a target's own `aria-label`; hidden targets naming without being referenced; a fall back to `aria-label` when the targets hold no text — tests/blitz-tests/tests/accessibility_label.rs:8-305) and `accessibility_description` 3 (`aria-description` as the description; `aria-describedby` targets as `described_by` in order; targets outside the tree excluded — tests/blitz-tests/tests/accessibility_description.rs:8-101), each its own auto-discovered target, both outside the a11y leg's six.
    sidecar: >-
      test-plan §1 tests/blitz-tests: the inventory gains upstream's `accessibility_label` (10) and `accessibility_description` (3), merged at 2026-10-17-upstream-sync-engine-debts.
    rationale: >-
      The chunk's new aria-labelledby / aria-describedby / aria-description paths are tested at the integration tier by these two merged files (report Coverage of new surfaces; Counts `newt`: test-plan.md:26 changed; Outcome (tests) entries 24–27: each run once); the inventory does not name them.
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:162-167 (newt), :305-312 (coverage rows), New text rows tests/blitz-tests/tests/accessibility_label.rs 8-305 and accessibility_description.rs 8-101; test-plan.md:26
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → tests/blitz-tests
    change: >-
      Beside the two merged checks, record: observed absent — a test naming the builder's merged `aria-expanded` (boolean), `aria-disabled` (`"true"` → disabled) or `aria-level` (with its tag and role defaults, `default_level_for_tag` / `default_level_for_role`) reads · searched: the test functions the chunk added (`accessibility_label.rs`'s 10 and `accessibility_description.rs`'s 3 name only labels and descriptions; `accessibility.rs` gained no test module), as read at 2026-10-17-upstream-sync-engine-debts; the snapshot's `enabled` reads the `disabled` attribute, not AccessKit, so `aria-disabled` moves no snapshot reading.
    sidecar: >-
      test-plan §1 tests/blitz-tests: records as observed absent a test of the merged aria-expanded · aria-disabled · aria-level reads (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      The report credits these three closed-value reads to "upstream's (#1014; in the merged accessibility_label/description files and blitz-dom)" tests, yet its own New text listing names no test function for any of them and lists no test module in accessibility.rs's added ranges — a new engine path for every document with no test at the integration tier the builder's other reads carry (`accessibility_states`).
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:313-317; New text packages/blitz-dom/src/accessibility.rs 1-3 · 15-16 · 41-46 · 61-102 · 118-126 · 190-217 · 243-295 · 362-384 (362-374, 376-383 the level defaults); tests/blitz-tests/tests/accessibility_label.rs 8-305 and accessibility_description.rs 8-101 rows
  - detector: D-tests-coverage
    severity: warning
    section: §10 Quality Gates & Coverage Targets → Coverage floors
    change: >-
      Add: since 2026-10-17-upstream-sync-engine-debts both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, excluding dioxus-native-dom's crate-private `keyboard_compat.rs` (upstream's temporary keyboard-types 0.7/0.8 bridge, 564 lines, its 530 arms untested) until upstream retires it, on the operator's P4 answer (inputs#I2); the floors and `coverage_floor.py` are unchanged; at the merged tree dioxus-native-dom reads 2472/2934 = 84.25 % (equal to the baseline), escher-driver 98.19 %, blitz-test-harness 91.69 %, and the leg ends `coverage floor: 3 of 3 crates at or above their floors`, `keyboard_compat` absent from `target/coverage/lcov.info`; the exclusion is pinned by `test_coverage_leg_reaches_the_floor_on_each_crate` asserting `--ignore-filename-regex` among the leg's words (.github/scripts/ci-leg.sh:41-42; .github/scripts/ci-leg.sh:44-45; .github/scripts/test_ci_workflows.py:327).
    sidecar: >-
      test-plan §10 Coverage floors: the coverage leg excludes keyboard_compat.rs by filename until upstream retires it; floors unchanged, dioxus-native-dom 84.25 %, 3 of 3 (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      keyboard_compat is a new module with no test ("tests: ✗ for its 530 arms (excluded from coverage by filename, P4 fork 3)"); the test-plan must state the exclusion that keeps the dioxus-native-dom floor honest, and its new readings (report Schema / config; Counts `cov`: test-plan.md:343 ×2 changed; Expected amendments: test-plan §10).
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:79-84, :179-187, :287-289, :321-322; New text .github/scripts/ci-leg.sh 41-42 · 44-45, .github/scripts/test_ci_workflows.py 327
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration → Coverage
    change: >-
      The `coverage` leg's two commands now read `cargo llvm-cov --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' --lcov --output-path target/coverage/lcov.info` and `cargo llvm-cov report --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$'` (the regex on both, since 2026-10-17-upstream-sync-engine-debts, §10), the floor script's chain unchanged; cite .github/scripts/ci-leg.sh:41-42 and :44-45 for the two lines.
    sidecar: >-
      test-plan §9 Coverage: both cargo llvm-cov lines take the keyboard_compat.rs filename exclusion (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      Second statement of the coverage leg's command that the exclusion retires (report Counts `cov`: test-plan.md:335 ×5 changed, "the leg's two `cargo llvm-cov` lines").
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:79-82, :182-183; New text .github/scripts/ci-leg.sh 41-42 · 44-45
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy → Directory pattern
    change: >-
      The blitz-tests count reads 127 `.rs` files, 126 test targets and the unbuilt `all.rs`, since 2026-10-17-upstream-sync-engine-debts's upstream merge added `accessibility_description.rs` and `accessibility_label.rs`, each its own auto-discovered target with no manifest line, upstream's `all.rs` gaining their two `mod` lines and staying unbuilt (the tree before it measured 125 and 124 — this line's 124 and 123 were one short, as measured at escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/evidence/before.md); and the run-time read adds "and among 137 at 2026-10-17-upstream-sync-engine-debts" to the `all`-executable history (0 named `all`).
    sidecar: >-
      test-plan §2 Directory pattern: blitz-tests 127 files / 126 targets (the prior 124/123 corrected, one short), the two merged files in the history clause, 137 executables with no `all` (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      The report disproves the standing count by measurement and moves it with the two merged test files (Spec claims disproved: test-plan.md:66 @c2359; Counts `files` 125/124 → 127/126, `newt` test-plan.md:66 @c4381, `exe` 135 → 137 test-plan.md:66 @c5008; Addendum inputs#I4: "test-plan.md:66 moves to the merged 127 files / 126 targets").
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:157-167, :192-194, :248-252, :521; New text tests/blitz-tests/tests/all.rs 4 · 6
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration → Per-OS test tallies
    change: >-
      Re-read at 2026-10-17-upstream-sync-engine-debts on CI#39056637407, the final HEAD `ae8c8daf` (green 17/17, each equal to CI#39043080454's on the merge before its reword): linux 187 · 971 · 0 · 12; macos and coverage 168 · 968 · 0 · 12; windows 168 · 944 · 0 · 9; the a11y job 6 · 43 · 0 · 0; the stand repeat 141 · 576 · 0 · 21; ios and android build only — linux − macos still the 19 `Doc-tests` lines, macos − windows now 24 passed · 3 ignored (the named unix-gated binaries not re-read) — as measured at escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/evidence/ci.md; the moved counts are the two merged files (+13 passed) and the bump crate's empty `unittests src/main.rs` line gone.
    sidecar: >-
      test-plan §9 Per-OS test tallies: re-read at CI#39056637407 — linux 187 · 971 · 0 · 12, macos/coverage 168 · 968, windows 168 · 944 · 0 · 9 (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      The tests run moved with the merge's two test files and bump's departure; the report's Outcome (tests) criterion re-reads the per-OS tallies (Counts `tally`: test-plan.md:325 changed, "last read at 2026-10-14-quality-gates: linux 185 · 951 · 0 · 12").
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:168-178, :285-286, :420-422
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration → workspace tally history
    change: >-
      Append to the dated tally history: re-counted at 2026-10-17-upstream-sync-engine-debts: 187 result lines, 971 passed · 0 failed · 12 ignored, from 186 · 958 (+13 passed, +2 −1 result lines: blitz-tests' `accessibility_description` 3 and `accessibility_label` 10 in two new result lines, upstream's; the bump crate's `unittests src/main.rs` line, 0 · 0 · 0, gone with it), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/evidence/counts.md).
    sidecar: >-
      test-plan §9 tally history: gains 2026-10-17-upstream-sync-engine-debts's 187 · 971 · 0 · 12 (+13 passed: the two merged ARIA files; bump's empty line gone).
    rationale: >-
      Counts `tally`: test-plan.md:331 changed ("the dated tally history, which gains this chunk's entry"); the workspace tally 186 · 958 · 0 · 12 → 187 · 971 · 0 · 12 (fast, entry 23); Deviations: 187 result lines, not the predicted 188.
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:168-177, :331-332, :461
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract → Agent-run contract → Proof
    change: >-
      Append to the agent-run history: re-counted at 2026-10-17-upstream-sync-engine-debts, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` unmoved at passed 168 · failed 0 · ignored 7 over 40 files, and `run all` passed 546 · failed 0 · ignored 12, +13 over 533 (upstream's `accessibility_description` 3 and `accessibility_label` 10, two new files with no `stand_` prefix, picked up by `run all` with no script change) — as measured at escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md.
    sidecar: >-
      test-plan §3 agent-run Proof: gains 2026-10-17-upstream-sync-engine-debts — run stand 168 · 0 · 7 unmoved, run all 546 · 0 · 12 (+13, the two merged files).
    rationale: >-
      Counts `runall`: test-plan.md:116 changed ("the agent-run history … gains this chunk's entry"); Outcome (tests): `run stand` 168 · 0 · 7, `run all` 546 · 0 · 12, `agent-run.sh` unchanged (Harness / gate surface).
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:195-201, :215-216, :468-469
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → apps
    change: >-
      The inventory row reads "apps (rdme, examples, accesskit_xplat)": the `bump` crate (`apps/bump`) left the workspace with upstream's `23b9b26a9` "Delete the bump crate" at 2026-10-17-upstream-sync-engine-debts, its empty `unittests src/main.rs` result line gone; the slice s04 search stays a dated reading.
    sidecar: >-
      test-plan §1 apps: bump leaves the no-tests inventory (deleted upstream, merged at 2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      Crates / modules: `apps/bump` (crate `bump`) leaves the workspace and its `members`; Counts `bump`: test-plan.md:10 changed (the "apps (rdme, bump, examples, accesskit_xplat)" inventory line).
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:66, :131-132, :168-169
  - detector: D-tests-framework
    severity: warning
    section: §3 Test Harness Contract → blitz-test-harness (Harness) → Core
    change: >-
      The crate re-exports `keyboard_types::Key`, `keyboard_types::Modifiers` and, since 2026-10-17-upstream-sync-engine-debts's move to keyboard-types "0.8", `keyboard_types::NamedKey` (`pub use keyboard_types::{Key, Modifiers, NamedKey};`) — the two types `press_with` takes and the named-key enum a 0.8 `Key::Named` holds — so a crate that depends on the harness names a key with no dependency of its own (escher-driver among them) (packages/blitz-test-harness/src/lib.rs:32).
    sidecar: >-
      test-plan §3 Harness Core: the keyboard re-export gains NamedKey with keyboard-types 0.8 (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      Symbols / APIs: blitz-test-harness/src/lib.rs:32 now re-exports `{Key, Modifiers, NamedKey}`; Counts `kbt` and `reexp`: test-plan.md:88 changed ("the two types `press_with` takes" — now three, with `NamedKey`).
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:45-47, :109-113, :119-121; New text packages/blitz-test-harness/src/lib.rs 32
  - detector: D-tests-framework
    severity: warning
    section: §3 Test Harness Contract → Runners and invocation → Third-party test frameworks
    change: >-
      Beside the slice s05 search, record: blitz-tests carries the third-party assertion crate test-that 0.6.0 with test-that-macro 0.6.0 as dev-dependencies (0.5.2 until 2026-10-17-upstream-sync-engine-debts; reviewed by hand at that bump: Apache-2.0, the same repository, no build script, proc-macro flag unchanged), whose `verify_that!` and `TestResult<()>` upstream's merged `accessibility_label` and `accessibility_description` checks assert through, run by the standard built-in harness.
    sidecar: >-
      test-plan §3 Third-party test frameworks: names test-that 0.6.0 (bumped from 0.5.2, upstream merge) as blitz-tests' assertion dev-dependency (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      The chunk's new tests use an assertion framework the test-plan never names while it reads third-party test frameworks "observed absent"; the Dependencies bullet moves test-that 0.5.2 → 0.6.0 (dev-dependencies of blitz-tests, reviewed by hand), and the New text head lines of both new files are `verify_that!(` under `-> TestResult<()>` functions.
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:72-73; New text tests/blitz-tests/tests/accessibility_label.rs 8-28 @9, 18-27 and accessibility_description.rs 8-28 @9, 18-27
  - detector: D-tests-framework
    severity: warning
    section: §3 Test Harness Contract → Runners and invocation → Rust tests
    change: >-
      Add to the standard-harness line: beside `assert!`/`assert_eq!`, blitz-tests' merged upstream checks (`accessibility_label`, `accessibility_description`) return `TestResult<()>` and assert with test-that's `verify_that!` under the same `#[test]` harness.
    sidecar: >-
      test-plan §3 Rust tests: notes test-that's verify_that!/TestResult in the merged upstream checks (2026-10-17-upstream-sync-engine-debts).
    rationale: >-
      Second statement of the assertion style the merged tests no longer follow alone ("`#[test]` functions and `assert!`/`assert_eq!`"); the report's New text shows both new files asserting through `verify_that!`.
    basis: >-
      escher-0.2.0/chunks/2026-10-17-upstream-sync-engine-debts/report.md:72-73; New text tests/blitz-tests/tests/accessibility_label.rs 18-27 and accessibility_description.rs 18-27
    dependent-of: D-tests-framework
note: >-
  D-tests-obs-harness holds — agent-run.sh, its status shape and the stand log format are unchanged (report Harness / gate surface; obs criterion MET), so test-plan §3 and obs-plan §3 still agree. Not proposed, for the orchestrator's judgement: (1) the dual keyboard-types hazard — a test module glob-importing `dioxus::prelude::*` must spell `::keyboard_types` for the workspace's 0.8 (Deviations; six test sites) — is a test-writing convention §5 could carry; (2) the vibey-script bindings (#1211) are credited to upstream's preact.rs, which gained 2 lines and no test function, so no test is named for them; (3) the test-plan's citations into execute.rs's unit table, ci-leg.sh and dioxus-native-dom files are the citation sweep's.
