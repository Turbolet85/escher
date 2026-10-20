proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "add, beside the stand checks' entries, since 2026-10-19-visual-lint `stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process: a planted page (`lint_page`, tests/blitz-tests/tests/stand_lint.rs:21-103) reads exactly its seven findings in document order with their reasons — overlap naming `fx-overlap-b`, both clips seen inside their boxes and narrower, a truncated text box wider than its 80 px box, contrast 2.8490272 equal to `text_contrast`'s against 4.5 — and its negatives read nothing (tests/blitz-tests/tests/stand_lint.rs:135-186); each lean task at boot reads 0 findings (:188-205); the lint reads and changes nothing (:207-243); after twelve Creates the last CRUD row reads `clipped` inside `crud-list` and its `click` is refused `off-screen` (:245-280); an app with no font is refused `fonts-unavailable` (:282-304); picked up by `run stand` and the stand-repeat leg by its prefix; the reading of a clipped or off-screen node holding text also as `truncated` is asserted by no check"
    sidecar: "test-plan §1: `stand_lint` 5 added — the visual lint's planted defects, the stand at boot, no change, the CRUD clip vs `off-screen`, `fonts-unavailable` (2026-10-19-visual-lint)"
    rationale: "Report Changes → Files (new `tests/blitz-tests/tests/stand_lint.rs`), Symbols / APIs (`Session::lint`), Coverage of new surfaces «tests unit (2) + integ (`stand_lint` 5, both modes)»; Outcome (tests, v020-05 in process) MET with the readings; Decisions «Untested reading». A new integration check the scope summary does not state is a coverage-record gap."
    basis: "test-plan.md:27; tests/blitz-tests/tests/stand_lint.rs:135-304"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → blitz-dom covered behaviours"
    change: "add: since 2026-10-19-visual-lint the read-only reader `BaseDocument::text_extent` (`TextExtent { content, text }`), by blitz-tests' `text_extent` 3, both layout modes, on HTML pages booted with the stand's options — a line that cannot wrap reads wider than its box (tests/blitz-tests/tests/text_extent.rs:34-56), a wrapped paragraph reads inside its box (:58-83), and an element with no inline content, or a stale id made by `remove_and_drop_node`, reads `None` (:85-110); its own auto-discovered target, run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`; its stated limits (RTL and vertical text read as LTR horizontal lines, a transformed box read at its client rect) are asserted by no check"
    sidecar: "test-plan §1: blitz-dom's `text_extent` reader covered by blitz-tests' `text_extent` 3 (2026-10-19-visual-lint)"
    rationale: "Report Changes → Files (new `packages/blitz-dom/src/text_extent.rs`, `tests/blitz-tests/tests/text_extent.rs`), Symbols / APIs (`TextExtent`, `text_extent`, its `None` cases and rustdoc limits), Coverage of new surfaces «tests integ (`text_extent` 3, both modes, both builds through the fast leg)», Deviation 8; Outcome gate `run text_extent` 3 · 0 · 0."
    basis: "test-plan.md:12; tests/blitz-tests/tests/text_extent.rs:34-110"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "«92 unit tests in fifteen files, 58 inline and 34 in four `#[path]` test files» → «94 unit tests in sixteen files, 60 inline and 34 in four `#[path]` test files» (92 in fifteen until 2026-10-19-visual-lint prepended to the history), adding lint.rs 2 (since 2026-10-19-visual-lint: the six defects named in `Defect::ALL` order, and findings ordered by document order then the defect order — packages/escher-driver/src/lint.rs:526-604); and beside «the executor's verbs and its refusals are covered by the fourteen `stand_act_*` integration files» add that `Session::lint`'s readings over a held instance are covered by `stand_lint`, for the same reason"
    sidecar: "test-plan §1 escher-driver: unit tests 92 in fifteen files → 94 in sixteen, lint.rs 2; `Session::lint` covered by `stand_lint` (2026-10-19-visual-lint)"
    rationale: "Report Changes → Files (new `packages/escher-driver/src/lint.rs`), Coverage of new surfaces «tests unit (2)», New text lint.rs `mod tests` 526-604 (`the_six_defects_are_named_in_order` 530-546, `findings_follow_document_order_then_the_defect_order` 575-603); Reverted «a third `lint.rs` unit test … removed», so 2 stands; Counts workspace +10 passed = 5 + 3 + 2."
    basis: "test-plan.md:22; packages/escher-driver/src/lint.rs:526-604"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "add: since 2026-10-19-visual-lint lint.rs that `Defect::ALL` names the six defects in order (`overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`) and that findings come in document order, then `Defect::ALL` order (packages/escher-driver/src/lint.rs:526-604)"
    sidecar: "test-plan §4 escher-driver: lint.rs's two unit tests stated (2026-10-19-visual-lint)"
    rationale: "Report Changes → Symbols / APIs (`Defect::ALL`, `Defect::name()`, «Findings come in document order, then `Defect::ALL` order»), Coverage «tests unit (2)», New text lint.rs 526-604. The §4 per-file unit coverage restates the §1 list."
    basis: "test-plan.md:146; packages/escher-driver/src/lint.rs:526-604"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern)"
    change: "«138 `.rs` files, 137 test targets and the unbuilt `all.rs`» → «140 `.rs` files, 139 test targets and the unbuilt `all.rs`», prepending «138 and 137 until 2026-10-19-visual-lint added `stand_lint.rs` and `text_extent.rs`, each its own auto-discovered target with no manifest line» to the history"
    sidecar: "test-plan §2 Directory pattern: blitz-tests `.rs` files 138 → 140, targets 137 → 139 (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «blitz-tests `.rs` files 138 → 140 and test targets 137 → 139 (the unbuilt `all.rs` unchanged)», pattern `bt-files` «test-plan.md:68 @c3309,3326» changed; Harness / gate surface none (no manifest change)."
    basis: "test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern, the `fixtures` readers)"
    change: "«read through `mod fixtures;` by 19 checks — 18 `stand_*.rs` (…)» → «by 20 checks — 19 `stand_*.rs` (…, `stand_lint` added since 2026-10-19-visual-lint)», `font_face_sniffed_source.rs` kept"
    sidecar: "test-plan §2 Directory pattern: `fixtures` readers 19 (18 `stand_*`) → 20 (19 `stand_*`), `stand_lint` added (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «`tests/fixtures/mod.rs` readers 19 checks (18 `stand_*` + `font_face_sniffed_source`) → 20 (19 `stand_*` + it), `stand_lint` declaring `mod fixtures;`», pattern `fixtures-readers` «test-plan.md:68 @c2451» changed."
    basis: "test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests (fixtures))"
    change: "«19 checks declare `mod fixtures;` — 18 `stand_*` files and `font_face_sniffed_source`» → «20 checks … — 19 `stand_*` files (`stand_lint` since 2026-10-19-visual-lint) and `font_face_sniffed_source`»"
    sidecar: "test-plan §3 fixtures helper: readers 19 → 20 (`stand_lint`) (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts pattern `fixtures-readers` «test-plan.md:104 @c818,854» changed, `stand_lint` declaring `mod fixtures;`."
    basis: "test-plan.md:104"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Test-data bootstrap of the 5-command contract"
    change: "«`run stand` runs the 18 `stand_*` readers of the shared module» → «the 19 `stand_*` readers (`stand_lint` since 2026-10-19-visual-lint)», and the measured «`run stand` 178 · 0 · 7 … as measured at …/2026-10-19-shared-fixtures/report.md» gains the re-reading «`run stand` 183 · 0 · 7 at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md», `run font_face_sniffed_source`'s reading kept"
    sidecar: "test-plan §3 test-data bootstrap: `run stand`'s shared-module readers 18 → 19 and its reading 178 → 183 passed (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts: pattern `fixtures-readers` «test-plan.md:132 @c421» changed (the `stand_*` readers 18 → 19), pattern `stand-count` «test-plan.md:132, under `fixtures-readers`» changed («`run stand` passed 178 · failed 0 · ignored 7 over 44 files → 183 · 0 · 7 over 45»); Outcome gate `run stand` 183 · 0 · 7."
    basis: "test-plan.md:132"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers (blitz-tests (session checks))"
    change: "«(25 readers; 24 before `stand_replay`, …)» → «(26 readers since 2026-10-19-visual-lint added `stand_lint`, which reads `hold`, `act`, `click`, `type_into`, `refused`, `focused` and `slug`; 25 before `stand_lint`, 24 before `stand_replay`, …)»"
    sidecar: "test-plan §3 session_common readers 25 → 26, `stand_lint` added (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «`tests/session_common/mod.rs` readers 25 → 26, `stand_lint` declaring `mod session_common;` (it reads `hold`, `act`, `click`, `type_into`, `refused`, `focused`, `slug`)», pattern `session-readers` «test-plan.md:103 @c2926 \"25 readers\"» changed."
    basis: "test-plan.md:103"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions (Directory pattern, the `session_common` readers)"
    change: "«`tests/session_common/mod.rs`, read through `mod session_common;` by 25 checks (24 before `stand_replay.rs`, …)» → «by 26 checks (25 before `stand_lint.rs` at 2026-10-19-visual-lint, 24 before `stand_replay.rs`, …)»"
    sidecar: "test-plan §2 Directory pattern: session_common readers 25 → 26, `stand_lint.rs` added (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «`tests/session_common/mod.rs` readers 25 → 26»; the report's `session-readers` sweep dispositions :68 as a path citation, but :68 also states the reader count «by 25 checks» — the same claim restated, found by reading, so it moves with :103."
    basis: "test-plan.md:68"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract (Proof)"
    change: "append to the count history: «re-counted at 2026-10-19-visual-lint, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 183 · failed 0 · ignored 7 over 45 files — +5 passed (`stand_lint` 5, a new file picked up by its `stand_` prefix with no script change) — and `run text_extent` 3 · 0 · 0 (a new file with no `stand_` prefix, read by name and by `run all`); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md»"
    sidecar: "test-plan §3 Proof: `run stand` 178 over 44 files → 183 over 45 (`stand_lint` +5), `run text_extent` 3 · 0 · 0 (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «`run stand` passed 178 · failed 0 · ignored 7 over 44 files → 183 · 0 · 7 over 45 (`stand_lint` +5)», pattern `stand-count` «test-plan.md:119, the count history, @c9471, a new reading appended» and `stand-files` «test-plan.md:119 @c9534, the history, appended»; Harness / gate surface «`run stand` … pick up `stand_lint` by its prefix, and `run text_extent` reads it by name»; Outcome gates."
    basis: "test-plan.md:119"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Legs)"
    change: "the stand-repeat leg's «one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 44 files, …» → «45 files, … and `stand_lint` since 2026-10-19-visual-lint», each joining by its prefix"
    sidecar: "test-plan §9 Legs: stand-repeat's `stand_*.rs` files 44 → 45, `stand_lint` joining by its prefix (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «`stand_*.rs` files 44 → 45, read by `run stand` and the stand-repeat leg», pattern `stand-files` «test-plan.md:323 @c1299» changed; Harness / gate surface (`ci-leg.sh` byte-unchanged, the leg picks up `stand_lint` by its prefix)."
    basis: "test-plan.md:323"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "«run the 44 `stand_*.rs` files» → «run the 45 `stand_*.rs` files (44 until 2026-10-19-visual-lint added `stand_lint`, whose CI `Stand checks repeated` on CI#39348671810 read 159 · 639 · 0 · 21, +3 · +15)»"
    sidecar: "test-plan §10 Flakiness budget: stand-repeat files 44 → 45 (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts pattern `stand-files` «test-plan.md:352 @c294» changed; the per-OS table «stand repeat 156 · 624 · 0 · 21 → 159 · 639 · 0 · 21»."
    basis: "test-plan.md:352"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts (Per-OS test tallies)"
    change: "append: «Re-read at 2026-10-19-visual-lint on CI#39348671810, the pre-CI commit `5a9b806de` (green 17/17): linux 201 · 1055 · 0 · 12; macos and coverage 182 · 1052 · 0 · 12; windows 182 · 1010 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 159 · 639 · 0 · 21; ios and android build only — +2 result lines and +10 passed on the four test jobs over CI#39336988025 (`stand_lint` 5, `text_extent` 3, lint.rs's 2 units), +3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/ci.md»"
    sidecar: "test-plan §9 Per-OS tallies: CI#39348671810 appended — linux 201 · 1055, macos/coverage 182 · 1052, windows 182 · 1010, stand repeat 159 · 639 (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «Per-OS CI tallies, previous run CI#39336988025 → this run CI#39348671810. Pattern `ci-tally`: … changed (test-plan.md:333)» with its before/after table; pattern `ws-tally` «test-plan.md:333 @c2896» changed; Cross-project claims (CI#39348671810, sha `5a9b806de224`, green 17/17)."
    basis: "test-plan.md:333"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline (`cargo test --workspace`)"
    change: "append: «re-counted at 2026-10-19-visual-lint: 201 result lines, 1055 passed · 0 failed · 12 ignored, from 199 · 1045 — +2 result lines (`stand_lint`, `text_extent`) and +10 passed (`stand_lint` 5, `text_extent` 3, escher-driver's lint.rs 2), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-19-visual-lint/evidence/counts.md)»"
    sidecar: "test-plan §9 Local baseline: workspace 199 · 1045 · 0 · 12 → 201 · 1055 · 0 · 12 (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «Workspace tally 199 result lines · 1045 passed · 0 failed · 12 ignored → 201 · 1055 · 0 · 12. Basis: gate entry 14 over the fresh `target/ci-logs/test.log`. Pattern `ws-tally`: … test-plan.md:339 @c18243» changed."
    basis: "test-plan.md:339"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Coverage floors"
    change: "«since 2026-10-19-test-splits escher-driver reads 4266/4350 = 98.07 %, 3 of 3 crates still at or above their floors» → keep it as history and add «and since 2026-10-19-visual-lint, which added lint.rs, 4634/4732 = 97.93 %, the 97 % floor holding, 3 of 3 crates at or above their floors (the `Coverage report` job of CI#39348671810)»"
    sidecar: "test-plan §10 Coverage floors: escher-driver 98.07 % → 97.93 % (4634/4732), floor 97 % holds (2026-10-19-visual-lint)"
    rationale: "Report Changes → Counts «escher-driver line coverage 4266/4350 = 98.07 % → 4634/4732 = 97.93 %; floor 97 % holds, 3 of 3 crates at or above their floors. Basis: the `Coverage report` job of CI#39348671810 … Pattern `cov-driver`: … changed (test-plan.md:351 @c2168)»."
    basis: "test-plan.md:351"
note: "D-tests-framework and D-tests-obs-harness hold — the runner is the built-in cargo harness, `scripts/agent-run.sh`, `ci-leg.sh` and the workflows are byte-unchanged (Harness / gate surface none), and no status shape or log format moved; Retired is none, so no D-retired. All proposals carry D-tests-coverage; the dependents name it. The :68 session_common reader count (@c1374, «by 25 checks») is not in the report's changed rows — its sweep dispositioned :68 as a path citation — found by reading. The gated v020-05 arm (founder's word on F1) is plan-sanctioned and owes no test-plan text."
