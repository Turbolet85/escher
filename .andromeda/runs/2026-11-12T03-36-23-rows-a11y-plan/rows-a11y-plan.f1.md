@row_add n=1 row=decision/a11y-ci-leg area=3-a11y-assertion-harness-contract rec=1,2
name: The a11y CI leg
rule: The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check.
why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option.
guard: LegScriptTest's a11y tuple
@row_add n=2 row=decision/active-descendant-unread area=5-keyboard-navigation rec=3,4,5
name: aria-activedescendant kept unread
rule: The CRUD person rows are listbox options that are not focusable, the tree's focus is the list and never the option, and the tree builder reads no aria-activedescendant, so an assistive technology learns the current row only from its selected reading.
why: SC 4.1.2 work outside the tier-0 target, kept recorded and unread; ratified
guard: accessibility_active_descendant_unread
@row_add n=3 row=decision/select-list-colours-engine-constants area=6-visual-design-verification rec=6,7
name: Open select list colours are engine constants
rule: An open select list's highlight text, highlight background, disabled text and border colours are engine constants, in no sheet and not tokens, and the harness asserts the list's highlight and disabled rows as painted against them.
why: the engine holds no tokens
guard: select_popup_painted
@row_add n=4 row=contract/accessibility-tree/author-id area=7-screen-reader-support rec=8,9
boundary: accessibility tree
item: author_id
promise: In a Dioxus document every element node carries its stable element id as AccessKit author_id; no TextRun, document-root or Window node carries one.
@row_add n=5 row=contract/snapshot/value area=7-screen-reader-support rec=10,11
boundary: snapshot
item: value
promise: The snapshot's value is read from the element: the editor's text, else a range input's current value, else the value attribute.
@row_add n=6 row=contract/snapshot/file-input-value area=7-screen-reader-support rec=12,11
boundary: snapshot
item: file input value
promise: A file input reads MASKED_VALUE where its value attribute is non-empty, its path in no field of the snapshot.
@row_add n=7 row=decision/actionable-key-rule area=7-screen-reader-support rec=13,14
name: Actionable-key rule
rule: Every element an agent can act on (focusable, carrying an interactive role or an event listener) reads an author key, so its author_id is that key and holds under any edit of the code around it.
why: ratified
guard: unkeyed_actionable
