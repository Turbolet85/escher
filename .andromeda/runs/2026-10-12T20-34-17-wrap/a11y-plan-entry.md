
## 2026-10-12-headless-screenshot — a password painted masked; the shot one more reader of bounds
**Section:** §6 Visual Design Verification → Focus ring · §7 Screen Reader Support → Accessibility tree output (the snapshot model's readers)
**Change:**
- §7: was a password's typed text in no field of the snapshot or tree; now in no painted frame either — bullets laid out by `BaseDocument::layout_as_text_input`, in every document and renderer through blitz-paint, windowed included; equal-length values paint identical frames, focused or not, both layout modes; the snapshot still reads `MASKED_VALUE`, the tree unchanged; a click inside a filled field places the caret by the real glyph widths — not measured, owned by a CARRY on "Stand requirement sweep"; the windowed paint not run here.
- §7: the driver's `screenshot` is one more reader of the model's bounds and of nothing else — an element shot by stable id is its bounds rounded outward and cropped to what can be seen — and a shot changes nothing: no focus moves, `has_changes` stays false, the next diff is empty, both layout modes.
- §6: a focused password input paints its caret at the bullet of its index in the input's caret colour (`caret_color`), its bullets in the text colour, the field scrolling on the bullets.
**Why:** painted colours, the caret and focus are a11y surface; the mask is the founder's fork 2, an engine change for every document ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer).
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/
