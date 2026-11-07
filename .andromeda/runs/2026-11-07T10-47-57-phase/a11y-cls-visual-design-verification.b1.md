@answer n=1 entity=visual-design-verification#1.1 v=no
why: blitz-paint's contrast helper for border bevels and scrollbar thumbs; the chunk changes no paint and names no stand_* check.
@answer n=2 entity=visual-design-verification#1.2 v=no
why: default scrollbar thumb contrast stroke in blitz-paint and scrollbars.rs; the buffered render does not reach scrollbar paint.
@answer n=3 entity=visual-design-verification#1.3 v=no
why: near-white author thumb hover and drag in scrollbar_drag.rs; no scrollbar code is reached by the chunk.
@answer n=4 entity=visual-design-verification#1.4 v=yes
why: names stand_contrast, a stand_* check the chunk re-asserts, which witnesses the 17-row contrast record in both layout modes.
@answer n=5 entity=visual-design-verification#1.5 v=no
why: Harness::text_contrast's None past a blended ancestor, pinned by a blitz-test-harness test; no stand_* check is named.
@answer n=6 entity=visual-design-verification#1.6 v=no
why: mix-blend-mode and isolation paint through blitz-paint's render_element and blend module, which the buffered render does not touch.
@answer n=7 entity=visual-design-verification#1.7 v=yes
why: names stand_contrast's the_load_page_meets_its_ratios and cells_meets_its_ratios, both asserted in both layout modes, within the re-asserted stand_* checks.
@answer n=8 entity=visual-design-verification#1.8 v=yes
why: names stand_lint and its boot reading of no contrast or other finding on the lean tasks, a stand_* check the chunk re-asserts.
@answer n=9 entity=visual-design-verification#1.9 v=no
why: Session::lint's contrast reading of a blended text through the shared reader; no stand_* check is named and the chunk does not touch the lint.
@answer n=10 entity=visual-design-verification#1.10 v=yes
why: names stand_lint's the_stand_at_boot_reads_its_measured_findings, which reads [0, 0, 0, 0] at boot, a stand_* check the chunk re-asserts.
@answer n=11 entity=visual-design-verification#1.11 v=yes
why: names stand_lint and stand_act_popup for no overlap on an open list at boot, both stand_* checks the chunk re-asserts.
@answer n=12 entity=visual-design-verification#1.12 v=yes
why: names stand_contrast reading the flight booker's select#flight-type pair at 17.40 as a control row, a stand_* check the chunk re-asserts.
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted's open-list paint measurement; no stand_* check is named and the chunk does not touch the select list paint.
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the engine default chosen-row pair in default.css, measured by select_interaction; no stand_* check is named and no select CSS is reached.
@answer n=15 entity=visual-design-verification#1.15 v=no
why: SelectListView's colour constants in the engine's select list; paint is not touched and no stand_* check is named.
@answer n=16 entity=visual-design-verification#1.16 v=no
why: the open list's one-line row height limit in docs and select list layout; the chunk does not reach it and names no stand_* check.
@answer n=17 entity=visual-design-verification#2.1 v=no
why: absence record for contrast checks across the s05 files; the chunk adds no contrast check.
@answer n=18 entity=visual-design-verification#2.2 v=no
why: absence record for contrast or user-preference media handling; the chunk adds no such handling.
@answer n=19 entity=visual-design-verification#3.1 v=no
why: browser urlbar focus border in apps/browser CSS; the chunk edits no browser CSS and names no stand_* check.
@answer n=20 entity=visual-design-verification#3.2 v=no
why: new-tab search input focus outline in apps/browser CSS; not reached by the chunk and no stand_* check is named.
@answer n=21 entity=visual-design-verification#3.3 v=no
why: todomvc focus outline value in its stylesheet; not reached by the chunk and no stand_* check is named.
@answer n=22 entity=visual-design-verification#3.4 v=no
why: seven_guis input focus replacement by border or box-shadow is app CSS the chunk uses but does not change, and the scope names no outline value.
@answer n=23 entity=visual-design-verification#3.5 v=yes
why: names stand_keyboard, stand_contrast, stand_cells_served and stand_non_lean_tasks as witnesses of the Cells grid focus outline, all stand_* checks the chunk re-asserts.
@answer n=24 entity=visual-design-verification#3.6 v=no
why: counter and transparent button focus outlines in app CSS; the chunk does not reach them and no stand_* check is named.
@answer n=25 entity=visual-design-verification#3.7 v=no
why: rdme stylesheet focus-visible outline; not reached by the chunk and no stand_* check is named.
