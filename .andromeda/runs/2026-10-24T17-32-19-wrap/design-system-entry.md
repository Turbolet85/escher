
## 2026-10-24-stand-requirement-sweep — the list-bullet font's third citation re-pointed
**Section:** §Typography (typography.md: List bullets)
**Change:** was `packages/blitz-dom/src/layout/construct.rs:1106-1112`; now `packages/blitz-dom/src/layout/list.rs:96-102` — the marker's text pushed into the bullet layout; the row's font stack and its other two citations unchanged.
**Why:** the citation sweep read the old range holding none of the row's claim (its row dispositioned «claim false»), and the marker's push into the bullet layout read at `list.rs:96-102`.
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/
