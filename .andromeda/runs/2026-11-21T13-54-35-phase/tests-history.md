@history_item n=1 marker=amd-e42t37s6
**amd-e42t37s6** — 2026-10-31-select-popup retired the select limits the list ended, `size` on a `multiple` select read as `size × 1.2em` through `size_rows` (capped 65534, `0` or a bad value hinting nothing, pinned by `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in` and `size_reads_a_whole_number_above_zero_capped`), and KEPT «a single select's `size` stays unread» and a closed single select's option refused `covered` (F9). This chunk retires that kept limit (F8's partial reversal), so the test-scope-summary limits list and the checks that pin it change.

@history_item n=2 marker=amd-uqr4ympz
**amd-uqr4ympz** — 2026-10-31-engine-debts restated the select model's limits as asserted by `select_interaction` (7 → 18) and `stand_act_select` (5 → 10), with «`size` is unread» among them, and test-strategy's two `docs/capabilities.md` bullets. It is the origin of the unread-`size` claim this chunk overturns for a single select, and of the checks that hold the limits.

@history_item n=3 marker=amd-gtfexkqk
**amd-gtfexkqk** — `select_popup` (25 tests, both layout modes, a named `stand-repeat` word): a click, Enter or Space opening the list on a single select; hostile markup (`size`, 2,000 options) keeping every box finite; `size` making a `multiple` select that many rows tall; each open, move, scroll and close asking a redraw. A sized single select must now open no list, so these rows are where a single select with `size` > 1 diverges and the hostile `size` row's meaning moves.

@history_item n=4 marker=amd-7r3js4av
**amd-7r3js4av** — `node/select_list.rs` gained six inline unit tests, among them `size` a whole number above zero, capped, and a scroll staying within the rows and bringing a row in by the least movement; `open_select`, `select_list_bounds`, `select_row_layout` read through `select_popup`, `select_popup_painted`, `stand_act_popup`. `size_rows` is the piece the chunk builds on; its unit-test row is where a single-select reading of it is pinned.

@history_item n=5 marker=amd-di2mgndg
**amd-di2mgndg** — what `select_interaction` pins for a `multiple` select: rows with boxes of their own, a 17-row key table and a 10-row click table, typed text, a disabled select taking nothing; the chosen-row pair `#000000` on `#c8c8c8` at 12.56:1 (`a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair`); `press` with no Ctrl and Ctrl+A not modelled. The sized single select's list box reuses this row model and colour pair; its key and click tables are the precedent for the single list box's.

@history_item n=6 marker=amd-xvxacj7j
**amd-xvxacj7j** — `~ decision/chosen-select-row-colours`: a chosen select row painted `#000000` on `#c8c8c8`, an open list's highlighted row sharing the pair (2026-11-20-select-list-and-selection-debts). A sized single select's chosen row in a list box falls under this row and its guard.

@history_item n=7 marker=amd-3b27q2wc
**amd-3b27q2wc** — `node/select.rs` holds eight inline table tests: one chosen option on a single select, a `multiple` select's others left alone, a step from a given row, a span, the option list through a group. A single list box's key steps rest on these selectedness and step rules.

@history_item n=8 marker=amd-efvgllgo
**amd-efvgllgo** — the standing select checks read through the list: `stand_keyboard` 23 (flight-type walk with the list open), `select_interaction`'s 14-row `KEYS` holding `PageDown` and no Enter because Enter opens the list, `stand_act_select`'s click opening the list. With `size` > 1 Enter/Space/click no longer open a list on a single select, so these tables are where a sized select's keys must be distinguished from the unsized popup path.

@history_item n=9 marker=amd-dqo5clox
**amd-dqo5clox** — `stand_act_popup` (12, both layout modes): a click opening the list, keys, Escape, a closed option `covered`, a row past the scrollport `off-screen` until `scroll`, a shot of a row, the lint on a row. The driver-level precedent for a sized single select's rows (no list opens; rows are real boxes as in a `multiple` select), run by `run stand` and `stand-repeat`, not the a11y leg.

@history_item n=10 marker=amd-ylvtajnx
**amd-ylvtajnx** — `select_popup_painted` (6, both modes): highlight and disabled rows against `SelectListView`'s constants, a closed select's frame byte-equal before and after, a group header's weight, a scrolled list's thumb painted and taking no hit; blitz-paint's `render/select_list.rs` holds no unit test. A list box painted in flow is a paint change beside this check; the frame equalities and the thumb rule bear on it.

@history_item n=11 marker=amd-23w5gd5l
**amd-23w5gd5l** — trap: CI#41233234207 went red on `Test (macos)` alone through `font-embolden` on the select list's painted header; a paint change under that feature needs its local witness (`--features blitz-paint/font-embolden --test select_popup_painted`), and `cargo` stops at the first failing target. Any painted change to select rows here must be witnessed under embolden.

@history_item n=12 marker=amd-yx45xaju
**amd-yx45xaju** — the `embolden` step became the last step of `fast` (`test_fast_leg_ends_on_the_font_embolden_witness`), the local witness of `select_popup_painted`'s header weight. A new painted select check would run under that step if it joins it.

@history_item n=13 marker=amd-xnhp3j2e
**amd-xnhp3j2e** — `~ decision/expected-value-pinned-as-literal` gains the exception: a pixel-row pin that font-embolden moves is pinned as a stated band with its reason where the test crate cannot see the feature. Applies to any pixel-row pin a painted list box adds.

@history_item n=14 marker=amd-3w2vxh7t
**amd-3w2vxh7t** — the select list's pages: `hostile_html` with a hostile `size`; `select_popup`'s `page`, `sized_page`, `long_page` and the rest, `select_popup_painted`'s and `stand_act_popup`'s pages, each in its check's file; `option_needles()` in seven_guis' shared module. A sized single-select page sits beside `sized_page`; shared-vs-in-file placement follows the fixture rules.

@history_item n=15 marker=amd-7lyhp7v2
**amd-7lyhp7v2** — the list paints through `paint_scene` alone, driven headlessly by the harness, the session and the binary, its windowed path read and not run; what an AT reads `unrunnable-here`; stated limits (thumb not draggable, highlight contrast, a larger font clips, highlight in no snapshot) owned by residual rows. Backs `decision/select-list-painted-by-paint-scene`; a list box in flow keeps the headless-only reach.

@history_item n=16 marker=amd-4g5qmthy
**amd-4g5qmthy** — `covered`'s fixed meaning gained «an option of a closed select lies under it, click the select first, then the option» (F9), its help pinned by `schema_tests`, `refusal` and `json`. A sized single select's options are no longer under a closed box, so that text and `contract/refusal-cause/covered` must not refuse them.

@history_item n=17 marker=amd-glk62rg7
**amd-glk62rg7** — the lint skips an entry in a select's box, `Entry.in_select_box` (closed select via `closed_select_of`, or an open list's row); a `multiple` select's rows lint as usual; `stand_lint` 9 pins the stand at `[0, 0, 0, 0]`. Which branch a sized single select's rows take (closed-select skip vs `multiple`-like rows) is a lint reading this chunk must state and pin.

