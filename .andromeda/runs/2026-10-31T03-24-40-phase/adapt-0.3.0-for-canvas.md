# Adapting 0.3.0's planned work to the 0.4.0 canvas plans (the escher overseer's reconnaissance, 2026-10-29)

**Why.** The founder, 2026-10-29: «давай теперь сделай разведку чтоб понять возможно то что мы планируем делать сейчас
стоит адаптировать с учетом планов, чтоб не получилось что мы что то строим а в итоге прийдется полностью переделывать
изза новых требований». The 0.4.0 plans are in `canvas-and-node-graph.md`: virtual nodes inside a canvas, the canvas
observed as data, a `drag` verb, and pan and zoom through a transform.

**Read at `a6c6a37c7`**: `escher-0.3.0/requirements.md`, `working-route.md`,
`packages/dioxus-native-dom/src/element_id.rs`, `snapshot.rs`, and `packages/escher-driver/src/execute.rs`,
`command.rs`, `wire.rs`. These are the overseer's reading of the code, not measurements.

## What already fits (no rework expected)
- **The snapshot is built from the accesskit tree** (`snapshot.rs:98`: «its id and role from that tree»). That tree
  carries non-DOM nodes natively, so a canvas's virtual nodes can enter it without a new carrier.
- **An action lands on a point.** `click` and `type` press the centre of the target's bounds
  (`execute.rs:360`, `:427-439`). A virtual node with bounds can be clicked the same way, as long as the canvas app
  hit-tests its own pointer events.
- **The wire is versioned** (`wire.rs:37`, `v3`), so a new verb is an additive, versioned change.

## Where 0.3.0 could build something 0.4.0 would have to redo
1. **The stable-id grammar** (`element_id.rs`). Today an id is an author key, an anchored path (`//`), a component path,
   or a document path (`/…`), all derived from DOM and vdom elements. A virtual node has no DOM element. If D2, B3,
   E2's recorded checks and replay store ids in this grammar, 0.4.0 needs a form for `{canvas element}` + `{virtual
   key}` that collides with none of the four. **Adapt in «The UI graph» (v030-05): reserve that form now (a separator no
   current id can hold), even if nothing produces it yet.**
2. **The node model of the UI graph and of «What can I do here»** (v030-05, v030-06). If the graph and the actionable
   list are keyed on a DOM element (`Target.node: NodeId`, `execute.rs:354`), a virtual node does not fit. **Adapt: key
   them on the agent-visible node (the accesskit node with its bounds), with an optional owner element, and let a
   handler sit on an owner element and serve sub-targets.** A canvas typically has one listener that dispatches inside.
3. **Verbs as a closed list in B3 and E2** (v030-06, v030-09). If «What can I do here» and auto-exploration hard-code
   `click`/`type`/`press`/`scroll`, adding `drag` (two points or a node plus a destination) means reshaping both.
   **Adapt: read the per-node verbs from the schema's verb table, and let the recorded-check step format carry verbs
   with more than one target, versioned.**
4. **Handler coverage** (v030-07) counts DOM listeners. A canvas's internal handlers are invisible to it. **Adapt:
   state that limit in its acceptance, so 0.4.0 extends it rather than redefines it.**
5. **Layout explained** (v030-11) reads Taffy and Stylo. A virtual node's place is the app's own. **Adapt: let its
   answer name the "placed by the app inside canvas X" case, so the field set is not closed.**
6. **Bundled fonts** (v030-14). Canvas text (`GlyphRun`) must draw from the same bundled fonts, or a canvas reads
   differently across hosts. **Adapt: in its acceptance, name canvas text as a reader of the bundled set.**
7. **Engine debts** (v030-04). The transform `visible_region` fix should be proven on a pan/zoom container like the
   node graph's (`canvas-and-node-graph.md`).

## What 0.3.0 should not take on
The canvas renderer itself, the CPU renderer's custom-paint gap, `drag`, and virtual-node production all stay in 0.4.0.
The adaptations above keep 0.3.0's shapes open; they add no 0.4.0 capability.
