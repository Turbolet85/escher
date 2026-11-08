@answer n=1 entity=visual-design-verification#1.1 v=no
why: its rule lives in blitz-paint color.rs and render/border.rs, which the session does not reach
@answer n=2 entity=visual-design-verification#1.2 v=no
why: its rule lives in blitz-paint render.rs and the scrollbars test, neither reached by the session
@answer n=3 entity=visual-design-verification#1.3 v=no
why: its test lives in scrollbar_drag.rs, which the session does not reach
@answer n=4 entity=visual-design-verification#1.4 v=no
why: its rule lives in the blitz-test-harness contrast.rs and stand_contrast.rs, neither reached, and the session moves no served markup or colour
@answer n=5 entity=visual-design-verification#1.5 v=no
why: its rule lives in blitz-test-harness contrast.rs and blitz-paint, neither reached by the session
@answer n=6 entity=visual-design-verification#1.6 v=no
why: its rule lives in blitz-paint blend.rs and render.rs, neither reached by the session
@answer n=7 entity=visual-design-verification#1.7 v=no
why: its tests live in stand_contrast.rs, which the session does not reach, and no served screen's colours moved
@answer n=8 entity=visual-design-verification#1.8 v=yes mode=stale
why: execute.rs:310-312 and json.rs:363-391 sit in files the graph verb on every carrier reaches, so their line citations can move
@answer n=9 entity=visual-design-verification#1.9 v=no
why: its rule lives in blitz-test-harness contrast.rs, which the session does not reach
@answer n=10 entity=visual-design-verification#1.10 v=no
why: its rule lives in escher-driver lint.rs and stand_lint.rs, neither reached, and the flight booker's served markup did not move
@answer n=11 entity=visual-design-verification#1.11 v=yes mode=stale
why: docs/capabilities.md:230-232 sits in a file the session reaches and edits by removing the click not-built line, so its lines can move
@answer n=12 entity=visual-design-verification#1.12 v=yes mode=stale
why: flight_booker.rs:178-190 sits in a file the session reaches, where use_graph_signal is wrapped, so the cited lines can move
@answer n=13 entity=visual-design-verification#1.13 v=no
why: its rule lives in select_popup_painted.rs, which the session does not reach
@answer n=14 entity=visual-design-verification#1.14 v=no
why: its rule lives in blitz-dom default.css and select_interaction.rs, neither reached by the session
@answer n=15 entity=visual-design-verification#1.15 v=no
why: its rule lives in blitz-dom select_list.rs, which the session does not reach
@answer n=16 entity=visual-design-verification#1.16 v=yes mode=stale
why: docs/capabilities.md:233-235 sits in a file the session reaches and edits, so its lines can move
@answer n=17 entity=visual-design-verification#2.1 v=no
why: the session brings no contrast check into any file it reaches, so the absence stands
@answer n=18 entity=visual-design-verification#2.2 v=no
why: the session brings no contrast or prefers- handling into any file it reaches, so the absence stands
@answer n=19 entity=visual-design-verification#3.1 v=no
why: its rule lives in apps/browser browser.css, which the session does not reach
@answer n=20 entity=visual-design-verification#3.2 v=no
why: its rule lives in apps/browser about-newtab.css, which the session does not reach
@answer n=21 entity=visual-design-verification#3.3 v=no
why: its rule lives in examples/todomvc todomvc.css, which the session does not reach
@answer n=22 entity=visual-design-verification#3.4 v=yes mode=stale
why: temp_converter.rs:85-89 and flight_booker.rs:187-190 and 201-204 sit in files the session reaches, so the cited lines can move
@answer n=23 entity=visual-design-verification#3.5 v=yes mode=stale
why: cells.rs:600-603 sits in a file the session reaches, so the cited lines can move
@answer n=24 entity=visual-design-verification#3.6 v=no
why: its rule lives in examples/counter and examples/transparent app.rs, neither reached by the session
@answer n=25 entity=visual-design-verification#3.7 v=no
why: its rule lives in apps/readme github-markdown.css, which the session does not reach
