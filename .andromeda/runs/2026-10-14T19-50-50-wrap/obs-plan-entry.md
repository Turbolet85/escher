
## 2026-10-14-stand-a11y-assertions — the silence census over the extended checks
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand census)
**Change:** `stand_contrast` and `stand_keyboard` with their new tests, and `stand_accessibility_ids` (its three tests reading the tree through `&dyn blitz_dom::Document`), each still install no subscriber, read no env var, print nothing and bind no listener; the silence probe reads 0 over the three files and the shared module. The shared module's reader count stays 19.
**Why:** the chunk added tests to three checks and none installs a sink or prints; `stand_accessibility_ids` was not named in the census before.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/
