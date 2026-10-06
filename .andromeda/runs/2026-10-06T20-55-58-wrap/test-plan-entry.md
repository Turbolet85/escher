
## 2026-10-06-id-stability-across-code-edits — counts and coverage after the anchored tier and the actionable-key check
**Section:** §1 Coverage scope → dioxus-native-dom · → dioxus-native and stylo_taffy · → tests/blitz-tests · §2 Test levels observed · → Test function naming · §3 Crate-local test helpers · → Agent-run contract → Proof · §5 Boundaries covered · §7 Builders and options · §9 Local baseline
**Change:**
- §1 unit census: was 18 unit tests in four files, six on the stable element id; now 29 in five — thirteen on the id (seven of them on the anchored tier), four on `unkeyed_actionable` in the new `actionable.rs`, nine on the snapshot, one document test, two touch tests. The `#[test]` census names `actionable.rs`.
- §1 stand coverage gains `stand_id_edits` (5 checks: the ids held across code edits) and `stand_actionable_keys` (3 checks: every actionable element keyed; the three other tasks pinned at 2 · 3 · 676); stand files 9 → 11.
- §3 Proof chain, one link: `run stand` 33 → 41 `ok`.
- §9 baseline chain, one link: 471 · 0 · 5 over 125 result lines → 490 · 0 · 5 over 127.
- Ten line citations re-pointed (`dioxus_document.rs`, `element_id.rs`).
**Why:** the chunk's own measured counts; earlier links of both chains stay as written.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/
