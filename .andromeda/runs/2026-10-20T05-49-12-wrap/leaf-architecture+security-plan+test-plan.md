group: architecture+security-plan+test-plan
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:6   class: retired
  leaf-text: «(`Session::lint`, PROVISIONAL: in process only)»
  source: .andromeda/architecture.md:202 «The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part):»
  replace-with: «(`Session::lint`, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part))»

- leaf: .claude/docs/services/escher-driver.md:17   class: retired
  leaf-text: «(since 2026-10-19-visual-lint; in process only, PROVISIONAL)»
  source: .andromeda/architecture.md:202 «The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part):»
  replace-with: «(since 2026-10-19-visual-lint; in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part))»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «and with an empty text deletes the selection, so an empty text clears;»
  source: .andromeda/architecture.md:196 «says that on a select the text chooses the first option whose label starts with it (packages/escher-driver/src/schema.rs:208-209)»
  replace-with: «and with an empty text deletes the selection, so an empty text clears; on a select the text is the engine's typeahead, choosing the first option whose label starts with it, as `type`'s help says;»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «`covered` (the hit at that page point never reaches it).»
  source: .andromeda/architecture.md:92 «an option is not clicked (a driver `click` on one is refused `covered`)»
  replace-with: «`covered` (the hit at that page point never reaches it; a select's option is not clicked — a `click` on one is refused `covered`).»

- leaf: .claude/docs/services/escher-driver.md:29   class: stale
  leaf-text: «whose fourteen `stand_act_*` checks drive the schema»
  source: .andromeda/test-plan.md:22 «covered by the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)»
  replace-with: «whose fifteen `stand_act_*` checks drive the schema»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «in blitz-tests — fourteen files — the first twelve»
  source: .andromeda/test-plan.md:22 «covered by the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)»
  replace-with: «in blitz-tests — fifteen files — the first twelve»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «2 tests in both layout modes); beside them `stand_screenshot_repeat`»
  source: .andromeda/architecture.md:359 «a fifteenth, `stand_act_select`, a select driven by id — at boot its node, `ComboBox` with its name and value and its options as its children, `type` with a label choosing that option,»
  replace-with: «2 tests in both layout modes) and, since 2026-10-20-select-by-id, a fifteenth, `stand_act_select` (a select driven by id — at boot its node, `ComboBox` with its name and value and its options as its children, `type` with a label choosing that option, a click then a key, the Tab walk skipping a disabled select, a disabled select and an unchosen option refused, 5 tests in both layout modes); beside them `stand_screenshot_repeat`»
