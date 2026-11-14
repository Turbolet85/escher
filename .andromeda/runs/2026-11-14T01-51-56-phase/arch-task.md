@task_prose part=chunk
Gives a run of the driver (a held Session and the harness it holds) one seed, from which every randomness source its app and page can read draws.
Adds an engine-provided randomness source a hosted Dioxus app reads, and makes a page script's Math.random in a seeded run draw from the seed.
Proves it with an integration check, both layout modes: two runs of one seed read the same snapshot text and diffs at every step, two seeds read a difference.
Carries the seed through start, the verb table, the kept check and help where it enters there, and updates the capability statement and its pin.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the capability statement of the driver, held to the driver schema by its pin — edits: the seeded-run capability and its pin
@task_item n=1 part=needs
A run's seed: Session::start, or a consuming builder method beside with_time, takes one seed, and every randomness source the run's app and page read draws from it, so two runs of one seed read the same state at every step.
@task_item n=2 part=needs
The app-facing source: a hosted Dioxus app reads its randomness through one engine-provided source, a root context or hook in dioxus-native-dom, never from the host.
@task_item n=3 part=needs
The page-facing source: the blitz-vibey-script JS bootstrap replaces Math.random in a seeded run so a page script draws from the run's seed.
@task_item n=4 part=needs
The test harness's options (HarnessOptions) carry the seed into the Harness the session holds, beside its one virtual clock.
@task_item n=5 part=needs
If start takes a seed: a new argument on the start verb across the ratified carriers (command line, socket wire, MCP tools, kept check), its argument kind, validation and refusal faults, and whether a seed argument widens the validated verb set or the session boundary under the ratification rules.
@task_item n=6 part=needs
If the seed is recorded: the kept check file's opening record and Check::replay carry it, so a kept check replays the run it recorded through the one run path.
@task_item n=7 part=needs
help's self-description, shared by command line and MCP, names the seed where start takes it, and the capability statement stays held to the driver schema.
@task_item n=8 part=needs
A seed's value stays out of every log line: the driver logs no content, the command span and tool_call span field no argument value, and telemetry scrubs what prints, unless a ratified decision says otherwise.
@task_item n=9 part=needs
No new listener, port, socket or environment-variable read carries a seed: the one local Unix-domain listener, no served network port, and the environment variables the binaries read stay as they are.
@task_item n=10 part=needs
The proof's subject: a fixture app or a seven_guis stand surface that reads randomness, since the six served stand tasks read none; a stand surface keeps the stand offline, its pinned environment and the session apps' one table.
@task_item n=11 part=needs
The compared state: snapshot text and snapshot diffs at every step of a scripted sequence, which carry no process-local values.
@task_item n=12 part=needs
The check cannot pass vacuously: two runs given different seeds read a difference.
@task_item n=13 part=needs
The windowed shell and the browser app are not seeded runs: the seed reaches only headless driver runs.
@task_item n=14 part=needs
The session library depends on no app and stays without side channels; the engine-provided source is reached by dioxus-native-dom root contexts, the way Dioxus services are provided.
@task_item n=15 part=needs
New public items (the seed parameter, the source, the harness option) carry item docs, and their builder methods follow the consuming builder naming.
@task_standing n=1
convention: a new integration test lives in blitz-tests, one target per file, one behaviour per file
home: design-philosophy · conventions
@task_standing n=2
convention: a new scenario runs in both layout modes
home: conventions
@task_standing n=3
convention: a new test has a descriptive name and asserts its fixture's condition first
home: conventions
@task_standing n=4
convention: expect and unwrap in new code carry a justification, and new modules and public items carry docs
home: conventions
@task_standing n=5
convention: new code keeps its crate's per-crate line coverage floor
home: stack-and-technologies
@task_standing n=6
convention: a new Dioxus fixture or example keeps the Dioxus example shape and registers its signals with the graph
home: conventions
@task_item n=1 part=not_needed
WPT runner, its result kinds, statuses, outputs, diff-to-PR tooling and repository dispatch: no WPT test or report changes.
@task_item n=2 part=not_needed
Select lists, form controls, form submission and their default actions: the chunk adds no control behaviour.
@task_item n=3 part=not_needed
CSS layout, painting, scrollbars, text decoration, tables, transforms and the absences of CSS features: the seed changes no style, layout or paint.
@task_item n=4 part=not_needed
The browser app's history, persistence, about pages, toolbar and publish job: the browser app is not a seeded run.
@task_item n=5 part=not_needed
Accessibility tree ARIA states and the platform AccessKit adapter: nothing the snapshot reads from them changes.
@task_item n=6 part=not_needed
Bundled fonts, font fallback and the fonts-unavailable refusal: bundled fonts are the next entry, not this chunk.
@task_item n=7 part=not_needed
Networking: blitz-net, the HTTP cache, per-host request caps, script fetching and URL scheme routing.
@task_item n=8 part=not_needed
Upstream sync, cherry-pick and deferred dependency-bump rows: the chunk takes nothing from another tree.
@task_item n=9 part=not_needed
Cold-agent and agent-run scripts, their events, exit codes and verdict fields: auto-exploration and replay as the seed's later readers are not this chunk's work.
@task_item n=10 part=not_needed
Start-in-a-state: the chunk adds no way to start a run in a given state.
