@task_prose part=chunk
A behaviour-preserving restructure: the driver's command module is split into modules so that no file of the split reads 800 code lines or more, with the session library's public paths and visibility unchanged.
Escher's additions inside three upstream engine functions (the accessibility node build, the key-or-input event handler, the presentational hints computation) move into functions of escher's own, one call left in each upstream body, so each reads upstream's own cognitive score (19 · 46 · 167).
No new behaviour, verb, field, carrier, log site, span, cause or exit; the plan names, for each moved body, the existing checks that prove it unchanged.
At its HEAD the chunk re-measures, by the audit's own recipes, the files over 800 lines, the functions over cognitive 15 and the stand checks' duplication (bar under 47 / 550); every cargo run is held until the operator's quiet window.
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command module — the session library's commands behind its CLI, MCP and schema surfaces — edits: split into modules, each under 800 code lines
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree build over AccessKit (roles, names, ARIA states, hidden content) — edits: build_accessibility_node, escher's additions moved into escher's own functions
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the engine's keyboard event handling (activation, Tab focus moves, select and range input keys) — edits: handle_key_or_input_event, escher's additions moved into escher's own functions
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: the engine's presentational hints from element attributes into style — edits: compute_presentational_hints, escher's additions moved into escher's own functions
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the codebase-health audit's recipe reading files over 800 code lines and functions over cognitive 15
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the stand checks' duplication reading, bar under 47 / 550
@task_item n=1 part=needs
The restructure's invariant: the units moved — the driver's command module (its commands as the CLI, MCP, schema and session surfaces serve them), escher's additions to the accessibility node build (roles, names, ARIA states, hidden and inert content), to the key-or-input handler (Enter and Space activation, Tab and Shift+Tab focus moves, select and range input keys) and to the presentational hints — each does what it did; held by the driver's existing schema, CLI, MCP and session checks and the existing accessibility, keyboard and presentational-hint checks, named per body; a moved body no existing check would catch is a gap the plan states.
@task_item n=2 part=needs
The accessibility tree stays built through the document's own method and the document trait: escher's helpers split out of the node build are called from that build and live with it, not in a new entry point.
@task_item n=3 part=needs
Accessibility code behind its feature: helpers moved out of the node build stay behind the same accessibility feature gate the build stands behind, and the session library still always enables accessibility.
@task_item n=4 part=needs
One read per ARIA attribute: the moved additions keep each ARIA attribute read once, in the one place it is read now, and do not split one attribute's read across the helper and the upstream body.
@task_item n=5 part=needs
Role mapping and name precedence stay single-sourced: a moved addition carries its role, name or state rule whole into the helper, never a second copy beside upstream's.
@task_item n=6 part=needs
The size measure: no file of the command module's split reads 800 code lines or more, read by the codebase-health audit's own recipe at the chunk's HEAD.
@task_item n=7 part=needs
The cognitive measure: each of the three upstream functions reads upstream's own score at 547d7514b (19 · 46 · 167) by rust-code-analysis-cli; the one call left in each body adds no nesting or branch to the scoring.
@task_item n=8 part=needs
Upstream's text in the three functions stays upstream's: only escher's additions move, told apart by a diff against upstream's text at 547d7514b, not by blame; an addition that cannot leave without changing upstream's lines beyond one call is a fork (technical to the operator, founder's to founder-forks.md).
@task_item n=9 part=needs
The session library's public API is unchanged: every path a caller names, re-exports included, resolves as before and no item changes visibility outside the crate; tests and consumers reach the moved commands by the same paths.
@task_item n=10 part=needs
No new log site or span: the driver's command spans and the escher log lines keep their names, fields and targets through the split, under the existing target allowlist and redaction.
@task_item n=11 part=needs
No new driver verb, field, carrier, refusal cause or exit: the snapshot, actions answer, screenshot and refusal surfaces the command module serves keep their shapes; the readers of one snapshot model stay readers of that one model.
@task_item n=12 part=needs
The stand checks' duplication is re-measured at HEAD by the stand reading recipe and stays under 47 / 550.
@task_item n=13 part=needs
Existing accessibility assertions are Rust tests over the AccessKit tree run in the accessibility test leg; the checks the plan names for a moved accessibility body are those tests, re-run unchanged once the operator gives a quiet window.
@task_item n=14 part=needs
Run constraint: reading, research and the plan run with no cargo build; the readings (size, cognitive, duplication) need no build; the phase stops at the baselines and tells the operator.
@task_item n=1 part=not_needed
Accessibility tier, reach and capability statement decisions: the chunk claims no capability and changes no reach.
@task_item n=2 part=not_needed
Platform adapter rows (accesskit_xplat as adapter, no adapter on stand binaries, assistive technology reach, adapter creation order): the chunk adds and moves no adapter.
@task_item n=3 part=not_needed
Contrast measurement and its thresholds, the lint's contrast finding, stand accent colour and scrollbar thumb stroke: the chunk changes no colour reading, lint or paint.
@task_item n=4 part=not_needed
Open select list paint (engine colour constants, one row height, no motion, chosen-row colour pair): the chunk moves no painting code.
@task_item n=5 part=not_needed
Viewport scheme, zoom and hidpi reshaping decisions: the chunk touches no viewport or text shaping.
@task_item n=6 part=not_needed
Pointer timing rows (double click threshold, double click on the document's clock): the chunk moves no pointer handling.
@task_item n=7 part=not_needed
UI graph state registration contracts (use_graph_state, undeclared, unconfirmed): the chunk touches no app-side state registration.
@task_item n=8 part=not_needed
Cognitive accessibility of the stand (invalid state beyond colour or class, the shared Tab order table): the chunk changes no stand app.
@task_item n=9 part=not_needed
The sections with no recorded intent (SLO invariants and budgets, anti-patterns, decisions log): nothing there to bind.
