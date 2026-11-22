@task_prose part=chunk
A behaviour-preserving restructure: the escher-driver command module is split into modules so no file of the split reads 800 code lines or more, with the escher_driver public API unchanged.
Escher's additions in three fork-grown upstream blitz-dom functions (build_accessibility_node, handle_key_or_input_event, compute_presentational_hints) move into escher's own functions, one call left in each upstream body.
Each of the three upstream functions then reads upstream's own cognitive score at 547d7514b (19, 46, 167) by rust-code-analysis-cli; upstream's own text stays as it is.
No new behaviour, verb, field, carrier, log site, span, cause or exit; the plan names, per moved body, the existing checks that prove it unchanged.
At its HEAD the chunk re-measures, by the audit's own recipes, the files over 800 code lines, the functions over cognitive 15 and the stand checks' duplication (bar under 47 / 550), as evidence for the gated v040-08 reading.
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the escher-driver command module — the driver's commands behind the escher_driver public API, held by the driver's schema, CLI, MCP and session checks — edits: the module split into child modules, each under 800 code lines
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the blitz-dom accessibility tree builder — edits: build_accessibility_node, escher's additions moved to escher's own function with one call left
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: blitz-dom keyboard event dispatch and its default actions — edits: handle_key_or_input_event, escher's additions moved to escher's own function with one call left
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: blitz-dom presentational hints, legacy attributes cascading at the presentational-hints level — edits: compute_presentational_hints, escher's additions moved to escher's own function with one call left
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the codebase-health audit's bars recipe — files over 800 code lines and functions over cognitive complexity 15, re-run at HEAD
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the stand checks' duplication reading (jscpd), bar under 47 / 550, re-run at HEAD
@task_item n=1 part=needs
The restructure's invariant: the driver command module's bodies (moved into child modules) and escher's additions in build_accessibility_node (the accessibility tree's node build), handle_key_or_input_event (keyboard and input event default actions) and compute_presentational_hints (legacy attributes as presentational hints) each do exactly what they did; the existing driver schema, command-line, MCP and session checks and the accessibility, keyboard and presentational-hint checks hold this, and a moved body no check would catch is a stated gap.
@task_item n=2 part=needs
The size measure the split is read by: no file of the split reads 800 code lines or more, counted by the codebase-health audit's own recipe.
@task_item n=3 part=needs
The cognitive complexity ceiling and how the three upstream functions are measured: each reads upstream's own cognitive score at 547d7514b by rust-code-analysis-cli, so the left call adds no nesting or branch; the other ten fork-grown functions over the ceiling stay as they are.
@task_item n=4 part=needs
The escher_driver public API stays the same: every path a caller names, re-exports included, resolves as before, no item changes visibility outside the crate, crate internals stay restricted, and no serde_json type appears on the public API.
@task_item n=5 part=needs
Where escher's additions to upstream code live: in functions of escher's own beside upstream's body, reached by one call, upstream's own text left as upstream's so a later upstream sync (two-parent merge from the merge base) meets upstream's body plus one call; direct engine changes stay flagged as escher's and upstreamable.
@task_item n=6 part=needs
How tests reach moved units: unit-test modules that move keep their test paths, split test modules stay children of their module, and the driver's unit tests stay in path children of what they test.
@task_item n=7 part=needs
The driver's closed sets the split must not widen: the validated verb set and the verb table's order, the refusal causes closed at eight, the command span and its fields, and the driver's spans-only, side-effect-free library rule — no verb, field, carrier, span, cause or exit added.
@task_item n=8 part=needs
Feature and platform gating of the moved helpers: escher's functions stay under the same cargo features and cfg gates as the code they came from (the accessibility feature gating, desktop-only cfg lists, platform gating), and blitz-dom tracing stays gated per call site with no new log site.
@task_item n=9 part=needs
The re-measure at HEAD by the audit's own recipes with no fresh audit record: files over 800, functions over cognitive 15, and the stand checks' duplication under 47 / 550; these readings feed the gated chunk's v040-08 reading and claim no capability themselves.
@task_item n=10 part=needs
The keyboard default actions escher added stay in their order when moved to a helper: Enter and Space activation, select list keys, range keyboard stepping, and handlers running before the default action, held by the existing keyboard checks.
@task_item n=11 part=needs
The accessibility node build escher added stays as it reads when moved to a helper: ARIA states, descriptions, native checked and toggled states, author ids and node names, held by the existing accessibility tree checks.
@task_item n=12 part=needs
The gates the moved code must still pass: formatting and clippy with lints allowed locally, rustdoc warnings as errors, and the per-crate line coverage floors of escher-driver and blitz-dom.
@task_item n=13 part=needs
Every cargo run is held until the operator's quiet window: reading, research and the plan run with no build, and the bars, cognitive and duplication readings need no cargo build.
@task_standing n=1
convention: each new module file of the split carries module docs and item docs, with section delimiter comments where the module keeps sections
home: conventions
@task_standing n=2
convention: unit tests sit at the end of their source file, kept as child test modules of the module they test
home: conventions
@task_standing n=3
convention: each expect or unwrap, unsafe block and local lint allow moved into a new function or module keeps its justification beside it
home: conventions
@task_item n=1 part=not_needed
The WPT runner's tooling and outputs (result kinds, statuses, reftest images, diff-to-PR script, repository dispatch): the chunk runs and changes none of them.
@task_item n=2 part=not_needed
The browser app and its persistence, history store, about pages and toolbar: no browser code is reached.
@task_item n=3 part=not_needed
Paint and rendering decisions (paint scene, scrollbars, select list painting, renderer choice, screenshots on the CPU renderer): no paint code moves.
@task_item n=4 part=not_needed
Layout engine decisions over Taffy, inline layout, tables as grid, text decoration and scroll geometry: no layout code moves.
@task_item n=5 part=not_needed
The JavaScript runtime and script contracts (Boa, script globals, script fetcher, microtask drains): the chunk touches no script code.
@task_item n=6 part=not_needed
CI infrastructure of its own: skia archive verification, the package-install script, actions pinning, CI caches, publish workflows and the Nix flake; the chunk adds or edits no CI job.
@task_item n=7 part=not_needed
Cold-agent and agent-run tooling, their events, exit codes and verdict contracts: no cold-agent run is part of this chunk.
@task_item n=8 part=not_needed
The name registers (binaries, crates, environment variables, the MCP server name): the chunk adds no binary, crate, variable or server.
@task_item n=9 part=not_needed
Stack choices and dependency decisions (crate picks, version pins, workspace dependencies, cargo-deny audit): no dependency is added, removed or moved.
@task_item n=10 part=not_needed
The capability statement and the root README's counts: no capability is claimed and no master row lands in this chunk's plan.
@task_item n=11 part=not_needed
The escher-telemetry bootstrap and its sink, scrub and line format: the chunk adds no log line and edits no telemetry code.
