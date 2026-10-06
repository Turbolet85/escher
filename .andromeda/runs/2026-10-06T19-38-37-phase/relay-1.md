Question put to the founder at this phase's P1 (the route entry leaves it to promotion):

"The route entry leaves this to be settled at promotion: which edits to an app's code must leave an in-place element's id unchanged? (Today an element with an HTML `id` reads that id; every other element reads a positional path like `TaskShell/Crud/div:0/div:1/div:0/div[7]`, and its children inherit that path even under a keyed parent.)"

Options offered:
1. "Anchor at keyed ancestors (Recommended)" — Keyed elements survive any edit that leaves them in place. Unkeyed elements survive any edit outside their nearest keyed ancestor's subtree, plus edits inside it that don't touch their own relative path. Needs a rule change: an unkeyed element's path starts at its nearest keyed ancestor instead of the component root. Path ids under keyed ancestors change once, now, before the diff and driver depend on them; the id tests for v010-01/02 are re-pinned and the architecture contract is amended at wrap (a change to a locked contract, so it is yours to ratify).
2. "Today's rule, proven" — No rule change; prove and document which edits keep an id.
3. "Every actionable element keyed" — a stand rule plus a check: every focusable or role-bearing stand element must carry an author key, and the check fails on one that reads a positional path. Can be combined with the first option.

The founder's answer, verbatim:

Selected: "Anchor at keyed ancestors (Recommended)"

Notes: "Combine it with the third option: both rules in this chunk. (1) An unkeyed element path starts at its nearest keyed ancestor. (2) Every element an agent can act on (focusable or role-bearing) must carry an author key, and a check fails on one that reads a positional path, naming the element and the remedy. Also: v010-16 observed_gap in the matrix is still v010-01 text ("no semantic id exists") and is stale; correct it through matrix.py refine or a dated note before the claim."
