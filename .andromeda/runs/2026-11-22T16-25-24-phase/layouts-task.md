@task_prose part=chunk
A behaviour-preserving restructure: the driver's command module is split into modules so that no file of the split reads 800 code lines or more, with the driver crate's public paths unchanged.
Escher's additions in three grown upstream functions of blitz-dom (the accessibility node builder, the key-or-input event handler, the presentational-hints computation) move into functions of escher's own, one call left in each upstream body, each body then reading upstream's own cognitive score.
No new behaviour; the plan names, for each moved body, the existing checks that prove it unchanged.
Its gates re-measure at its HEAD, by the audit's own recipes, the files over 800 code lines, the functions over cognitive 15 and the stand checks' duplication, as evidence for the gated dependency-and-CI-debts chunk.
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command module, which serves escher-session's verbs and answers — edits: split into modules, each under 800 code lines
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: blitz-dom's accessibility tree building — edits: function build_accessibility_node, escher's additions moved into a function of escher's own
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: blitz-dom's keyboard and text input event handling — edits: function handle_key_or_input_event, escher's additions moved into a function of escher's own
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: blitz-dom's presentational hints from element attributes into style — edits: function compute_presentational_hints, escher's additions moved into a function of escher's own
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the codebase-health audit's bar reading — files over 800 code lines and functions over cognitive 15, re-measured at HEAD
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the stand checks' duplication reading, bar under 47 / 550, re-measured at HEAD
@task_item n=1 part=needs
The restructure's invariant: the driver's command module (escher-session's verb dispatch, answers, refusals, exit codes and MCP tools it serves) and escher's additions to accessibility node building, key-or-input event handling and presentational hints keep doing exactly what they do; nothing an existing check reads may move; held by the driver's schema, CLI, MCP and session checks and by the accessibility, keyboard and presentational-hint checks, named per moved body in the plan, with a body no check would catch stated as a gap.
@task_item n=2 part=needs
The size measure: every file of the split reads under 800 code lines by the audit's own recipe, not by any other line count.
@task_item n=3 part=needs
The driver crate's public API stays the same: every path a caller names, re-exports included, resolves as before, and no item changes visibility outside the crate; the split's new modules expose nothing new.
@task_item n=4 part=needs
The cognitive measure: each of the three upstream functions reads upstream's own score at 547d7514b (19, 46, 167) by rust-code-analysis-cli; the one call left in each body adds no nesting or branch.
@task_item n=5 part=needs
Upstream's own text in the three functions stays upstream's; only escher's additions move, so a later upstream sync meets upstream's body plus one call; the fork's share is told by a diff against upstream's text at 547d7514b, not by blame.
@task_item n=6 part=needs
Escher's additions stand in functions of escher's own, one call in the upstream body, in the existing one-call form of push_inline_box.
@task_item n=7 part=needs
No new behaviour, verb, field, carrier, log site, span, cause or exit is added by either part.
@task_item n=8 part=needs
The other ten fork-grown upstream functions over the cognitive ceiling stay untouched.
@task_item n=9 part=needs
The gated re-read: no fresh audit record and no capability claimed; this chunk's re-measured readings at its HEAD are the evidence the gated dependency-and-CI-debts run reads for v040-08's four clauses.
@task_item n=10 part=needs
No master row lands in this chunk's plan; the standing rule (a fork addition to an upstream function over the cognitive ceiling goes into a function of escher's own, one call in upstream's body) is landed by the wrap as an architecture decision beside the cognitive-complexity ceiling.
@task_item n=11 part=needs
Run constraint: every cargo run is held until the operator gives a quiet window; reading, research and plan with no build; the phase stops at the baselines and tells the operator.
@task_item n=12 part=needs
Fork routing: a new founder fork goes to a founder-forks.md file and stops; technical forks, such as an addition interleaved with upstream's control flow, go to the operator by dialog.
@task_item n=1 part=not_needed
Stand stable element ids contracts of surface-desktop-native: the chunk changes no stand page markup or id.
@task_item n=2 part=not_needed
ShellProvider window chrome contracts: the chunk touches no window drag, maximize, minimize, close or decoration control.
@task_item n=3 part=not_needed
The stand app's own page-building decisions (stand accent colour, fixed document skeleton, headless stand pinned viewport, load page network provider, order page controls, flight type select boot reading, session roles sharing the stand boot): the chunk authors no stand page.
@task_item n=4 part=not_needed
The cold-agent.sh and agent-run.sh pipe contracts and the cold-agent prompt decisions: the chunk changes no script pipe or agent prompt.
@task_item n=5 part=not_needed
The multi_doc and paint_bench command lines: bench binaries outside the driver and the three upstream functions.
@task_item n=6 part=not_needed
The absence of arbitrary index.html templates: the chunk adds no document template.
@task_item n=7 part=not_needed
surface-mobile-native and surface-web-spa: unmeasured areas with no rows the chunk reaches.
