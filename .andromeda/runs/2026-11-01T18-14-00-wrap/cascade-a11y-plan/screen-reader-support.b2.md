### screen-reader-support#9.2 · values · sr-language · ## 7. Screen Reader Support · lines 82–82
- blitz-dom default stylesheet maps the `dir` attribute with attribute selectors for bidi isolation (packages/blitz-dom/assets/default.css:14-36)
### screen-reader-support#9.3 · values · sr-language · ## 7. Screen Reader Support · lines 83–83
- 11 fixtures set an html lang attribute (examples/assets/google.html:2; examples/assets/gosub.html:3; examples/assets/gosub_reduced.html:2; examples/assets/graphite.html:2; examples/assets/graphite_blog_section.html:2; examples/assets/graphite_software_overview.html:2; examples/assets/newservo.html:2; examples/assets/pseudo.html:2; examples/assets/servo-new-reduced-1.html:2; examples/assets/servo-new-reduced.html:2; examples/assets/servo-new.html:2)
### screen-reader-support#9.4 · values · sr-language · ## 7. Screen Reader Support · lines 84–84
- Pages declare `lang="en"` (examples/assets/servo.html:2; examples/preact/index.html:2; examples/preact/core_dom_apis.html:2)
### screen-reader-support#10.1 · absence · sr-language · ## 7. Screen Reader Support · lines 90–90
- Observed absent — an html lang attribute in the other 10 s02 fixtures · searched: `lang=` over the 21 s02 files
### screen-reader-support#11.1 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 96–96
- The captured GitHub fixture uses an `sr-only` tooltip element (examples/assets/github_profile_reduced2.html:68)
### screen-reader-support#11.2 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 97–97
- The google fixture's links carry descriptive aria-labels such as "Gmail (opens a new tab)" (examples/assets/google.html:3052; examples/assets/google.html:3061)
### screen-reader-support#11.3 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 98–98
- The google fixture hides decoration with aria-hidden and uses aria-atomic on suggestion options (examples/assets/google.html:3339; examples/assets/google.html:3318)
### screen-reader-support#11.4 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 99–99
- The servo-new fixture hides the burger's bar spans with aria-hidden (examples/assets/servo-new.html:34-36)
### screen-reader-support#11.5 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 100–100
- servo.html fixture images carry `alt` text ("Home", "Linux Foundation Europe logo") (examples/assets/servo.html:39; examples/assets/servo.html:331)
### screen-reader-support#11.6 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 101–101
- servo_header_reduced.html fixture image has no `alt` attribute (examples/assets/servo_header_reduced.html:14)
### screen-reader-support#12.1 · values · sr-ime · ## 7. Screen Reader Support · lines 107–107
- Focusing a text input enables IME and sets the IME cursor area to the input's content box; blurring disables IME (packages/blitz-dom/src/node/node.rs:782-795; packages/blitz-dom/src/node/node.rs:805-812)
### screen-reader-support#12.2 · values · sr-ime · ## 7. Screen Reader Support · lines 108–108
- A text input already focused when its text editor is constructed enables IME there — `create_text_editor` calls `shell_provider.set_ime_enabled(true)` — so an input focused before its editor exists has IME on with no fresh focus, in every document; pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/blitz-dom/src/layout/construct.rs:878; packages/blitz-dom/src/layout/construct.rs:885-890; packages/dioxus-native-dom/src/dioxus_document.rs:745-779) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
### screen-reader-support#12.3 · values · sr-ime · ## 7. Screen Reader Support · lines 109–109
- IME commit, preedit and disable events are applied to the text editor; `DeleteSurrounding` is a TODO (packages/blitz-dom/src/node/text.rs:763-803)
### screen-reader-support#13.1 · absence · sr-tree · ## 7. Screen Reader Support · lines 115–115
- screen-reader or accessibility API integration · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files
