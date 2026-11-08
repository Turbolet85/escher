@account n=1 entity=input-validation#1.65 v=not_touched
why: the report adds component, state and event names to the graph answer, not accessible names, and snapshot.rs, snapshot_text.rs and snapshot_diff.rs still resolve and write names as the row says, with mcp_log still reading 0 name needles
@account n=2 entity=input-validation#1.67 v=not_touched
why: clickable reads ElementData::is_disabled, and the snapshot's enabled still reads the disabled attribute by presence, so the three closed parses and their exits hold on the changed snapshot.rs
@account n=3 entity=input-validation#1.69 v=proposed
why: the snapshot's closed state readings gain NodeState.clickable, a closed bool written in the text only when true and as a boolean in JSON, inside the same ratified carriers (F6)
@account n=4 entity=input-validation#1.74 v=not_touched
why: the range value still leaves only as form entry, input event value and NodeState.value; the report states the graph carries no control value
@account n=5 entity=input-validation#1.77 v=not_touched
why: dioxus_document.rs's change to handle_event sets and clears the registry dispatch around runtime.handle_event and dispatches as before, so the select's input and change dispatch and its exits hold
@account n=6 entity=input-validation#1.84 v=not_touched
why: the multiple select's set still leaves by the listed exits; the graph answer carries no control value, per the report
@account n=7 entity=input-validation#1.85 v=not_touched
why: execute.rs gains the Graphed arm alone; the driver's type on a select and its fixed help are unchanged by the report
@account n=8 entity=input-validation#1.86 v=not_touched
why: the row names no select_of caller, so graph.rs reading select_of for an option's owner moves none of its claims; closed_select_of, is_select and clear_select_typeahead callers are unchanged
@account n=9 entity=input-validation#1.87 v=not_touched
why: blitz-dom is unchanged (the report's diff gate is green), so the select list's public surface holds
@account n=10 entity=input-validation#1.88 v=not_touched
why: the graph answer carries no control value and a typed sentinel occurs 0 times in it, so the mask holds on every carrier as stated
@account n=11 entity=input-validation#1.91 v=not_touched
why: blitz-dom is unchanged and the graph reader does not call text_extent, so Session::lint stays its one non-test caller
@account n=12 entity=input-validation#1.92 v=not_touched
why: blitz-dom is unchanged and no new caller of visible_region is named by the report
@account n=13 entity=input-validation#1.106 v=proposed
why: the Dioxus bridge in dioxus_document.rs gains the registry dispatch on every document's event path and the in-process use_graph_state registry (F4, F5), detail beside its Dioxus boundary row
@account n=14 entity=input-validation#1.137 v=not_touched
why: the 7GUIs tasks' signals become use_graph_signal with declarations only; the report changes no date, formula, diameter or flight-type validation
@account n=15 entity=input-validation#1.138 v=not_touched
why: cells-grid keeps its closed key match and no role, and the report reads it clickable false, so the row holds
