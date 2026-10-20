proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "add after the `stand_load_wait` entry: since 2026-10-20-select-by-id `select_interaction` 7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix (run by the workspace `test` leg, by name and by `run all`, not by `run stand`), in both layout modes on a controlled single-choice select page whose `oninput` writes `selected` from the event's value — the select lays out as a box holding its chosen option; the choice seeded from the markup; ArrowDown · ArrowUp · Home · End choosing an enabled option over a 14-row table; typed text choosing the first enabled option its label starts, over a 7-row table; typed text kept for one second of the document's clock; a disabled select taking no key and no click; an app writing `selected` on another option re-seeding the choice — each table row asserting `has_changes` and one `input` and one `change` per changed choice, none for no change (tests/blitz-tests/tests/select_interaction.rs:113-343); and `stand_act_select` 5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a page writing `selected` statically: at boot `fx-pick` reads `ComboBox`, `Fruit`, value `b`, its three options under it named Apple · Banana · Cherry, Banana selected; `type --id fx-pick --text Cherry` accepted and settled, its diff naming `fx-pick` with `value` in `differs` and `fx-banana` / `fx-cherry` with `selected`, the next snapshot value `c`; a click then a key choosing an option; the Tab walk `fx-before` → `fx-pick` → `fx-named` → `fx-after` → `fx-before`, the disabled `fx-off` skipped, equal before and after the choices; a disabled select and an unchosen option refused (`click --id fx-apple` refused `covered`) (tests/blitz-tests/tests/stand_act_select.rs:79-256); picked up by `run stand` and the stand-repeat leg by its prefix — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md"
    sidecar: "test-plan §1 tests/blitz-tests: `select_interaction` 7 (the select's keys, typeahead, events, disabled gate, re-seed) and `stand_act_select` 5 (the select through `Session::run`) entered"
    rationale: "Report Changes Files: `tests/blitz-tests/tests/select_interaction.rs` and `stand_act_select.rs` new; Coverage of new surfaces: tests integ `select_interaction` 7, `stand_act_select` 5; Outcome (tests) v020-08 met and (a11y) keyboard operable met; Deviation 6 (the controlled fixture app, `stand_act_select` static); Stated limits (`click --id fx-apple` refused `covered`, measured); New text rows select_interaction.rs 113-343 and stand_act_select.rs 79-256"
    basis: ".andromeda/test-plan.md:27"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "the `accessibility_label` entry reads `accessibility_label` 13 (10 until 2026-10-20-select-by-id, which added three for HTML's labelable elements in both layout modes — a `<label for>` names a select and a textarea, a wrapping `<label>` names a select and a textarea, a `<label>` names no hidden input, tests/blitz-tests/tests/accessibility_label.rs:309-432) beside its earlier ten"
    sidecar: "test-plan §1 tests/blitz-tests: `accessibility_label` 10 → 13 (the labelable label's three tests)"
    rationale: "Report Counts / qualifiers moved, pattern `acc-label`: `accessibility_label` tests 10 → 13, stale test-plan.md:27 @c13334 («`accessibility_label` 10»); Outcome (a11y) a `<label>` names a `select` and a `textarea`, a hidden input nothing — `accessibility_label` 13 passed (+3, both modes); New text rows accessibility_label.rs 309-348, 350-388, 390-432"
    basis: ".andromeda/test-plan.md:27"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours"
    change: "add: since 2026-10-20-select-by-id the single-choice `select` model — its rules by four inline `node::select` tests (§4) and, over a Dioxus page in both layout modes, by blitz-tests' `select_interaction` 7 (the box holding the chosen option, the markup seed, the arrows, `Home` and `End`, the typeahead and its one second of the document's clock, one `input` and one `change` per changed choice, the `is_disabled` gate on key and click, a `selected` write re-seeding the choice); its stated limits — a `multiple` select and an `optgroup`'s options not modelled, no popup painted, an option with no author key reading in `unkeyed_actionable` — are asserted by no check"
    sidecar: "test-plan §1 blitz-dom covered behaviours: the single-choice select model and its proofs stated, its stated limits asserted by no check"
    rationale: "Report Symbols / APIs (the select's rules, Keyboard, Pointer, Mutator, Seeding); Coverage of new surfaces: tests unit select.rs 4, integ `select_interaction` 7; Outcome (arch) and (a11y) met; Stated limits (plan step 17): `multiple` and `optgroup` not modelled, no popup, the option in `unkeyed_actionable` «the plan's statement, not measured»"
    basis: ".andromeda/test-plan.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom covered behaviours"
    change: "add: since 2026-10-20-select-by-id a `<label>` binds every labelable element (`label_bound_input_element`), its name read checked by `accessibility_label`'s three new tests, while its click arm — a click on a select's label re-dispatched to the select, focusing it — is asserted by no check"
    sidecar: "test-plan §1 blitz-dom covered behaviours: a click on a select's label focusing the select recorded as asserted by no check"
    rationale: "Report Symbols / APIs: `label_bound_input_element` now matches HTML's labelable elements, its two callers moved in behaviour — the accessibility tree's label loop and `handle_click`'s label arm («a click on a select's label re-dispatches to the select, focusing it»); Coverage of new surfaces names `accessibility_label` +3 (names) and no check of the label click; the named tests of `select_interaction` and `stand_act_select` (New text rows) hold none for it"
    basis: ".andromeda/test-plan.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom (document, mutator, net, form)"
    change: "add: since 2026-10-20-select-by-id form.rs writes a named `select`'s entries — every chosen option that is not disabled, its `option_value` (packages/blitz-dom/src/form.rs:276-290) — with no test of its own, beside the range input's `a_range_input_submits_its_current_value`"
    sidecar: "test-plan §1 blitz-dom form: a select's form entry recorded as untested"
    rationale: "Report Symbols / APIs, Form (form.rs:276-290): a named `select`'s entries; Deviation 4 (every chosen, enabled option written); Coverage of new surfaces lists no test for the form entry and the +21 named tests (`node::select::tests` 4, `snapshot::tests` 2, `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3) hold none in form.rs — a new path with no test at the tier its range-input precedent set"
    basis: ".andromeda/test-plan.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom (node)"
    change: "add: since 2026-10-20-select-by-id `node/select.rs` holds 4 inline table tests of the select's rules — the selectedness setting algorithm keeping one chosen option, a step skipping disabled options and stopping at the ends, typed text matching the first enabled label it starts, text stripped and collapsed (packages/blitz-dom/src/node/select.rs:195-301)"
    sidecar: "test-plan §1 blitz-dom (node): `node/select.rs`'s 4 inline unit tests entered"
    rationale: "Report Symbols / APIs: new module `blitz-dom::node::select`, `mod tests` (195-301, 4 table tests); Counts: +21 by name, `node::select::tests` 4; New text rows select.rs 213-234, 236-263, 265-287, 289-300"
    basis: ".andromeda/test-plan.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → blitz-dom (node)"
    change: "add: since 2026-10-20-select-by-id four `node/select.rs` tests pin the select's rules over stated tables — HTML's selectedness setting algorithm (the last chosen, else the first option not disabled), `stepped` skipping disabled options and stopping at the ends, `matched` choosing the first enabled option whose label starts with the text and nothing for an empty text, and `strip_and_collapse` (packages/blitz-dom/src/node/select.rs:195-301)"
    sidecar: "test-plan §4 blitz-dom (node): the select rules' four unit tests described"
    rationale: "Report Symbols / APIs (`settled` 155-164, `stepped` 166-181, `matched` 183-193, `strip_and_collapse` 147-153, `mod tests` 195-301); New text rows select.rs 213-300 name the four tests"
    basis: ".andromeda/test-plan.md"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: "the count reads «75 unit tests in nine files — since 2026-10-20-select-by-id two snapshot tests, `a_select_reads_its_chosen_options_value` over the four-row `SELECT_VALUES` table and `a_selects_options_are_its_children_named_by_their_labels` (an option's `label` attribute naming it, `Cherries`) (packages/dioxus-native-dom/src/snapshot.rs:932-1014; 73 until that chunk); since 2026-10-19-session-recording …»"
    sidecar: "test-plan §1 dioxus-native-dom: 73 → 75 unit tests (the snapshot's select value and option-children tests)"
    rationale: "Report Symbols / APIs, Snapshot: unit tests `SELECT_VALUES` (936-953), `a_select_reads_its_chosen_options_value` (955-975), `a_selects_options_are_its_children_named_by_their_labels` (977-1014); Counts: +21 by name, `snapshot::tests` 2; Deviation 3 (`Cherries`); Coverage of new surfaces: snapshot unit tests 2"
    basis: ".andromeda/test-plan.md"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: "the snapshot-builder clause reads «twenty on the snapshot builder (eighteen until 2026-10-20-select-by-id) — … and, since that chunk, a select reading its chosen option's value and its options read as its children named by their labels»"
    sidecar: "test-plan §1 dioxus-native-dom: snapshot-builder tests eighteen → twenty"
    rationale: "Same fact as the primary (`snapshot::tests` +2, report Counts +21 by name); the same line restates the snapshot builder's test count («eighteen on the snapshot builder», its range citing snapshot.rs), which the two new snapshot.rs tests move"
    basis: ".andromeda/test-plan.md"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "the target count reads «143 `.rs` files, 142 test targets and the unbuilt `all.rs` (141 and 140 until 2026-10-20-select-by-id added `select_interaction.rs` and `stand_act_select.rs`, each its own auto-discovered target with no manifest line; 140 and 139 until 2026-10-20-load-wait …)»"
    sidecar: "test-plan §2 Directory pattern: blitz-tests 141 → 143 `.rs` files, 140 → 142 test targets"
    rationale: "Report Counts, pattern `bt-files`: blitz-tests `.rs` files 141 → 143 and test targets 140 → 142, stale test-plan.md:68 @c3507,3524, @c3567 its history clause gaining this chunk's step; Harness / gate surface: two new blitz-tests targets"
    basis: ".andromeda/test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "`tests/session_common/mod.rs` is read through `mod session_common;` by 28 checks (27 before `stand_act_select.rs` at 2026-10-20-select-by-id, 26 before `stand_load_wait.rs` …) — … the fifteen `stand_act_*.rs` …, six of which declare both (`stand_act_select` declares no `mod common;`)"
    sidecar: "test-plan §2 Directory pattern: `session_common` readers 27 → 28, `stand_act_*` fourteen → fifteen (`stand_act_select`)"
    rationale: "Report Counts, pattern `session-common`: readers 27 → 28 (`stand_act_select`), stale test-plan.md:68 @c1328; pattern `stand-act`: fourteen → fifteen, stale test-plan.md:68 «the fourteen `stand_act_*.rs`»; `stand_act_select` declares no `mod common;` (pattern `fixtures`, «neither new check declares `mod common;`»)"
    basis: ".andromeda/test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "`tests/fixtures/mod.rs` is read through `mod fixtures;` by 23 checks — 21 `stand_*.rs` (`stand_act_select` since 2026-10-20-select-by-id, `stand_load_wait` since 2026-10-20-load-wait, …), `font_face_sniffed_source.rs` and, since 2026-10-20-select-by-id, `select_interaction.rs`"
    sidecar: "test-plan §2 Directory pattern: `fixtures` readers 21 → 23 (`stand_act_select`, `select_interaction`)"
    rationale: "Report Counts, pattern `fixtures`: readers 21 → 23 (20 → 21 `stand_*` files, `font_face_sniffed_source`, and `select_interaction`, a non-`stand_*` reader), stale test-plan.md:68 @c2542"
    basis: ".andromeda/test-plan.md:68"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (fixtures)"
    change: "«23 checks declare `mod fixtures;` — 21 `stand_*` files (`stand_act_select` since 2026-10-20-select-by-id; `stand_load_wait` since 2026-10-20-load-wait, reading `HeldNet`; `stand_lint` since 2026-10-19-visual-lint), `font_face_sniffed_source` and, since 2026-10-20-select-by-id, `select_interaction`»"
    sidecar: "test-plan §3 fixtures helpers: 21 → 23 readers (`stand_act_select`, `select_interaction`)"
    rationale: "Report pattern `fixtures`: stale test-plan.md:104"
    basis: ".andromeda/test-plan.md:104"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Test-data bootstrap of the 5-command contract"
    change: "«`run stand` runs the 21 `stand_*` readers of the shared module (20 until 2026-10-20-select-by-id added `stand_act_select`), `run {file}` the two non-stand readers — `run font_face_sniffed_source`, which holds a load, and, since 2026-10-20-select-by-id, `run select_interaction` — and `run all` the others»; the closing reading «`run stand` 190 · 0 · 7, as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md»"
    sidecar: "test-plan §3 test-data bootstrap: `run stand` runs 21 `stand_*` fixture readers, two non-stand readers, `run stand` 185 → 190 · 0 · 7"
    rationale: "Report pattern `fixtures`: stale test-plan.md:132 («`run stand` runs the 20 `stand_*` readers» → 21); the same line's «`run font_face_sniffed_source`, the one non-stand reader» is retired by `select_interaction`, «a non-`stand_*` reader»; its closing «`run stand` 185 · 0 · 7» moves with pattern `stand-pass` (185 → 190, ignored 7, failed 0) though the report's sweep lists only :119 for it"
    basis: ".andromeda/test-plan.md:132"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: "«so do the fifteen driver-action checks `stand_act_*`, …» and the reader count «28 readers since 2026-10-20-select-by-id added `stand_act_select`; 27 before `stand_act_select`, since 2026-10-20-load-wait added `stand_load_wait` …»"
    sidecar: "test-plan §3 session checks helpers: `session_common` 27 → 28 readers, driver-action checks fourteen → fifteen (`stand_act_select`)"
    rationale: "Report pattern `session-common`: stale test-plan.md:103 @c2926 («27 readers»); pattern `stand-act`: stale test-plan.md:103 («the fourteen driver-action checks»)"
    basis: ".andromeda/test-plan.md:103"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: "«six of the fifteen driver-action checks» (`stand_act_select`, the fifteenth, declares no `mod common;`)"
    sidecar: "test-plan §3 stand checks helpers: «six of the fourteen» → «six of the fifteen» driver-action checks"
    rationale: "Report pattern `stand-act`: stale test-plan.md:102 «six of the fourteen driver-action checks» → six of the fifteen — `stand_act_select` declares no `mod common;`"
    basis: ".andromeda/test-plan.md:102"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → escher-driver"
    change: "«the executor's verbs and its refusals are covered by the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth since 2026-10-20-select-by-id)»"
    sidecar: "test-plan §1 escher-driver: `stand_act_*` files fourteen → fifteen (`stand_act_select`)"
    rationale: "Report pattern `stand-act`: fourteen → fifteen (`stand_act_select` the fifteenth), stale test-plan.md:22"
    basis: ".andromeda/test-plan.md:22"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "«while the fifteen `stand_act_*` files drive it in process through a `Session` (§5)»"
    sidecar: "test-plan §4 escher-driver: `stand_act_*` files fourteen → fifteen"
    rationale: "Report pattern `stand-act`: stale test-plan.md:146"
    basis: ".andromeda/test-plan.md:146"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session ↔ held instance"
    change: "add before «(the fifteen `stand_act_*` files, §1)», which reads fifteen for fourteen: since 2026-10-20-select-by-id a `type` on a single-choice select chooses the first option whose label starts with the text — accepted and settled, its diff naming the select with `value` in `differs` and the two options whose `selected` moved, the select's options read as its children named by their labels, a disabled select skipped by the Tab walk and refused, in both layout modes (`stand_act_select`; tests/blitz-tests/tests/stand_act_select.rs:79-256)"
    sidecar: "test-plan §5 Session ↔ held instance: `type` on a select and its proof `stand_act_select` stated; `stand_act_*` fourteen → fifteen"
    rationale: "Report pattern `stand-act`: stale test-plan.md:185; Outcome (tests) v020-08 met (`stand_act_select` 5, both modes: `type --id fx-pick --text Cherry` accepted and settled, diff `value` and `selected`, Tab walk skipping `fx-off`); Symbols / APIs, Driver: `type`'s help «on a select, the text chooses the first option whose label starts with it» (schema.rs:208-209)"
    basis: ".andromeda/test-plan.md:185"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle"
    change: "session-proof: «the fifteen driver-action files, `Session::run` in each, both layout modes — … `stand_act_transformed` 2 · `stand_act_select` 5 (55; 50 before `stand_act_select` at 2026-10-20-select-by-id, 48 before `stand_act_transformed`, …)»"
    sidecar: "test-plan contract session-lifecycle (session-proof): driver-action files fourteen → fifteen, `stand_act_select` 5, 50 → 55 tests"
    rationale: "Report pattern `stand-act`: stale registries/contracts/test-plan/session-lifecycle.md:11 («the fourteen driver-action files»); Outcome: `stand_act_select` 5 passed in both layout modes through `Session::run`"
    basis: ".andromeda/registries/contracts/test-plan/session-lifecycle.md:11"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "append: re-counted at 2026-10-20-select-by-id, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 190 · failed 0 · ignored 7 over 47 files — +5 passed (`stand_act_select` 5, a new file picked up by its `stand_` prefix with no script change); `select_interaction`, no `stand_` prefix, is read by name and by `run all`; `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md"
    sidecar: "test-plan §3 Proof: `run stand` re-counted 185 · 0 · 7 over 46 → 190 · 0 · 7 over 47 files (`stand_act_select` 5)"
    rationale: "Report patterns `stand-files` (46 → 47) and `stand-pass` (185 → 190, ignored 7, failed 0; gate entry 15): test-plan.md:119 @c10555 re-derived, its re-count clause gaining this chunk's; Harness / gate surface: `stand_act_select` picked up by its `stand_` prefix, no script change"
    basis: ".andromeda/test-plan.md:119"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Legs"
    change: "the stand-repeat leg's file count reads «47 files, … `stand_load_wait` since 2026-10-20-load-wait and `stand_act_select` since 2026-10-20-select-by-id»"
    sidecar: "test-plan §9 Legs: stand-repeat runs 47 `stand_*.rs` files (`stand_act_select` joins by its prefix)"
    rationale: "Report pattern `stand-files`: 46 → 47, stale test-plan.md:324; Harness / gate surface: `stand_act_select` picked up by `stand-repeat` by its `stand_` prefix, no script change"
    basis: ".andromeda/test-plan.md:324"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets → Flakiness budget"
    change: "«run the 47 `stand_*.rs` files … (§9 Legs; 46 until 2026-10-20-select-by-id added `stand_act_select`, whose CI `Stand checks repeated` on CI#39371468367 read 165 · 660 · 0 · 21, +3 · +15; 45 until 2026-10-20-load-wait …)»"
    sidecar: "test-plan §10 Flakiness budget: 46 → 47 `stand_*.rs` files; the stand repeat on CI#39371468367 165 · 660 · 0 · 21"
    rationale: "Report pattern `stand-files`: stale test-plan.md:353; Outcome: CI#39371468367 green 17/17, `Stand checks repeated` 165 · 660 · 0 · 21 (+3 lines, +15 passed — `stand_act_select` × 3 rounds)"
    basis: ".andromeda/test-plan.md:353"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Per-OS test tallies"
    change: "append: re-read at 2026-10-20-select-by-id on CI#39371468367, the pre-CI commit `14818317c` (green 17/17): linux 204 · 1083 · 0 · 12; macos and coverage 185 · 1080 · 0 · 12; windows 185 · 1038 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 165 · 660 · 0 · 21 — +2 result lines and +21 passed on the four test jobs over CI#39360124515 (`node::select::tests` 4, `snapshot::tests` 2, `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3), +3 · +15 on the stand repeat — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/ci.md"
    sidecar: "test-plan §9 Per-OS tallies: re-read on CI#39371468367 — linux 204 · 1083, macos/coverage 185 · 1080, windows 185 · 1038, stand repeat 165 · 660"
    rationale: "Report Counts, pattern `tally`: test-plan.md:334 re-derived, a new clause; Per-OS CI tallies (CI#39371468367, `evidence/ci.md`): linux 204 · 1083 · 0 · 12; macos and coverage 185 · 1080 · 0 · 12; windows 185 · 1038 · 0 · 9; a11y 6 · 43 · 0 · 0; stand-repeat 165 · 660 · 0 · 21; +21 by name"
    basis: ".andromeda/test-plan.md:334"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline"
    change: "append to the `cargo test --workspace` re-counts: re-counted at 2026-10-20-select-by-id: 204 result lines, 1083 passed · 0 failed · 12 ignored, from 202 · 1062 — +2 result lines (`select_interaction`, `stand_act_select`) and +21 passed (`node::select::tests` 4, `snapshot::tests` 2, `select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-20-select-by-id/evidence/counts.md)"
    sidecar: "test-plan §9 Local baseline: workspace tally 202 · 1062 → 204 · 1083 · 0 · 12"
    rationale: "Report Counts, pattern `tally`: workspace tally 202 · 1062 · 0 · 12 → 204 · 1083 · 0 · 12 (gate entry 18 over `target/ci-logs/test.log`), test-plan.md:340 re-derived, the load-wait clauses staying as history; +21 by name (`evidence/counts.md`)"
    basis: ".andromeda/test-plan.md:340"
note: "D-tests-framework and D-tests-obs-harness hold — the chunk's checks run on the standard harness (`cargo test`, `agent-run.sh` unchanged, `stand-repeat` taking `stand_act_select` by its prefix) and change no harness, status shape or log format; no Retired bullet, so no D-retired proposal. escher-driver's unit count is unmoved (the `type` help pin rides the existing schema_tests.rs test at 77-78, the +21 naming no escher-driver unit), so test-plan.md:22's count stays; the report states no coverage-floor reading for CI#39371468367, so §10 Coverage floors is left. All dependent-of entries point at the D-tests-coverage primary of the same fact (stand-act, session-common, fixtures, stand-files, snapshot::tests, node::select::tests)."
