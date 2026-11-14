@task_prose part=chunk
Gives a driver run (a held Session and the test harness it holds) one seed, from which every randomness source the run's app and page can read draws, so two runs of one seed read the same state at every step.
Adds an engine-provided randomness source a hosted Dioxus app reads in place of the host, and seeds a page script's Math.random in a seeded run.
Lets the seed enter through the session start or a builder beside the virtual-clock one, and, if the seed is recorded, through the kept check's recording and replay and the driver's self-description.
Proves it with an integration check in both layout modes on an app that reads randomness: two runs of one seed read the same snapshot text and diffs at every step, two runs of different seeds read a difference.
@task_file n=1 path=packages/escher-driver/src/session.rs mark=edits
what: the driver's held session, the run it starts and the builders that configure it — edits: Session::start, a seed builder beside with_time
@task_file n=2 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's verb table, every verb with its arguments and result fields — edits: the start verb's entry, if it takes a seed argument
@task_file n=3 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording written as a replayable kept check — edits: the recorded check, if the seed is recorded
@task_file n=4 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of a kept check against a fresh run — edits: the replay, if the seed is recorded
@task_file n=5 path=packages/escher-driver/src/describe.rs mark=edits
what: help's self-description of the driver, its verbs and their arguments — edits: help's self-description
@task_file n=6 path=docs/capabilities.md mark=edits
what: the published list of what the driver offers, and its pin to help's self-description
@task_file n=7 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the test harness that hosts an app headlessly for a run — edits: HarnessOptions
@task_file n=8 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: the Dioxus document hosting a native app, its root context — edits: a root context or hook an app reads its randomness from
@task_file n=9 path=packages/blitz-vibey-script/src/runtime.rs mark=edits
what: the page script runtime and its bootstrap scripts — edits: the bootstrap scripts, where Math.random is replaced
@task_file n=10 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven GUIs stand the driver serves, its tasks and surfaces — edits: a fixture or a stand surface that reads randomness, for the proof
@task_item n=1 part=needs
The run's seed as a value a session is started with: one seed per run, every randomness source the app and page read drawing from it, two runs of one seed reading the same state at every step.
@task_item n=2 part=needs
The session's start verb on the driver's command line, socket and MCP carriers: a seed argument, if added, is a new argument kind on a ratified carrier, and its form follows the existing command form of the escher-session command line.
@task_item n=3 part=needs
The MCP server's tools listing: a start tool that takes a seed lists that argument as the verb table does.
@task_item n=4 part=needs
Help's self-description of the driver: a seed argument or builder is described there and the published capability list stays pinned to it.
@task_item n=5 part=needs
The session recording as a replayable kept check: if the seed is recorded, the recorded check file carries it and the replay re-runs under it, so a kept check replays the run it recorded.
@task_item n=6 part=needs
The held instance reads as a fresh boot and the session roles share the stand boot: a seeded run's boot draws from the seed, so a fresh boot of one seed reads the same state as another.
@task_item n=7 part=needs
The served stand tasks: none reads randomness today, so the proof needs a fixture or a new stand surface that does; a new stand surface is served and booted like the others.
@task_item n=8 part=needs
Indirect, through the host log and every driver answer: a seed's value is written into no log line, no log field and no answer echo by the content rule — caller input is never written back.
@task_item n=9 part=needs
Indirect, through the driver's carriers: no new listener, port, socket or environment read carries a seed.
@task_item n=10 part=needs
The snapshot answer's text and diffs as the state two runs compare at every step of a scripted sequence, in both layout modes.
@task_item n=11 part=needs
The proof's own check: two runs of one seed read the same state, and two runs of different seeds read a difference, so the check cannot pass vacuously.
@task_item n=12 part=needs
Indirect, through a page script document: Math.random in a seeded run draws from the run's seed, not the host thread's generator.
@task_standing n=1
convention: every new stand control or surface carries a stable keyed id set as an attribute, names its role and state by attributes alone, and lints clean at boot
home: surface-desktop-native
@task_standing n=2
convention: every new verb argument is listed in the verb table, help's self-description and the MCP tools listing alike
home: surface-cli
@task_standing n=3
convention: every new log line or answer carries no caller-supplied value back out
home: surface-cli
@task_item n=1 part=not_needed
The windowed shell's window chrome controls (drag, maximise, minimise, close, decorations): the windowed shell is not a seeded run.
@task_item n=2 part=not_needed
The agent-run and cold-agent test pipes and their exits, usage and run directories: the chunk changes no agent pipe.
@task_item n=3 part=not_needed
The multi_doc and paint_bench example command lines: neither is a driver run.
@task_item n=4 part=not_needed
Scrolling, clipping and visible-region rules, open list row bounds and the CRUD list keyboard region: the chunk changes no layout or input routing.
@task_item n=5 part=not_needed
The stand accent colour and the headless stand pinned viewport: the chunk changes no visual styling or viewport.
@task_item n=6 part=not_needed
The fonts-unavailable refusal and any bundled-font rule: bundled fonts are the next entry, not this chunk.
@task_item n=7 part=not_needed
The coverage role, graph answer and screenshot answer: the chunk changes none of what they return.
@task_item n=8 part=not_needed
Arbitrary index.html templates for a document: the chunk adds no document template.
@task_item n=9 part=not_needed
The mobile-native and web-spa surfaces: the browser app and mobile layouts are not seeded runs.
