@row_add n=1 row=contract/snapshot-node/highlighted area=pii-scrubbing-compliance fact=F1 detector=D-obs-pii
boundary: the snapshot node
item: highlighted
promise: A closed bool, true on the open list's highlighted row and carrying no page text, written as a node's "highlighted" key only when true inside the snapshot and diff answers; never logged or fielded, no span or log field carrying it.
basis: fct-s4gho2nl
because: The chunk adds highlighted to the driver's node-key schema inside the ratified snapshot and diff carriers with no new log field or span, so the PII section must state what of the highlight crosses and that it is never logged.
@row_add n=2 row=decision/active-descendant-from-engine-state area=pii-scrubbing-compliance fact=F2 detector=none
name: Active descendant named from engine state
rule: The engine names an open single select's highlighted option as the tree's active descendant from its own state, and a page's aria-activedescendant stays unread.
why: An engine change ratified; nothing is read from the page's attribute.
basis: fct-voh3czmv
because: The accessibility builder now sets the active descendant from the open list's current option while reading nothing from a page's aria-activedescendant, a choice about what page input is read that future work must respect.
