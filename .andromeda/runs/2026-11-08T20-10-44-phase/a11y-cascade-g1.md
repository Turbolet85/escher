@carried n=1 label=a11y-plan/a11y-strategy fact=fct-d5bs73pt
why: the coverage command must read handlers only through the feature-gated read-only graph and actionable readers, and any record of exercised handlers must hold on every app whether it lives in a gated module or the ungated signal_registry.
quote: its read-only UI graph reader (`DioxusDocument::graph`, a `UiGraph`, its `graph` module) and its read-only actionable-node reader (`DioxusDocument::actionable_nodes`, an `ActionableNode`, re-exported from the crate root beside `UnkeyedActionable`), which shares the one crate-private actionable rule with the check in the same `actionable` module, while its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated
@carried n=1 run=2 label=a11y-plan/a11y-scope-summary fact=fct-r6zswm2d
quote: the session library `escher-driver`, which seven_guis depends on since 2026-10-07-driver-session, names `accessibility` on `dioxus-native-dom` since 2026-10-07-act-by-id — unconditionally, with no `[features]` table of its own: its executor reads the snapshot, which stands behind that feature
@carried n=1 run=3 label=a11y-plan fact=fct-kbjv4usk
quote: it reaches seven_guis through `escher-driver`, which names it unconditionally, while seven_guis' own manifest names none
@carried n=2 label=a11y-plan/a11y-scope-summary fact=fct-gwtg4dlh
why: the coverage list must key each handler on the graph's stable node id with its optional owner and never on an engine NodeId or its integer form.
quote: `NodeId::as_u64` is described as useful for interop with integer-id APIs such as AccessKit (packages/blitz-traits/src/node_id.rs:15-22)
@carried n=3 label=a11y-plan/aria-patterns-roles fact=fct-x7vrlpec
why: the count must span exactly the handlers on agent-visible graph nodes, so a handler on an excluded node or its children is outside it, as the acceptance's limit states.
quote: `aria-hidden="true"` keeps the node but marks it hidden (`is_hidden`); `hidden`, `display: none` and `visibility: hidden` exclude it — an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes and a `multiple` select's rows do not: it stays in the tree, `hidden`, `inert` and `visibility: hidden` still excluding it (packages/blitz-dom/src/accessibility.rs:284-285; packages/blitz-dom/src/accessibility.rs:287-300); children of a hidden element are excluded
@carried n=3 run=2 label=a11y-plan fact=fct-zeglb7pr
quote: `aria-hidden="true"` keeps a node, marked hidden; `hidden`, `display: none` and `visibility: hidden` exclude it and its children, an `option` under a `select` excepted from `display: none`.
@binding n=1
**a11y-strategy (feature gating) ↔ a11y-scope-summary (driver feature reach)**: the coverage command in escher-driver must read the graph's handlers through `DioxusDocument::graph`, gated by `accessibility`, which the driver gets only through its unconditional edge, adding no feature table or second reader; an exercised-handler record placed in the ungated `signal_registry` must still hold where no state is registered (a11y-plan/a11y-strategy fct-d5bs73pt) (a11y-plan/a11y-scope-summary fct-r6zswm2d)
@binding n=2
**aria-patterns-roles (hidden content) ↔ a11y-scope-summary (node ids)**: the counted set must be the handlers of graph nodes, which are the tree's agent-visible nodes after hidden exclusion, each keyed on the graph id with its optional owner and never on `NodeId` or `as_u64` (a11y-plan/aria-patterns-roles fct-x7vrlpec) (a11y-plan/a11y-scope-summary fct-gwtg4dlh)
