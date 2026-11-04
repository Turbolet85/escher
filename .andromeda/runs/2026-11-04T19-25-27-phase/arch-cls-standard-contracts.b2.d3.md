@answer n=1 entity=standard-contracts#6.6 v=no
why: Stylo style-data filtering and the scoped style-resolve context are not read by the probe's four links or its harness runs.
@answer n=2 entity=standard-contracts#7.1 v=no
why: crate-private layout pass types and Taffy trait implementations are not called or read by the probe; layout is reached only through settle.
@answer n=3 entity=standard-contracts#8.1 v=no
why: a mix-blend-mode stacking-context arm is a paint and style rule that no probe reading touches.
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node-data variant, range-input, serialization and Widget inventory is not what element_id or the listener map rules over.
@answer n=5 entity=standard-contracts#10.1 v=no
why: the disabled-control gate on pointer and activation keys is not named by the scope, whose clicks target the stand's enabled controls.
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled is the crate-private select list's reading of a disabled row; the probe reads no select.
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private select rules and readers belong to the select, which the probe reads not at all.
@answer n=8 entity=standard-contracts#10.4 v=no
why: SelectCursor, RowGesture and apply_select_choice are select internals; the probe reads no select.
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow are the open select list's read types and colours; the probe reads no select list.
@answer n=10 entity=standard-contracts#10.6 v=no
why: SelectList.fresh holds a just-opened list against a focus restyle; no link of the UI graph is a select.
@answer n=11 entity=standard-contracts#10.7 v=no
why: the select list's row cap and placement rule is a select popup rule the probe does not reach.
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow is a geometry and layout flag, and the scope says no link of the UI graph reads geometry.
@answer n=13 entity=standard-contracts#10.9 v=no
why: IS_INLINE_WITH_ONLY_BLOCK_CHILDREN governs layout and box painting, which no link of the UI graph reads.
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData::classes is the class attribute's Stylo atom list; the probe reads data-dioxus-id, not class.
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state sets the :open state of details and dialog elements, which the probe's Dioxus stand does not read.
@answer n=16 entity=standard-contracts#11.1 v=yes
why: NetProvider::is_noop and NetHandler::error carry the offline stand boot (item 5) and the harness load counting that the probe's runs settle through.
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and the blitz-net Provider are not on the offline stand's path; the stand's load page uses its own LoadNet.
@answer n=18 entity=standard-contracts#13.1 v=yes
why: the BlitzShellEvent variants (Poll, RequestRedraw) and View::resume are the windowed shell loop the probe reads for the windowed path (item 10).
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list paints only the open select list, which the probe does not read.
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are text-painting internals that no probe reading reaches.
@answer n=21 entity=standard-contracts#13.4 v=no
why: blend and isolation paint layers are not painted by any link the probe reads.
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness is an upstream-sync CARRY on paint; the probe paints no blended element.
@answer n=23 entity=standard-contracts#14.1 v=yes
why: BlitzShellProvider's request_redraw sends RequestRedraw through the shell's event loop, the redraw path the windowed reading reads beside each Poll line (item 10).
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the Harness constructors, pump, poll(None), settle, input helpers and load counting are the harness the hazard is measured through (item 7).
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the VirtualClock that only tick and advance_time move is the harness's time the probe's runs read (item 7).
