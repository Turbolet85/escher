@task_prose part=chunk
Splits the driver's command module into modules so no file of the split reads 800 code lines or more, with every public path of the escher_driver crate resolving as before and no item changing visibility outside the crate.
Moves escher's additions out of three grown upstream engine functions (accessibility node building, key and input event handling, presentational hints) into functions of escher's own, leaving one call in each upstream body and upstream's own text untouched.
Behaviour is unchanged: no new verb, field, carrier, log site, span, refusal cause or exit; the plan names the existing checks that prove each moved body.
Re-measures at its HEAD, by the audit's own recipes and with no build, the files over 800 lines, the functions over cognitive 15 (the three upstream functions to read upstream's own scores 19, 46 and 167) and the stand checks' duplication (bar under 47 / 550).
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command module: the driver verbs, the single command validation gate, the command span and the refusals of driver calls — edits: split into modules under 800 code lines, public paths kept by re-export
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree builder, the nodes it hands the accessibility adapter — edits: the function build_accessibility_node, escher's additions moved into a function of escher's own
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the engine's key and input event handling for focused controls — edits: the function handle_key_or_input_event, escher's additions moved into a function of escher's own
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: the engine's presentational hints, markup attributes read into style — edits: the function compute_presentational_hints, escher's additions moved into a function of escher's own
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the audit's size and cognitive bars reading: files over 800 code lines and functions over cognitive 15
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the audit's duplication reading of the stand checks, bar under 47 / 550
@task_item n=1 part=needs
The restructure's invariant: the driver's command module (driver verbs, the one validation gate, the command span, refusals) split into modules, and escher's additions in accessibility node building, key and input event handling and presentational hints moved into escher's own functions, each doing what it did; held by the existing driver schema, CLI, MCP and session checks and the accessibility, keyboard and presentational-hint checks, named per moved body, a body no check would catch moving stated as a gap.
@task_item n=2 part=needs
The split keeps one validation path for every carrier: the single command validation gate stays one unit that every verb and carrier still passes, not copied or forked across the new modules, and the fixed order of target refusals stays in that one place.
@task_item n=3 part=needs
The split keeps one command span per call: no new module opens a span, adds a span field, or adds a log site; the span stays content-free and a refusal is still logged by its cause name only.
@task_item n=4 part=needs
The closed driver verb schema and the closed refusal cause set gain nothing: the split adds no verb, argument kind, answer field, refusal cause, session error, carrier or exit code, and the command schema still reads no clock.
@task_item n=5 part=needs
Where a moved driver unit may live and what it may expose: new modules are private to the crate, the public API is kept only by re-export at the same paths, and no item gains visibility outside the crate, so no process-local handle or NodeId gains a new exit.
@task_item n=6 part=needs
The driver code after the split still installs no subscriber and reads no environment beyond RUST_LOG; a moved unit adds no env read, print or panic on call-supplied input.
@task_item n=7 part=needs
The helper moved out of accessibility node building keeps accessible names, descriptions and stable ids out of logs, sets the stable id as the accessibility author id as before, and keeps the content classes on the adapter crossing; it adds no log site and no panic.
@task_item n=8 part=needs
The helper moved out of key and input event handling keeps every key rule it carries (enter and space activation, select and range keys, typeahead kept private, masked password input) acting at the same point of upstream's control flow, reached by one call that adds no branch or nesting to upstream's body.
@task_item n=9 part=needs
The helper moved out of presentational hints keeps reading markup attributes by a typed parse, without log or panic, with the select size hint cap as before.
@task_item n=10 part=needs
Upstream's own text in the three functions stays upstream's: only escher's additions move, so a later upstream sync meets upstream's body plus one call; an addition interleaved with upstream's control flow that cannot leave by one call is a fork for the operator, or the founder's where only touching upstream's text beyond one call would serve.
@task_item n=11 part=needs
The measure the chunk is read by: code lines per file by the audit's own bars recipe (every file of the split under 800), cognitive score per function by rust-code-analysis-cli (the three upstream functions at upstream's 19, 46 and 167), the stand checks' duplication under 47 / 550; these readings are the evidence the gated dependency-and-CI chunk reads, and no fresh audit record and no master row land here.
@task_item n=12 part=needs
How tests reach moved units: existing checks keep reaching driver units through the crate's unchanged public paths and engine behaviour through the same entry points; no check is rewritten to follow a moved unit and no new test, fixture, CI job or log line is added.
@task_item n=13 part=needs
Every cargo run (build, test, baseline) is held until the operator gives a quiet window; reading, research, the plan and the no-build measurements run first, and the phase stops at the baselines.
@task_item n=14 part=needs
The other ten fork-grown upstream functions over the cognitive ceiling stay as they are; the standing rule that a fork addition to an upstream function over the ceiling goes into escher's own function with one call in upstream's body is landed by the wrap as an architecture row, not by this chunk's plan.
@task_item n=1 part=not_needed
Data protection rows: browser history in sqlite, cookies and HTTP cache, TLS, clipboard and file dialogs, kept check files and their owner-only lifecycle, coverage line content; the chunk touches no stored data.
@task_item n=2 part=not_needed
Outbound network rows: outbound HTTP through blitz-net or reqwest, per-host concurrency, sub-resource GETs, abort signals, load waits, CORS, same-origin, CSP, request timeout and response size absences, and the opt-in CDP devtools server; no network path is reached.
@task_item n=3 part=not_needed
Script and resource loading rows: the script fetcher's URL schemes, file URL loading, JS fetch methods, iframe and CSS import nesting limits, stale iframe responses, stylesheet and font-face fetching, SVG href and mask fetches, HTML parsed without scripting, bootstrap native helpers.
@task_item n=4 part=not_needed
Dependency choice and audit rows: cargo-deny audit, advisory ignores, licence gate, engine dependency pins, git dependencies, workspace-inherited versions, skia archive and binaries download, base64, png and serde_json choices; the chunk adds, removes or bumps no dependency.
@task_item n=5 part=not_needed
Secret, token and release rows: CI secrets in GitHub Actions, read-only CI tokens, publish job guards, signing material removal, release publishing from upstream, live-run logins and credential probes; no CI workflow or secret is touched.
@task_item n=6 part=not_needed
Repository scripts and harness rows: cold-agent runs, agent-run, apt-install, the coverage floor checker, the build-directory sweep, and the WPT runner, quarantine and post-results workflow; none of these scripts is edited.
@task_item n=7 part=not_needed
Painting rows: password painted as bullets, select list colours and painting over the page, border radii scaling, inline-box heights, scrollbar fade; no paint code moves.
@task_item n=8 part=not_needed
Embedding and app rows: rdme raw HTML passthrough, Dioxus dangerous_inner_html, Dioxus-native JS evaluation, the Dioxus scheme and form data, external navigation to the system browser, the browser app's JavaScript opt-in.
@task_item n=9 part=not_needed
Engine error rows outside the moved bodies: error pages and their history and favicon skips, history store errors, NetHandler errors, CSSOM script API errors, JS module loading failures, WPT result kinds, parallel style traversal.
@task_item n=10 part=not_needed
Empty or unrecorded areas: the threat model summary's unmeasured items, compliance controls, the security anti-patterns list and the security decisions log, which hold no rows.
