
## 2026-10-14-quality-gates — the stand's a11y files also run in `stand-repeat`
**Section:** §9 CI Integration
**Change:** `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, as `stand_*` files, also run three times in the `stand-repeat` leg and its job `Stand checks repeated`, under the zero-failure budget (test-plan §10). The a11y leg and job are unchanged, their per-file counts as before.
**Why:** the stand-repeat leg runs every `stand_*.rs` file, on the founder's «Повторное задание, ноль сбоев (Рекомендую)», relayed verbatim by the overseer as his chosen option label.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/
