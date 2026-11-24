@row_add n=1 row=decision/drag-and-drop-tested-headless area=test-strategy fact=F1 detector=D-tests-coverage
name: Drag and drop tested through the UI event entry
rule: Drag and drop is tested headlessly through the document's UI event entry with hand-written event handlers; the windowed OS drop source has no test of its own.
why: The windowed OS drop source turns OS drags into drag UI events and cannot run headless.
guard: drag_and_drop
basis: fct-pr3l6fxd
because: The chunk adds drag and drop, its drag dispatch tested over the document's UI event entry while the windowed drop source stays untested, unrunnable headless, a tier choice no row states.
@row_add n=2 row=name/other/drag-and-drop area=test-scope-summary fact=F6 detector=none
class: other
name: drag_and_drop
basis: fct-xppretyh
because: The merge adds upstream's blitz-tests target drag_and_drop as its own target, so the name is occupied.
@row_add n=3 row=name/other/flex-br area=test-scope-summary fact=F6 detector=none
class: other
name: flex_br
basis: fct-xppretyh
because: The merge adds upstream's blitz-tests target flex_br as its own target, so the name is occupied.
@row_none n=1 fact=F2 type=7
what: An absence inside one file, the dropped-file reader's untested lines; the coverage floor stands unchanged and the route's deferral holds the gap.
@row_none n=2 fact=F3 type=5
what: An inventory of which targets run where; the workspace test leg and run all already state that every blitz-tests target runs, and the code holds the leg lists.
@row_none n=3 fact=F4 type=4
relied: preventDefault on a non-cancelable event such as pointerleave does not cancel it.
what: Engine behaviour owned by architecture's event-order decision; the pointer_leave test that pins it is code.
@row_none n=4 fact=F5 type=5
what: An inventory of two windowed examples; the absence of an automated test that launches a window already stands as a row.
