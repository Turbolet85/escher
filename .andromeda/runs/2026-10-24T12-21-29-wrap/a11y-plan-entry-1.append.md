
## 2026-10-24-upstream-sync-polish-and-ship — the dropped-node hit skip, and the stand graph re-read
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: Dioxus crates) · §6 Visual Design Verification (visual-design-verification.md: Visibility and hit-testing)
**Change:**
- Visibility and hit-testing: a hit also skips a node dropped since the last resolve, pointing to architecture §Standard Contracts → Scrolling, selection, tree; the `inert` hit cite re-pointed (`node.rs:1437-1446`).
- Dioxus crates: re-read on the upstream take read at `f9cbb2756` and `tokio` 1.54.1 — no package enters or leaves `Cargo.lock`, and the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit`, so no stand binary builds a platform adapter.
**Why:** the hit skip is an upstream engine change for every document, ratified by the founder's own word «Десять коммитов (Рекомендую)», relayed verbatim by the overseer (2026-10-24, chunk inputs#I4); the adapter's absence from the stand is re-read at each lock change.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/
