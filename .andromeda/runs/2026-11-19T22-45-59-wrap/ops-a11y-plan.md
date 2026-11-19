@row_set n=1 row=decision/a11y-tree-refresh-on-change cell=rule seen=1 fact=F6 detector=none
value: The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree.
basis: fct-xcqgt2ri, fct-lfuqrgmb
because: The refresh moved from the poll's drained changed set to the redraw after resolve, which reads and then clears the set, so the row's poll site is now false.
@row_set n=2 row=decision/accessibility-tree-through-document-method cell=rule seen=1 detector=none
value: The accessibility tree is built from the document on the initial tree request and rebuilt in a redraw after style resolve when the changed set is non-empty, both paths going through the document trait's accessibility-tree method.
basis: fct-xcqgt2ri, fct-lfuqrgmb
because: The poll no longer refreshes the tree; the redraw rebuilds it from the wrapping document after resolve, so the row's poll path is false.
@row_set n=3 row=decision/hidden-content-excluded cell=rule seen=1 fact=F5 detector=none
value: An aria-hidden true node is kept but marked hidden, while hidden, inert, display none, visibility hidden and visibility collapse exclude the node and its children from the accessibility tree.
basis: fct-ohaxmeez, fct-bhfcjucw
because: The W9 ruling taken with the merge keeps an element with visibility collapse out of the tree as visibility hidden is, so the row's list of exclusions is incomplete.
@row_set n=4 row=contract/accessibility-tree/hidden-and-inert-nodes cell=promise seen=1 detector=none
value: Nodes with display: none, visibility: hidden or visibility: collapse or carrying inert, and their descendants, are excluded from the accessibility tree and the snapshot, a select's option excepted from display: none.
basis: fct-ohaxmeez, fct-bhfcjucw
because: visibility collapse now excludes a node from the tree as visibility hidden does, so the promise's list of exclusions is incomplete.
@row_set n=5 row=decision/hidden-content-excluded-from-a11y-tree cell=rule seen=1 detector=none
value: Nodes with `display: none`, `visibility: hidden` or `visibility: collapse` or carrying `inert`, and their descendants, are excluded from the accessibility tree and so from the snapshot.
basis: fct-ohaxmeez, fct-bhfcjucw
because: visibility collapse now excludes a node from the tree as visibility hidden does, so the rule's list of exclusions is incomplete.
@row_add n=1 row=contract/accessibility-tree-hidden-content/visibility-collapse area=4-aria-patterns-roles detector=none
boundary: Accessibility tree hidden content
item: visibility: collapse
promise: visibility: collapse excludes the node and its children from the tree, as visibility: hidden does.
basis: fct-ohaxmeez, fct-bhfcjucw
because: The W9 ruling adds visibility collapse as its own item of the hidden-content contract, which holds one row an item.
@row_add n=2 row=decision/programmatic-focus-fires-focus-events area=keyboard-navigation fact=F1 detector=none
name: Programmatic focus change fires focus events
rule: A programmatic focus change fires blur, focusout, focus and focusin through the engine's event queue.
why: Upstream's behaviour, taken with the merge.
guard: programmatic_focus_changes_dispatch_focus_events; inert_subtree; stand_act_select
except: The inert fix-up and the driver's select focus change focus silently and fire none of the four.
state: provisional
basis: fct-xcqgt2ri, fct-trpvqtky
because: set_focus_to and clear_focus now queue the four focus events while the inert fix-up and the driver's select focus stay silent, a convention with its exception that no row states.
@row_add n=3 row=decision/script-events-untrusted area=keyboard-navigation fact=F2 detector=none
name: isTrusted read by who dispatched the event
rule: An event dispatched by script reads isTrusted false, and an event the user agent dispatches reads true.
state: provisional
basis: fct-trpvqtky, fct-ov42vbyr
because: The W6 ruling makes a script-dispatched event read isTrusted false, a contract at the event boundary that no row states.
@row_add n=4 row=decision/cancelled-pointerup-still-clicks area=keyboard-navigation fact=F3 detector=none
name: A cancelled pointerup still clicks
rule: A pointerup cancelled by a handler still produces the click and still ends a scrollbar drag.
guard: a_cancelled_pointerup_still_clicks; a_cancelled_pointerup_still_ends_a_scrollbar_drag
state: provisional
basis: fct-ohaxmeez, fct-7kzh3vb4, fct-7jasdaqu
because: The W7 ruling taken with the merge keeps the click and the drag end after a cancelled pointerup, a decision no row of the click walk states.
@row_add n=5 row=decision/cancelled-pointerdown-counts-for-double-click area=keyboard-navigation fact=F4 detector=none
name: A cancelled pointerdown still counts for a double click
rule: A pointerdown cancelled by a handler still counts towards a double click but selects no text.
why: Upstream's behaviour, taken with the merge.
guard: a_cancelled_pointerdown_counts_towards_a_double_click_without_selecting
basis: fct-xcqgt2ri, fct-7yl25uuk
because: Upstream's default-prevented pointerdown handling arrived with the merge, and the double-click rows say nothing of a cancelled pointerdown.
