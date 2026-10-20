# design extract

## Relevance
partial — the chunk is headless, and the boundaries say it paints no dropdown popup. Design applies only where the fixture's `select` takes the engine's default form-control styling and text. It also applies to the driver's agent-read CLI/MCP surface, if the chunk adds a verb or an argument. Brand Identity, Anti-Patterns, Self-Validation Protocol, Depth Strategy's chosen approach and Motion's expression level read `> NO RECORDED INTENT`, so nothing is cited from them.

## Constraints
- A form `select` takes its default typography from the blitz-dom user-agent stylesheet's form-control row: inputs, selects and buttons in `system-ui, sans-serif` (per design-system §Typography). Whether `packages/blitz-dom/assets/default.css` already holds a `select` rule, and what it sets, is research's question.
- A document receives the `DEFAULT_CSS` user-agent stylesheet once, through `DocumentConfig::default()` (per design-system §Surface: desktop-native — Tokens). A fixture page holding a `select` therefore reads the engine's default control styling unless the page styles the control itself. Whether the shared fixture boot in `tests/blitz-tests/tests/fixtures/mod.rs` goes through that default is research's question.
- Text is painted only through a font context that resolves a `sans-serif` family. A driver `screenshot` of an app with no such family is refused `fonts-unavailable`; text is never silently skipped (per design-system §Typography — Loading). A fixture page that screenshots or lints its `select` needs the bundled face or the system fonts.
- The engine's default focus colour for an input is the outline `#4D90FE` (per design-system §Color Palette — Border Progression). If the select's focus is painted, it follows that default, and the chunk adds no new focus colour.
- The 7GUIs stand's colours are measured row for row against their declared values: the accent `#3a5ce5`, the page `#f5f5f5` and the text `#1a1a1a` (per design-system §Color Palette — Core Colors). The scope says the stand gains no `select`, so the chunk moves no stand colour.
- `escher-session` is a surface an agent reads. Every answer is one uncoloured line of JSON on stdout: no ESC byte, no terminal test, and the exit status carries the class. The `mcp` role's replies are single-line JSON-RPC with no ESC byte (per design-system §Surface: cli — Component Patterns). Any answer to choosing an option must follow this, whether it comes from an existing verb or a new one.

## Patterns to follow
- The range input's precedent: an engine form control with a current value of its own, driven by the existing verbs and read in the snapshot's `value`. It paints through the engine's default control styling and adds no styling of its own (per design-system §Color Palette — Engine-painted defaults, where the engine's form controls take their colours from engine defaults, not tokens).
- A fixture page that sets its own `font-family: sans-serif` body, like the repository's example fixtures (per design-system §Typography — Example fixtures), so its text measures and paints with the bundled face.
- New CLI or MCP answer content uses the schema's own words in one JSON line, as `help`, `screenshot` and the session errors already do (per design-system §Surface: cli — Component Patterns).

## Anti-patterns to avoid
- No colour, ESC byte or terminal-dependent output on any driver answer line, CLI or MCP (per design-system §Surface: cli — Component Patterns).
- Do not paint a screen while skipping its text when no `sans-serif` family resolves. Refuse `fonts-unavailable` before anything is painted (per design-system §Typography — Loading).
- Make no stand colour or stand CSS change on this chunk's account: the stand's measured colour rows are a record (per design-system §Color Palette — Core Colors).

## Contract bindings
- Token contrast ↔ a11y SC 1.4.3. Each row in `stand_contrast` is asserted against its §Color Palette value (per design-system §Color Palette — Core Colors). A select added to the stand would add rows; the fixture-only proof adds none.
- Default focus outline ↔ a11y SC 2.4.3 and layouts' focus order. The select's place in the Tab sequence belongs to layouts and a11y; design binds only the `#4D90FE` default outline (per design-system §Color Palette — Border Progression).
- CLI and MCP no-ESC reading ↔ the tests harness. On every answer line, the seven_guis checks' shared reader and the shared `Speaker` assert that no ESC byte appears (per design-system §Surface: cli — Component Patterns).
- Font loading ↔ the driver's `fonts-unavailable` refusal and the lint's text reads (per design-system §Typography — Loading).

## Acceptance criteria contributions
- (design) Any driver answer this chunk adds or reaches for a `select`, on the command line or over MCP, reads as one line with no ESC byte, as asserted by the shared reader and `Speaker` (per design-system §Surface: cli — Component Patterns).
- (design) The 7GUIs stand's measured colour rows are unchanged, and `stand_contrast` passes with its `TEXT_ROWS` and `STATE_ROWS` unmoved (per design-system §Color Palette — Core Colors).
- (design) The select fixture boots with a font context that resolves `sans-serif`. Any screenshot or lint of it is not refused `fonts-unavailable` (per design-system §Typography — Loading).
- (design) The fixture's `select` adds no hardcoded colour or token of its own beyond the engine's user-agent defaults, unless the fixture needs one and states why (per design-system §Surface: desktop-native — Tokens).
