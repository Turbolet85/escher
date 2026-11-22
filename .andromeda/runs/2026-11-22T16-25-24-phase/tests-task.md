@task_prose part=chunk
A behaviour-preserving restructure: the driver's command module is split into modules so no file of the split reads 800 code lines or more.
Escher's additions in three grown upstream blitz-dom functions (accessibility node build, key or input event handling, presentational hints) move into escher's own functions, one call left in each upstream body.
Each of the three upstream functions then reads upstream's own cognitive score at 547d7514b (19, 46, 167); no capability is claimed and no master row lands.
It re-measures at its HEAD, by the audit's own recipes, the files over 800, the functions over cognitive 15 and the stand checks' duplication (bar under 47 / 550), for the gated dependency-and-CI-debts run to read.
Every cargo build or test run is held until the operator gives a quiet window; reading, research and the plan run with no build.
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command module of escher_driver, behind the public escher_driver API — edits: split into modules, each under 800 code lines
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: blitz-dom's accessibility tree builder, an upstream function grown by escher's additions — edits: build_accessibility_node
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: blitz-dom's keyboard and text-input event handling, an upstream function grown by escher's additions — edits: handle_key_or_input_event
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: blitz-dom's presentational hints computed into style, an upstream function grown by escher's additions — edits: compute_presentational_hints
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the codebase-health audit's recipe reading files over 800 code lines and functions over cognitive complexity 15, re-run at HEAD
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the stand checks' duplication reading (jscpd), bar under 47 / 550, re-run at HEAD
@task_item n=1 part=needs
Restructure invariant: the driver command module's bodies (its verbs and their answers as the driver's schema, CLI, MCP and session checks read them) and escher's additions in the accessibility node build, the key or input event handler and the presentational hints computation move, and each does what it did; the existing driver schema, CLI, MCP and session checks and the accessibility, keyboard and presentational-hint checks hold it, named per moved body, and a body with no check that would fail when it moved is a gap the plan states.
@task_item n=2 part=needs
File-size bar: no file of the driver command module's split reads 800 code lines or more, measured by the codebase-health audit's own recipe.
@task_item n=3 part=needs
The escher_driver public API stays the same: every path a caller names, re-exports included, resolves as before, and no moved item changes visibility outside the crate; the split's modules expose nothing new.
@task_item n=4 part=needs
How a test reaches a moved unit: unit tests in inline test modules move with the code they test into the new module files, their test paths kept, with no visibility widened for a test to reach a unit.
@task_item n=5 part=needs
Cognitive-complexity ceiling: each of the three upstream functions reads upstream's own score at 547d7514b (19, 46, 167) by rust-code-analysis-cli; the one call added may add no nesting or branch, and the ceiling is read on the host.
@task_item n=6 part=needs
Upstream's own text in the three functions stays upstream's; only escher's additions move, into functions of escher's own, so a later upstream sync meets upstream's body plus one call (as the existing one-call precedent for inline boxes already is); where an addition is interleaved with upstream's control flow, it goes to the operator as a technical fork, or to a founder-forks file if only upstream's text beyond one call can free it.
@task_item n=7 part=needs
The other ten fork-grown upstream functions over the cognitive ceiling stay as they are, and upstream-authored oversize files and oversize integration-test targets kept whole are no part of the split.
@task_item n=8 part=needs
Nothing is added: no new behaviour, driver verb, field, carrier, log site, span, refusal cause or exit code; the command span and the driver's per-call log line stay as they are written.
@task_item n=9 part=needs
Re-measure at HEAD with no build: files over 800 and functions over cognitive 15 by the audit's recipe, the three functions by rust-code-analysis-cli, and the stand checks' duplication under 47 / 550 by the stand reading (upstream's duplicated test pairs kept as upstream's); these readings are the evidence the gated run reads for v040-08's clauses, beside the boundary audit record and CI on that HEAD.
@task_item n=10 part=needs
Every cargo build, test, clippy or mutation run is held until the operator's quiet window (another project times a mutation tier on the host); the phase stops at the baselines and tells the operator.
@task_item n=11 part=needs
The moved code passes the build's gates on that HEAD: clippy over the workspace and over upstream blitz-dom, the minimum supported Rust version, the per-crate coverage floors, and the fast and slow CI legs green as a whole, not per package.
@task_item n=12 part=needs
Accessibility checks keep their leg: the checks proving the moved accessibility additions stay in the a11y leg they run in, and stand checks run under the zero-failure budget over their repeat rounds.
@task_item n=13 part=needs
A mutation reading of a moved unit, if the plan takes one, is read per unit on the dev host only, with the scoped run filter anchored on the moved unit's verb, and only in the quiet window.
@task_standing n=1
convention: a new Rust unit test sits in an inline test module of the source file it tests, on the built-in test harness
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=2
convention: a new test is controlled red-before-green: it is seen failing against a broken unit before it passes
home: unit-test-strategy
@task_standing n=3
convention: a new test pins its expected value as a literal and its failure prints no argument value
home: unit-test-strategy
@task_standing n=4
convention: a new test's data is stated in code, nothing seeded, with shared fixtures in one module
home: 3-test-harness-contract · 7-test-data-fixtures
@task_item n=1 part=not_needed
The cold-agent pipe: its verbs, events, exit codes, verdict fields, isolation and live-run rules; the chunk runs no cold agent and changes none of its scripts.
@task_item n=2 part=not_needed
The agent-run five-command contract: its verbs, selections, events, state and exit codes; the chunk changes no harness script.
@task_item n=3 part=not_needed
The WPT runner: its CLI, outputs, status set, pinned commit, Thai font and timeout multiplier; the chunk changes no runner and no WPT pin.
@task_item n=4 part=not_needed
CI workflow plumbing: SHA-pinned actions, secrets, permissions, caches, trigger branches, bounded package installs and the apt-install script's exit codes; the chunk adds or edits no CI job or script.
@task_item n=5 part=not_needed
The skia archive witness and the matrix build of skia from the verified archive; that is the dependency-and-CI-debts chunk's subject, whose reds do not intersect this chunk.
@task_item n=6 part=not_needed
Harness contrast and lint-defect contracts, the contrast formula and required ratio; the chunk moves no harness reading.
@task_item n=7 part=not_needed
Mocking and stubbing discipline: network stand-ins, stand timer ticks, the virtual clock's advance and the mocking-library absence; the chunk adds no stand-in.
@task_item n=8 part=not_needed
End-to-end rows on painting and rendering: the select list painted by the scene painter, buffered render on the Dioxus bridge, scrollbar thumbs and fade, the font-embolden witness; the chunk touches none of them.
@task_item n=9 part=not_needed
The coverage exclusion of the keyboard compatibility module; the chunk does not touch that module or its exclusion.
