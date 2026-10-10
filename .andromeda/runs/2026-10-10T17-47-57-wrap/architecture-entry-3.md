
## 2026-10-10-snapshot-states — scope rows: twelve driver-action checks, seventeen readers, a third accessibility check
**Section:** §Conventions → Tests · §Existing Scopes → blitz-tests
**Change:**
- Conventions → Tests: was "the eleven driver-action checks `stand_act_*`"; now twelve.
- blitz-tests: was "the eleven files … 37 tests and two ignored children"; now twelve files, `stand_act_states` the twelfth, 40 tests and two ignored children, the covered behaviours gaining a step that returns the node whose pressed, selected or invalid reading it changed. `mod session_common;` was read by sixteen checks, now seventeen; `mod common;` stays at 17.
- blitz-tests, accessibility coverage: gains `accessibility_states` — the builder's three state attributes on a parsed page, every value of each grammar, on a role that supports the attribute and one that does not, `aria-checked` setting none, 4 tests in both layout modes. It runs in the workspace test leg and per package; the `a11y` CI leg stays its three named files.
**Why:** the chunk added two check files and no manifest line; each count is a measured reading of the tree.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/
