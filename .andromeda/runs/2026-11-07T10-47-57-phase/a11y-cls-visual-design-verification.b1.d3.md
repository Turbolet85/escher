@answer n=1 entity=visual-design-verification#1.1 v=no
why: the WCAG helper for bevels and scrollbar thumbs is blitz-paint colour code the chunk does not touch, and no stand_* check is named over it
@answer n=2 entity=visual-design-verification#1.2 v=no
why: the default scrollbar thumb contrast stroke is paint in blitz-paint, witnessed by scrollbars.rs, not a stand_* check the chunk re-asserts
@answer n=3 entity=visual-design-verification#1.3 v=no
why: the near-white author thumb hover and drag rule is in scrollbar_drag.rs, a scrollbar paint and input check outside the chunk's scope
@answer n=4 entity=visual-design-verification#1.4 v=yes
why: stand_contrast's 17 rows and its pass, fail and exempt record are re-asserted through stand_* in both modes, so the buffered render must keep the stand's painted colours and values
@answer n=5 entity=visual-design-verification#1.5 v=no
why: the None reading past a blend in Harness::text_contrast is a unit-level reader behaviour the chunk does not touch, and its stand-row remark is carried by #1.4 and #1.7
@answer n=6 entity=visual-design-verification#1.6 v=no
why: blend layer and isolation paint in blitz-paint's render and blend modules is paint the chunk does not change, its DOM replay leaves paint as it is
@answer n=7 entity=visual-design-verification#1.7 v=yes
why: stand_contrast's two tests, the_load_page_meets_its_ratios and cells_meets_its_ratios, are stand_* checks the chunk re-asserts in both modes
@answer n=8 entity=visual-design-verification#1.8 v=yes
why: stand_lint's at-boot reading of [0, 0, 0, 0] and the no-contrast-finding result on the lean tasks are stand_* values the chunk re-asserts
@answer n=9 entity=visual-design-verification#1.9 v=no
why: Session::lint's contrast reading past a blend is a lint reader behaviour the chunk does not touch, and no stand_* check witnesses it
@answer n=10 entity=visual-design-verification#1.10 v=yes
why: the flight booker reads no overlap at boot in both modes, witnessed by stand_lint's the_stand_at_boot_reads_its_measured_findings, a stand_* check the chunk re-asserts
@answer n=11 entity=visual-design-verification#1.11 v=yes
why: no overlap on an open list row at stand boot is witnessed by stand_lint and stand_act_popup, both stand_* checks the chunk re-asserts
@answer n=12 entity=visual-design-verification#1.12 v=yes
why: the stand's select#flight-type reads 17.40 as a control row and as a return-chosen STATE_ROWS row in stand_contrast, a re-asserted stand_* value; the user-agent select layout itself is not touched
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted and the open-list constants are not a stand_* check and the chunk neither re-measures nor touches them
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the engine-default chosen-row pair in default.css is paint measured by select_interaction, not a stand_* check, and the chunk does not change it
@answer n=15 entity=visual-design-verification#1.15 v=no
why: the SelectListView colour constants are engine paint the chunk does not change and no stand_* check reads them
@answer n=16 entity=visual-design-verification#1.16 v=no
why: the open-list row height and clipping limit is select paint and layout the chunk does not touch
@answer n=17 entity=visual-design-verification#2.1 v=no
why: an observed absence of contrast checks in s05 files is a search record, not a rule over the chunk's code or tests
@answer n=18 entity=visual-design-verification#2.2 v=no
why: an observed absence of contrast and user-preference media handling is a search record, not a rule the chunk must honour
@answer n=19 entity=visual-design-verification#3.1 v=no
why: the browser urlbar focus border is apps/browser CSS the chunk does not touch and no focus check names it
@answer n=20 entity=visual-design-verification#3.2 v=no
why: the new-tab search input outline is apps/browser CSS the chunk does not touch
@answer n=21 entity=visual-design-verification#3.3 v=no
why: todomvc's :focus outline rule is in its own stylesheet, outside the chunk's code and tests
@answer n=22 entity=visual-design-verification#3.4 v=no
why: seven_guis focus indication is author CSS the chunk does not change, and no stand_* check is named over the select's focus border
@answer n=23 entity=visual-design-verification#3.5 v=yes
why: the Cells current-cell focus outline is named as within the coverage of stand_keyboard, stand_contrast, stand_cells_served and stand_non_lean_tasks, which the chunk re-asserts
@answer n=24 entity=visual-design-verification#3.6 v=no
why: the counter and transparent focus outlines are example-app CSS the chunk does not touch and no stand_* check names
@answer n=25 entity=visual-design-verification#3.7 v=no
why: the rdme stylesheet focus outline is apps/readme CSS the chunk does not touch
