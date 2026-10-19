
## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — the per-sync lock record of the second partial take
**Section:** §Dependency Security (Pinning)
**Change:** a sibling follows the 2026-10-18 record. The 2026-10-19 upstream sync took `38ca4e777` (`:focus-within`, #1226) and `17de049b1` (the WPT pin, #1228) by no-commit cherry-picks (`79603d1ef`), the merge base staying `23b9b26a9`. No dependency enters, leaves or moves: `Cargo.toml`, `Cargo.lock` and `deny.toml` are byte-identical to the tree before it, Taffy is the registry "0.15", and the root manifest has no git dependency. Upstream's four Taffy-main commits (git Taffy `ea07c4c3`) are again not taken. The audit leg reads `advisories ok` and `licenses ok` with `deny.toml` unchanged. The partial take is PROVISIONAL.
**Why:** each upstream sync keeps one lock record. F1 «Cherry-pick two (Recommended)» was answered by the operator as delegate (2026-10-19, by question dialog in the phase session), so the take is PROVISIONAL until the Epoch 4 boundary batch. No dependency moved, so nothing new is reviewed or banned.
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/
