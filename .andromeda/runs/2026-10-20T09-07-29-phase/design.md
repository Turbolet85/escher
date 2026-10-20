# design extract

## Relevance
partial — the chunk adds no visual surface, tokens, motion or iconography. Its walk reaches an agent through the driver's carriers, and design-system §Surface: cli §Component Patterns states those carriers' output form. The fixture's bounds and byte counts also depend on the font loading in design-system §Typography (Loading).

## Constraints
- design-system §Surface: cli §Component Patterns requires `escher-session` to write one line of JSON per command on stdout: the answer, the refusal or a session error's object, keyed by the driver schema's own words, and nothing else there. A part of a large screen, whatever its form, is bound by this. Whether the 10,000-byte budget per part also fits this one-line form is research's question.
- design-system §Surface: cli §Component Patterns requires every answer line to be uncoloured. It allows no colour dependency, no ESC byte and no code that asks whether a stream is a terminal, so a part's line reads the same piped or not.
- design-system §Surface: cli §Component Patterns requires the exit status to carry the class (`0` accepted · `1` refused · `2` usage · `3` session error), read from the schema's `EXIT_STATUSES` table that `help` serves. A walk past its last part, or a stale walk, gets one of these classes. Adding an exit is a boundary fork the founder decides (inputs#I1), not a design choice.
- design-system §Surface: cli §Component Patterns requires the `mcp` role's stdout to hold only newline-delimited JSON-RPC 2.0 lines, every reply one line and uncoloured. A `snapshot` result is the screen text itself, so a part read over MCP keeps the same one-line, no-ESC form.
- design-system §Surface: cli §Component Patterns requires `escher-session help` to answer as one uncoloured line of JSON with status `0`, reaching no session. Any new verb, argument or result field that the boundary fork chooses then appears in that line in the same form.
- design-system §Typography (Loading) records that the stand's headless boot registers bundled DejaVu Sans through `build_single_font_ctx`, with system fonts off. It also records that without `system-fonts` in blitz-tests, text measures 0x0 and font-dependent assertions pass vacuously. The fixture's screen size and its parts' byte counts include `bounds` and depend on text layout, so the fixture boots with a font that makes them real and repeatable.
- design-system §Color Palette records each stand colour as measured by `stand_contrast` (accent `#3a5ce5`, page `#f5f5f5`, text `#1a1a1a`). The chunk's boundary keeps the stand as built, so none of these values moves.

## Patterns to follow
- Write an answer's line from schema words, as `screenshot` does (`{"width":…,"height":…,"png":"<base64>"}`, one uncoloured line, status `0`). Over MCP, a `snapshot` result is the screen text, not wrapped JSON (per design-system §Surface: cli §Component Patterns).
- Boot the fixture with one font, like the stand's headless boot (`build_single_font_ctx` through `HarnessOptions.font_ctx`), so the snapshot's bounds and text bytes are the same on every run (per design-system §Typography (Loading)).
- A read moves no time: `click`, `type` and `press` leave the harness's `VirtualClock` where it was, and `advance` alone moves it (per design-system §Motion (Animation runtime)). So reading the parts of one screen does not move the clock between parts. Whether a part read keeps to this is research's question.

## Anti-patterns to avoid
- Putting anything on the command line's stdout besides the one answer line: a log line, a progress line per part, or a coloured or ESC-bearing line (per design-system §Surface: cli §Component Patterns).
- Making a part's output depend on whether stdout is a terminal, for example by wrapping, paging or colouring when attached to a TTY (per design-system §Surface: cli §Component Patterns).

## Contract bindings
- CLI and MCP output form ↔ tests: design-system §Surface: cli §Component Patterns says the no-ESC reading is asserted on every answer line by the seven_guis checks' shared reader (`examples/seven_guis/tests/common/mod.rs`) and on every MCP reply by the shared `Speaker`. A part's answer passes through both.
- Exit statuses and `help` ↔ the driver schema and security: `EXIT_STATUSES` and the `help` self-description come from the schema's tables (design-system §Surface: cli §Component Patterns). A new exit, verb, argument or field is a boundary fork the founder decides (scope's inputs#I1), registered in security-plan.
- Fixture font ↔ tests harness: the fixture boot in `tests/blitz-tests/tests/fixtures/mod.rs` binds to design-system §Typography (Loading) and its font context.

## Acceptance criteria contributions
- Each part read through `escher-session` is one line of JSON on stdout with no ESC byte and nothing else on stdout. Its exit status is one of `EXIT_STATUSES`'s four (per design-system §Surface: cli §Component Patterns).
- Each part read through the `mcp` role is one JSON-RPC reply line with no ESC byte (per design-system §Surface: cli §Component Patterns).
- The fixture's over-budget screen is measured with a real font (non-zero text bounds), so "passes 10,000 bytes" and "each part within the budget" are not met vacuously (per design-system §Typography (Loading)).
- No stand colour moves: `stand_contrast`'s rows read as before (per design-system §Color Palette).
