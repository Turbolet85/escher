# Report — 2026-10-14-stand-a11y-assertions

**Chunk:** Stand a11y assertions — SC 2.1.1 keyboard reach, SC 2.4.3 focus order, SC 1.4.3 contrast asserted on every stand control; an a11y CI leg gating merges
**Date:** 2026-10-14
**Commits:** since `state.yaml` `last_wrap` 2026-10-14T06:16:15Z (the prior wrap's commit `ccfcda48`): `aab4cda97 chore(2026-10-14-stand-a11y-assertions): operator pre-CI commit, for the run this chunk verdict reads` (70 files, `git show --stat aab4cda9`). The basis of every diff below is `ccfcda48`, the parent of the oldest pre-CI commit, to the working tree.

## Changes (structured — detectors read this)
- **Files:** 15 changed, all on the plan's Files to modify, 0 new (`gate.py scope`: `clean — changed 15 · listed 15 · recorded 0`):
  - Stand task CSS, re-coloured: `examples/seven_guis/src/tasks/{counter,flight_booker,timer,crud,circle_drawer,cells,temp_converter}.rs`. `crud.rs` also gains `aria_label: "People"` on `crud-list` (crud.rs:59).
  - Stand checks: `tests/blitz-tests/tests/{common/mod.rs, stand_contrast.rs, stand_keyboard.rs, stand_accessibility_ids.rs}`.
  - Log checks: `examples/seven_guis/tests/{host_log,mcp_log}.rs`.
  - CI scripts: `.github/scripts/ci-leg.sh`, `.github/scripts/test_ci_workflows.py`.
  - Chunk evidence: `evidence/hover-active.md` (implement step 1) and `evidence/ci.md` (the operator pass, entries 12–14).
  - Not source: chunk folder (`scope.md`, `research.md`, `plan.md`, `inputs/` I1–I6) and run dirs `.andromeda/runs/2026-10-14T06-38-55-phase/`, `…T07-05-28-implement/` and this wrap's `…T19-50-50-wrap/`.
- **Symbols / APIs:** No public API, crate surface, IPC method, port, socket or env var. The new items are test-only (coordinates from `## New text, by line`):
  - `tests/blitz-tests/tests/common/mod.rs`:
    - `pub const ENABLED_TAB_ORDER: [(LeanTask, &[&str]); 2]` (72-99, head @74). The flight booker after `flight-return`: `back-btn`, `flight-one-way`, `flight-return`, `flight-start`, `flight-return-date`, `flight-book`. CRUD after a row is selected: `back-btn`, `crud-filter`, `crud-list`, `crud-name`, `crud-surname`, `crud-create`, `crud-update`, `crud-delete`. Measured equal to the predicted order (DOM order plus the controls disabled at boot).
    - `pub const REGION_NAMES: [(&str, &str); 1] = [("crud-list", "People")]` (133-136).
    - `controls`, `tab_order` and `INPUT_NAMES` are unchanged.
  - `tests/blitz-tests/tests/stand_contrast.rs`:
    - Shared reader `fn read(harness, selector, at) -> Reading` (168-187). It holds the classification `measure` used to hold; `measure` keeps its "the snapshot lists the control" assertion. The exempt class is still exactly `enabled == Some(false)`, read from the snapshot by id. A `label[for=…]` row has no id and is never exempt.
    - `const TEXT_ROWS: [(LeanTask, &str, Reading); 14]` (206-279).
    - `enum Reach { ReturnChosen, StartUnparseable, Booked, RowSelected, Hover, Active }` (281-297).
    - `const STATE_ROWS: [(LeanTask, Reach, &str, Reading); 19]` (299-417) and `fn reach` (419-471).
    - `fn tally` (566-575).
    - Three tests: `every_measured_control_meets_its_required_ratio` (577-605), `every_list_row_and_text_outside_the_controls_meets_its_ratio` (607-635) and `every_colour_state_meets_its_ratio` (637-661).
    - The five former `Fail` rows of the control record are restated `Pass(pair(0xffffff, 0x3a5ce5, 5.47, …))`, size and weight unchanged (lines 59 · 69 · 89 · 100 · 125).
  - `tests/blitz-tests/tests/stand_keyboard.rs`:
    - Helpers `fn enable_by_keyboard` (209-235), `const ENABLED_BUTTONS: [(LeanTask, &str); 2]` (270-275) and `fn crud_ready_at` (277-289).
    - Three tests: `the_enabled_states_extend_the_walk_in_order` (237-268), `enter_and_space_on_the_controls_enabled_later_read_as_their_click` (291-322) and `every_control_is_reached_by_tab` (324-342).
  - `tests/blitz-tests/tests/stand_accessibility_ids.rs`:
    - The `//!` doc is extended (5-12) and `use blitz_dom::Document;` imported (20).
    - Helpers `const INTERACTIVE_ROLES: [Role; 9]` (26-37: Button, TextInput, Slider, ListBox, ListBoxOption, CheckBox, RadioButton, ComboBox, Link), `fn shell_tree` (39-43: `let document: &dyn Document = &harness.doc; document.accessibility_tree()`, the call `AccessibilityState::update_tree` makes), `fn mode` (45-47) and `fn tree_focus` (354-361).
    - Three tests: `every_interactive_role_element_reads_a_name` (255-308), `the_tree_the_shell_hands_the_adapter_carries_stable_ids_only` (310-352) and `a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds` (363-410).
  - `examples/seven_guis/tests/host_log.rs:57-64` and `mcp_log.rs:71-78`: `let list_name = harness.attr("#crud-list", "aria-label")`, asserted `is_some()`, chained into `names`.
- **Crates / modules:** none added, removed or changed in shape. blitz-tests, seven_guis and the CI scripts are changed in their test and script bodies only. `git diff --quiet ccfcda48 -- packages Cargo.toml Cargo.lock deny.toml scripts examples/seven_guis/Cargo.toml examples/seven_guis/src/{app,stand,session_host}.rs tests/blitz-tests/Cargo.toml tests/blitz-tests/tests/session_common .github/workflows` exits 0 (implement gate 7).
- **Dependencies:** none added or bumped (`Cargo.toml`, `Cargo.lock` unchanged, gate 7).
- **Schema / config:**
  - The stand's accent family moves one step darker in every task CSS that declares it. The replacements ran in the order `#2a4cd3`→`#1f3fc1`, then `#3a5ce5`→`#2a4cd3`, then `#4a6cf7`→`#3a5ce5`.
  - The new accent is rest `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, against the old `#4a6cf7` / `#3a5ce5` / `#2a4cd3`. Ratios of white on each: 5.47 · 6.83 · 8.27 (the operator's figures, inputs#I4; 5.47 and 6.83 measured in `stand_contrast`, 8.27 measured on the active rows).
  - The 3 `rgba(74, 108, 247, …)` sites became `rgba(58, 92, 229, …)` with alpha kept: temp_converter.rs:77-78, the focus glow at 0.15; circle_drawer.rs:185-186 at 0.15 and :192 at 0.35, the fills.
  - The diff touches colour lines only: 26 insertions and 26 deletions (implement outcome). Basis: `grep -r -i -o -E '#4a6cf7|74, ?108, ?247' examples/seven_guis/src | wc -l` reads 0, against 17 on the untouched tree (gate 5).
  - Non-text uses that move with the token: the flight booker's `.date-input:focus` border, the timer's `.progress-fill` and `accent-color`, cells' selected outline (cells.rs:470), and circle drawer's toolbar and dialog buttons and the dialog border.
  - No rule added or removed. No selector, radius, shadow, size or weight changed.
  - The `a11y` CI leg list (`.github/scripts/ci-leg.sh:34`) is now `cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids`.
  - `test_ci_workflows.py:289-290`, `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`: the a11y tuple gains `"stand_keyboard", "stand_contrast", "stand_accessibility_ids"` in that order.
  - No `ci.yml` job, step, cache rule, permission or upload changed; `RESTORE_ONLY_JOBS` and the job-name pin stand.
  - No scrub, redaction or violation schema changed.
- **Spec-master edits:** none. No `.andromeda/` master or registry file changed in this chunk's diff; the sweep's pre-pass baseline is HEAD `aab4cda9`, every master row reading `standing`.
- **Counts / qualifiers moved:** Pattern counts are the site sweep's (`cascade.py sweep --pre-pass --patterns-file {run_dir}/report-patterns.toml`, baseline `aab4cda9`); `standing` = master and registry rows.
  - **`stand_contrast`'s control record** moves from pass 9 · fail 5 · exempt-inactive 3 · no-text 1 to pass 14 · fail 0 · exempt-inactive 3 · no-text 1, over the same 18 rows.
    - The five former fails (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`) read `#ffffff` on `#3a5ce5` at 5.47.
    - `record-tally`: standing 3 — architecture.md:263, test-plan.md:26, a11y-plan.md:229 (each ×3), every one stating the old tally, so all 3 change. Leaf 3.
  - **The old accent and its 4.39 fail ratio.** `old-accent` (`#4a6cf7|74, ?108, ?247|4\.39`): standing 3.
    - Changing: design-system.md:24 (the accent row, ×2) and a11y-plan.md:229 (×2, the §6 fail statement).
    - Kept: test-plan.md:142's "4.48 and 4.39 within 0.005" is the harness unit table's tolerance figure. That fixture keeps `#4a6cf7` by design (plan, gate 1 note).
    - Leaf 5: CLAUDE.md:45, .claude/rules/a11y.md:38, .claude/docs/{a11y-summary.md:15, design-summary.md:13, services/seven_guis.md:24}.
  - **The hover and active shades.** `accent-shades`: standing 1 (design-system.md:24 ×2, "hover `#3a5ce5`, active `#2a4cd3`"). Now hover `#2a4cd3` and active `#1f3fc1`; changes. Leaf 1 (design-summary.md:13).
  - **"SC 1.4.3 is not met on the five accent buttons" is false.** `not-met`: standing 2, design-system.md:24 and a11y-plan.md:229 (×2); both change. Leaf 4: CLAUDE.md:45, rules/a11y.md:38, a11y-summary.md:15, design-summary.md:13.
  - **The `a11y` leg runs six files where it ran three:**
    - Its log now reads accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 5 · stand_keyboard 14 (`target/ci-logs/a11y.log`, implement gate 4).
    - `a11y-leg`: standing 24. The claim sites are:
      - architecture.md:141 (the CI contracts' leg list, ×2)
      - test-plan.md:26 (×5, among them "not by the `a11y` leg, which stays three files" twice)
      - test-plan.md:315 (the leg list, ×2)
      - a11y-plan.md:68 ("the `a11y` leg staying its three files")
      - a11y-plan.md:77 ("not in the `a11y` leg")
      - a11y-plan.md:229 ("not in the `a11y` leg")
      - a11y-plan.md:340 (§9, ×4)
    - The other 17 rows are path citations of `accessibility_hidden.rs` / `focusability_updates.rs`, the job's `save-if: false` restore and the bootstrap registry line, and stay unchanged: architecture.md:91, 115, 155, 263; security-plan.md:119; test-plan.md:145, 317, 330; a11y-plan.md:21, 65, 79, 109, 146, 147, 148, 211; `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:4`. test-plan.md:330 is a history count.
    - `file-counts`: standing 4 — test-plan.md:26 (×4: "stays three files" ×2, `stand_contrast` 2, `stand_keyboard` 11), test-plan.md:116 (×2, history deltas, no change), test-plan.md:330 (×2, history deltas, no change), a11y-plan.md:68. Leaf 1 (tests-summary.md:30).
  - **Per-file test counts**, from the a11y log and `run stand`: `stand_contrast` 2 → 5 (+3), `stand_keyboard` 11 → 14 (+3), `stand_accessibility_ids` 5 → 8 (+3).
  - **`run stand`'s `run.end`** moves from passed 153 · failed 0 · ignored 7 over 38 files to passed 162 · failed 0 · ignored 7 over 38 files (+9, no new file; implement gate 9, `agent-run.sh status`).
    - `run-stand` (`\b153\b`): standing 25. Of these, 23 are line-number digits in citations (`:149-153`, `:153` …), sweep noise left unchanged.
    - The claim site is test-plan.md:116 ("`run.end` reads passed 153 · failed 0 · ignored 7 over 38 files"), which changes.
    - test-plan.md:330's "153 result lines, 647 passed" is a refusal-detection history reading, unchanged.
    - Leaf 1 (tests-summary.md:30 ×2).
  - **Workspace totals** move from 182 · 928 · 0 · 12 to 182 result lines, 937 passed · 0 failed · 12 ignored (+9, blitz-tests only).
    - Basis: `target/ci-logs/test.log` (implement gate 10). Re-read at this wrap from entry 13's log, `$TMPDIR/andromeda-gate/2026-10-14-stand-a11y-assertions/run-20261014T071904Z/13.log`: 182 `test result:` lines, 937 · 0 · 12.
    - `ws-totals`: standing 1, test-plan.md:330 ("182 result lines, 928 passed · 0 failed · 12 ignored"), which changes. Leaf 1 (tests-summary.md:30).
  - **blitz-tests file and target counts** are unchanged: 121 files, 120 targets, no file added.
  - **`crud-list`'s accessible name** moves from `""` to `People`.
    - `list-name`: standing 3. Changing: a11y-plan.md:280 and :283 ("the accessible name `""`"). No change: obs-plan.md:267 (an `ErrorEvent`'s `filename ""`, a different claim sharing the token).
    - Leaf 2: a11y-summary.md:22 and services/seven_guis.md:30, "the list reads no accessible name".
    - `crud-list`: standing 10, rows architecture.md:133, 136, 263; layout-templates.md:10 (×4), :45; test-plan.md:26 (×3), :100; a11y-plan.md:196, 280, 326. Of these, layout-templates.md:10 (the Primary-screens CRUD region) and a11y-plan.md:280 take the name; the rest are Tab-order and scrolling facts that stay. Leaf 8.
  - **Interactive-role naming** is now asserted over every interactive-role tree node of the four lean tasks (9 roles), not only the 15 named controls plus `INPUT_NAMES`. `common-tables`: standing 3 (architecture.md:263, test-plan.md:100, a11y-plan.md:280), each naming `INPUT_NAMES` or `controls`; `REGION_NAMES` and `ENABLED_TAB_ORDER` join them. 0 rows for `TEXT_ROWS` / `STATE_ROWS` (new names).
  - **SC 2.1.1 / 2.4.3 in the enabled states.**
    - The Tab walk was asserted at boot only. It now also covers the flight booker after `flight-return` (six stops) and CRUD after a row is selected (eight stops), each reached by keyboard alone.
    - Every `controls(task)` id lies in the union of `tab_order` and `ENABLED_TAB_ORDER`, asserted as an empty set difference.
    - Enter and Space on `crud-update` and `crud-delete`, enabled, each read as its click: 2 buttons × 2 keys = 4 rows.
    - `stand-checks` (`stand_contrast|stand_keyboard|stand_accessibility_ids`): standing 26, leaf 21. These are the masters' statements of what each check covers. Rows are in architecture.md (86, 91, 136, 263 ×15), security-plan.md:119, design-system.md:24, test-plan.md (26 ×18, 66, 91, 100, 116, 330), obs-plan.md:70, a11y-plan.md (75, 77, 144, 148, 158, 166, 186, 196, 213 ×9, 229 ×4, 280, 326) and `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3`. Each that states a check's coverage or count is the detectors' to read against the new tests above. Path-citation rows (`stand_keyboard.rs:N-M`) belong to P2's citation sweep.
  - **The tree the shell hands the adapter.**
    - `<DioxusDocument as Document>::accessibility_tree` through `&dyn blitz_dom::Document` is asserted equal to `harness.doc.accessibility_tree()`: the same focus and root, by id, node for node.
    - Every `author_id` on it is `DioxusDocument::element_id` of its node, and none is empty, all digits or holds `0x`. Each task's handed tree carries at least `controls(task).len()` ids.
    - `adapter-tree`: standing 7 — architecture.md:122 (×4, the `update_tree(&dyn Document)` fact), architecture.md:136 (×3), security-plan.md:123 (×3, the `id` row), test-plan.md:26 (×5), a11y-plan.md:14, :280, :282. security-plan.md:123 and a11y-plan.md:280 take the `&dyn Document` proof; the rest restate the override and stay. Leaf 10.
  - **The refresh's two halves are proven headlessly** on the stand. After a drained changed set, a Tab leaves `has_changes()` true and the `&dyn Document` tree's focus reads the new stop. Counter's whole `tab_order`, two Tabs, is walked, draining before each. Enter on the focused `counter-increment` then leaves `has_changes()` true and `counter-value`'s name reads `0` → `1` in that tree.
    - The platform-adapter witness is still owed (F1, inputs#I2).
    - `refresh`: standing 6 — architecture.md:121, :122 (×3), :197; security-plan.md:123 (×4); a11y-plan.md:36 (×4), :46. a11y-plan.md:36/:46 (§2) and security-plan.md:123 take "the two halves proven headlessly on the stand, the windowed witness still owed"; the architecture rows restate the mechanism and stay. Leaf 6.
  - **Platform adapter not built on the stand: unchanged** (F1). `stand-adapter`: standing 27. Of these, 25 are `seven_guis_native` binary-name rows that stay. The claim sites are architecture.md:136 ("no platform adapter") and a11y-plan.md:14 (×2, "builds no platform adapter"); both stand true, and only their owner moves (route: re-homed, P5). Also security-plan.md:246 ("no platform adapter enters the stand graph"), true and unchanged.
  - **`aria-activedescendant` and `inert`: unchanged facts**, with owners moving (F2, F3):
    - `activedesc`: standing 3 — layout-templates.md:10, a11y-plan.md:196, :283.
    - `inert`: standing 10 — architecture.md:86, :136, :263; security-plan.md:144; test-plan.md:24, :26, :183; obs-plan.md:70; a11y-plan.md:186, :213. test-plan.md:183 and obs-plan.md:70 are a different "inert" (a host started inert), no change.
    - `route-entry` ("Stand a11y assertions"): standing 7 — architecture.md:86, test-plan.md:26, a11y-plan.md:36, :186, :196, :229, :283. Each names this entry as the owner of a gap, and its owner moves when the chunk completes: a discharge, or the residuals lines P5 writes. Leaf 10, among them CLAUDE.md:45.
  - **The silent census** now runs over `stand_contrast.rs`, `stand_keyboard.rs`, `stand_accessibility_ids.rs` and `common/mod.rs`. It reads 0 at exit 1 (implement gate 6; the pattern `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`).
    - `obs-silent` (`installs no (subscriber|sink)|no subscriber|install no subscriber`): standing 8. The claim site is obs-plan.md:70 (×12, the census of checks that install no subscriber). The rest are crate and binary facts, unchanged.
    - `silent-census`: the wording "silent census" never fires in the masters (control unfired, refusal: `control: silent-census never fired over the seven masters' pre-pass text (aab4cda9)`). The pattern was re-fired as `\bcensus` (standing 5: architecture.md:133, security-plan.md:126, :127, :291, layout-templates.md:45), none of them this census.
  - **Hover and active.** `hover-active`: standing 6. The claim site is a11y-plan.md:229 ("colour-changing states not measured (hover and active, …)"), which changes. The other 5 are `:hover` restyle-path facts, unchanged: architecture.md:11, :91, design-system.md:301, test-plan.md:54, a11y-plan.md:241.
  - **`LegScriptTest`'s a11y row.** `legscript`: standing 1 (test-plan.md:145).
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:**
  - The `a11y` CI leg runs six files (`ci-leg.sh:34`), and its pin `LegScriptTest` (`test_ci_workflows.py:289-290`) is restated.
  - The `a11y` CI job definition is unchanged; it runs the leg script, so the hosted job now runs six files.
  - No agent-run verb, status shape or verdict shape changed; `run stand` picks the three files' new tests up by the `stand_` prefix with no script change.
- **Cross-project / external claims:**
  - **CI run.** CI#38527662148, on the pushed sha `aab4cda979ba79aadbe2c9f16e1b848321c2620a` (`origin/build/escher-0.1.0` equal to local HEAD): `verdict: green · checks 16/16 · wall 796 s · runs CI#38527662148 completed/success`.
    - The 16 checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy · Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features].
    - Basis: `ci.py conclusion --sha HEAD --wait 1800`, exit 0, polled once over 2 s (`evidence/ci.md`).
    - The verdict was taken on the tree of `aab4cda9`; this wrap's commit adds to it.
  - **`inputs.py verify`** (wrap P1): `inputs: 6 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 5 · uncited 2 · unparsed 0`. I5 and I6 were printed `UNCITED` before this report; both are cited here.
    - `I1 · message: the operator, as the arguments of /andromeda-phase, 2026-10-14 06:38Z · copy message · n/a` — cited scope.md, research.md, plan.md (inputs#I1).
    - `I2 · message: the overseer, relaying the founder by question dialog, 2026-10-14 06:55Z · copy message · n/a` — F1–F4, the founder's labels «Пока нет, перенести (Рекомендую)», «Оставить записанным (Рекомендую)», «Оставить, перенести (Рекомендую)», «Расширить задание a11y (Рекомендую)» (inputs#I2).
    - `I3 · ../additional/escher-overseer/relays/a11y-assertions-founder-answers.md · copy no-repo · unchanged` — the overseer's record of F1–F4 (inputs#I3).
    - `I4 · message: the operator, by question dialog, 2026-10-14 06:57Z · copy message · n/a` — F5 "One step darker, stand-wide"; ratios by hand: 5.47 · 6.83 · 8.27; black on `#4a6cf7` 4.78 (inputs#I4).
    - `I5 · message: the operator, at the P5 review, 2026-10-14 07:04Z · copy message · n/a` — the operator's word on the P5 card (inputs#I5), carried below.
    - `I6 · message: the operator, two messages in this session, 2026-10-14 · copy message · n/a` — snapped at this wrap (`--step wrap`; run-dir copy `{run_dir}/relay-1.md`). Its facts (inputs#I6):
      - entries 12 and 13 are done: "hygiene clean, fast green, aab4cda9 pushed";
      - entry 12's output was lost to the pause-clear and rests on the pre-pause context;
      - the wrap's own P7 hygiene read is the check on it, so no re-run is needed;
      - this wrap runs P1 only and stops.
  - **The P5 review card**, inputs#I5: `.andromeda/runs/2026-10-14T06-38-55-phase/p5-card-1.md`, the one card any entry names, so it governs. The operator's word (inputs#I5, his own authority on the card's placements): "yes — apply. … Beyond the plan: 1–3 go to residuals, target next, as the owners the founder's re-home answers left to this card. Each residuals line names his label. Row 4 is approved conditionally, as written." Its `## Beyond the plan` rows, verbatim:
    1. **The stand's platform adapter and its windowed AT witness** — CARRYs 2, 3 (its platform half), 4 and 6, and the
       platform-tree half of CARRY 1. These are re-homed by the founder's F1 (inputs#I2), which named no destination.
       Proposed owner: a `.andromeda/residuals.md` line, `(target: next)`. It covers the adapter on the stand binary, a
       witness at an active AT of ids, names, the three states and the refresh on change, and the by-level stderr re-read.
       The headless halves are proven here.
    2. **The CRUD list's current option in the tree** (CARRY 11; F2 «Оставить записанным (Рекомендую)»). Proposed owner:
       a `.andromeda/residuals.md` line, `(target: next)`. It records an `aria-activedescendant` read in the builder, or a
       roving focus, as SC 4.1.2 work outside the tier-0 target, with the gap staying stated in a11y-plan §5 and
       layout-templates.
    3. **The engine honours no `inert`** (CARRY 12; F3 «Оставить, перенести (Рекомендую)»). Proposed owner: a
       `.andromeda/residuals.md` line, `(target: next)`. `stand_keyboard::the_engine_honours_no_inert` stays the pin, and
       whether an inert subtree takes a hit stays unmeasured.
    4. **Conditional on step 1:** if `Harness::text_contrast` does not read the `:hover` / `:active` background, the
       hover and active rows (CARRY 8's first clause) stay unasserted. Proposed owner: a `CARRY:` on "Stand requirement
       sweep" (working-route.md:96), naming the reader's limit and `evidence/hover-active.md`.
  - Row 4's condition did not fire: step 1 read the declared `:hover` and `:active` backgrounds (`evidence/hover-active.md`), so no CARRY is owed by it. Rows 1–3 are P5's to land as `.andromeda/residuals.md` lines, `(target: next)`, each naming the founder's label.
- **Reverted / negative API facts:** none. Two scratch probes (`scratch_hover_active_probe.rs` at step 1, `scratch_rows_probe.rs` measuring `TEXT_ROWS` / `STATE_ROWS`) were written and deleted before the gates by design. Neither is in the tree (`gate.py scope` clean).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - research.md §Measurements' declared-hex figure for the CRUD labels, `#333` at 11.59, does not hold for the filter label. `label[for=crud-filter]` reads `#000000` on `#f5f5f5` at 19.26 (`stand_contrast.rs` `TEXT_ROWS`, measured by `Harness::text_contrast`). That label carries no class; `label[for=crud-name]` and `label[for=crud-surname]` do read `#333333` on `#f5f5f5` at 11.59. A chunk-artifact claim, no master: the sweep holds no master row for the label's colour. The plan's premise holds: every `TEXT_ROWS` row passes, the lowest 9.74 (`label[for=timer-duration]`).
  - CARRY 9's hypothesis on route entry "Stand a11y assertions" (working-route.md:94): "a selected row reads `#ffffff` on `#4a6cf7` = 4.39, a fail (predicted at that chunk's phase, not measured)". It is moot after the re-colour. The selected row is measured `#ffffff` on `#3a5ce5` at 5.47, `Pass` (`STATE_ROWS`, `RowSelected`). Route text, not a master; P5's.
  - Masters stating the pre-chunk record, now false, are listed under Counts / qualifiers moved and in the Expected amendments below: a11y-plan.md:229, design-system.md:24, the leg's three files, `crud-list`'s name `""`, the counts. Each is a planned amendment, not a claim this chunk disproved by surprise.
- **Expected amendments (from plan):** one line per entry of the plan's list (plan.md:372-390). Hits are the site sweep's `standing` rows, registries included.
  - **a11y-plan §6.** SC 1.4.3 asserted on the stand: the record at 14 · 0 · 3 · 1, the list rows, the text outside the controls and the reached colour states, hover and active asserted (step 1 read yes); "not met on the five accent buttons" retired.
    - Carried: Counts → `stand_contrast`'s control record, hover and active, Symbols → `stand_contrast.rs`.
    - Sites: `record-tally` a11y-plan.md:229 ×3; `not-met` a11y-plan.md:229 ×2; `old-accent` a11y-plan.md:229 ×2; `hover-active` a11y-plan.md:229; `a11y-leg` a11y-plan.md:229 ("not in the `a11y` leg"); a11y-plan.md:77 (`a11y-leg`, `stand-checks` ×2, "states the 18 control rows"). Also hits in other masters: architecture.md:263 and test-plan.md:26 (`record-tally`).
  - **a11y-plan §5.** The enabled-state walks and `ENABLED_TAB_ORDER`; the tree's current-option gap and `inert` re-homed (F2, F3).
    - Carried: Counts → SC 2.1.1 / 2.4.3 in the enabled states, and `aria-activedescendant` and `inert`.
    - Sites: `stand-checks` a11y-plan.md:213 ×9, :166, :158; `activedesc` a11y-plan.md:196; `inert` a11y-plan.md:186, :213; `route-entry` a11y-plan.md:186, :196 (the owner named is this entry).
  - **a11y-plan §7.** `crud-list` named, the interactive-role sweep. Carried: Counts → `crud-list`'s accessible name, interactive-role naming. Sites: `list-name` a11y-plan.md:280, :283; `common-tables` a11y-plan.md:280; `route-entry` a11y-plan.md:283.
  - **a11y-plan §9.** The `a11y` leg at six files, its counts. Carried: Counts → the `a11y` leg, Harness / gate surface. Sites: `a11y-leg` a11y-plan.md:340 ×4, :68; `file-counts` a11y-plan.md:68.
  - **a11y-plan §1 and §2.** The adapter decision re-homed (F1); the refresh's two halves proven headlessly, the platform witness owed. Carried: Counts → the refresh's two halves, platform adapter not built on the stand. Sites: `stand-adapter` a11y-plan.md:14 ×2; `refresh` a11y-plan.md:36 ×4, :46; `route-entry` a11y-plan.md:36; `adapter-tree` a11y-plan.md:14.
  - **design-system §Color Palette (Core Colors).** The accent row's three new values and ratios, the 4.39 fail retired; the citations into the seven task files re-pointed.
    - Carried: Schema / config (the accent family), Counts → the old accent, hover and active shades, "not met".
    - Sites: design-system.md:24 (`old-accent` ×2, `accent-shades` ×2, `not-met`, `stand-checks` — its citation `stand_contrast.rs:41-129`).
    - The task-file citations are P2's citation sweep: every changed line of the seven task files keeps its line count, so numbers move only in `stand_contrast.rs`, which gained lines.
  - **layout-templates §Surface: desktop-native → Primary screens.** `crud-list` named by `aria-label`. Carried: Changes → Files (crud.rs:59), Counts → `crud-list`'s accessible name. Sites: `crud-list` layout-templates.md:10 ×4; `activedesc` layout-templates.md:10 (the gap stays stated).
  - **test-plan §1, §3, §9.** The three files' counts, `run stand` and the workspace totals, the `a11y` leg list and its local reading.
    - Carried: Counts → per-file test counts, `run stand`'s `run.end`, workspace totals, the `a11y` leg.
    - Sites: `file-counts` test-plan.md:26 ×4; `run-stand` test-plan.md:116; `ws-totals` test-plan.md:330; `a11y-leg` test-plan.md:26 ×5, :315 ×2; `stand-checks` test-plan.md:26 ×18; `record-tally` test-plan.md:26 ×3.
  - **test-plan §4.** `LegScriptTest`'s a11y row. Carried: Schema / config, Counts → `LegScriptTest`'s a11y row. Sites: `legscript` test-plan.md:145.
  - **architecture §Standard Contracts → CI contracts.** The leg's list. Carried: Schema / config, Harness / gate surface. Sites: `a11y-leg` architecture.md:141 ×2. Also architecture.md:263 (`record-tally` ×3, `stand-checks` ×15, `common-tables`, Existing Scopes), outside this entry's named section; hits, so a detector reads it.
  - **security-plan §Input Validation `id` row.** The tree the shell hands the adapter, proven through `&dyn Document`; the adapter still not built on the stand. Carried: Counts → the tree the shell hands the adapter. Sites: `adapter-tree` security-plan.md:123 ×3; `refresh` security-plan.md:123 ×4.
  - **obs-plan §3.** The silent census restated over the extended checks. Carried: Counts → the silent census. Sites: `obs-silent` obs-plan.md:70 ×12; `stand-checks` obs-plan.md:70 ×6.
- **Coverage of new surfaces:**
  - `crud-list` `aria-label="People"` (a name on an existing region; no new element) → validation n/a · instrumentation n/a · PII redacted✓ (the label's text is among `host_log` / `mcp_log`'s name needles, 0 at `trace` under both builds, gate 3) · tests integ (`every_interactive_role_element_reads_a_name`, both modes) · a11y WCAG✓ (SC 4.1.2 name; region reached by Tab, `ENABLED_TAB_ORDER`) · tokens n/a (no colour).
  - The re-coloured accent (no new element; existing hex literals in each task's `const CSS`, edited in place) → validation n/a · instrumentation n/a · PII n/a · tests integ (`stand_contrast`'s three new tests and the census, gate 5) · a11y WCAG✓ (SC 1.4.3, every non-exempt row `Pass`) · tokens: the hex literals stay where they were declared. The plan rejected a project token or custom property because design-system §Color Palette marks the token set NOT YET MEASURED.
  - The widened `a11y` CI leg (a CI step's argument list, no new job) → validation n/a · instrumentation n/a · PII n/a · tests unit (`LegScriptTest`) · a11y n/a · tokens n/a.
  - No new external surface, hot-path operation, listener, port, env var or crate.

## Deviations from intent
- Implement's 7 deviations (`.andromeda/runs/2026-10-14T07-05-28-implement/outcome-2026-10-14-stand-a11y-assertions.md`), each with its justification:
  1. **Step 6, `enter_and_space_on_the_controls_enabled_later_read_as_their_click`.** The plan said "the way `BUTTONS` rows are" and stated no change flag. Update with unchanged fields changes nothing, so a key that did nothing would pass. The rows are therefore reached keyboard-only: Tab to the list, ArrowDown, Tab to `crud-name`, type `x`, Tab to the button. Each asserts that its click changes the screen beyond focus and that each key reads as that click: 2 buttons × 2 keys = 4.
  2. **`TEXT_ROWS`, `label[for=crud-filter]`,** reads `#000000` on `#f5f5f5` at 19.26, not research's `#333` at 11.59 (Spec claims disproved, first bullet). The table holds the measurement; every row passes, the lowest 9.74.
  3. **`STATE_ROWS`, active rows.** CRUD's buttons and `#back-btn` declare no `:active` rule, so their active rows read the hover pair: `crud-create` white on `#2a4cd3` at 6.83, `back-btn` `#333` on `#e4e4e4` at 9.94. A table comment says so.
     - `back-btn`'s hover and active rows are read on the Counter task.
     - The plan's `booked-msg` is addressed as `#flight-booked`. The plan's condition (a selector-addressable element) holds.
     - Rows: return-chosen 3, unparseable 2 (`flight-book` exempt at `#ffffff` on `#a0aec0` 2.26), booked 1, row-selected 3, hover 5, active 5 — 19, pass 18 · exempt 1.
  4. **`stand_contrast` reader.** The classification moved from `measure` into a shared `read(harness, selector, at)`, so the three tables share one predicate. The exempt class is unchanged.
  5. **`every_interactive_role_element_reads_a_name`.** The plan stated no node count. The test asserts that each task's interactive nodes number at least its `controls` rows, that every one is named and that each `REGION_NAMES` row is among them, the `ListBox` role and its name in the tree and the snapshot.
  6. **`a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds`.** It walks Counter's whole `tab_order` (two Tabs, draining before each), not "Tab once". It then presses Enter on the focused `counter-increment` rather than clicking it.
  7. **A second scratch probe**, `scratch_rows_probe.rs`, measured the `TEXT_ROWS` / `STATE_ROWS` values step 5 says are measured at implement. It was deleted before the gates, like step 1's.
- **Operator pass.** Entry 14 ran twice. A first firing was in the window before the pause; its background task completed, exit 0, with an empty output file. It was re-fired as written in this session after the clear: exit 0, `verdict: green`.
- **Entry 12's printed output was lost to the pause-clear,** and no log of it stands in the gate tool's log directory. On the operator's ruling (inputs#I6) it rests on the pre-pause context, and the wrap's own P7 hygiene read (gate-contract §Hygiene, P7 step 3c) is the check on it, with no re-run.
- **Entry 13's log** was first believed lost and was found at P1 in the gate tool's log directory. `evidence/ci.md` was corrected to carry its reading.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 15 · listed 15 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 56`, wrap P1).

## Decisions & corrections
- **The founder's forks F1–F4** (inputs#I2; his record inputs#I3), his labels verbatim:
  - F1 «Пока нет, перенести (Рекомендую)» — the stand builds no platform adapter now, re-homed.
  - F2 «Оставить записанным (Рекомендую)» — no `aria-activedescendant` read, re-homed.
  - F3 «Оставить, перенести (Рекомендую)» — `inert` stays pinned, re-homed.
  - F4 «Расширить задание a11y (Рекомендую)» — the `a11y` leg widens; no new job, no required status check, no repository setting.
- **The operator's fork F5** (inputs#I4): the accent one step darker, stand-wide, rest `#3a5ce5` · hover `#2a4cd3` · active `#1f3fc1`. A stand change, no engine change. His hand-checked ratios were 5.47 · 6.83 · 8.27, and black on `#4a6cf7` 4.78, not 4.79.
- **The operator's P5 word** (inputs#I5): apply. Beyond-the-plan rows 1–3 go to `.andromeda/residuals.md`, `(target: next)`, each naming the founder's label. Row 4 is approved conditionally; its condition did not fire (step 1 read yes).
- **The operator's word at take-up** (inputs#I1): pipeline tools bare or read whole; no mutation runs (the founder's word); every founder fork raised as a dialog for the operator to take to the founder.
- **The operator's ruling at this wrap** (inputs#I6): entry 12 rests on the pre-pause context; the P7 hygiene read is its check; no re-run. Also: P1 only, then stop.
- **Measured:** `Harness::text_contrast` reads the declared `:hover` background after `move_mouse_to` and the `:active` background after `mouse_down_at`, the same in both layout modes (`evidence/hover-active.md`). So the reader can assert pointer-state colours, where research had held this an open question.
- **Measured:** a button with no `:active` rule keeps its hover background while pressed, so its active contrast row reads the hover pair: CRUD's buttons and `#back-btn` (`evidence/hover-active.md`).
- **Sweep hazard:** the pattern `\b153\b` for `run stand`'s count hit 25 master rows. 23 of them are line-number digits inside citations (`:149-153`, `:153`). A bare count token in a site pattern collides with citation digits, and needs the count's own wording (`passed 153`) to read its claim sites.
- **Sweep hazard:** the masters never word the silence check "silent census". The pattern's control did not fire (cascade's refusal). They state it as "installs no subscriber" (obs-plan.md:70), and "census" alone finds only other censuses.
- **Guard refusals (this window):**
  - The bash guard's `cd` arm refused a read that `cd`'d into the skill's `references/` dir; re-issued once with an absolute path, and it ran.
  - The guard's heredoc-with-file-target arm refused a `cat >>` append to `{run_dir}/report-patterns.toml`; re-issued once as an Edit, and it ran.
- **Guard refusals (implement, from its outcome):** the `cd` arm refused two Setup reads that `cd`'d into the chunk dir; each was re-issued once with absolute paths and ran.
- **Edit anchor (implement, from its outcome):** an Edit whose anchor named a test `…read_as_their_click` failed string-not-found, because the file read `…read_as_its_click`. It was re-issued on another anchor.
- **The gate tool's operator logs outlive a context clear.** `gate.py run --operator` writes `$TMPDIR/andromeda-gate/{marker}/run-{UTC}/{n}.log`, and this window recovered entry 13's whole output from it (`run-20261014T071904Z/13.log`) after the clear lost the printed form. A bare-fired operator entry (entry 12, `gate.py hygiene`) leaves no such log.
- **Outcome basis:** implement's outcome holds no operator correction or ruling, and no recurrence of a standing learning.

## Outcome
- **Acceptance criteria,** re-asserted against the diff:
  - (design) **Met.** The census reads 0, exit 1 (gate 5). The accent is `#3a5ce5` / `#2a4cd3` / `#1f3fc1` in the seven task files, with only colour lines changed: 26 + 26. No radius, shadow, size, weight, selector or id-keyed rule changed.
  - (a11y) **Met.** `stand_contrast` asserts every measured non-exempt text-bearing row, in both modes:
    - the 18 control rows: 0 `Fail`, the five former accent rows at 5.47;
    - `TEXT_ROWS` 14: all pass, the lowest 9.74;
    - `STATE_ROWS` 19: 18 pass and 1 exempt, the lowest pass 5.11 (`flight-start` invalid, `#c53030` on `#fff5f5`).
    - The exempt class is exactly `enabled == Some(false)`.
  - (a11y) **Met**, hover and active asserted: in `STATE_ROWS`, 5 hover and 5 active rows, with `evidence/hover-active.md` holding step 1's reading.
  - (a11y) **Met.** `the_enabled_states_extend_the_walk_in_order` (both modes, keyboard-only reach) and `every_control_is_reached_by_tab` (empty set difference over `LeanTask::ALL`).
  - (a11y) **Met.** `enter_and_space_on_the_controls_enabled_later_read_as_their_click`, 4 rows.
  - (a11y) **Met.** `every_interactive_role_element_reads_a_name`; `crud-list` reads `People` in the tree and the snapshot.
  - (security) **Met.** `the_tree_the_shell_hands_the_adapter_carries_stable_ids_only`.
  - (a11y) **Met.** `a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds`.
  - (security) **Met.** `host_log` and `mcp_log` green with the list's name among the needles (gate 3).
  - (tests) **Met.** `ci-leg.sh a11y` exits 0 running six files (gate 4). `ci-leg.sh fast` is green (gate 10; entry 13's log, CI scripts `Ran 79 tests` OK). The pushed sha's CI reads `verdict: green`, 16/16, with "Accessibility (a11y) tests" among the checks (CI#38527662148).
  - (tests) **Met.** `run stand` ends `run.end` passed 162 · failed 0 · ignored 7, outcome `passed`: 153 + 9, exactly the tests added (3 + 3 + 3).
  - (obs) **Met.** The silence census reads 0 (gate 6).
  - (arch) **Met.** `git diff --quiet ccfcda48 -- packages …` exits 0 (gate 7): no engine, adapter, manifest or `ci.yml` change, and no env var, listener, port or crate.
- **Gates,** by `run`, from implement's block (`gate.py run`, `entries 14 · green 11 · red 0 · recorded 0 · timeout 0 · not-run 3`, 1 iteration):
  - `cargo test -p blitz-test-harness --locked` — green · exit 0.
  - `cargo test -p blitz-tests --locked --test stand_contrast --test stand_keyboard --test stand_accessibility_ids` — green · exit 0.
  - `cargo test -p seven_guis --locked --test host_log --test mcp_log` — green · exit 0.
  - `bash .github/scripts/ci-leg.sh a11y` — green · exit 0, six files: accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 5 · stand_keyboard 14.
  - `grep -r -i -o -E '#4a6cf7|74, ?108, ?247' examples/seven_guis/src | wc -l` — green · exit 1 · last line 0.
  - `cat … | grep -c -E 'println!|…|bind\('` (the silence census) — green · exit 1 · last line 0.
  - `git diff --quiet ccfcda487b8123fc722e0a1fe8e1eda9d62c660d -- packages …` — green · exit 0.
  - `bash scripts/agent-run.sh boot` — green · exit 0.
  - `bash scripts/agent-run.sh run stand` — green · exit 0, `run.end` 162 · 0 · 7, outcome `passed`.
  - `bash .github/scripts/ci-leg.sh fast` — green · exit 0; workspace 182 · 937 · 0 · 12.
  - `bash .github/scripts/ci-leg.sh doc` — green · exit 0.
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg `operator`) — fired by the operator before the pre-CI commit. Its printed output was lost to the pause-clear; it rests on the operator's word, "hygiene clean" (inputs#I6), recorded in `evidence/ci.md`. The operator rules this wrap's P7 hygiene read its check, with no re-run.
  - `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` (leg `operator`) — green on its log, `run-20261014T071904Z/13.log`:
    - 182 result lines, all `ok`, 937 · 0 · 12;
    - CI scripts `Ran 79 tests` · `OK`;
    - the push `ccfcda487..aab4cda97  build/escher-0.1.0 -> build/escher-0.1.0`;
    - `origin/build/escher-0.1.0` = HEAD `aab4cda9`. Recorded in `evidence/ci.md`.
  - `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (leg `operator`) — exit 0, `contains verdict: green` held: `aab4cda979ba verdict: green · checks 16/16 · wall 796 s · runs CI#38527662148 completed/success`. Recorded in `evidence/ci.md`.
  - No smoke: no boot path changed (the plan states no live leg and no boot-path smoke).
- **Watches:** none — no `watch:` folded.
- **Outcome basis:**
  - The operator pass ran. Setup 4's commit list from `ccfcda48`: `aab4cda97 chore(2026-10-14-stand-a11y-assertions): operator pre-CI commit, for the run this chunk verdict reads`, the one pre-CI commit and no fix push. The final HEAD's CI run is CI#38527662148, green, recorded in `evidence/ci.md`.
  - Implement's one run, `.andromeda/runs/2026-10-14T07-05-28-implement/outcome-2026-10-14-stand-a11y-assertions.md`, is read as its P4 report; this window does not hold that run. No plan revision or later run supersedes it.
  - No operator directive between implement and this report, besides the operator pass (inputs#I6).
  - Post-implement artifacts: `evidence/ci.md`.
- **Process hygiene:**
  - Implement's census reads every process its runs started as terminated: the `cargo test` runs, the `gate.py run` block with its cargo, agent-run, `escher-session` hosts and `mcp` children. agent-run starts no daemon.
  - Re-measured at this wrap's P1 (`ps -eo pid,comm,args`, 2026-10-14T19:53Z): no `escher-session`, `seven_guis`, `cargo`, `rustc`, `gate.py` or `code-graph` process.
  - This wrap fired `python scripts/code-graph.py refresh` in the background at Setup. `.andromeda/cache/.refresh-done` is present, and it is not running at the census.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: ccfcda48 (the parent of the oldest pre-CI commit aab4cda9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/ci-leg.sh — added 1 line(s) in 1 range(s)
added: 34
### .github/scripts/test_ci_workflows.py — added 2 line(s) in 1 range(s)
added: 289-290
### examples/seven_guis/src/tasks/cells.rs — added 1 line(s) in 1 range(s)
added: 470
### examples/seven_guis/src/tasks/circle_drawer.rs — added 7 line(s) in 6 range(s)
added: 155 · 163 · 185-186 · 192 · 223 · 231
### examples/seven_guis/src/tasks/counter.rs — added 3 line(s) in 3 range(s)
added: 59 · 67 · 71
### examples/seven_guis/src/tasks/crud.rs — added 4 line(s) in 4 range(s)
added: 59 · 232 · 267 · 274
### examples/seven_guis/src/tasks/flight_booker.rs — added 5 line(s) in 5 range(s)
added: 172 · 188 · 210 · 220 · 224
### examples/seven_guis/src/tasks/temp_converter.rs — added 2 line(s) in 1 range(s)
added: 77-78
### examples/seven_guis/src/tasks/timer.rs — added 5 line(s) in 5 range(s)
added: 139 · 166 · 181 · 190 · 194
### examples/seven_guis/tests/host_log.rs — added 8 line(s) in 1 range(s)
added: 57-64
  - 59-64 «let names: Vec<String> = rows»
### examples/seven_guis/tests/mcp_log.rs — added 8 line(s) in 1 range(s)
added: 71-78
  - 73-78 «let names: Vec<String> = rows»
### tests/blitz-tests/tests/common/mod.rs — added 33 line(s) in 2 range(s)
added: 72-100 · 133-136
- 72-99 @74 «pub const ENABLED_TAB_ORDER: [(LeanTask, &[&str]); 2] = [»
  - 75-85 «(»
  - 86-98 «(»
### tests/blitz-tests/tests/stand_accessibility_ids.rs — added 190 line(s) in 4 range(s)
added: 5-12 · 20 · 24-47 · 254-410
- 26-37 @27 «const INTERACTIVE_ROLES: [Role; 9] = [»
- 39-43 @40 «fn shell_tree(harness: &Harness<DioxusDocument>) -> TreeUpdate {»
- 45-47 «fn mode(incremental: bool) -> &'static str {»
- 255-308 @256 «fn every_interactive_role_element_reads_a_name() {»
  - 257-307 «for incremental in [false, true] {»
- 310-352 @311 «fn the_tree_the_shell_hands_the_adapter_carries_stable_ids_only() {»
  - 312-351 «for incremental in [false, true] {»
- 354-361 @355 «fn tree_focus(tree: &TreeUpdate) -> Option<String> {»
  - 356-360 «tree.nodes»
- 363-410 @364 «fn a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds() {»
  - 365-409 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_contrast.rs — added 403 line(s) in 11 range(s)
added: 8-14 · 18 · 59 · 69 · 89 · 100 · 125 · 168-188 · 197-201 · 206-472 · 565-661
- 168-187 @171 «fn read(harness: &Harness<DioxusDocument>, selector: &str, at: &str) -> Reading {»
  - 172-178 «let enabled = selector.strip_prefix('#').and_then(|id| {»
  - 181-186 «match harness.text_contrast(node.unwrap()) {»
- 206-279 @208 «const TEXT_ROWS: [(LeanTask, &str, Reading); 14] = [»
  - 209-213 «(»
  - 214-218 «(»
  - 219-223 «(»
  - 224-228 «(»
  - 229-233 «(»
  - 234-238 «(»
  - 239-243 «(»
  - 244-248 «(»
  - 249-253 «(»
  - 254-258 «(»
  - 259-263 «(»
  - 264-268 «(»
  - 269-273 «(»
  - 274-278 «(»
- 281-297 @283 «enum Reach {»
- 299-417 @300 «const STATE_ROWS: [(LeanTask, Reach, &str, Reading); 19] = [»
  - 301-306 «(»
  - 307-312 «(»
  - 313-318 «(»
  - 319-324 «(»
  - 325-330 «(»
  - 331-336 «(»
  - 337-342 «(»
  - 343-348 «(»
  - 349-354 «(»
  - 355-360 «(»
  - 361-366 «(»
  - 367-372 «(»
  - 373-378 «(»
  - 379-384 «(»
  - 385-390 «(»
  - 391-398 @393 «(»
  - 399-404 «(»
  - 405-410 «(»
  - 411-416 «(»
- 419-471 @421 «fn reach(harness: &mut Harness<DioxusDocument>, reach: Reach, id: &str, at: &str) {»
  - 422-470 «match reach {»
- 566-575 @567 «fn tally(readings: &[Reading]) -> (usize, usize, usize, usize) {»
  - 569-574 «(»
- 577-605 @578 «fn every_measured_control_meets_its_required_ratio() {»
  - 580-604 «for incremental in [false, true] {»
- 607-635 @608 «fn every_list_row_and_text_outside_the_controls_meets_its_ratio() {»
  - 610-634 «for incremental in [false, true] {»
- 637-661 @638 «fn every_colour_state_meets_its_ratio() {»
  - 640-660 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_keyboard.rs — added 140 line(s) in 3 range(s)
added: 7-10 · 35 · 209-343
- 209-235 @213 «fn enable_by_keyboard(harness: &mut Harness<DioxusDocument>, task: LeanTask, mode: &str) {»
  - 214-218 «let (stop, key, enabled): (&str, Key, &[&str]) = match task {»
  - 220-225 «for (index, id) in enabled.iter().enumerate() {»
  - 229-234 «for (index, id) in enabled.iter().enumerate() {»
- 237-268 @238 «fn the_enabled_states_extend_the_walk_in_order() {»
  - 239-267 «for incremental in [false, true] {»
- 270-275 @272 «const ENABLED_BUTTONS: [(LeanTask, &str); 2] = [»
- 277-289 @280 «fn crud_ready_at(harness: &mut Harness<DioxusDocument>, id: &str, mode: &str, index: usize) {»
  - 283-286 «assert!(»
- 291-322 @292 «fn enter_and_space_on_the_controls_enabled_later_read_as_their_click() {»
  - 293-321 «for incremental in [false, true] {»
- 324-342 @325 «fn every_control_is_reached_by_tab() {»
  - 326-341 «for task in LeanTask::ALL {»
