@task_prose part=chunk
Seeded randomness (v030-13): a run of the driver (a held Session, and the test harness it holds) is given one seed, and every source of randomness its app and page can read draws from it.
A Dioxus app hosted by the driver reads its randomness through one engine-provided source (a root context or hook) fed by the run's seed, not from the host.
A page script's Math.random in a seeded run draws from the run's seed, replaced in the script engine's bootstrap.
It adds an integration check, both layout modes, that two runs of one seed read the same snapshot text and diffs at every step of a scripted sequence on an app that reads randomness, and that two different seeds read a difference.
The windowed shell and the browser app stay unseeded; the seed is recorded in the kept check only if the plan chooses to.
@task_file n=1 path=docs/capabilities.md mark=edits
what: the driver's capability statement, edited by hand, and its pin — edits: the entry for a run's seed
@task_item n=1 part=needs
The driver session's start: a seed given to a run through Session start or a builder beside its virtual-clock option, the session library that always enables accessibility.
@task_item n=2 part=needs
The test harness options: a seed carried into the harness a session holds, so the hosted document and app read it.
@task_item n=3 part=needs
The Dioxus document: a new engine-provided randomness source (a root context or hook) an app reads, fed from the run's seed.
@task_item n=4 part=needs
The page script engine: its bootstrap scripts replace Math.random so a page script in a seeded run draws from the run's seed.
@task_item n=5 part=needs
The verb table and the driver carriers (command line, socket, MCP, kept check): a seed argument on start is a new argument on a ratified carrier, and its wire form follows the carriers' keyed-field form.
@task_item n=6 part=needs
The kept check's recording and replay: if the seed is recorded, a replay of a kept check runs under the seed it recorded.
@task_item n=7 part=needs
The help verb's self-description and the hand-edited capability statement name the seed.
@task_item n=8 part=needs
The state the proof compares is the snapshot text form and the snapshot diff, read from the snapshot model taken from the accessibility tree; two runs of one seed must give equal text and equal diffs at every step.
@task_item n=9 part=needs
The proof's answers must not hang on per-process hash iteration order: any snapshot, diff or actions answer whose order follows a randomly seeded hash map would make two runs of one seed differ.
@task_item n=10 part=needs
The proof's negative half: two runs given different seeds read a difference, so the check cannot pass vacuously.
@task_item n=11 part=needs
The proof runs on an app that reads randomness: a fixture or a new stand surface in seven_guis, since the stand reads none today.
@task_item n=12 part=needs
No new log field, listener, port, socket or env read carries a seed's value: the escher log content rule keeps it out of every line, and a new span or event names no seed value.
@task_standing n=1
convention: a new stand surface or fixture states its expected roles and names once in the shared expectation tables, and every interactive node on it carries a name
home: screen-reader-support
@task_standing n=2
convention: a new stand surface meets text contrast as asserted on the stand and reads no lint finding at boot
home: visual-design-verification
@task_standing n=3
convention: a new stand surface's focus order is asserted against the shared tab-order table
home: cognitive-accessibility
@task_standing n=4
convention: a new stand check runs in three rounds and its accessibility assertions are Rust tests over AccessKit in the a11y test leg
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Platform adapter rows: the stand builds no platform adapter, and the windowed shell is no seeded run, so no adapter choice or default feature changes.
@task_item n=2 part=not_needed
ARIA role mapping and hidden-content rows: no role, hidden or inert rule changes for a seed.
@task_item n=3 part=not_needed
ARIA property reads and accessibility tree state contracts (aria-checked, aria-pressed, aria-selected and the like): the chunk adds no attribute read.
@task_item n=4 part=not_needed
Keyboard activation, select list keys, range input keys, click and focus decisions: no input path changes for a seed.
@task_item n=5 part=not_needed
UI graph state registration rows: starting in a state and auto-exploration are later readers of the seed, not this chunk's work.
