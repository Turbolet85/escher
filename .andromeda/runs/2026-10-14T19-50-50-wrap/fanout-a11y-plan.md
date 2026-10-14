proposals:
  - detector: D-a11y-surface
    severity: warning
    section: §6 Visual Design Verification → Color contrast
    change: >-
      Restate the stand-contrast bullet at the measured truth since 2026-10-14-stand-a11y-assertions. The `Harness::text_contrast`
      reader paragraph stays as written. Then: the stand's accent moved one step darker, stand-wide, in every task CSS that declares
      it: rest `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, white on each 5.47 · 6.83 · 8.27. This was the operator's F5, a stand
      change and no engine change. `stand_contrast`'s control record over the same 18 rows reads pass 14 · fail 0 · exempt-inactive 3 ·
      no-text 1. The five former fails (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`) read
      `#ffffff` on `#3a5ce5` at 5.47. The exempt class is still exactly `enabled == Some(false)` from the snapshot, and a `label[for=…]`
      row is never exempt. Beside it, one shared reader `read` (stand_contrast.rs:168-187) serves two more tables. `TEXT_ROWS` holds
      14 rows, the CRUD list rows and the text outside the controls (stand_contrast.rs:206-279); all pass, the lowest 9.74
      (`label[for=timer-duration]`). `STATE_ROWS` holds 19 rows reached by `Reach` (stand_contrast.rs:281-297; :299-417; :419-471):
      return-chosen 3, start-unparseable 2, booked 1, row-selected 3, hover 5, active 5. They read pass 18 · exempt 1 (`flight-book`
      disabled, `#ffffff` on `#a0aec0`, 2.26), the lowest pass 5.11 (`flight-start` invalid, `#c53030` on `#fff5f5`). `text_contrast`
      reads the declared `:hover` background after `move_mouse_to` and the `:active` one after `mouse_down_at`, in both layout modes
      (evidence/hover-active.md). A button with no `:active` rule keeps its hover background while pressed, so CRUD's buttons' and
      `back-btn`'s active rows read the hover pair. Three tests assert the three tables in both layout modes
      (stand_contrast.rs:577-605; :607-635; :637-661). SC 1.4.3 is met on every measured non-exempt stand row. The check runs in the
      workspace `test` leg, per package and in the `a11y` leg (§9). Retire four claims: the 9 · 5 · 3 · 1 tally, the `#4a6cf7` 4.39
      fail, "SC 1.4.3 is therefore not met on the stand's five accent buttons" and "not in the `a11y` leg". Also retire the clause
      making the route entry "Stand a11y assertions" owner of the re-colouring, the unmeasured colour states, the list rows and the
      text outside the controls: all are now measured.
    sidecar: >-
      §6 Color contrast: stand_contrast restated at the re-coloured accent (#3a5ce5 · #2a4cd3 · #1f3fc1). The control record is now
      14 · 0 · 3 · 1, with TEXT_ROWS 14 and STATE_ROWS 19 (hover and active included). SC 1.4.3 is met on every measured non-exempt
      row and the check runs in the a11y leg. The "not met" claim and the route-entry owner are retired (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts / qualifiers moved: `record-tally`, `old-accent`, `not-met`, `hover-active` and `a11y-leg` all name
      a11y-plan.md:229 as a claim site that changes. Its Schema / config bullet gives the accent family and its ratios. Its Symbols
      bullet lists `read`, `TEXT_ROWS`, `STATE_ROWS`, `Reach` and the three tests. Expected amendments, "a11y-plan §6", lists the sites.
      Decisions record F5 and the two hover/active measurements. Outcome gives (a11y) Met, with the 18 rows at 0 Fail, TEXT_ROWS 14
      with the lowest 9.74, and STATE_ROWS 19 at 18 pass and 1 exempt with the lowest 5.11.
    basis: >-
      .andromeda/a11y-plan.md:229; tests/blitz-tests/tests/stand_contrast.rs:168-187; tests/blitz-tests/tests/stand_contrast.rs:206-279;
      tests/blitz-tests/tests/stand_contrast.rs:299-417; tests/blitz-tests/tests/stand_contrast.rs:577-661;
      escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/evidence/hover-active.md
  - detector: D-a11y-surface
    severity: warning
    section: §6 Visual Design Verification → NOT YET MEASURED note
    change: >-
      Retire the note's closing clause "the CRUD list rows, the text outside the controls and the colour-changing states are not yet".
      These are measured since 2026-10-14-stand-a11y-assertions by `stand_contrast`'s `TEXT_ROWS` and `STATE_ROWS`, hover and active
      included (Color contrast above). Focus-ring, target-size and typography-readability tokens stay NOT YET MEASURED.
    sidecar: >-
      §6 NOT YET MEASURED note: the list rows, text outside the controls and colour states are no longer listed as unmeasured
      (2026-10-14-stand-a11y-assertions).
    rationale: >-
      This restates the §6 claim the primary retires: the colour-changing states, the list rows and the text outside the controls are
      unmeasured. The report's Outcome (a11y) Met covers TEXT_ROWS 14 and STATE_ROWS 19 with 5 hover and 5 active rows. The report
      gives no line for the note.
    basis: >-
      .andromeda/a11y-plan.md §6 (the section's closing NOT YET MEASURED note); tests/blitz-tests/tests/stand_contrast.rs:206-279;
      tests/blitz-tests/tests/stand_contrast.rs:299-417
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: §3 A11y Assertion Harness Contract → Contrast verification harness
    change: >-
      `stand_contrast` states three tables, read by one shared reader `read` and asserted row for row in both layout modes. They are
      the 18 control rows (beside the painted-pixel witness), since 2026-10-14-stand-a11y-assertions 14 `TEXT_ROWS` (list rows and
      text outside the controls) and 19 `STATE_ROWS` (colour states reached, hover and active included). It runs in the workspace
      `test` leg, per package and in the `a11y` leg (§9). Cite tests/blitz-tests/tests/stand_contrast.rs:168-187 and :577-661.
      Retire "not in the `a11y` leg".
    sidecar: >-
      §3 Contrast verification harness: stand_contrast's three tables and its place in the a11y leg (2026-10-14-stand-a11y-assertions).
    rationale: >-
      This duplicates the §6 claims the primary retires: only the 18 control rows, and not in the `a11y` leg. The report lists
      a11y-plan.md:77 under `a11y-leg` and `stand-checks` ×2 ("states the 18 control rows") in Expected amendments, "a11y-plan §6".
      The leg list is in Schema / config (ci-leg.sh:34).
    basis: >-
      .andromeda/a11y-plan.md:77; .github/scripts/ci-leg.sh:34; tests/blitz-tests/tests/stand_contrast.rs:168-187
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: §9 CI Integration
    change: >-
      ci.yml's `a11y` job, "Accessibility (a11y) tests", runs `bash .github/scripts/ci-leg.sh a11y`. Since
      2026-10-14-stand-a11y-assertions that is `cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles
      --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids`. Its log reads
      accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 5 ·
      stand_keyboard 14 (.github/scripts/ci-leg.sh:34). The leg was widened by the founder's F4 «Расширить задание a11y
      (Рекомендую)», with no new job, no required status check and no repository setting. The job definition is unchanged: slow tier,
      restoring the test job's cache and saving none. `LegScriptTest`'s a11y tuple is restated
      (.github/scripts/test_ci_workflows.py:289-290). The pushed sha's CI#38527662148 reads green 16/16 with "Accessibility (a11y)
      tests" among the checks. Retire "it runs existing tests and adds no stand assertion" and the three-file list. Keep the
      `focusability_updates` 3 → 4 history clause.
    sidecar: >-
      §9 CI Integration: the a11y leg runs six files (adding stand_keyboard, stand_contrast and stand_accessibility_ids) on the
      founder's F4, with the LegScriptTest pin restated (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Schema / config and Harness / gate surface bullets give the six-file list at ci-leg.sh:34 and the pin at
      test_ci_workflows.py:289-290. Its Counts give `a11y-leg` a11y-plan.md:340 ×4 and the per-file log reading. Decisions record F4.
      Cross-project claims record the CI run.
    basis: >-
      .andromeda/a11y-plan.md:340; .github/scripts/ci-leg.sh:34; .github/scripts/test_ci_workflows.py:289-290
  - detector: D-a11y-surface
    severity: warning
    section: §3 A11y Assertion Harness Contract → A11y testing tool pick
    change: >-
      The state-check bullet's tail should read "it runs in the workspace test leg and per package, not in the `a11y` leg, which runs
      six files since 2026-10-14-stand-a11y-assertions (§9)". This retires "the `a11y` leg staying its three files".
    sidecar: >-
      §3 tool pick: the a11y leg is six files, not three (2026-10-14-stand-a11y-assertions).
    rationale: >-
      This restates the §9 three-file claim the §9 proposal retires. The report's Counts list `a11y-leg` and `file-counts` at
      a11y-plan.md:68 ("the `a11y` leg staying its three files"). Expected amendments, "a11y-plan §9", list the site.
    basis: >-
      .andromeda/a11y-plan.md:68; .github/scripts/ci-leg.sh:34
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: §5 Keyboard Navigation → Keyboard handling (engine and shell)
    change: >-
      Extend the SC 2.1.1 measurement on the Enter / Space bullet. Since 2026-10-14-stand-a11y-assertions, Enter and Space also read as
      the click of the two stand buttons enabled after boot, `crud-update` and `crud-delete`. Each is reached by keyboard alone (Tab to
      the list, ArrowDown, Tab to `crud-name`, type, Tab to the button), and each click changes the screen beyond focus. That is
      2 buttons × 2 keys = 4 rows, in both layout modes (tests/blitz-tests/tests/stand_keyboard.rs:270-275;
      tests/blitz-tests/tests/stand_keyboard.rs:291-322).
    sidecar: >-
      §5 Keyboard handling: Enter and Space read as the click on crud-update and crud-delete once enabled, 4 rows
      (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts bullet "SC 2.1.1 / 2.4.3 in the enabled states" covers Enter and Space on `crud-update` and `crud-delete`,
      4 rows. Deviation 1 gives the keyboard-only reach. Expected amendments, "a11y-plan §5", give `stand-checks` a11y-plan.md:166.
    basis: >-
      .andromeda/a11y-plan.md:166; tests/blitz-tests/tests/stand_keyboard.rs:291-322
  - detector: D-a11y-surface
    severity: warning
    section: §5 Keyboard Navigation → Test harness pattern
    change: >-
      To the `stand_keyboard` bullet add what holds since 2026-10-14-stand-a11y-assertions. The Tab walk is also asserted in the enabled
      states, each reached by keyboard alone (`enable_by_keyboard`) and walked in order against the shared table `ENABLED_TAB_ORDER`.
      The flight booker after `flight-return` has six stops: `back-btn`, `flight-one-way`, `flight-return`, `flight-start`,
      `flight-return-date`, `flight-book`. CRUD after a row is selected has eight: `back-btn`, `crud-filter`, `crud-list`, `crud-name`,
      `crud-surname`, `crud-create`, `crud-update`, `crud-delete` (tests/blitz-tests/tests/stand_keyboard.rs:209-235; :237-268;
      tests/blitz-tests/tests/common/mod.rs:72-99). Every `controls(task)` id over `LeanTask::ALL` lies in `tab_order` ∪
      `ENABLED_TAB_ORDER`, asserted as an empty set difference (tests/blitz-tests/tests/stand_keyboard.rs:324-342). Enter and Space on
      the controls enabled later give 4 rows (§5 Keyboard handling). Name `ENABLED_TAB_ORDER` among the shared module's readers beside
      `tab_order` and `focused`.
    sidecar: >-
      §5 Test harness pattern: stand_keyboard walks the enabled states (ENABLED_TAB_ORDER) and asserts every control reached by Tab
      (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Symbols bullet lists `ENABLED_TAB_ORDER` (common/mod.rs:72-99) with its two sequences, measured equal to the
      prediction. It also lists `enable_by_keyboard`, `the_enabled_states_extend_the_walk_in_order` and
      `every_control_is_reached_by_tab`. Its Counts bullet "SC 2.1.1 / 2.4.3 in the enabled states" applies, and Expected amendments,
      "a11y-plan §5", list `stand-checks` a11y-plan.md:213 ×9. The `stand_keyboard` file count is 11 → 14.
    basis: >-
      .andromeda/a11y-plan.md:213; tests/blitz-tests/tests/common/mod.rs:72-99; tests/blitz-tests/tests/stand_keyboard.rs:237-268;
      tests/blitz-tests/tests/stand_keyboard.rs:324-342
  - detector: D-a11y-surface
    severity: warning
    section: §5 Keyboard Navigation → Script and framework exposure
    change: >-
      In the falsy-boolean bullet's `inert` clause, replace "owned by a CARRY on the route entry "Stand a11y assertions"". The fact
      stands. The new owner is a `.andromeda/residuals.md` line, (target: next), re-homed by the founder's F3 «Оставить, перенести
      (Рекомендую)» (2026-10-14). `stand_keyboard::the_engine_honours_no_inert` stays the pin, and whether an inert subtree takes a hit
      stays unmeasured.
    sidecar: >-
      §5 inert: the owner moves from the route entry "Stand a11y assertions" to a residuals line (target: next), on the founder's F3
      (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts list `inert` and `route-entry` at a11y-plan.md:186, as unchanged facts whose owner moves (F3). Decisions give
      F3's label verbatim. The P5 card's row 3 and the operator's word (inputs#I5) send it to `.andromeda/residuals.md`, `(target: next)`,
      naming the founder's label.
    basis: >-
      .andromeda/a11y-plan.md:186
  - detector: D-a11y-surface
    severity: warning
    section: §5 Keyboard Navigation → Per-surface keyboard shortcuts
    change: >-
      In the seven_guis CRUD bullet, replace "recorded, not closed, owned by a CARRY on the route entry "Stand a11y assertions"". The new
      owner is a `.andromeda/residuals.md` line, (target: next), re-homed by the founder's F2 «Оставить записанным (Рекомендую)»
      (2026-10-14). It records an `aria-activedescendant` read in the builder, or a roving focus, as SC 4.1.2 work outside the tier-0
      target, and the gap stays stated here. The list is also named `People` by `aria-label` since 2026-10-14-stand-a11y-assertions
      (examples/seven_guis/src/tasks/crud.rs:59).
    sidecar: >-
      §5 CRUD list: the missing aria-activedescendant moves to a residuals line (target: next) on the founder's F2, and the list is
      named People (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts list `activedesc` and `route-entry` at a11y-plan.md:196 as unchanged facts whose owner moves (F2). The P5
      card's row 2 and inputs#I5 give the residuals destination and the SC 4.1.2 framing. The Files bullet gives crud.rs:59 `aria_label:
      "People"`.
    basis: >-
      .andromeda/a11y-plan.md:196; examples/seven_guis/src/tasks/crud.rs:59
  - detector: D-a11y-surface
    severity: warning
    section: §7 Screen Reader Support → Accessibility tree output
    change: >-
      Retire the clause that the CRUD list reads `ListBox` with the accessible name `""`, so not every interactive-role element on the
      stand is named, and that the unnamed list is owned by a route entry. Since 2026-10-14-stand-a11y-assertions the list is named
      `People` by `aria-label` (examples/seven_guis/src/tasks/crud.rs:59). `every_interactive_role_element_reads_a_name` asserts, in both
      layout modes, that every interactive-role tree node of the four lean tasks reads a non-empty name. It covers 9 roles in
      `INTERACTIVE_ROLES`: Button, TextInput, Slider, ListBox, ListBoxOption, CheckBox, RadioButton, ComboBox, Link. Each task's count is
      at least its `controls` rows, and each `REGION_NAMES` row's name is in the tree and the snapshot
      (tests/blitz-tests/tests/stand_accessibility_ids.rs:26-37; :255-308). `REGION_NAMES` (tests/blitz-tests/tests/common/mod.rs:133-136)
      joins `controls` and `INPUT_NAMES` as stated expectation. The tree the shell hands the adapter is asserted equal to
      `harness.doc.accessibility_tree()` by id, node for node, with the same focus and root. That tree is `Document::accessibility_tree`
      through `&dyn blitz_dom::Document`, the call `AccessibilityState::update_tree` makes. Every `author_id` on it is the node's
      `element_id`, and none is empty, all digits or holds `0x` (tests/blitz-tests/tests/stand_accessibility_ids.rs:39-43; :310-352).
    sidecar: >-
      §7 tree output: crud-list named People, the interactive-role naming sweep over 9 roles (REGION_NAMES), and the &dyn Document tree
      carrying stable ids only (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts list `list-name` a11y-plan.md:280 (the name `""`), `common-tables` a11y-plan.md:280 (`REGION_NAMES` joins) and
      `adapter-tree` a11y-plan.md:280, which takes the `&dyn Document` proof. Its Symbols bullet lists `INTERACTIVE_ROLES`, `shell_tree`
      and the two tests. Expected amendments, "a11y-plan §7", list the site.
    basis: >-
      .andromeda/a11y-plan.md:280; examples/seven_guis/src/tasks/crud.rs:59; tests/blitz-tests/tests/stand_accessibility_ids.rs:255-308;
      tests/blitz-tests/tests/stand_accessibility_ids.rs:310-352; tests/blitz-tests/tests/common/mod.rs:133-136
  - detector: D-a11y-surface
    severity: warning
    section: §7 Screen Reader Support → Accessibility tree output
    change: >-
      In the attribute-state bullet, make two changes. First, the missing `aria-activedescendant` becomes owned by a
      `.andromeda/residuals.md` line, (target: next), on the founder's F2 «Оставить записанным (Рекомендую)», not by a CARRY on the route
      entry "Stand a11y assertions". Second, "and the list reads the accessible name `""`" becomes "the list named `People` by
      `aria-label` since 2026-10-14-stand-a11y-assertions (examples/seven_guis/src/tasks/crud.rs:59)". That leaves one gap open, not two.
    sidecar: >-
      §7 attribute states: the list's empty name is retired (now People), and the aria-activedescendant owner moves to a residuals line
      on F2 (2026-10-14-stand-a11y-assertions).
    rationale: >-
      This restates the two claims the §7 :280 and §5 :196 proposals retire. The report lists `list-name` a11y-plan.md:283 ("the
      accessible name `""`"), `activedesc` a11y-plan.md:283 and `route-entry` a11y-plan.md:283.
    basis: >-
      .andromeda/a11y-plan.md:283; examples/seven_guis/src/tasks/crud.rs:59
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: §2 A11y Strategy → Accessibility tree lifecycle
    change: >-
      Replace the refresh bullet's closing witness clause (proven at the document level and by both feature builds, its windowed witness
      owed on "Stand a11y assertions"). Since 2026-10-14-stand-a11y-assertions both halves are proven headlessly on the stand, in both
      layout modes. On Counter, after a drained changed set, each Tab of the whole `tab_order` leaves `has_changes()` true, and the focus
      of the `&dyn Document` tree (the tree the shell hands the adapter) reads the new stop. Enter on the focused `counter-increment` then
      leaves `has_changes()` true, and `counter-value`'s name reads `0` → `1` in that tree
      (tests/blitz-tests/tests/stand_accessibility_ids.rs:363-410). The platform-adapter windowed witness is still owed. It is re-homed by
      the founder's F1 «Пока нет, перенести (Рекомендую)» to a `.andromeda/residuals.md` line, (target: next).
    sidecar: >-
      §2 refresh on change: both halves proven headlessly on the stand through the &dyn Document tree. The windowed witness is still owed
      and re-homed to a residuals line on F1 (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts bullet "The refresh's two halves are proven headlessly" lists `refresh` a11y-plan.md:36 ×4 and :46, which take
      "the two halves proven headlessly on the stand, the windowed witness still owed". `route-entry` a11y-plan.md:36 names this entry as
      owner. The P5 card's row 1 and inputs#I5 give the residuals destination. Deviation 6 gives the walk.
    basis: >-
      .andromeda/a11y-plan.md:36; tests/blitz-tests/tests/stand_accessibility_ids.rs:363-410
  - detector: D-a11y-surface
    severity: warning
    section: §2 A11y Strategy → Role derivation (semantic HTML and ARIA)
    change: >-
      In the state-attributes bullet, replace "and the windowed witness is owned by a route entry". The new wording: the windowed witness
      at an active assistive technology of the three states is owned by a `.andromeda/residuals.md` line, (target: next), re-homed by the
      founder's F1 «Пока нет, перенести (Рекомендую)». The headless half, the tree the shell hands the adapter, is proven on the stand
      (§7).
    sidecar: >-
      §2 state attributes: the windowed witness of the three states moves to a residuals line on F1
      (2026-10-14-stand-a11y-assertions).
    rationale: >-
      This restates the windowed-witness ownership the :36 proposal re-homes. The report lists `refresh` a11y-plan.md:46. P5 card row 1
      covers "a witness at an active AT of ids, names, the three states and the refresh on change".
    basis: >-
      .andromeda/a11y-plan.md:46
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: §1 A11y Scope Summary → A11y scope (Dioxus crates)
    change: >-
      After "so no assistive technology is given a tree by the stand binary still", add that it stays so on the founder's F1 «Пока нет,
      перенести (Рекомендую)» (2026-10-14). The stand builds no platform adapter now. The adapter on the stand binary and its windowed AT
      witness are owned by a `.andromeda/residuals.md` line, (target: next). The headless halves (the `&dyn Document` tree carrying stable
      ids only, the refresh) are proven on the stand at 2026-10-14-stand-a11y-assertions (§2, §7).
    sidecar: >-
      §1 Dioxus crates: the stand's no-adapter state is kept on the founder's F1, and the adapter and its AT witness move to a residuals
      line (2026-10-14-stand-a11y-assertions).
    rationale: >-
      The report's Counts bullet "Platform adapter not built on the stand: unchanged (F1)" names a11y-plan.md:14 ×2 as true, with the
      owner moving. Expected amendments, "a11y-plan §1 and §2", say "The adapter decision re-homed (F1)". Decisions give F1's label
      verbatim. P5 card row 1 and inputs#I5 give the residuals destination.
    basis: >-
      .andromeda/a11y-plan.md:14
note: >-
  D-a11y-obs-schema finds no drift: the report says "No scrub, redaction or violation schema changed", and §3's NOT YET MEASURED
  violation-schema note stands. D-a11y-surface: the chunk adds no new interactive element, only a name on the existing `crud-list` and a
  re-colour. The proposals restate the §1–§9 coverage records the chunk moved. In the keyed contract `§3 → Bootstrap phases (derive for
  route / setup-project)`, lines 3 and 4 stay true as written (18 control rows measured; the `a11y` job), so no proposal is made for
  them. Lines 75, 158, 282 and 326 were read and still hold. The three residuals lines (F1, F2, F3) are P5's to land; these proposals
  name them as owners on the operator's word (inputs#I5).
