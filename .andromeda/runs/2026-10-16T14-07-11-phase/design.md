# design extract

## Relevance
partial — the chunk adds no rendered surface and no token. Design's stake is narrower: the handler split must leave the stand's measured colour and focus states where they are, and the README's prose must not misstate the CLI surface.

## Constraints
- design-system §Color Palette (Core Colors, seven_guis accent row) records the accent `#3a5ce5` with hover `#2a4cd3` and active `#1f3fc1`, and makes each a reached colour state of `stand_contrast`. It also requires every measured stand colour to equal its declared value. So splitting `handle_pointerdown` / `handle_click` must leave every hover and active state the stand reaches, and its colour, unchanged. Whether those states are set inside the arms being extracted is a question for research.
- design-system §Color Palette (Semantic Colors) records the flight booker's invalid (`#e53e3e` / `#fff5f5` / `#c53030`) and success (`#ebf8ee` / `#68d391` / `#276749`) colours. A refactor of the click and keyboard-activation arms must not change which of these states a click, an Enter or a Space reaches.
- design-system §Border Progression records the blitz-dom default focus outline for inputs, `#4D90FE` (`packages/blitz-dom/assets/default.css`). The button-focus arm and the range input's click-focus decide which element takes that styling, so after the split, focus lands where it did before. Whether `default.css` styles a focused `button` at all is a question for research.
- design-system §Surface: cli §Component Patterns describes `escher-session` as a surface an agent reads:
  - one uncoloured line of JSON on stdout, with no ESC byte;
  - exit statuses `0` accepted · `1` refused · `2` usage · `3` session error;
  - `help` as the self-description;
  - the `mcp` role writing JSON-RPC lines only.

  Any README sentence or Try-it command that describes this output must agree with that record. Whether the README states any of it today is a question for research.

## Patterns to follow
- Measured colour is the design record (design-system §Color Palette, Core Colors). The stand's colours are pinned by `stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS`, so a behaviour-preserving refactor should move no row and no declared value.
- Name the CLI surface in the schema's own words (design-system §Surface: cli §Component Patterns). README prose about the driver should use the verb names and the exit-status meanings the self-description serves, not a paraphrase that drifts from them.

## Anti-patterns to avoid
- Do not change a stand colour, or edit the `stand_contrast` tables, to get a refactor through. A moved row means a behaviour change, and that belongs to the founder (design-system §Color Palette, Core Colors).
- Do not describe the CLI output in the README as coloured, multi-line, or dependent on whether it runs in a terminal (design-system §Surface: cli §Component Patterns).

## Contract bindings
- design §Color Palette ↔ a11y SC 1.4.3. `stand_contrast` asserts the accent's text/background pairs and the hover and active states the pointer handlers reach.
- design §Border Progression (focus outline) ↔ a11y SC 2.4.3. Through `stand_keyboard`'s Tab tables (`common::tab_order`, `common::ENABLED_TAB_ORDER`), a button click's focus and keyboard activation are a11y surface.
- design §Surface: cli ↔ the README's prose. The README restates a surface whose shape design-system records.

## Acceptance criteria contributions
- `stand_contrast` passes unchanged in both layout modes, with `TEXT_ROWS` and `STATE_ROWS` not edited by the chunk's diff (per design-system §Color Palette).
- The chunk's diff changes no colour value in `packages/blitz-dom/assets/default.css` or in any `examples/seven_guis/src/tasks/*.rs` style block (per design-system §Border Progression).
- Every README statement about `escher-session` output matches the recorded surface: one uncoloured JSON line on stdout and exit `0` · `1` · `2` · `3` (per design-system §Surface: cli §Component Patterns).