@history_item n=18 marker=amd-bekasacs
**amd-bekasacs** — the lint's closed-select skip was introduced (`stand_lint` 7 → 8, the flight booker's option `overlap` gone, `closed_select_of` read from the element, never the role); the planted overlap became two rows of a `multiple` select. Precedent for how a list box's rows are linted and pinned.

@history_item n=19 marker=amd-wzxmumz2
**amd-wzxmumz2** — the stand's flight booker became a select (`flight-type`, `Role::ComboBox`): shared tables 17 rows, `ENABLED_TAB_ORDER`, `stand_act_ids` options refused `covered` at the select's client rect, `stand_contrast` and `stand_keyboard` rows, the lint's option `overlap` once owned by a residual. The stand's only select is unsized, so its tables should stay unmoved — a regression lead for this chunk.

@history_item n=20 marker=amd-pvesgq4q
**amd-pvesgq4q** — 2026-10-20-select-by-id entered `select_interaction` and `stand_act_select`, the select model's proofs and limits, `type` on a select (`contract/session-run/type-on-select`) proved by `stand_act_select`. Origin of the select checks this chunk extends.

@history_item n=21 marker=amd-7tzdnepx
**amd-7tzdnepx** — `type` focuses any select, single or `multiple`, with no click and no DOM focus event (`Target.select`, `focus_select`); `schema_tests` pins `type`'s help on any select. `type` on a sized single select keeps this path (`contract/driver-session/type-replaces`).

@history_item n=22 marker=amd-nbn2v2aj
**amd-nbn2v2aj** — session-lifecycle: `type` on a `multiple` select focused with no click, since a click lands on the row at the centre; `stand_act_select` 5 → 10. The same reason applies to a sized single select whose centre is a row.

