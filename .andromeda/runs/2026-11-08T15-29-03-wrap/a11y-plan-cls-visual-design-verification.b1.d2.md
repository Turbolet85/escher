@answer n=1 entity=visual-design-verification#1.1 v=no
why: the WCAG helper lives in packages/blitz-paint, which the session does not reach.
@answer n=2 entity=visual-design-verification#1.2 v=no
why: the scrollbar stroke lives in blitz-paint render.rs and the scrollbars.rs test, neither reached by the session.
@answer n=3 entity=visual-design-verification#1.3 v=no
why: scrollbar_drag.rs is not reached and the author thumb behaviour is unchanged.
@answer n=4 entity=visual-design-verification#1.4 v=no
why: the contrast harness and stand_contrast.rs are not reached, and contrast verification is unchanged.
@answer n=5 entity=visual-design-verification#1.5 v=no
why: the blend reader lives in blitz-test-harness contrast.rs, which the session does not reach.
@answer n=6 entity=visual-design-verification#1.6 v=no
why: blend painting lives in blitz-paint, which the session does not reach.
@answer n=7 entity=visual-design-verification#1.7 v=no
why: the load-page and Cells contrast tests live in stand_contrast.rs, which is not reached, and no contrast reading moved.
@answer n=8 entity=visual-design-verification#1.8 v=yes mode=stale
why: execute.rs and json.rs are reached whole so the cited lines 319-321 and 386-414 may have moved, and the lint verb on every carrier is the example the task names under the instance-verb count that moved 9 to 10.
@answer n=9 entity=visual-design-verification#1.9 v=no
why: the blend reader lives in blitz-test-harness contrast.rs, which the session does not reach, and the lint's contrast reading is unchanged.
@answer n=10 entity=visual-design-verification#1.10 v=no
why: the overlap rule lives in escher-driver lint.rs, which the session does not reach.
@answer n=11 entity=visual-design-verification#1.11 v=yes mode=stale
why: docs/capabilities.md is a reached file, so its cited lines 245-247 may have moved and no report shows the stated limit holds on the changed file.
@answer n=12 entity=visual-design-verification#1.12 v=no
why: default.css, select_interaction.rs and flight_booker.rs are not reached and the closed select's pair is unchanged.
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted.rs is not reached and the open list's constants and measured pair are unchanged.
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the default sheet's chosen-row pair lives in blitz-dom default.css and select_interaction.rs, neither reached.
@answer n=15 entity=visual-design-verification#1.15 v=no
why: the open list's colour constants live in blitz-dom select_list.rs, which the session does not reach.
@answer n=16 entity=visual-design-verification#1.16 v=yes mode=stale
why: docs/capabilities.md is a reached file, so its cited lines 248-250 may have moved and no report shows the stated limit holds on the changed file.
@answer n=17 entity=visual-design-verification#2.1 v=no
why: the session adds no contrast check, and the s05 files searched for contrast are not among the reached files.
@answer n=18 entity=visual-design-verification#2.2 v=no
why: the session adds no media-preference handling, and the s11 files searched for it are not among the reached files.
@answer n=19 entity=visual-design-verification#3.1 v=no
why: apps/browser/assets/browser.css is not reached and the urlbar focus border is unchanged.
@answer n=20 entity=visual-design-verification#3.2 v=no
why: about-newtab.css is not reached and its search-input focus rule is unchanged.
@answer n=21 entity=visual-design-verification#3.3 v=no
why: todomvc.css is not reached and its focus outline rule is unchanged.
@answer n=22 entity=visual-design-verification#3.4 v=no
why: temp_converter.rs and flight_booker.rs are not reached and their focus rules are unchanged by the session.
@answer n=23 entity=visual-design-verification#3.5 v=no
why: cells.rs is not reached and the grid focus outline is unchanged.
@answer n=24 entity=visual-design-verification#3.6 v=no
why: counter and transparent app.rs are not reached and their focus outlines are unchanged.
@answer n=25 entity=visual-design-verification#3.7 v=no
why: github-markdown.css is not reached and its focus outline rule is unchanged.
