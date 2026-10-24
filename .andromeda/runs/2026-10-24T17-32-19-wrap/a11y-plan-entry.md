
## 2026-10-24-stand-requirement-sweep — three a11y witnesses and `select_interaction` in `stand-repeat`
**Section:** §3 A11y Assertion Harness Contract (body: the state check; the merged-states · inert · active-descendant bullet) · §5 Keyboard Navigation (keyboard-navigation.md: the select key and typeahead check) · §9 CI Integration (ci-integration.md)
**Change:** `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread` run three rounds in the `stand-repeat` leg by its named `--test` list (was «the workspace test leg and per package» alone), `accessibility_merged_states` not; `select_interaction` too, beside `stand_act_select` by its prefix; their membership is held by `stand_requirement_sweep`'s leg check (test-plan §1); the a11y leg's six files and its job are unchanged, CI#40114977936 reading 6 · 43.
**Why:** the founder's own word «Список в скрипте (Рекомендую)» (2026-10-24, relayed verbatim by the overseer, chunk inputs#I1).
**Kept:** the a11y leg's six files.
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/
