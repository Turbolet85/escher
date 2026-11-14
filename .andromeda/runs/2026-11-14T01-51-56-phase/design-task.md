@task_prose part=chunk
Builds seeded randomness (v030-13): a run of the driver, a held Session and the test harness it holds, is given one seed, and every source of randomness its app and page can read draws from that seed.
The app-facing source: a Dioxus app hosted by the driver reads randomness through one engine-provided source fed by the run's seed, not from the host.
The page-facing source: a page script's Math.random in a seeded run draws from the run's seed instead of the host thread RNG.
The proof: an integration check, in both layout modes, that two runs of one seed read the same snapshot text and diffs at every step of a scripted sequence on an app that reads randomness, and that two different seeds read a difference.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the driver's capability statement and its pin, the self-description of what the driver offers — edits: the seeded-run capability entry and its pin
@task_item n=1 part=needs
The seed's entry point on the driver: Session::start taking a seed, or a builder beside with_time, so a held session and the harness it holds are started from one seed.
@task_item n=2 part=needs
The test harness's options (HarnessOptions) carry the run's seed to the document and app it builds.
@task_item n=3 part=needs
The engine-provided randomness source for a Dioxus app: a root context or hook in the dioxus-native integration that an app hosted by the driver reads, seeded from the run.
@task_item n=4 part=needs
The page script runtime's Math.random: replaced in the script bootstrap so a page script in a seeded run draws from the run's seed rather than the host's RNG.
@task_item n=5 part=needs
Determinism of state a run returns: two runs given one seed read the same snapshot text and the same snapshot diffs at every step.
@task_item n=6 part=needs
If start takes a seed, the escher-session start command's arguments gain a seed argument on the ratified carriers (command line, socket, MCP role, kept check); the seed value's kind is a new argument kind on a ratified carrier.
@task_item n=7 part=needs
The kept check's recording and replay: if the seed is recorded, a kept check carries the seed of the run it recorded and replays it from that seed.
@task_item n=8 part=needs
The help command's self-description of the driver names the seed argument and seeded runs where start takes a seed.
@task_item n=9 part=needs
A refused or malformed seed argument answers with the driver's usage-error and exit-status conventions, the exit status read from the schema's table.
@task_item n=10 part=needs
A seed's value appears in no log line, stderr line or session error by the content rule, and no new listener, port, socket or environment read is added to carry it.
@task_item n=11 part=needs
The seed sits beside the run's virtual clock (with_time) as run-start configuration: seeding moves no clock and changes no settle, advance or load-wait behaviour.
@task_item n=12 part=needs
The stand or a fixture app that reads randomness, served as a stand task or fixture for the proof; the six existing served stand tasks keep their answers.
@task_item n=13 part=needs
The integration proof runs in both layout modes and fails if two different seeds read the same state, so it cannot pass vacuously.
@task_item n=14 part=needs
Driver command-line and MCP output stay read by agents: any new seed field in an answer is plain text with no colour.
@task_standing n=1
convention: each new escher-session argument or command answer follows the command table, the help self-description and the MCP role's result conventions
home: surface
@task_standing n=2
convention: each new usage or session error a new argument can raise carries its exit status from the schema's table and its stderr line shape
home: surface-cli
@task_standing n=3
convention: each new run-start configuration a test or harness passes is applied under the one virtual clock, never the host's time
home: motion-calibrated-to-expression-level-unrecorded · motion
@task_item n=1 part=not_needed
Typography and font rows (font context, bundled fonts, system fonts in tests, fonts-unavailable screenshot refusals): bundled fonts are the next entry, v030-14, not this chunk.
@task_item n=2 part=not_needed
Color palette rows: the chunk adds no colour, colour scheme or engine colour.
@task_item n=3 part=not_needed
Spacing rows: no margins, cell spacing or list placement change.
@task_item n=4 part=not_needed
Depth strategy rows: no paint order, z-index, stacking or hit-testing change.
@task_item n=5 part=not_needed
Border radius and brand identity: no recorded intent and nothing the chunk touches.
@task_item n=6 part=not_needed
Iconography rows: cursor icons, cursor resolution and favicons are untouched.
@task_item n=7 part=not_needed
Surface web-spa: the browser app is not a seeded run.
@task_item n=8 part=not_needed
Select list rows (list events, accessibility of a select, list rows in snapshot and screenshot, select colours and placement): no select behaviour changes.
@task_item n=9 part=not_needed
WPT runner output and result colour rows: the WPT runner is not reached.
@task_item n=10 part=not_needed
Shell provider rows (window drag, minimize, maximize, decorations, set_cursor): the windowed shell is not a seeded run.
