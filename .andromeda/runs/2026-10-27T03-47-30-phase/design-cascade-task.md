# Task — design · 2026-10-27-readme-and-records

## The chunk
Rewrites the README's status and descriptive sections to show 0.1.0 and 0.2.0 done and verified and 0.3.0 planned with its six stages, the project written «Escher» in prose, crate, binary and path names left lowercase.
Restates the capabilities statement's cold-agent paragraph to the 2026-10-25 live run (six apps, twelve tools, 66 calls, 0 wrong, a screenshot and a lint called); the statement's pin and the README's tool count stay as the driver schema holds them.
Re-points two verified ledger refs in the 0.1.0 and 0.2.0 verification matrices to the tests' current names.
Moves four stand rows out of the design system's product surface into architecture and layout-templates (the seven_guis accent row, the success row, the cards item, the stand's DejaVu Sans item), named as an expected wrap amendment since phase edits no spec source.
Re-reads that the repository's own ignore file already ignores the 0.3.0 incubator; no code, test, engine, driver, CI or schema change.

## Reached files
- README.md · edits §Status, the "Later:" line, "What Escher is", the driver paragraph, "Try it"
- docs/capabilities.md · edits the cold-agent run paragraph (lines 288-291) and any other line naming the 2026-10-15 run as the latest
- .github/scripts/test_capability_statement.py · re-reads the statement's pin (verb and cause names, five counts, the README's tool count)
- escher-0.1.0/verification-matrix.json · edits the v010-17 verified ref
- escher-0.2.0/verification-matrix.json · edits the v020-05 verified ref
- tests/blitz-tests/tests/stand_snapshot_state.rs · re-reads the test name exactly_the_chosen_flight_type_reads_selected
- tests/blitz-tests/tests/stand_lint.rs · re-reads the test name the_stand_at_boot_reads_its_measured_findings
- .andromeda/design-system.md · edits the accent row, the success row, the cards item, the stand's DejaVu Sans item (moved out, a wrap amendment)
- .andromeda/architecture.md · edits the section each moved stand row lands in (a wrap amendment)
- .andromeda/layout-templates.md · edits the section each moved stand row lands in (a wrap amendment)
- .gitignore · re-reads line 39, the escher-0.3.0-incubator/ ignore (git check-ignore -v)
- .git/info/exclude · edits the two incubator lines at its end, if P3 rules their removal this chunk's

## Needs
- The seven_guis accent row: `#3a5ce5` with its hover `#2a4cd3` and active `#1f3fc1`, its non-text uses and its SC 1.4.3 pass, wherever the plan states it, the Core Colors body line and its detail entity both; it moves out with its values unchanged.
- The stand's success row among the semantic colours and status states; it moves out with its values unchanged.
- The stand's cards item, wherever the plan holds it (a surface or component entry for the seven_guis stand); it moves out with its values unchanged.
- The stand's DejaVu Sans item, the bundled font the stand boots with, wherever typography or a surface entry states it; it moves out with its values unchanged.
- Any other design-system line, pointer or cross-reference that cites one of those four rows, so nothing points at a row that has left the product surface.
- The design system's extraction: its body with a Detail pointer under each heading and its category files; a row moved out of a category file leaves its heading's pointer and the body consistent, read through the master's door (detail.py check) at P3.
- The subjects list at the end of the design-system body and each detail group's label: a subject or group that the move empties, or whose rows all leave, is restated or dropped with it.
- A design-system section left holding no stand row after the move, which keeps its NOT YET MEASURED or absence statement as it stands rather than gaining a claim.
- The contrast figures carried by the accent row (white on the accent at rest, on hover and when active), which the moved row keeps exactly, so stand_contrast's recorded rows and the a11y record stay unmoved.
- The driver's command line as an agent-read surface (stdout one JSON line per command, no colour or ESC byte, exit statuses 0 · 1 · 2 · 3, the mcp role's JSON-RPC lines): where the README's "Try it" or the driver paragraph describes the command line or its output, it states it as that entry records.
- The screenshot's fonts-unavailable refusal, read only where the README or the statement already names a screenshot or the lint; a line this chunk writes about them states no more than that entry and the statement hold.

## Standing conventions
- (none)

## Not needed
- Brand Identity, Anti-Patterns, Self-Validation Protocol and the Design Decisions Log: each reads NO RECORDED INTENT and the chunk records no design intent.
- The mobile-native surface (tokens, the URL bar, the back button, capture, its stack): the browser app's Android build is not touched.
- The web-spa surface (its tokens, the canvas on `#f5f5f5`): no WASM page changes.
- The desktop-native titlebar, custom widget, devtools overlay, window and stack entries: no windowed shell or browser chrome changes.
- The browser iconography, its icon sizes and the cursor and favicon rules: no icon, asset or cursor changes.
- The spacing, border-radius and depth entries, the hit-testing rule at a clipping box included: no layout, radius or hit behaviour changes.
- The motion entries on the into-view scroll, the harness settle, the VirtualClock and the load wait: the README names the virtual clock and the load wait only as the capabilities statement states them, and no motion value changes.
- The cli tokens (the WPT runner's owo-colors output): no WPT runner change.
- The typography loading rules on the blitz-tests system-fonts feature and on a WASM font context: no test or build feature changes.
- Engine-painted default colours and the palette's other values, and stand entries that hold none of the four named rows (the stand load page's component pattern among them): they stay where they are.
