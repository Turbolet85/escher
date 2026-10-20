
## 2026-10-19-visual-lint — `text_contrast`'s first non-test caller
**Section:** §3 Contrast verification harness · §6 Color contrast
**Change:**
- §3: `Harness::text_contrast` was read by tests alone; its first non-test caller is now escher-driver's in-process visual lint (`Session::lint`, PROVISIONAL: in process only), whose `contrast` reading keeps the same exempt rule.
- §6: a new bullet — the lint's `contrast` reading: a node with direct non-whitespace text or a text control, read through `text_contrast` and reported below `required_ratio()`, exempt exactly where `enabled` reads `Some(false)`, `None` not measured, no second formula or threshold. The stand at boot reads no `contrast` finding on the four lean tasks in both layout modes, the nodes outside `stand_contrast`'s rows included; a planted low-contrast row reads 2.8490272 against 4.5, equal to `text_contrast`'s; the `a11y` leg reads 43 tests over 6 targets, unmoved.
**Why:** this chunk's own reader of SC 1.4.3 (the plan's reviewed expected amendment); the lint's in-process scope is PROVISIONAL for the founder's own word (delegate the operator, 2026-10-20).
**Ref:** .andromeda/runs/2026-10-20T00-54-29-wrap/
