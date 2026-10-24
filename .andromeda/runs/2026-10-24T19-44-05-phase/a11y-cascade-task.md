# Task — a11y · 2026-10-24-a11y-re-read

## The chunk
A re-read of the stand's a11y checks after 0.2.0's engine changes. It is not an engine change, and it adds no carrier, verb, field, span, listener, env read or crate.
It re-asserts stand_keyboard's Tab orders (boot and enabled states) and Enter/Space activation in both layout modes. A sequence that 0.2.0 moved is re-recorded with the change named, and a stand control that 0.2.0 made keyboard-operable (the flight booker's select) gets its own row.
It re-asserts stand_contrast's control rows, text rows and colour-state rows, so SC 1.4.3 holds on every non-exempt stand row.
It widens the a11y CI leg beyond its six files to the 0.2.0 a11y witnesses: the state, label, description, merged-state, inert, active-descendant and select checks. The leg's pin in test_ci_workflows.py moves with it, and the ci.yml job keeps its slow tier with no required status check.

## Reached files
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/common/mod.rs
- .github/scripts/ci-leg.sh · merge surface
- .github/scripts/test_ci_workflows.py · merge surface
- .github/workflows/ci.yml

## Needs
- The a11y tier, SC 2.1.1 · 1.4.3 · 2.4.3. The chunk re-asserts all three on the stand.
- The engine's focusability rule. It decides each lean task's Tab order, both at boot and in the enabled states: tabindex, the native focusable elements, `disabled` read by its presence on a form control alone, and an inert node or subtree not focusable.
- The carve-out from the focusability rule: HTML's focus fixup is not built. A node that holds focus as it becomes inert keeps it, and that fix stays a residual the chunk does not absorb.
- Disabled-ness keyed one way. The `:disabled` state, the pointer gates, the activation keys and focusability all read the attribute's presence, and the stand's enabled-state Tab orders rest on it.
- Indirect, through the Dioxus bridge's boolean attributes: a falsy `disabled`, `hidden` or `inert` value removes the attribute. That removal is what makes a stand control enabled and focusable in the enabled-state orders.
- Enter and Space activation. Enter dispatches a Click on the focused button, summary, submit/reset/button input or link, and a released Space on a button, summary or the listed input types. A disabled form control is excluded. stand_keyboard asserts this on each stand button.
- A button click's focus and a select click's focus. A click on a button or a select focuses it, a CRUD row click still clears focus, and the double-click window is read on the document's clock.
- The single-choice select's keyboard model: ArrowDown · ArrowUp · Home · End, typeahead held for 1 s of the document's clock, and one input then one change per change. The flight booker's select is the candidate new row in stand_keyboard (SC 2.1.1).
- The range input's keyboard and pointer model on the Timer slider: the arrows step it and Home · End take it to its minimum and maximum, and a click focuses it without moving it. It is a stand control that the Tab and activation re-read passes over.
- The CRUD list's keyboard model. `crud-list` (named `People`) is the third CRUD Tab stop, and ArrowDown · ArrowUp · Home · End move its selection. The tree's focus is the list, never an option, and `aria-activedescendant` stays unread (pinned by accessibility_active_descendant_unread).
- The keyboard harness pattern. stand_keyboard drives Tab and Shift+Tab through the harness's press / press_with in both layout modes. At every step the engine's focus, the snapshot's `focused` node and the tree's focus must agree, and every stand control lies in one of the two stated orders.
- The tree the shell hands the adapter, which must equal the harness's tree node for node, and the stable `author_id` on every node. Both are asserted by stand_accessibility_ids, which stays in the a11y leg.
- The contrast harness. Harness::text_contrast reads computed styles, contrast_ratio is the WCAG 2.x formula, 3.0 is required of large text and 4.5 otherwise, and the result is None under opacity below 1, a filter or a background image.
- The stand contrast record: three tables (18 control rows, 14 TEXT_ROWS, 19 STATE_ROWS, hover and active included), with a painted-pixel witness of each measured background. A row is exempt exactly where the snapshot's `enabled` reads false, and the stand accent is white on #3a5ce5.
- Indirect, through the shared exempt rule: escher-driver's visual lint reads `contrast` with stand_contrast's exempt rule and has no second formula or threshold. A changed exempt rule or threshold would move the lint too.
- State colour beside a non-colour cue. The flight booker's invalid dates show red styling and also state `aria-invalid`, its mode buttons state `aria-pressed`, and the CRUD rows state `aria-selected`. The colour-state rows and the headless no-colour cue rest on this.
- The builder's state mapping, which the widened leg's accessibility_states witness asserts: aria-selected → selected; aria-pressed → toggled except on the five checkable roles, where aria-checked takes its place; aria-invalid → invalid; `role="switch"` → Switch.
- The builder's merged upstream reads, which the widened leg's accessibility_label, accessibility_description and accessibility_merged_states witnesses assert: aria-labelledby outranking `<label>` and aria-label, aria-describedby, aria-description, aria-expanded, aria-disabled and aria-level.
- Inert visibility and hit-testing, which inert_subtree witnesses in the widened leg. An inert subtree is skipped by Tab and Shift+Tab, passed through by a hit and absent from the tree, and removing `inert` restores all three.
- Hidden-content exclusion and role mapping, which stay in the leg through accessibility_hidden and accessibility_roles. A select's option is excepted from `display: none`.
- The a11y assertion harness contract's statement of which leg each witness runs in. Today accessibility_states, label, description, merged_states, inert_subtree and active_descendant_unread are described as running outside the a11y leg's six files, and that reading moves with the widening.
- The CI integration contract: ci.yml's a11y job runs `ci-leg.sh a11y` over a named file list in the slow tier with no required status check. The three stand_* files also run three rounds in stand-repeat under the zero-failure budget.
- The boundary carve-out: native checkbox and radio checkedness is unread by the builder, owned by a residual with target next. The re-read must not count it as a defect to fold.

## Not needed
- The platform adapter: its creation before the window is shown, the per-platform AccessKit gaps, and the windowed assistive-technology witness. The scope excludes all three, and no stand binary builds an adapter.
- The scope summary's statement of how the `accessibility` feature reaches the Dioxus crates and seven_guis. The chunk changes no manifest or feature.
- The landmark and HTML-AAM element mapping inventory, and the role-derivation TODO on overriding strong native semantics. No check here re-reads landmarks.
- Language and direction (html lang), text alternatives and hidden decoration in the examples/assets fixtures, and input-method handling. The re-read touches none of them.
- The focus-ring, target-size and typography-readability tokens. The chunk measures text contrast only and changes no focus indicator.
- Motion, prefers-reduced-motion, prefers-color-scheme and the zoom and user-preference rules. No stand check reads them.
- Password and file-input masking (MASKED_VALUE, bullets painted, the caret placed by them). It belongs to the snapshot and paint surfaces, not to Tab order, activation or contrast.
- The actionable-key rule and unkeyed_actionable. The chunk adds no stand control and checks no author key.
- The snapshot's text form, parts, diff and MCP results, and the driver's refusal by `enabled`. These are driver surfaces the chunk does not change.
- Cognitive orientation and status cues, and the not-yet-measured timeout, plain-language and authentication items.
- The obs log format and the undefined a11y violation JSON schema. The chunk adds no log field and no violation output.
- The observed-absent scope and assertion searches over the old slices. They are historical search records, not rules over the stand checks.
- The SLO budgets, the anti-patterns and the decisions log. All three hold no recorded intent.
