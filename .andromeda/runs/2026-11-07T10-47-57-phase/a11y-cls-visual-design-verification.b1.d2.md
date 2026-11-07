@answer n=1 entity=visual-design-verification#1.1 v=no
why: blitz-paint's own bevel and scrollbar-thumb contrast helper is read by no re-asserted check; stand_contrast uses the harness restatement on text pairs
@answer n=2 entity=visual-design-verification#1.2 v=no
why: the default scrollbar thumb stroke is read by scrollbars.rs, a check the scope does not name, and the chunk does not touch scrollbar paint
@answer n=3 entity=visual-design-verification#1.3 v=no
why: the near-white author thumb on hover and drag is read by scrollbar_drag.rs, which the scope does not name, and no thumb or drag code changes
@answer n=4 entity=visual-design-verification#1.4 v=yes
why: the stand_contrast rows, tables, required ratios and exempt rule are read by stand_contrast, a stand_* check the scope re-asserts
@answer n=5 entity=visual-design-verification#1.5 v=yes
why: text_contrast's mix-blend-mode walk is read on every stand_contrast row, and stand_contrast is re-asserted through stand_*
@answer n=6 entity=visual-design-verification#1.6 v=no
why: blend-layer paint plumbing and the CARRY on blended frames concern no stand row or re-asserted check, and the chunk does not change paint
@answer n=7 entity=visual-design-verification#1.7 v=yes
why: the_load_page_meets_its_ratios and cells_meets_its_ratios are stand_contrast tests, a stand_* check the scope re-asserts
@answer n=8 entity=visual-design-verification#1.8 v=yes
why: stand_lint's at-boot contrast reading of [0, 0, 0, 0] and its planted-row reading are re-asserted stand_* checks that read text_contrast
@answer n=9 entity=visual-design-verification#1.9 v=yes
why: Session::lint's contrast reads the same blend None on stand nodes, which stand_lint re-asserts
@answer n=10 entity=visual-design-verification#1.10 v=yes
why: the_stand_at_boot_reads_its_measured_findings in stand_lint is a re-asserted stand_* check reading the overlap exclusions
@answer n=11 entity=visual-design-verification#1.11 v=yes
why: stand_lint and stand_act_popup read the open-list row lint on the stand, and both are re-asserted stand_* checks
@answer n=12 entity=visual-design-verification#1.12 v=yes
why: the stand's select#flight-type pair at 17.40 is read by stand_contrast as a control row and as a STATE_ROWS row
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted reads the painted open list against SelectListView constants, a check the scope does not name, and stand_contrast's list-open rows are text_contrast readings, not this
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the engine-default multiple-select chosen-row pair is measured by select_interaction, which the scope does not name, and no re-asserted check reads it
@answer n=15 entity=visual-design-verification#1.15 v=no
why: the SelectListView colour constants are read by select_popup_painted only, which the scope does not name, and the chunk changes no colour
@answer n=16 entity=visual-design-verification#1.16 v=no
why: the open list's one-line row height is a stated docs limit read by no re-asserted check, and the chunk does not change select layout
@answer n=17 entity=visual-design-verification#2.1 v=no
why: this is a search record of absent contrast checks in s05 files that states no rule the chunk's code or tests must honour
@answer n=18 entity=visual-design-verification#2.2 v=no
why: this is a search record of absent contrast or user-preference media handling that states no rule the chunk's code or tests must honour
@answer n=19 entity=visual-design-verification#3.1 v=no
why: the browser urlbar focus outline is CSS paint in apps/browser, not a stand task and not named by the scope
@answer n=20 entity=visual-design-verification#3.2 v=no
why: the new-tab search input focus outline is CSS paint in apps/browser, not named by the scope
@answer n=21 entity=visual-design-verification#3.3 v=no
why: the todomvc :focus outline is CSS paint that no named check reads, and the chunk does not touch todomvc
@answer n=22 entity=visual-design-verification#3.4 v=no
why: the seven_guis input focus indicators are CSS paint that no named check reads, and the entity names no witness check
@answer n=23 entity=visual-design-verification#3.5 v=yes
why: it names stand_keyboard as the witness of Cells' grid focus outline, and stand_keyboard is a stand_* check the scope re-asserts
@answer n=24 entity=visual-design-verification#3.6 v=no
why: the counter and transparent 4px focus outlines are CSS paint that no named check witnesses
@answer n=25 entity=visual-design-verification#3.7 v=no
why: the rdme focus outline is CSS paint in apps/readme, not a stand task and not named by the scope