@history_item n=23 marker=amd-r5npmhk4
**amd-r5npmhk4** — the open list through the process checks: `cli_commands`, `host_log` (`hl-popup`, unix) and `mcp_log` each drive a select list, `option_needles()` reading 0 option needles at `trace`. If the sized select is driven through the binary, these are the precedents and needle helper.

@history_item n=24 marker=amd-gn53juc4
**amd-gn53juc4** — `scroll_into_view(node_id)`: a row of an open select list moves the list's `scroll_y` alone (`contract/harness-input/scroll-into-view`). A list box in flow is a scrolling box of real rows; how its rows are brought into view must be stated against this clause.

@history_item n=25 marker=amd-wjjnfnof
**amd-wjjnfnof** — `~ contract/session-run/changed-node-states`: an acting call returns the node with its `selected`, `invalid` or `highlighted` reading, `highlighted` as a press in an open list does. A list box's key moves choose rather than highlight; the diff readings for it rest on this row.

@history_item n=26 marker=amd-laaterv7
**amd-laaterv7** — `+ contract/harness-contrast/highlight-contrast`: `highlight_contrast` answers only for an open list's highlighted row. Bears on whether a list box's chosen row is read by `text_contrast` or this reader.

@history_item n=27 marker=amd-yeoub2u6
**amd-yeoub2u6** — `~ decision/contrast-reader-reads-only`: both contrast readers (an element's text and an open list's highlighted row) read and change nothing.

@history_item n=28 marker=amd-rxg73dpf
**amd-rxg73dpf** — `~ decision/one-contrast-formula`: the lint's contrast reads through the harness's contrast readers, no second formula. A list box's rows read by the lint must go through the same readers.

@history_item n=29 marker=amd-nnpyvs63
**amd-nnpyvs63** — the contrast reader `text_contrast` and `stand_contrast` (WCAG 2.x ratio table, 3.0 / 4.5 threshold), read-only, no span; backs `contract/harness-contrast/text-contrast`, `required-ratio` and `decision/wcag-2-contrast-ratio`.

@history_item n=30 marker=amd-smvshwtp
**amd-smvshwtp** — `text_contrast`'s `None` list gained a `mix-blend-mode` other than `normal` (`contract/text-contrast/no-reading`), no stand row moved.

@history_item n=31 marker=amd-ppyjbwlj
**amd-ppyjbwlj** — `stand_contrast`'s `STATE_ROWS` 18 → 20 reached states with the select list, 19 pass and 1 exempt. A list box's chosen-row state would be another reached state row.

