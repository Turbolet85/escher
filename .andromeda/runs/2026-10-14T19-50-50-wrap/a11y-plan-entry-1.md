
## 2026-10-14-stand-a11y-assertions — SC 1.4.3 met on the stand, three contrast tables, the a11y leg at six files
**Section:** §3 Contrast verification harness, A11y testing tool pick · §6 Color contrast and its NOT YET MEASURED note · §9 CI Integration
**Change:**
- §6: the stand's accent one step darker (rest `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`; white 5.47 · 6.83 · 8.27). `stand_contrast`'s record now 14 · 0 · 3 · 1, the five accent controls at 5.47 (was 9 · 5 · 3 · 1, `#4a6cf7` at 4.39). One shared reader classifies two more tables: `TEXT_ROWS` 14 (list rows, text outside the controls, all pass) and `STATE_ROWS` 19 (reached states, hover and active included; 18 pass, 1 exempt). Hover and active are read by pointer; a button with no `:active` rule keeps its hover pair. SC 1.4.3 met on every measured non-exempt row (was "not met on the five accent buttons"). The note no longer lists the list rows, outside text and colour states as unmeasured; the route-entry owner of all four is retired.
- §3: `stand_contrast` states three tables and runs in the `a11y` leg; the state check is "not in the `a11y` leg, which runs six files" (was "the leg staying its three files").
- §9: the leg runs six files — `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` added, the job unchanged; was "it runs existing tests and adds no stand assertion".
**Why:** the re-colour is the operator's F5, a stand change and no engine change; the leg's widening is the founder's F4 «Расширить задание a11y (Рекомендую)», by question dialog in the overseer session, relayed by the overseer as his chosen option label — no new job, no required status check, no repository setting.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/
