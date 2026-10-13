
## 2026-10-12-headless-screenshot — the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot` and the MCP image block
**Section:** §Standard Contracts → Driver session (escher-driver) · CLIs
**Change:**
- Was ten verbs, six asked of the instance; now eleven, seven: `screenshot` after `scroll` — one optional `id` (`ArgKind::Id`), result `width` · `height` (counts) and `png` (`FieldKind::Png`, base64 text), all always present; its help states two limits (an element shot is cropped to what can be seen; a thumb painted within 0.7 s of a scroll fades by wall time where overlay scrollbars are painted). `FieldKind` nine → ten. `Command` gains `Screenshot { id: Option<String> }`; `describe()` counts 11 verbs and 9 causes.
- Was `Cause` closed at eight, the executor returning six; now nine, seven: `fonts-unavailable` last, from `screenshot` alone, checked before anything is painted.
- `Outcome` gains `Shot { width, height, png: Vec<u8> }`. `screenshot` dispatches nothing and is no step; given an `id` it resolves it as `click` does, crops the frame to the element's visible part rounded outward (private `crop`), a crop of no pixel refused `off-screen`; a font context with no `sans-serif` family (`can_paint_text` false) is refused `fonts-unavailable`; the frame is the harness's, written as a PNG by the private `image::png`; a shot changes nothing. On the stand a whole shot is an 800 × 600 PNG equal to the harness's raster.
- Written forms gain the shot's line `{"width","height","png"}`, its base64 hand-written (`image::base64`, padded) — no base64 crate. Was every MCP result one text block bounded by its text; now an accepted `screenshot` is an `image` block then a text block (the line without `png`), a refused one one text block, and a result holding an image is bounded by its whole written form (`ToolResult::bounded`). `tools/list` eleven.
- The `command` span keeps its eight fields: a shot records `verb` (and `cause`), never the image, its size or dimensions; `tool_call.tool` reads `screenshot`.
- CLIs: `escher-session screenshot [--id]`.
**Why:** the chunk built the headless screenshot; the image in band is the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13).
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/
