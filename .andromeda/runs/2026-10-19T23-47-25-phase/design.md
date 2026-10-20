# design extract

## Relevance
partial — the chunk adds no styled UI. Its design touchpoints are three: the agent-read `escher-session` / `mcp` output surface its new command answers on, the stand's recorded colour pairs that its contrast reading must agree with, and the engine's clip and paint-order behaviour that its `clipped` and `overlap` readings rest on.

## Constraints
- Per design-system §Surface: cli (Component Patterns), `escher-session` is a surface an agent reads, not a person. A command prints exactly one line of JSON on stdout, keyed by the driver schema's own words, and nothing else there. It uses no colour and no colour dependency, writes no ESC byte on any answer line, and runs no terminal test. A lint answer (its findings, ids and reasons) falls under this rule.
- Per design-system §Surface: cli (Component Patterns), the exit status carries the class `0` · `1` · `2` · `3`, read from the schema's `EXIT_STATUSES` table that `help` serves. A lint command adds no new exit. Its refusals and session errors take the existing classes.
- Per design-system §Surface: cli (Component Patterns), every reply of the `mcp` role is one uncoloured newline-delimited JSON-RPC line with no ESC byte. Whatever content blocks a lint tool result carries keep that shape.
- Per design-system §Color Palette (Core Colors, seven_guis accent row), the stand's measured pairs are recorded as passing SC 1.4.3. They are white `#ffffff` text on the five accent controls' `#3a5ce5` at 5.47 against 4.5, on the hover shade `#2a4cd3` at 6.83 and on the active shade `#1f3fc1` at 8.27, and every measured stand colour equals its declared value. A lint contrast reading at stand boot must agree with those recorded pairs.
- Per design-system §Typography, the stand's text is sans-serif at a 14px base (home title 36px, weight 700), and the headless stand paints with bundled DejaVu Sans with system fonts off. Large-text classification for `TextContrast::required_ratio` (3.0 vs 4.5) depends on these sizes and weights. A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` (§Typography, Loading). Whether a lint of text, contrast or truncation needs the same gate is research's question.
- Per design-system §Depth Strategy (Engine depth behavior), a node is a stacking-context root under named conditions (opacity, position, z-index, transforms, paint effects, isolation). Children paint in stacking order (negative z, paint children, positive z), and hit-testing walks the reverse. A hit stops at the padding box of a node paint clips by `overflow` or by `contain: paint`, the root element excepted. An `overlap` reading ("painted over another") and a `clipped` reading must follow this order and this clip, not raw tree order.
- Per design-system §Depth Strategy (Observed values), the stand's cards carry `box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08)` (the circle dialog 0.10). A shadow is paint outside the border box and does not by itself make two boxes overlap. Whether the lint's box reading includes it is research's question.

## Patterns to follow
- The `screenshot` verb's output shape on both carriers (design-system §Surface: cli, Component Patterns). It is one uncoloured line on stdout with status `0`, and on MCP a text block holding the JSON line. A read-only verb added beside it follows the same shape.
- `help`'s self-description and a session error's fixed `remedy` (design-system §Surface: cli, Component Patterns). Fixed table text serves every new word a lint answer introduces, such as a finding's defect class.
- The stand's contrast record (design-system §Color Palette, seven_guis accent row): a measured ratio for each declared colour pair in each reached state. The lint's contrast reason states its measured ratio and required ratio the same way.

## Anti-patterns to avoid
- Adding colour, a colour crate, an ESC byte or a terminal test (`is_terminal`, `supports_*`) to colour or format lint findings for a human reader (design-system §Surface: cli, Component Patterns).
- Changing a stand colour, size or weight to make the lint read clean at boot. The plan records the stand's colours as measured and passing (design-system §Color Palette, Core Colors), and this chunk reports defects and fixes none.

## Contract bindings
- Token contrast ↔ a11y: a lint `contrast` finding is a reading of SC 1.4.3, and `stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS` hold the stand's recorded pairs (design-system §Color Palette ↔ a11y-plan §Contrast). A stand-boot lint that disagrees with them is a fault in one of the two.
- CLI surface ↔ the driver schema (arch §Standard Contracts, Driver session): the no-colour / one-line / `EXIT_STATUSES` rule of design-system §Surface: cli binds the new verb's `json` written form, its `cli` ending and its `mcp/result` blocks.
- Font availability ↔ harness screenshot and contrast (design-system §Typography, Loading ↔ `blitz-test-harness`): the bundled DejaVu Sans is what makes the stand's text measurable headlessly.

## Acceptance criteria contributions
- (design) `escher-session lint` writes exactly one line of JSON on stdout with no ESC byte. Its status is one of the existing `EXIT_STATUSES` (0 · 1 · 2 · 3), and no colour dependency enters the workspace (per design-system §Surface: cli).
- (design) Over the `mcp` role, a lint tool call's reply is one line with no ESC byte (per design-system §Surface: cli).
- (design) At stand boot the lint reports no `contrast` finding on any accent control whose pair is recorded as passing: white on `#3a5ce5` at 5.47 (per design-system §Color Palette).
- (design) On the planted-defect fixture, an element placed under a box that clips by `overflow` or `contain: paint` reads `clipped`. A sibling that paints over another in stacking order reads `overlap`, and an element that only holds the other does not (per design-system §Depth Strategy).
