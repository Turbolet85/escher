# design extract

## Relevance
partial — the chunk renders nothing new and adds no token, but three of its items touch design-plan readings: CARRY 3 (the hover-cursor unit tests and their font skip), CARRY 10 (the `escher-session` command-line paths) and the flakiness budget over stand checks that pin painted colours and text (`stand_contrast`, the screenshot checks).

## Constraints
- design-system §Iconography (Cursors and favicons (engine)) requires the hover cursor to come from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise. CARRY 3's two `hover_state_tests` (`..._reports_text_cursor`, `..._whitespace_reports_default_cursor`) must assert those two arms with a real reading, not pass on a skip. Whether they reach either arm today is research's question.
- design-system §Typography (Loading) says that without `system-fonts` text measures 0x0 and font-dependent assertions pass vacuously. A repaired CARRY 3 test, and any check a gate counts, must not pass through a font-absent path. Whether the "skipping: no usable font" branch is a font gap or the CARRY's `qual_name!` namespace hypothesis is research's question.
- design-system §Typography (Loading) names the deterministic font path: `build_single_font_ctx` registers one bundled font for every generic with system fonts off, which is how the seven_guis stand boots (`HarnessOptions.font_ctx`, DejaVu Sans). The flakiness budget's repeat runs over stand checks depend on that pinned font context. Whether any stand check under the budget reads system fonts instead is research's question.
- design-system §Color Palette (Core Colors, the seven_guis accent row) records the accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) with white-text ratios 5.47 · 6.83 · 8.27, as `stand_contrast` measures them. A flakiness budget that covers `stand_contrast` must count a ratio or colour mismatch as a failure. A tolerance or retry must never absorb one.
- design-system §Surface: cli (Component Patterns) requires `escher-session` to write one uncoloured line of JSON on stdout, with no ESC byte and no terminal test, so its output is the same whether piped or not. Its usage and session-error text go to stderr, and its exit status is `0` · `1` · `2` · `3`. CARRY 10's named checks (a write to a closed pipe dropped rather than a panic, and the 30 s `ANSWER_BOUND` per read) must keep that output shape on every path they reach.
- design-system §Typography (Loading) requires a driver `screenshot` of an app with no `sans-serif` family to be refused `fonts-unavailable`, with nothing painted. A coverage floor or flake measurement that runs the screenshot checks under a different font configuration must still read that refusal and never a text-less shot.

## Patterns to follow
- A font-dependent fixture boots with a bundled font through `build_single_font_ctx`, as the stand does (`examples/seven_guis/src/stand.rs`), per design-system §Typography (Loading). This is the candidate route for CARRY 3 if research finds a font gap rather than the namespace mechanism.
- Hover-cursor expectations are written against the engine's cursor rule order (keyword → text input → link → selectable text → Default), per design-system §Iconography (Cursors and favicons (engine)).
- Output of escher's command line is read by its shared no-ESC reader on every answer line (the seven_guis checks' reader and the MCP `Speaker`). New CARRY 10 checks reuse that reader rather than adding a second one, per design-system §Surface: cli (Component Patterns).

## Anti-patterns to avoid
- A font-dependent unit test that prints a skip and passes. It counts as coverage while asserting nothing, per design-system §Typography (Loading).
- Colour or an ESC byte on any `escher-session` stdout line, including any gate or repeat-runner wrapper that re-emits the binary's answer, per design-system §Surface: cli (Component Patterns).
- Moving a stand colour or ratio to make a repeat run pass. The accent row is a measured record, per design-system §Color Palette (Core Colors).

## Contract bindings
- design ↔ a11y: the accent row's ratios in design-system §Color Palette (Core Colors) are SC 1.4.3 readings that `stand_contrast` asserts. A flakiness budget over that file binds to a11y-plan §Contrast and to the `a11y` CI leg.
- design ↔ tests: the bundled-font boot in design-system §Typography (Loading) is the determinism premise of the test-plan §10 flakiness budget for stand checks, and the font precondition of CARRY 3's repaired tests.
- design ↔ security: CARRY 9's hint-less `@font-face` source sits beside the font-feature list (`woff`) in design-system §Typography (Loading). The plan records no font-format sniffing, so that check's authority is security-plan §Input Validation, not this plan.

## Acceptance criteria contributions
- CARRY 3's two hover tests fail when the cursor reading is wrong. They assert Text over the anonymous-block text and Default over its whitespace, with no skip branch that passes (per design-system §Iconography (Cursors and favicons (engine))).
- No check counted toward the coverage floor or the flakiness budget passes vacuously for want of a font. Each font-dependent one boots a bundled font context or fails loudly (per design-system §Typography (Loading)).
- Under the flakiness budget's repeat runs, `stand_contrast` reads the same colour pairs and ratios on every repeat (accent 5.47, hover 6.83, active 8.27), and any divergence counts against the budget (per design-system §Color Palette (Core Colors)).
- Every new CARRY 10 check that reads `escher-session` output asserts one line of JSON on stdout with no ESC byte, and the exit status from `0` · `1` · `2` · `3` (per design-system §Surface: cli (Component Patterns)).
