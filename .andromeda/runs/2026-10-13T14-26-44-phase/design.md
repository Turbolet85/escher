# design extract

## Relevance
partial — the chunk builds no UI and adds no token, but the upstream delta changes engine-painted and UA-stylesheet behaviour the design plan records (button UA styles #1175, image painting #1194/#1195 in `render.rs`, shared UA stylesheets #1189, hover end #1176, cursor timing #1177), and the merge must keep the agent-read CLI/MCP surface as it stands.

## Constraints
- The blitz-dom default stylesheet carries recorded engine values: link `rgb(0, 0, 238)` on `a[href]`, input focus outline `#4D90FE`, button background `#EFEFEF` (per design-system §Color Palette, Core Colors). #1175 edits the UA flex centering of buttons in that stylesheet. Whether it also moves any of these values is research's question. A moved value is re-read and recorded at the wrap, never patched back by us.
- The default button radius `1px` (default.css:103) and the form-control font `system-ui, sans-serif` are recorded engine defaults (per design-system §Border Radius; per design-system §Typography). #1175 touches the same button rules, so their survival is read after the merge, not assumed.
- The canvas background comes from the root element's background (body's when the root is transparent), and the engine paints its own scrollbar-thumb palette per colour scheme (per design-system §Color Palette, Engine-painted defaults, citing `render.rs` ranges). Upstream's `render.rs` edit (+77 −19) lands in that file, so whether those paths and line anchors move is research's question.
- The stand paints with the bundled DejaVu Sans for every generic font, system fonts off, through `build_single_font_ctx`. A driver `screenshot` is refused `fonts-unavailable` when `can_paint_text` reads false (per design-system §Typography, Loading). The merge must keep the stand's text paintable and the `document.rs` reader intact.
- Stacking-order paint (negative-z, paint children, positive-z) and the hit that stops at the padding box of an `overflow`-clipping box are recorded engine depth behaviour (per design-system §Depth Strategy, Engine depth behavior). Upstream edits `node/node.rs` (+30 −6) and `render.rs`, and both must still hold after the merge.
- Overlay scrollbars keep their timings: `FADE_DELAY` 500 ms, `FADE_DURATION` 200 ms, thumb appearance changing on hover and drag, a hidden thumb not summoned by hover. The harness clock is never advanced by settle (per design-system §Motion, This project's values / Animation runtime). #1176 (hover ended on pointer leave) changes hover state, so whether it meets the thumb-hover reading is research's question.
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet (per design-system §Surface: desktop-native, Tokens). #1189 shares the parsed UA stylesheets and their cascade data across documents. The stand boots fresh per boot and paints the same per process; whether sharing keeps that is research's question.

## Patterns to follow
- Cursor resolution goes CSS `cursor` keyword → `CursorIcon`, with Text / Pointer / Default fallbacks, through `ShellProvider::set_cursor` (per design-system §Iconography, Cursors and favicons). #1177 changes when the cursor is set after a sub-document handled the event. The `cursor-icon` dev-dependency it brings is taken as upstream's, and the mapping is not reworked by us.
- The stand stays deterministic: pinned viewport, bundled font, system fonts off (per design-system §Typography, Loading). This is what lets a pixel or bounds change from Taffy 0.15 or #1175 be read as a real change and re-measured.
- Engine-painted values belong to upstream's `default.css` and `render.rs` (per design-system §Color Palette, Engine-painted defaults). The merge takes upstream's side as one set and adds no hardcoded colour or size of ours.

## Anti-patterns to avoid
- No colour, no colour dependency, no ESC byte and no terminal test enters `escher-session`'s answers or its `mcp` role's replies through any merge resolution (per design-system §Surface: cli, Component Patterns).
- Do not hand-edit an upstream UA-stylesheet or paint value (button styles, focus outline, scrollbar palette) to bring back an old stand pixel or bound. A moved value is recorded, not reverted (per design-system §Color Palette, Core Colors).

## Contract bindings
- The focus outline `#4D90FE` and the button background `#EFEFEF` bind to a11y SC 1.4.3 contrast; painted colours are a11y surface (CLAUDE.md Critical Warnings). Any value #1175 moves gets a contrast re-read (design-system §Color Palette ↔ a11y-plan).
- Stand pixels bind to the tests harness. A whole `screenshot` is byte-identical across processes and equals the harness raster (`stand_screenshot_repeat.rs`), so a change from Taffy 0.15 / #1175 / #1189 shows up there first (design-system §Typography Loading ↔ test-plan).
- Scrollbar hover and fade bind to `tests/blitz-tests/tests/scrollbars.rs` and `scrollbar_drag.rs`, the checks behind the §Motion and §Color Palette scrollbar readings. #1176 is the delta that meets them (design-system §Motion ↔ test-plan).
- The CLI no-ESC reading binds to the seven_guis checks' shared answer reader and the MCP `Speaker`, which assert no ESC byte on every line (design-system §Surface: cli ↔ test-plan).

## Acceptance criteria contributions
- After the merge, every `escher-session` answer line and every `mcp` reply still carries no ESC byte and is uncoloured. The standing seven_guis CLI and MCP checks pass (per design-system §Surface: cli).
- After the merge, the stand's `screenshot` is not refused `fonts-unavailable`, and a whole shot stays byte-identical across two processes (per design-system §Typography).
- `scrollbars.rs` and `scrollbar_drag.rs` pass on the merged tree: thumb hover/drag appearance holds, and a hidden thumb is not summoned by hover (per design-system §Motion).
- The chunk report lists each design-plan-cited `default.css` or `render.rs` value or path the merge changes (button UA styles, focus outline, button radius, canvas-background or scrollbar palette paths), or states that none changed. This list feeds the wrap's design-system amendment (per design-system §Color Palette).
