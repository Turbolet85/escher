### screen-reader-support#11.6 · values · sr-text-alternatives · ## 7. Screen Reader Support · lines 92–92
- servo_header_reduced.html fixture image has no `alt` attribute (examples/assets/servo_header_reduced.html:14)
### screen-reader-support#12.1 · values · sr-ime · ## 7. Screen Reader Support · lines 98–98
- Focusing a text input enables IME and sets the IME cursor area to the input's content box; blurring disables IME (packages/blitz-dom/src/node/node.rs:771-784; packages/blitz-dom/src/node/node.rs:794-801)
### screen-reader-support#12.2 · values · sr-ime · ## 7. Screen Reader Support · lines 99–99
- A text input already focused when its text editor is constructed enables IME there — `create_text_editor` calls `shell_provider.set_ime_enabled(true)` — so an input focused before its editor exists has IME on with no fresh focus, in every document; pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/blitz-dom/src/layout/construct.rs:860; packages/blitz-dom/src/layout/construct.rs:867-872; packages/dioxus-native-dom/src/dioxus_document.rs:725-759) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
### screen-reader-support#12.3 · values · sr-ime · ## 7. Screen Reader Support · lines 100–100
- IME commit, preedit and disable events are applied to the text editor; `DeleteSurrounding` is a TODO (packages/blitz-dom/src/node/text.rs:763-803)
### screen-reader-support#13.1 · absence · sr-tree · ## 7. Screen Reader Support · lines 106–106
- screen-reader or accessibility API integration · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files