@history_item n=32 marker=amd-2ljhblkm
**amd-2ljhblkm** — the key-press panic past a dropped anonymous block retired (`key_press_after_container_lost_its_box`, 3 tests, both modes, a select's choice moving past the dropped block), `select_popup` taking its key in every row. Keys on a select in a list box share this path.

@history_item n=33 marker=amd-ffczzr5t
**amd-ffczzr5t** — `− decision/key-after-dropped-anonymous-block`: the post-layout fixup now gives focus up, the key acting only after a re-focus. A trap for any select key test that assumes focus stays.

@history_item n=34 marker=amd-3nw7osql
**amd-3nw7osql** — `+ decision/focus-test-pins-where-focus-lands`: a not-focused assertion no longer discriminates after the post-layout fixup; focus tests pin the landing node. Applies to the list box's focus and key checks.

@history_item n=35 marker=amd-jyrjp4tc
**amd-jyrjp4tc** — `change` from the engine alone (`decision/change-event-from-engine-only`): `input` then `change`, the script runtime synthesizing none. Choosing a row in a list box fires through this rule, as `select_popup`'s row choice does (one `input` then one `change`).

@history_item n=36 marker=amd-d4acxs6q
**amd-d4acxs6q** — the a11y leg names fourteen `--test` words, `select_interaction` and `stand_act_select` among them (`decision/a11y-leg-membership`). A new select check joins the a11y leg only if it witnesses SC 2.1.1 / 2.4.3, a tree reading or a recorded gap.

@history_item n=37 marker=amd-hva4ican
**amd-hva4ican** — the a11y leg reads 14 files, 112 tests (`select_interaction` 18, `stand_act_select` 10 among them). Growing those files moves the leg's tally and the pinned counts.

@history_item n=38 marker=amd-kube42bu
**amd-kube42bu** — `stand-repeat` names seventeen `--test` words, `select_popup` and `select_popup_painted` added; `test_stand_repeat_leg_runs_three_rounds` names them; the sweep test's reach covers them. A new non-`stand_` select target joins only by being named.

@history_item n=39 marker=amd-cixe5kae
**amd-cixe5kae** — `stand-repeat` runs named targets as literal `--test` words beside the `stand_*.rs` glob (`select_interaction` among the first ten), pinned by `test_stand_repeat_leg_runs_three_rounds`; `run stand` keeps its prefix rule.

@history_item n=40 marker=amd-w6i3vy4y
**amd-w6i3vy4y** — `stand-repeat`'s named targets ten → fifteen, the pin's line split and dated clauses dropped; precedent for adding a target to the leg and its pin.

@history_item n=41 marker=amd-6adlxvzj
**amd-6adlxvzj** — trap: the 0.2.0 sweep test reads `ci-leg.sh` as one text by `contains`, so dropping `--test select_interaction` from `stand-repeat` alone reads green while the a11y leg names it; membership held jointly with `test_stand_repeat_leg_runs_three_rounds`.

@history_item n=42 marker=amd-iriujbey
**amd-iriujbey** — `+ decision/requirement-witness-inside-stand-repeat`: each requirement names a witness run inside `stand-repeat` in a table pinned by function name. This chunk's residual closure needs a witness inside the leg.

@history_item n=43 marker=amd-n2omhba3
**amd-n2omhba3** — `~ decision/count-and-rename-sweeps`: a change of a CI leg's file count sweeps tests, examples and CI-script tests for literal pins (a11y count missed by a sweep). Applies if the chunk adds a select check to a leg.

@history_item n=44 marker=amd-zn44tekk
**amd-zn44tekk** — the capability statement's a11y leg file count and `stand-repeat` named-target count are held to `ci-leg.sh`, so a membership change reddens ci-scripts until `docs/capabilities.md` re-states it.

@history_item n=45 marker=amd-krjdo5z6
**amd-krjdo5z6** — `docs/capabilities.md`'s `select` model limits read four bullets; the list read by `select_popup`, `select_popup_painted`, `stand_act_popup`, keys, rows and groups by `select_interaction` and `stand_act_select`. The single-select `size` limit retiring changes those bullets and their claim map.

@history_item n=46 marker=amd-ebg5mftg
**amd-ebg5mftg** — refusal detection: the order `disabled` · `off-screen` · `covered` (`contract/driver-session/refusal-order`), `stand_act_scroll`, `scroll_into_view_nested`; a refused call leaves the instance unchanged. A list box's rows past its scrollport are refused `off-screen` until `scroll`, as `stand_act_popup` reads for the popup.

@history_item n=47 marker=amd-epgtg3nr
**amd-epgtg3nr** — a scrolled box's `bounds` stay fixed when its content scrolls, a click at its centre lands on the row there, and a raw harness `click` on an element outside a clipping box's padding box no longer lands on it (`contract/harness-input/clipped-click`). A list box is such a scrolled clipping box; trap: a check leaning on the old hit is found only by running the stand gates.

@history_item n=48 marker=amd-ur5xsokp
**amd-ur5xsokp** — `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands (`decision/scrolled-box-reads-where-it-stands`, guard `scrolled_box_absolute_position`). The list box's own and its rows' rects are read through these helpers.

@history_item n=49 marker=amd-447m7y2r
**amd-447m7y2r** — `+ contract/session-layout/changes-nothing` (an open list stays open), `+ decision/layout-reason-words-each-read` (every placing word and size cause read on a fixture in both modes, guard `layout_reason`), `+ decision/stated-tolerance-read-inclusively`. A list box in flow may add a placing word or size cause that needs its fixture row.

@history_item n=50 marker=amd-fcxxug5t
**amd-fcxxug5t** — `+ decision/no-change-read-control-needs-pending-work`: a control of a no-change read needs pending work, a pump on a settled screen being invisible.

@history_item n=51 marker=amd-cdohja6t
**amd-cdohja6t** — the `layout` verb's answer equal to the in-process read on every node of the served screens in both modes (`contract/session-run/layout-answer`), and the read changing nothing through the verb too.

@history_item n=52 marker=amd-arfsbrh6
**amd-arfsbrh6** — `contract/session-run/layout-answer` moved to the seven served screens, and `+ decision/caused-owner-pinned-as-graph-reads`.

@history_item n=53 marker=amd-mmdtifvl
**amd-mmdtifvl** — `decision/cross-os-readings-by-answer-digest` reads the seven served tasks' snapshot, lint and actions digests against one committed table; and `count-and-rename-sweeps` keeps a count-worded witness name. Any change to how a stand select reads moves that table.

@history_item n=54 marker=amd-n3udqhi7
**amd-n3udqhi7** — `stand_actions` (8): the flight booker's options `covered` with owner `flight-type` until the list opens; `crud-person-14` `off-screen` until `scroll`; the actions answer holds no value (`decision/actions-answer-holds-no-value`, `contract/driver-verbs/actions`). A sized single select's rows read as listed actionable nodes, not `covered`.

@history_item n=55 marker=amd-x5hh3g7d
**amd-x5hh3g7d** — `decision/one-actionable-rule`: both actionable readers read the current listeners, a select's options and group listed with the select as owner (guard `a_listener_removed_on_a_re_render_is_read_by_neither_reader`).

@history_item n=56 marker=amd-aj6rkfgc
**amd-aj6rkfgc** — `NodeState.clickable` (`contract/snapshot-node/clickable`), `NODE_FIELDS` fourteen, `stand_non_lean_tasks`' `CLICKABLE` table.

@history_item n=57 marker=amd-flbkdilp
**amd-flbkdilp** — `stand_lint` and `lint.rs`: `Defect::ALL` names the six defects in order, findings in document order then defect order (`contract/lint-defects/*`, `contract/lint-findings/order`).

@history_item n=58 marker=amd-rrbdvino
**amd-rrbdvino** — the harness shot: `screenshot`/`screenshot_rect` at the harness's own time through `VelloCpuImageRenderer` over white, no font refused (`decision/headless-screenshot`); the session's `screenshot` changing nothing (`contract/session-run/screenshot`).

@history_item n=59 marker=amd-6u4kftng
**amd-6u4kftng** — the overlay scrollbar fade reads the document's clock, a shot after a scroll repeating until `advance` (`decision/scrollbar-fade-on-document-clock`, guard `stand_virtual_clock`). A list box that scrolls shows thumbs on this clock.

@history_item n=60 marker=amd-uimtzpsg
**amd-uimtzpsg** — `+ decision/cross-os-readings-by-answer-digest`; `decision/stand-checks-bundled-font` gains guard `stand_bundled_font_families`. Row heights of `size × 1.2em` are font-measured under the bundled font.

@history_item n=61 marker=amd-gkbc5lsj
**amd-gkbc5lsj** — `decision/layout-tests-in-both-layout-modes` gains its except: upstream targets taken in a sync stay as written. Escher's own new select checks still run both modes.

@history_item n=62 marker=amd-nlavi4cc
**amd-nlavi4cc** — the shared `tests/fixtures/mod.rs` (`decision/shared-fixtures-in-one-module`): a page two checks share goes in `fixtures`; a page with one reader stays in its file; no print, tracing, env, file, socket.

@history_item n=63 marker=amd-ke4vohyd
**amd-ke4vohyd** — `decision/shared-stand-tables-in-one-module` except: a helper whose body or failure message differs stays in its check; `stand_element_ids` keeps its own boot.

@history_item n=64 marker=amd-3quv32td
**amd-3quv32td** — `+ contract/stand-boot/boot-alone` and `Harness::from_component_alone`, with `+ decision/component-alone-props-as-maker`; a sized select component can be booted alone at the stand's viewport and font (`contract/stand-boot/options`).

@history_item n=65 marker=amd-nsqxmuql
**amd-nsqxmuql** — `decision/complexity-ceiling-host-only` except: the engine's click handler is held below the ceiling, upstream's arms included. A list-box row click added to the click handler must keep it below 15.

@history_item n=66 marker=amd-htmd7cg7
**amd-htmd7cg7** — `clear_interaction_state_for_removed_node` took the select list's clearing, its 17.0 reading dated before that edit; an engine select change edits functions on the complexity record.

@history_item n=67 marker=amd-6eogcl6u
**amd-6eogcl6u** — `+ decision/oversize-test-targets-kept`: escher's integration targets over 800 code lines stay whole, one file and one target (the select checks among the large ones); extending them adds no split.

@history_item n=68 marker=amd-bc5fqyab
**amd-bc5fqyab** — `press` gained the bare `--ctrl` flag and `graph [--part]` on the command line (2026-11-20-driver-debts), closing the earlier «`press` carrying no Ctrl» limit on `multiple` selects; a list box's key model through the driver reads this flag set.

@history_item n=69 marker=amd-riq6ejg6
**amd-riq6ejg6** — `select_popup`'s hand-written `CountingShell` counts redraw requests for open, move, scroll and close; a list box's key moves and scrolls asking a redraw would be read the same way.
