group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
- leaf: .claude/docs/services/escher-driver.md:11   class: stale
  leaf-text: «and `is_multiple_select`, which `execute.rs` `locate` reads, both public and re-exported»
  source: .andromeda/architecture-detail/standard-contracts.md:21 «and `is_select`, the driver's select test, called by `execute.rs` `locate` alone, public and re-exported because escher-driver has no blitz-dom dependency to name `local_name!` with»
  replace-with: «and `is_select`, which `execute.rs` `locate` alone reads, both public and re-exported»
- leaf: .claude/docs/services/escher-driver.md:17   class: stale
  leaf-text: «`overlap` leaves out a closed select's options — `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and `overlaps` skips such an entry on either side»
  source: .andromeda/architecture-detail/standard-contracts.md:226 «The `overlap` reading leaves out a closed select's options and an open list's rows and groups: `Entry.in_select_box` reads a closed select's option through `closed_select_of`»
  replace-with: «`overlap` leaves out a closed select's options and an open list's rows and groups — `Entry.in_select_box` reads a closed select's option through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and is true also for an open list's row or group, and `overlaps` skips such an entry on either side»
- leaf: .claude/docs/services/escher-driver.md:17   class: omission
  leaf-text: «Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint`»
  source: .andromeda/architecture-detail/standard-contracts.md:225 «`clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut»
  replace-with: «Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint`, `clip_region` taking an open row's region from `visible_region` alone, with no `contain: paint` cut»
- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «(the centre of its `bounds` is outside `BaseDocument::visible_region` — the viewport and every scrolling box that holds it;»
  source: .andromeda/architecture.md:149 «a row or a group of an open select list reading the list's scrollport in place of its clipping boxes»
  replace-with: «(the centre of its `bounds` is outside `BaseDocument::visible_region` — the viewport and every scrolling box that holds it, a row or a group of an open select list reading the list's scrollport in place of its clipping boxes;»
- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «`covered` (the hit at that page point never reaches it;»
  source: .andromeda/architecture.md:56 «where a select list stands open a hit answers the list first, with no scrollbar, so a row lies past any box that clips its select»
  replace-with: «`covered` (the hit at that page point never reaches it, where a select list stands open a hit answering the list first, so a button under the list reads `covered`;»
- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «while a `click` on a `multiple` select's row is accepted and chooses it)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:210 «driving the select list through a held driver session: a click opening it and a click on a row choosing»
  replace-with: «while a `click` on a `multiple` select's row is accepted and chooses it, a `click` on a single select opens its list or closes one standing, as `click`'s help says of a select, and a `click` on an open list's row chooses it)»
- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «but for a `type` on a `multiple` select (`Target` carries `multiple`), which does not click, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead (`BaseDocument::clear_select_typeahead`, its one non-test caller), without which a second `type` joins the first's text, and a text opening on a space types nothing, since no label can match and the Space would flip a row»
  source: .andromeda/architecture-detail/standard-contracts.md:202 «`Target` carries `select`, and `type` does not click such a target, a click landing on a `multiple` select's row at the centre and opening a single select's list: the crate-private `focus_select`»
  replace-with: «but for a `type` on a select, single or `multiple` (`Target` carries `select`), which does not click, a click landing on a `multiple` select's row at the centre and opening a single select's list: the crate-private `focus_select` focuses it with no click, so `type` fires no DOM focus events on a single select, clears the typeahead (`BaseDocument::clear_select_typeahead`, its one non-test caller), without which a second `type` joins the first's text, and closes any list (`BaseDocument::close_select_list`), and a text opening on a space types nothing on any select, since no label can match and on a `multiple` select the Space would flip a row»
- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «dispatches its click (the engine's keyboard activation, [Scrolling and input]);»
  source: .andromeda/security-plan-detail/input-validation.md:260 «Enter opens its list, a Space with no typed text standing opens it (`press_select_key`)»
  replace-with: «dispatches its click (the engine's keyboard activation, [Scrolling and input]), and on a focused single select with its list closed a pressed `enter`, or a `space` with no typed text standing, opens its list, whose keys then move the highlight, choose and close, `press`'s help stating the keys on a closed select;»
- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «scrolls every scrolling box that holds the element and then the viewport, at once (`Harness::scroll_into_view`), and answers `in_view`»
  source: .andromeda/architecture-detail/standard-contracts.md:204 «an instant scroll of every scrolling box that holds the element and then of the viewport, a row of an open select list writing that list's `scroll_y` alone»
  replace-with: «scrolls every scrolling box that holds the element and then the viewport, at once (`Harness::scroll_into_view`), a row of an open select list writing that list's `scroll_y` alone, and answers `in_view`»
- leaf: .claude/docs/services/escher-driver.md:29   class: stale
  leaf-text: «whose fifteen `stand_act_*` checks drive the schema through `Session::run`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:112 «the executor's verbs and its refusals are covered by the sixteen `stand_act_*` integration files»
  replace-with: «whose sixteen `stand_act_*` checks drive the schema through `Session::run`»
- leaf: .claude/docs/services/escher-driver.md:32   class: omission
  leaf-text: «with no new verb, carrier, exit, cause, session error, span field or dependency: a ratified boundary widening.»
  source: .andromeda/architecture-detail/established-decisions.md:209 «The select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff»
  replace-with: «with no new verb, carrier, exit, cause, session error, span field or dependency: a ratified boundary widening. Since 2026-10-31-select-popup the select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff — `differs` naming `bounds`, a closed set inside the ratified crossing — and as pixels in a `screenshot`, with no new carrier, exit, cause, session error, node key, log field or span; an option's and a group's label leaving as pixels in every shot of an open list is a boundary widening, ratified.»
- leaf: .claude/docs/services/escher-driver.md:35   class: omission
  leaf-text: «three of the tool's own texts say measured limits — `covered`'s meaning, two readings — a box clipped»
  source: .andromeda/architecture-detail/standard-contracts.md:190 «an option of a closed select lies under it: click the select first, then the option»
  replace-with: «three of the tool's own texts say measured limits — `covered`'s meaning, which also says an option of a closed select lies under it and to click the select first, then the option, two readings — a box clipped»
- leaf: .claude/docs/services/escher-driver.md:36   class: omission
  leaf-text: «`help` exits 0 and `snapshot` with no session 3, neither printing a panic).»
  source: .andromeda/security-plan-detail/error-handling.md:48 «Key press after a hidden ancestor: a key press panics in `position_in_containing_block` (packages/blitz-dom/src/node/node.rs:1775)»
  replace-with: «`help` exits 0 and `snapshot` with no session 3, neither printing a panic). An engine panic stands outside this rule, not fixed, its owner a CARRY on «Upstream sync ahead of the UI graph»: a key press panics in `position_in_containing_block` when the focused control's anonymous block was dropped because an ancestor gained `hidden`, on any control, a remote page able to reach it.»
- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «in blitz-tests — fifteen files — the first twelve»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «the sixteen driver-action files, `Session::run` in each, both layout modes»
  replace-with: «in blitz-tests — sixteen files — the first twelve»
- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «run in both the `a11y` CI leg and the `stand-repeat` leg (by its `stand_` prefix));»
  source: .andromeda/test-plan-detail/test-scope-summary.md:210 «**tests/blitz-tests — `stand_act_popup`:** 12 tests, the sixteenth `stand_act_*` file, each in both layout modes, driving the select list through a held driver session»
  replace-with: «run in both the `a11y` CI leg and the `stand-repeat` leg (by its `stand_` prefix)) and, since 2026-10-31-select-popup, a sixteenth, `stand_act_popup` (the select list through a held session — a click opening it and a click on a row choosing, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option and a button under the list refused `covered`, a row past the scrollport `off-screen` until `scroll` brings it in, `type` leaving no list open, a shot of a row cropping to it and leaving the list open, the lint, the open screen under its budget with every actionable element keyed, two clicks on one point opening then closing, and a read after a step with no `advance`, 12 tests in both layout modes, in the `stand-repeat` leg by its prefix, not among the `a11y` leg's fourteen);»
- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «`stand_lint` (8, declaring»
  source: .andromeda/test-plan-detail/test-scope-summary.md:176 «`stand_lint` 9, declaring `mod fixtures;` and `mod session_common;`»
  replace-with: «`stand_lint` (9, declaring»
- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «the pair of overlapping rows it plants on a `multiple` select reads exactly `fx-row-a` × `fx-row-b`;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:216 «an open list's rows read no overlap on the stand, in both layout modes (`an_open_lists_rows_read_no_overlap_on_the_stand`»
  replace-with: «the pair of overlapping rows it plants on a `multiple` select reads exactly `fx-row-a` × `fx-row-b`; an open list's rows read no overlap on the stand (`an_open_lists_rows_read_no_overlap_on_the_stand`);»
- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «`stand_session_quiet`'s host installs no log sink, so it proves nothing about a host that does — `host_log` is the check that does.»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_virtual_clock` 2 — `Session::run` in both layout modes: a shot after a scroll repeats until `advance` moves the fade, and two clicks with no `advance` between them are a double click»
  replace-with: «`stand_virtual_clock` (2, `Session::run` in both layout modes: a shot after a scroll repeats until `advance` moves the fade, and two clicks with no `advance` between them are a double click; ratified by the founder, 2026-10-18). `stand_session_quiet`'s host installs no log sink, so it proves nothing about a host that does — `host_log` is the check that does.»
- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «exiting 1 with a line equal to `Refusal::malformed(fault).to_json()`, nothing typed written back)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:279 «seven_guis' `cli_commands` `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in `cc-popup`, the hosted shot equal to the in-process one»
  replace-with: «exiting 1 with a line equal to `Refusal::malformed(fault).to_json()`, nothing typed written back; since 2026-10-31-select-popup `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in `cc-popup`, the hosted shot equal to the in-process one)»
- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «its needles from `common::cells_needles(part)`)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «unix only, a select list opened and chosen through the host, `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` — 0 option needles at `trace`»
  replace-with: «its needles from `common::cells_needles(part)`; and a fifth, unix only, a select list opened and chosen through the host in `hl-popup`, `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` — 0 option needles at `trace`)»
- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «a second test, a recording `mcp` role that logs nothing of its checks)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «a select list opened and chosen through the role, `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing — 0 option needles at `trace`»
  replace-with: «a second test, a recording `mcp` role that logs nothing of its checks; and a third, a select list opened and chosen through the role, `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing — 0 option needles at `trace`)»
- leaf: .claude/docs/services/escher-driver.md:63   class: tree
  leaf-text: «`tests/flows/record_checks.sh` run through the binary»
  source: .andromeda/architecture-detail/occupied-resources.md:53 «`examples/seven_guis/tests/flows/record_checks.sh`»
  replace-with: «`examples/seven_guis/tests/flows/record_checks.sh` run through the binary»
