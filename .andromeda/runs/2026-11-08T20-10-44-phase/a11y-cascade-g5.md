@carried n=1 label=a11y-plan/screen-reader-support fact=fct-ienybbqr
why: The owed replay test must keep and re-assert an actions answer equal to the kept one, and on the stand that answer must hold these causes and the option's owner in both layout modes.
quote: In the `actions` answer the flight booker's two options read `covered` with owner `flight-type` until its list opens, `flight-return-date` reads `disabled`, and `crud-person-14` reads `off-screen` until a `scroll`, in both layout modes
@carried n=2 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
why: The coverage list must key each handler on the stable element id the graph serves, its AccessKit author_id, never on an engine NodeId.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one
@carried n=2 run=2 label=a11y-plan fact=fct-77ewqepn
quote: The actionable-key rule: every element an agent can act on — focusable, carrying an interactive role or an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it
@carried n=2 run=3 label=a11y-plan fact=fct-iq4fjjyz
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one
@carried n=3 label=a11y-plan/screen-reader-support fact=fct-73ao4vi3
why: The coverage read must keep the graph read-only and free of typed text, list only graph nodes that are snapshot nodes, and leave the reserved `{owner}///{key}` form to 0.4.0's virtual nodes.
quote: `stand_graph` asserts in both layout modes that every graph node is a snapshot node, that no served id holds the reserved separator `///` (the reserved `{owner}///{key}` form, spelled by `virtual_node_id`, which the walk never calls), and that a graph read changes nothing and holds no typed text
@carried n=4 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: The replayed actions answer must equal the kept one under this one snapshot model, the chunk adding no second role or name mapping.
quote: The snapshot model is a consumer of this output: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` takes each node's role, its name
@carried n=4 run=2 label=a11y-plan fact=fct-uz4kdss3
quote: The snapshot model consumes the tree: `DioxusDocument::snapshot` takes each node's role, name, focus and `selected`, `pressed` and `invalid` from `accessibility_tree`, never from the element, and `enabled` (the presence of `disabled`) and `value` from the element
@carried n=5 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: A replayed click or type refused as disabled dispatches nothing, so the coverage count must not read the handler it targets as exercised.
quote: the driver detects it on that reading and no other: `Session::run` refuses a `click` or a `type` as `disabled` when the snapshot node's `enabled` is `Some(false)`, the first of its three screen-level checks, with nothing dispatched and the ids reading `focused` and the tree's focus as they were before the call
@carried n=5 run=2 label=a11y-plan fact=fct-2hrkecxm
quote: the driver refuses a `click` or a `type` as `disabled` where the snapshot's `enabled` reads `Some(false)`, nothing dispatched.
@carried n=6 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: The coverage result and any new result field must carry no control value, so a password's MASKED_VALUE and a file input's marker never leave through it as text.
quote: a password input reads the fixed marker `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no field of the snapshot or of the tree
@carried n=6 run=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
quote: and a file input reads the same marker where its `value` attribute is non-empty, its path in no field of the snapshot
@carried n=6 run=3 label=a11y-plan fact=fct-2hrkecxm
quote: A password input's value reads as the fixed `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no snapshot field, tree node or painted frame
@carried n=7 label=a11y-plan/screen-reader-support fact=fct-6nobpd2c
why: The handlers the command counts must be read from the vdom walk's current listeners, as clickable is, never from the listener mark.
quote: `NodeState.clickable` is a `bool` after `focused`, true where the vdom walk's current listeners hold `click` and the element is no disabled form control (`ElementData::is_disabled`), read from the one snapshot model and the walk with no second role mapping
@carried n=7 run=2 label=a11y-plan fact=fct-77ewqepn
quote: `unkeyed_actionable` and `DioxusDocument::actionable_nodes` read one actionable rule, its listener leg the vdom walk's exact current listeners holding an event other than `mounted`, so an element whose listener a re-render removed is read by neither, the `data-dioxus-id` mark serving event dispatch alone.
@carried n=8 label=a11y-plan/screen-reader-support fact=fct-gg5bgkx2
why: The coverage command must list unexercised handlers from this read-only graph, by node id with its optional owner and handler event name, carrying ids and closed words only, never a value.
quote: `DioxusDocument::graph`, behind `accessibility`, is one more read-only reader beside the snapshot: a `UiGraph` holding one `GraphNode` per snapshot node in pre-order — its id, its `owner` (for an option or an optgroup, the stable id of its select), its component, its handlers with their declared writes, and the states it reads — and per registered state its declared writes, the writes a run confirmed or flagged (`undeclared`, `unconfirmed`) and its reading components; the answer carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value
@carried n=8 run=2 label=a11y-plan fact=fct-uz4kdss3
quote: the read-only UI graph (`DioxusDocument::graph`) holds one node per snapshot node, names and closed words only — never a second role or name mapping.
@carried n=9 label=a11y-plan/screen-reader-support fact=fct-vlvcclau
why: The owed replay test must record, keep and replay an actions step whose answer stays read-only and value-free, a differing answer named by its step and differing key.
quote: The `actions` verb (`Session::actions`) is one more read-only reader: for every actionable node it answers each verb of the verb table's instance rows taking an id with the cause `check_target` would refuse it with, a node resolving to no element reading `stale`; it changes nothing — no focus, snapshot or diff moves and an open list stays open — and writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, `MASKED_VALUE` or `NodeId`
@carried n=10 label=a11y-plan/screen-reader-support fact=fct-yfrcl4qo
why: The snapshot must keep its shape, so the chunk must add no node key and leave NODE_FIELDS at fourteen.
quote: No `expanded` node key exists: `NodeState` holds no such field and `NODE_FIELDS` reads fourteen, so a list's open state reaches the tree and not the snapshot
@binding n=1
**sr-author-id ↔ sr-snapshot (UI graph)**: the coverage key must be the graph node id, which is the stable element id the tree carries as `author_id` (a11y-plan/screen-reader-support fct-kf4qrij2), plus the graph's optional `owner` (a11y-plan/screen-reader-support fct-gg5bgkx2), never a `NodeId`; no id the command serves may hold the reserved `///` separator, which stays for 0.4.0's virtual nodes (a11y-plan/screen-reader-support fct-73ao4vi3).
@binding n=2
**sr-snapshot (disabled refusal) ↔ keyboard-navigation (disabled-ness)**: a replayed step the driver refuses as `disabled` dispatches nothing and must not count its handler as exercised (a11y-plan/screen-reader-support fct-ge4n5uhw); that refusal reads the same presence of `disabled` that keys focusability and activation (a11y-plan fct-s5i3di32).
@binding n=3
**sr-snapshot (listener reading) ↔ sr-actionable-keys**: the handlers counted must follow the one actionable rule's listener leg, the vdom walk's exact current listeners other than `mounted`, as `clickable` does (a11y-plan/screen-reader-support fct-6nobpd2c, a11y-plan fct-77ewqepn), so a listener a re-render removed is neither counted nor listed.
@binding n=4
**sr-snapshot (redaction) ↔ §3 A11y Assertion Harness Contract (obs allowlist)**: the coverage result and the replay test's failure naming must carry ids, component and event names, verb and cause words only, never `MASKED_VALUE`, a value or typed text (a11y-plan/screen-reader-support fct-vlvcclau, a11y-plan/screen-reader-support fct-ge4n5uhw), the same redaction escher's own log targets keep (a11y-plan fct-4abpljjc).
