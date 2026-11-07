@answer n=1 entity=visual-design-verification#1.1 v=no
why: the session's reached files are the dioxus-native-dom mutation buffer, document and lib, the probe test, and none of blitz-paint's color or border code is among them
@answer n=2 entity=visual-design-verification#1.2 v=no
why: the scrollbar thumb contrast stroke in blitz-paint render.rs and its scrollbars.rs test are outside the session's reached files
@answer n=3 entity=visual-design-verification#1.3 v=no
why: the near-white author thumb hover and drag test in scrollbar_drag.rs is not a file the session reached
@answer n=4 entity=visual-design-verification#1.4 v=no
why: the Harness::text_contrast reader, stand_contrast and their 17 rows are untouched, and no reached file is among the sources they cite
@answer n=5 entity=visual-design-verification#1.5 v=no
why: the mix-blend-mode reading in blitz-test-harness contrast.rs is not in a file the session reached
@answer n=6 entity=visual-design-verification#1.6 v=no
why: blend and isolation painting in blitz-paint blend.rs and render.rs is outside the session's reached files
@answer n=7 entity=visual-design-verification#1.7 v=no
why: the load page and Cells contrast tests in stand_contrast.rs are not in a file the session reached
@answer n=8 entity=visual-design-verification#1.8 v=no
why: the visual lint's contrast reading in escher-driver execute.rs and json.rs is not in a file the session reached
@answer n=9 entity=visual-design-verification#1.9 v=no
why: the lint's blend-blind contrast reading goes through contrast.rs and escher-driver, none of which the session reached
@answer n=10 entity=visual-design-verification#1.10 v=no
why: the driver lint's overlap handling for closed selects and open lists lives in escher-driver lint.rs, which the session did not reach
@answer n=11 entity=visual-design-verification#1.11 v=no
why: the lint's reading of an open list's rows is in escher-driver and stand_lint.rs, outside the session's reached files
@answer n=12 entity=visual-design-verification#1.12 v=no
why: the user-agent select rules in blitz-dom default.css and the flight booker's select styling in flight_booker.rs are not reached by the session
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted and its constants are not in any file the session reached
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the default sheet's chosen-row pair in default.css is outside the session's reached files
@answer n=15 entity=visual-design-verification#1.15 v=no
why: the open list's engine colour constants in blitz-dom select_list.rs are not in a file the session reached
@answer n=16 entity=visual-design-verification#1.16 v=no
why: the open list's one-line row height limit is stated in docs/capabilities.md, which the session did not touch
@answer n=17 entity=visual-design-verification#2.1 v=no
why: the session adds no contrast check to any file, so the absence of contrast checks in the s05 files still holds
@answer n=18 entity=visual-design-verification#2.2 v=no
why: the session adds no contrast or user-preference media handling, so the absence recorded over the s11 files still holds
@answer n=19 entity=visual-design-verification#3.1 v=no
why: the browser urlbar focus border and outline in apps/browser browser.css are not in a file the session reached
@answer n=20 entity=visual-design-verification#3.2 v=no
why: the new-tab search input's focus rule in about-newtab.css is outside the session's reached files
@answer n=21 entity=visual-design-verification#3.3 v=no
why: the todomvc focus outline rule in todomvc.css is not in a file the session reached
@answer n=22 entity=visual-design-verification#3.4 v=no
why: the seven_guis focus replacements in temp_converter.rs and flight_booker.rs are not in files the session reached
@answer n=23 entity=visual-design-verification#3.5 v=no
why: the Cells current-cell grid focus outline in cells.rs is outside the session's reached files
@answer n=24 entity=visual-design-verification#3.6 v=no
why: the counter and transparent button focus outlines in their app.rs files are not in files the session reached, and the session changes no focus styling
@answer n=25 entity=visual-design-verification#3.7 v=no
why: the rdme focus-visible outline in github-markdown.css is outside the session's reached files
