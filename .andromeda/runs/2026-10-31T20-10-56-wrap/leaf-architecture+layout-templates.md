group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «fifteen named blitz-tests targets»
  source: .andromeda/architecture-detail/standard-contracts.md:326 «on the same line, seventeen named blitz-tests targets outside that prefix as literal `--test` words»
  replace-with: «seventeen named blitz-tests targets»
- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «The fifteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed,select}.rs`»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «a sixteenth driver-action file, `stand_act_popup`, 12 tests in both layout modes»
  replace-with: «The sixteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed,select,popup}.rs`»
- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «`stand_keyboard` (21 tests, both layout modes,»
  source: .andromeda/architecture-detail/existing-scopes.md:106 «the stand keyboard check `stand_keyboard`, 23 tests in both layout modes»
  replace-with: «`stand_keyboard` (23 tests, both layout modes,»
- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «those two walks stated by `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`;»
  source: .andromeda/architecture-detail/existing-scopes.md:114 «`stand_keyboard`'s twenty-second and twenty-third, the flight type walk holding with its list open and after each close, and a click opening the list with focus staying on the select»
  replace-with: «those two walks stated by `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the twenty-second and twenty-third the flight type walk holding with its list open and after each close, and a click opening the list with focus staying on the select and the walk going on from it;»
- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «(`STATE_ROWS`, 18: 17 pass, 1 exempt»
  source: .andromeda/architecture-detail/existing-scopes.md:115 «(`STATE_ROWS`, 20: 19 pass, 1 exempt»
  replace-with: «(`STATE_ROWS`, 20: 19 pass, 1 exempt»
- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «`overlaps` skipping on either side an option of a closed select, the snapshot giving each such option its select's client rect,»
  source: .andromeda/layout-templates.md:62 «and for an open list's row or group; the snapshot gives a closed select's option its select's client rect and an open list's row its own `bounds`, a `multiple` select's rows linted as usual»
  replace-with: «`overlaps` skipping on either side an option of a closed select and an open list's row or group, the snapshot giving a closed select's option its select's client rect and an open list's row its own `bounds`,»
- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «then `snapshot --part 2`, in `hl-cells`)»
  source: .andromeda/architecture-detail/occupied-resources.md:63 «and for a select's list opened and chosen through the host (examples/seven_guis/tests/host_log.rs:282-393)»
  replace-with: «then `snapshot --part 2`, in `hl-cells`, and for a select's list opened and chosen through the host, in `hl-popup`)»
- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «`found`, `occurrences`, the unix-only `recorded_needles`»
  source: .andromeda/architecture-detail/occupied-resources.md:77 «`found`, `occurrences`, `option_needles` (the needles of a select's options), the unix-only `recorded_needles`»
  replace-with: «`found`, `occurrences`, `option_needles` (the needles of a select's options), the unix-only `recorded_needles`»
- leaf: .claude/docs/services/seven_guis.md:30   class: omission
  leaf-text: «each at the select's client rect.»
  source: .andromeda/layout-templates.md:16 «while it is open each row reads its own `bounds` in the snapshot and the diff, the highlight in neither the snapshot nor the tree, and after a close the rows read back at the select's client rect»
  replace-with: «each at the select's client rect. A click, Enter, or a Space with no typed text standing opens `select#flight-type`'s list, the engine's select list in every document; focus stays on the select and the Tab sequence holds with the list open and after each close, Tab choosing the current row and closing before focus moves; while it is open each row reads its own `bounds` in the snapshot and the diff, the highlight in neither the snapshot nor the tree, and after a close the rows read back at the select's client rect; no stand markup, id, class or style moved.»
