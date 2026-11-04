@carried n=1 label=security-plan fact=fct-n4q5kc73
why: Whatever the probe finds readable (a component name, a handler or event name, a signal's value) must stay off every carrier; if a later carrier would hold any of it, that is a widening the reading brings to the founder as a fork. It is never added here.
quote: the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
@carried n=2 label=security-plan/input-validation fact=fct-d2yejoup
why: Link (1), node to component, must be read against the stable id's existing component-path tier and must leave the id's rules as they are; the probe's code must write no id to a log, the DOM or the vdom.
quote: an element with no key reads one of three `/`-bearing paths — an anchored path `{key}//{segment}` (then `/{segment}` per further DOM level) under the nearest element of its own component instance that reads an author key, else its component path, else its document path, a template root owned by another component instance restarting at that component's chain, so an anchor acts inside one component only
@carried n=2 run=2 label=security-plan/input-validation fact=fct-d2yejoup
quote: the id is computed on demand and escher's own code writes it to no log, DOM or vdom
@carried n=3 label=security-plan/input-validation fact=fct-d2yejoup
why: Links (1), (3) and (4) read through runtime handles (ScopeId, ElementId, NodeId). The probe must keep each handle in process and must not put it on the id, the snapshot or any carrier.
quote: neither the id nor the snapshot carries a `NodeId`, `ElementId`, `ScopeId` or pointer
@carried n=4 label=security-plan/input-validation fact=fct-d2yejoup
why: Link (2), node to handlers (the "reacts to a click" reading), rests on the bridge's existing listener read. The probe must keep that reader free of any crossing, and must not make it a driver, CLI or MCP command, since a route entry owns that decision.
quote: it takes no argument, takes ids from `element_ids()` and roles from `accessibility_tree`, resolves each node through `BaseDocument::get_node` and skips one that does not resolve, logs nothing and never panics, and returns an `UnkeyedActionable` (`node`, `id`, `tag`, `role`, `focusable`, `interactive_role`, `listener`) for each actionable element whose id holds a `/`; an entry holds the id, tag and role and never the element's text content, accessible name or an attribute value of that element, and its `node` field is a process-local `NodeId`, meaningful in process only; it adds no crossing of its own — no wire form, no log, event, socket or file, and no driver, CLI or MCP command yet (whether it becomes one is owned by a route entry)
@carried n=5 label=security-plan/input-validation fact=fct-omhuceqe
why: The probe's reading of the bridge's element ids must use the existing parse unchanged, with no change to the bridge's mutation path.
quote: Parsed as `usize`; a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:48-55)
@carried n=6 label=security-plan/input-validation fact=fct-r5yt6pat
why: Any accessible name the probe meets while reading link (4), state to the nodes that read it, must reach no log. The probe's code must log no name.
quote: escher's own code logs no name, and a host that installs escher's sink writes none to stderr at any `RUST_LOG` level
@carried n=7 label=security-plan/input-validation fact=fct-3l4zlejk
why: A handler-to-state or state-to-node reading that touches a password or file input must never surface the typed text, its length or a host path, in the probe's evidence and in the founder's reading alike.
quote: never the text, never its length and never a file's host path
@carried n=8 label=security-plan fact=fct-cvyxuuhn
why: Where the probe drives the stand through the driver to make a handler write state, each act must go through `validate` inside `Session::run`, and the probe must add no verb.
quote: nothing public runs a `Command` that did not come from `validate`, and the socket, the command line and MCP all pass through it
@binding n=1
**the widening rule ↔ the scope's «Nothing crosses» boundary and the reading for the founder**: the probe adds no field, carrier or exit. Whether a component name, handler name or signal value ever crosses is authored text or an open set, so it is the founder's fork in the reading and is not decided by «The UI graph»'s plan alone (security-plan fct-n4q5kc73)
@binding n=2
**the stable id and `unkeyed_actionable`'s `listener` ↔ Logging & Monitoring (escher's sink) and the scope's links (1) and (2)**: link (1) reads against the component-path tier and link (2) against the bridge's listener read. Ids and listener readings stay in process with no log site, and any command built on them belongs to a route entry (security-plan/input-validation fct-d2yejoup)
@binding n=3
**no `ScopeId` / `NodeId` crossing ↔ the scope's «The UI graph is not built here»**: the probe's handles stay in process. A graph read that would carry a component or state reading is «The UI graph»'s to plan under this rule (security-plan/input-validation fct-d2yejoup)
@binding n=4
**the password and file mask ↔ Data Protection (local user data) and the founder's reading document**: no evidence file of the probe holds a masked control's value (security-plan/input-validation fct-3l4zlejk)
