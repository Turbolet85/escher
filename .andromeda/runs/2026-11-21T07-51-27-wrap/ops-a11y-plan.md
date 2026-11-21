@row_set n=1 row=contract/dioxus-document/reached-handlers cell=promise seen=1 fact=F1 detector=none
value: The id and event of each node whose listeners held the event, resolved before the handlers run, first-reached, deduplicated; a dispatch its target or an ancestor stops, or a handler's panic unwinds, counts the innermost listener alone, a non-bubbling event its target alone; empty unless switched on.
basis: fct-xyhdhgxl, fct-i44kzdy2, fct-63kznktn
because: The record now reads the stop, so a stopped, unwound or non-bubbling dispatch no longer counts every node whose listeners held the event, and the promise must say so.
@row_set n=2 row=contract/multiple-select/keys-and-row-clicks cell=promise seen=1 fact=F2 detector=none
value: A multiple select takes its own keys and row clicks, apart from the single-choice select's: plain arrows choose the current row alone, arrows with the action key held (Meta on macOS, Control elsewhere) move the current row, and Space flips it, so rows not adjacent are chosen from the keyboard.
basis: fct-xyhdhgxl, fct-a2inkm3h, fct-hxpbukvz, fct-63kznktn
because: The driver's press now carries the action key and the press help states that it moves the current row and Space flips it, a keyboard contract of the multiple select the row stated only as its own keys.
@row_add n=1 row=contract/ui-graph-answer/parts area=screen-reader-support fact=F4 detector=none
boundary: the UI graph answer
item: parts
promise: A UI graph answer over the 10,000-byte snapshot budget, or one a part is asked of, is read in parts of whole nodes, part 1 alone carrying the states and a later part's empty states meaning read part 1; a graph within the budget with no part asked is answered whole.
basis: fct-xyhdhgxl, fct-63kznktn, fct-qy3e3cbk
because: The graph answer is no longer unpaged: the graph verb takes a part and answers parts at the snapshot budget, a promise at the answer's boundary no row of the area states.
@row_none n=1 fact=F3 type=4
relied: The handler coverage checks under the Cells editor's Enter stop rely on a cell's click opening the editor unfocused, a click on the editor input coming before a key reaches it.
