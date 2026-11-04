@answer n=1 entity=visual-design-verification#1.1 v=no
why: the session reaches only dioxus_runtime_probe.rs, Cargo.lock and deny.toml, and blitz-paint's contrast helper sits in none of them
@answer n=2 entity=visual-design-verification#1.2 v=no
why: the scrollbar thumb stroke lives in blitz-paint and scrollbars.rs, neither reached, and Cargo.lock moves only pulldown-cmark
@answer n=3 entity=visual-design-verification#1.3 v=no
why: scrollbar_drag.rs is not reached and the session changes no author thumb colour or hover path
@answer n=4 entity=visual-design-verification#1.4 v=no
why: Harness::text_contrast, the stand_contrast tables and the color crate's luminance are unchanged, Cargo.lock moving only pulldown-cmark
@answer n=5 entity=visual-design-verification#1.5 v=no
why: the blend guard in blitz-test-harness is not among the reached files and no stand row moves
@answer n=6 entity=visual-design-verification#1.6 v=no
why: blitz-paint's blend module and render.rs are not reached and the probe adds no blended frame
@answer n=7 entity=visual-design-verification#1.7 v=no
why: stand_contrast.rs is not reached and its load-page and Cells tests read unchanged colours
@answer n=8 entity=visual-design-verification#1.8 v=no
why: the a11y leg stays 14 targets and 114 tests because the new target joins neither the a11y leg nor stand-repeat, and stand_lint.rs is not reached
@answer n=9 entity=visual-design-verification#1.9 v=no
why: the lint's contrast reader and the blend guard in blitz-test-harness are unchanged, the session reaching neither
@answer n=10 entity=visual-design-verification#1.10 v=no
why: lint.rs and stand_lint.rs are not reached and the flight booker's overlap reading is unchanged
@answer n=11 entity=visual-design-verification#1.11 v=no
why: stand_lint.rs and stand_act_popup.rs are not reached and the highlighted-row limit in docs/capabilities.md stays as it stood
@answer n=12 entity=visual-design-verification#1.12 v=no
why: default.css and select_interaction.rs are not reached and the stand's select#flight-type styling is unchanged
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted.rs is not reached and the open list's constants are unchanged
@answer n=14 entity=visual-design-verification#1.14 v=no
why: default.css and select_interaction.rs are not reached, so the multiple-select chosen-row pair holds
@answer n=15 entity=visual-design-verification#1.15 v=no
why: select_list.rs holds the same engine constants, the session reaching no blitz-dom file
@answer n=16 entity=visual-design-verification#1.16 v=no
why: the one-line row clip limit in docs/capabilities.md is untouched, no capability or select file being reached
@answer n=17 entity=visual-design-verification#2.1 v=no
why: the probe, reached whole, reads Dioxus listener, focus and state and adds no contrast check, so the absence still holds
@answer n=18 entity=visual-design-verification#2.2 v=no
why: the probe, reached whole, adds no prefers-* media handling or contrast check, so the absence still holds
@answer n=19 entity=visual-design-verification#3.1 v=no
why: browser.css is not reached and the urlbar focus border is unchanged
@answer n=20 entity=visual-design-verification#3.2 v=no
why: about-newtab.css is not reached and the new-tab search focus rule is unchanged
@answer n=21 entity=visual-design-verification#3.3 v=no
why: todomvc.css is not reached and its focus outline stays at zero
@answer n=22 entity=visual-design-verification#3.4 v=no
why: the seven_guis temp_converter and flight_booker focus rules are not reached, the probe measuring no outline
@answer n=23 entity=visual-design-verification#3.5 v=no
why: cells.rs outline rule is unchanged and the probe's Cells keydown reading does not move the grid focus ring or stand_keyboard's coverage
@answer n=24 entity=visual-design-verification#3.6 v=no
why: the counter and transparent button outlines are unchanged, neither app being reached
@answer n=25 entity=visual-design-verification#3.7 v=no
why: github-markdown.css is not reached and the readme focus outline is unchanged
