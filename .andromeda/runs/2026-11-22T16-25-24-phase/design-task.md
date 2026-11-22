@task_prose part=chunk
A behaviour-preserving restructure: the driver's command module is split into modules so that no file of the split reads 800 code lines or more, with the driver crate's public paths unchanged.
Escher's additions in three grown upstream functions (the accessibility node builder, the key-or-input event handler, the presentational hints computation) move into functions of escher's own, one call left in each upstream body, so each reads upstream's own cognitive score.
No behaviour changes: the plan names, for each moved body, the existing checks that prove it; no new verb, field, carrier, log site, span, cause or exit.
At its HEAD it re-measures, by the audit's own recipes, the files over 800 code lines, the functions over cognitive 15 and the stand checks' duplication, as evidence for the gated dependency-and-CI-debts run.
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command module — the code behind the escher-session commands, the mcp role and the exit statuses; split into modules under the 800-line bar
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree builder, which builds each node's accessibility entry, select state included — edits: the function build_accessibility_node, and a new escher-own function holding escher's additions
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the engine's keyboard and text-input event handling, select open and list keys included — edits: the function handle_key_or_input_event, and a new escher-own function holding escher's additions
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: the engine's presentational hints computation from HTML attributes (font face and size, table cell spacing and padding, body margins) — edits: the function compute_presentational_hints, and a new escher-own function holding escher's additions
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the codebase-health audit's recipe that reads files over 800 code lines and functions over cognitive 15
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the stand-test reading recipe that measures the stand checks' duplication against the bar under 47 / 550
@task_item n=1 part=needs
The restructure's invariant: the driver's command module's bodies (escher-session commands and their answers, the mcp role's results, stderr errors and exit statuses) and escher's additions in the accessibility node builder (select accessibility state), the key-or-input event handler (select gestures and list keys) and the presentational hints computation (font element face and size, table spacing and valign hints, body margins) do exactly what they do now; the driver's schema, CLI, MCP and session checks and the accessibility, keyboard and presentational-hint checks hold it, and a moved body no check would catch is a gap the plan states.
@task_item n=2 part=needs
The size measure: no file of the split command module reads 800 code lines or more, read by the codebase-health audit's own recipe, not by a raw line count.
@task_item n=3 part=needs
The driver crate's public API stays as it is: every path a caller names, re-exports included, resolves as before, and no item changes visibility outside the crate; the split's new modules are internal.
@task_item n=4 part=needs
Each of the three upstream functions reads upstream's own cognitive score at the upstream base (19, 46, 167) by rust-code-analysis-cli; the one call left in each body adds no nesting or branch the tool would score.
@task_item n=5 part=needs
Upstream's own text in the three functions stays upstream's; only escher's additions move, into functions of escher's own reached by one call, the one-call form an inline-box push already takes, so a later upstream sync meets upstream's body plus one call.
@task_item n=6 part=needs
Escher's share of each function is read by a diff against upstream's text at the upstream base, not by blame; an addition interleaved with upstream's control flow that cannot leave by one call is a technical fork for the operator, or a founder fork if the way out touches upstream's text beyond one call.
@task_item n=7 part=needs
The cognitive complexity ceiling of 15 and the standing rule this chunk's wrap lands beside it: a fork addition to an upstream function over the ceiling goes into a function of escher's own, one call in upstream's body; the ten other fork-grown functions over the ceiling stay as they are.
@task_item n=8 part=needs
Nothing new: no verb, field, carrier, log site, span, cause or exit is added, and nothing an existing check reads may move, including the driver's command-line and mcp output read by agents and its exit status table.
@task_item n=9 part=needs
The gated re-reading at this chunk's HEAD: files over 800, functions over cognitive 15 and the stand checks' duplication under 47 / 550, measured by the audit's own recipes with no fresh audit record, read by the gated dependency-and-CI-debts run beside the boundary audit and CI on that HEAD.
@task_item n=10 part=needs
The run constraint: every cargo run is held until the operator's quiet window; reading, research and the plan are done with no build, the size, cognitive and duplication readings need none, and the phase stops at the baselines.
@task_item n=1 part=not_needed
Brand identity and the empty anti-patterns, self-validation and design-decisions-log sections: no row there reaches a code restructure.
@task_item n=2 part=not_needed
Color palette rows (engine colour constants, select list colour pairs, color-scheme recascade, base colour config, colour tokens): the chunk moves no colour value or colour-scheme handling.
@task_item n=3 part=not_needed
Depth strategy rows (paint order, stacking contexts, hit-testing at paint clips, the select list painted after the root): the chunk touches no paint or hit-testing code.
@task_item n=4 part=not_needed
Motion rows on overlay scrollbar fade timings, smooth scroll-behavior and wheel cancel, into-view scroll and animation on the virtual clock: no scroll or animation code is moved.
@task_item n=5 part=not_needed
Iconography rows (cursor icon crate, cursor resolution order, favicon from icon link, set_cursor): no cursor or favicon code is reached.
@task_item n=6 part=not_needed
Border radius and web-spa rows: no radius and no WASM canvas page is reached.
@task_item n=7 part=not_needed
WPT runner output and result colour rows: the WPT runner is not touched.
@task_item n=8 part=not_needed
Shell provider window controls (maximize, minimize, decorations toggle, window drag), dioxus-native config (alpha mode, font context, WASM font context) and the JS window object sizes: none is reached by the moved bodies.
@task_item n=9 part=not_needed
Typography rows on system fonts in tests, bundled WASM font context, unavailable-family fallback, generic-family sans and document.fonts: the chunk changes no font loading or font context.
