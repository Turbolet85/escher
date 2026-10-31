### screen-reader-support#11.2 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 92–92
- The google fixture's links carry descriptive aria-labels such as "Gmail (opens a new tab)" (examples/assets/google.html:3052; examples/assets/google.html:3061)
### screen-reader-support#11.3 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 93–93
- The google fixture hides decoration with aria-hidden and uses aria-atomic on suggestion options (examples/assets/google.html:3339; examples/assets/google.html:3318)
### screen-reader-support#11.4 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 94–94
- The servo-new fixture hides the burger's bar spans with aria-hidden (examples/assets/servo-new.html:34-36)
### screen-reader-support#11.5 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 95–95
- servo.html fixture images carry `alt` text ("Home", "Linux Foundation Europe logo") (examples/assets/servo.html:39; examples/assets/servo.html:331)
### screen-reader-support#11.6 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 96–96
- servo_header_reduced.html fixture image has no `alt` attribute (examples/assets/servo_header_reduced.html:14)
### screen-reader-support#12.1 · values · sr-ime · ## 7. Screen Reader Support · lines 102–102
- Focusing a text input enables IME and sets the IME cursor area to the input's content box; blurring disables IME (packages/blitz-dom/src/node/node.rs:771-784; packages/blitz-dom/src/node/node.rs:794-801)
### screen-reader-support#12.2 · values · sr-ime · ## 7. Screen Reader Support · lines 103–103
- A text input already focused when its text editor is constructed enables IME there — `create_text_editor` calls `shell_provider.set_ime_enabled(true)` — so an input focused before its editor exists has IME on with no fresh focus, in every document; pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/blitz-dom/src/layout/construct.rs:860; packages/blitz-dom/src/layout/construct.rs:867-872; packages/dioxus-native-dom/src/dioxus_document.rs:741-775) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
### screen-reader-support#12.3 · values · sr-ime · ## 7. Screen Reader Support · lines 104–104
- IME commit, preedit and disable events are applied to the text editor; `DeleteSurrounding` is a TODO (packages/blitz-dom/src/node/text.rs:763-803)
### screen-reader-support#13.1 · absence · sr-tree · ## 7. Screen Reader Support · lines 110–110
- screen-reader or accessibility API integration · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files
