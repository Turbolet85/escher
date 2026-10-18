group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «the thirteen `stand_act_*` checks — `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot —»
  source: .andromeda/obs-plan.md:70 «the fourteen driver-action checks `stand_act_*` (the two children of `stand_act_spans` aside; […] the thirteenth, `stand_act_screenshot`, since 2026-10-12-headless-screenshot; the fourteenth, `stand_act_transformed`)»
  replace-with: «the fourteen `stand_act_*` checks — `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth —»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the builder's three state reads hold no log site, and the command span keeps its eight fields with none for a state.»
  source: .andromeda/obs-plan.md:74 «- The engine-debts checks are silent: the five new checks `accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `transformed_client_rect` and the driver check `stan»
  replace-with: «the builder's three state reads hold no log site, and the command span keeps its eight fields with none for a state; the engine-debts checks `accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `transformed_client_rect` likewise install no subscriber, read no env var and print nothing, their readings going into assertions only, and the seven engine changes they pin (the `inert` read, the toggle read per role, the password click caret, the transformed client rect, `Node::absolute_position`, the hit's stop at a `contain: paint` box, `ElementData::is_disabled`) and the driver's reworded fixed texts add no log site, span, field, log field or env read.»
