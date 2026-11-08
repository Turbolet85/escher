@task_prose part=chunk
Builds one read that gives, for the current screen of a held instance, each node, the component that rendered it, the handlers it carries, the state those handlers write and the nodes that read that state (v030-05).
Links 1 and 2 come from the dioxus-core vdom walk the stable ids already run; links 3 and 4 come from an escher-side signal registry in dioxus-native-dom that the seven_guis stand's signals move onto.
Adds a closed boolean snapshot field saying a node reacts to a click, which moves NODE_FIELDS (thirteen today), the schema help texts and describe(), and absorbs residual res-gvelrxja.
Masks values on every carrier, and logs no node, component, handler, state or value. Reserves a virtual-node id form, keys the graph on the agent-visible node with an optional owner element, and lets a handler on an owner serve sub-targets.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=2 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot node's reacts-to-a-click field, values masked as MASKED_VALUE
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: NODE_FIELDS, the schema help texts, VERBS
@task_file n=4 path=examples/seven_guis/src/stand.rs
@task_item n=1 part=needs
The driver command line and its mcp role are agent surfaces: a new verb, its outcome, written form and MCP tool must print only JSON on stdout, one line per command or JSON-RPC 2.0 lines, never colour or an ESC byte, with no terminal test, and keep the exit status classes (0 accepted, 1 refused, 2 usage, 3 session error).
@task_item n=2 part=needs
The cli surface component patterns for escher-session: the graph read's answer and its refusals cross this surface and follow its output rules.
@task_item n=3 part=needs
The reacts-to-a-click reading: an element that reacts to a click and states no role reads GenericContainer today; the new boolean must say it reacts, inside the ratified snapshot and diff carriers.
@task_item n=4 part=needs
Masked values: a password's or a file input's value, and any state or control value the graph carries, reads as MASKED_VALUE on every carrier.
@task_item n=5 part=needs
The seven_guis stand tasks (Home, Temperature Converter, Circle Drawer, Cells) whose signals move onto the registry, and the starting reading pinned on the four lean tasks in both layout modes.
@task_item n=6 part=needs
The graph is keyed on the agent-visible accesskit node with its bounds, as the snapshot is built, with an optional owner element; a canvas or custom-widget element whose one listener dispatches inside is the owner case.
@task_item n=7 part=needs
The web-spa surface renders into one canvas: the reserved virtual-node id form (canvas element plus virtual key) is direction for it, with no canvas renderer or virtual-node production in this chunk.
@task_item n=8 part=needs
The engine's cursor reading (Pointer inside links, Default otherwise) is not a reacts-to-a-click signal; the new field is read from the handlers the vdom walk finds, not from the cursor.
@task_standing n=1
convention: The driver's command line and mcp output carry only JSON on stdout with the exit status classes; any new verb, its test and its recorded step follow it.
home: Surface: cli, Component Patterns
@task_item n=1 part=not_needed
Color palette, core colors, surface scale, text hierarchy, semantic colors and border progression: the chunk paints nothing and adds no colour.
@task_item n=2 part=not_needed
The open select list's engine-painted colour constants: the chunk touches no select list painting.
@task_item n=3 part=not_needed
Typography, font loading and the screenshot fonts-unavailable refusal: the graph read takes no screenshot and measures no text.
@task_item n=4 part=not_needed
Spacing values and scales: the chunk changes no layout or spacing.
@task_item n=5 part=not_needed
Border radius: the chunk changes no box shape.
@task_item n=6 part=not_needed
Motion, the virtual clock, settle, advance and the load wait: the graph read moves no clock and drives no animation.
@task_item n=7 part=not_needed
Iconography of the browser toolbar, IconButton, favicons and bundle icons: the chunk touches no browser app icon.
@task_item n=8 part=not_needed
Desktop-native and mobile-native surface tokens, titlebar, url bar, back button, window and capture patterns: the chunk changes no shell or browser app surface.
@task_item n=9 part=not_needed
Depth strategy elevation values and hit-testing clip rules: the graph read does no hit-testing.
