@answer n=1 entity=visual-design-verification#1.1 v=no
why: the contrast helper lives in blitz-paint, which the session does not reach, and no paint or contrast code moved
@answer n=2 entity=visual-design-verification#1.2 v=no
why: the scrollbar contrast stroke and its test sit in blitz-paint and scrollbars.rs, outside the four reached files, and no paint code moved
@answer n=3 entity=visual-design-verification#1.3 v=no
why: the near-white thumb test sits in scrollbar_drag.rs, outside the reached files, and no scroll or paint code moved
@answer n=4 entity=visual-design-verification#1.4 v=no
why: stand_contrast and its record reads sit outside the reached files, and no contrast code moved while the replay applies the same mutations in the same order
@answer n=5 entity=visual-design-verification#1.5 v=no
why: the blend reader in blitz-test-harness contrast.rs sits outside the reached files, and no contrast code moved
@answer n=6 entity=visual-design-verification#1.6 v=no
why: mix-blend-mode paint lives in blitz-paint blend.rs and render.rs, outside the reached files, and no paint code moved
@answer n=7 entity=visual-design-verification#1.7 v=no
why: the load page and Cells contrast tests sit in stand_contrast.rs, outside the reached files, and no contrast code moved
@answer n=8 entity=visual-design-verification#1.8 v=no
why: the lint contrast reading and its stand_lint tests sit outside the reached files, and the a11y leg's 114 tests over 14 targets are unchanged
@answer n=9 entity=visual-design-verification#1.9 v=no
why: the blend exemption in blitz-test-harness contrast.rs sits outside the reached files, and no contrast code moved
@answer n=10 entity=visual-design-verification#1.10 v=no
why: the overlap geometry in escher-driver lint.rs and its stand_lint test sit outside the reached files, and no visual lint code moved
@answer n=11 entity=visual-design-verification#1.11 v=no
why: the open-list overlap and contrast-limit reads sit in escher-driver and stand tests outside the reached files, and no visual lint code moved
@answer n=12 entity=visual-design-verification#1.12 v=no
why: the user-agent select CSS in blitz-dom default.css and the flight booker select style sit outside the reached files, and no styles or select colours moved
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted.rs and the open list's measured pair sit outside the reached files, and the select list colours are untouched
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the multiple-select chosen-row pair in default.css sits outside the reached files, and no styles or select colours moved
@answer n=15 entity=visual-design-verification#1.15 v=no
why: the SelectListView constants in blitz-dom select_list.rs sit outside the reached files, and the select list colours are untouched
@answer n=16 entity=visual-design-verification#1.16 v=no
why: the row clipping limit and its capabilities doc sit outside the reached files, and no paint or select layout code moved
@answer n=17 entity=visual-design-verification#2.1 v=no
why: the session adds no contrast check to any file it reaches, since no contrast code moved, so the observed absence still holds
@answer n=18 entity=visual-design-verification#2.2 v=no
why: the session adds no contrast or user-preference media handling to any file it reaches, so the observed absence still holds
@answer n=19 entity=visual-design-verification#3.1 v=no
why: the browser urlbar focus border in browser.css sits outside the reached files, and no focus style moved
@answer n=20 entity=visual-design-verification#3.2 v=no
why: the new-tab search outline rule in about-newtab.css sits outside the reached files, and no focus style moved
@answer n=21 entity=visual-design-verification#3.3 v=no
why: the todomvc focus outline rule sits outside the reached files, and no focus style moved
@answer n=22 entity=visual-design-verification#3.4 v=no
why: the seven_guis focus styling in temp_converter.rs and flight_booker.rs sits outside the reached files, and no focus style moved
@answer n=23 entity=visual-design-verification#3.5 v=no
why: the Cells grid focus outline in cells.rs sits outside the reached files, and the stand_keyboard inventory is unmoved
@answer n=24 entity=visual-design-verification#3.6 v=no
why: the counter and transparent focus outlines sit in their app files outside the reached files, and no focus style moved
@answer n=25 entity=visual-design-verification#3.7 v=no
why: the rdme focus outline rule in github-markdown.css sits outside the reached files, and no focus style moved
