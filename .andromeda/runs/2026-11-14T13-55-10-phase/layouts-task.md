@task_prose part=chunk
Builds one auto-exploration command over a held app: from boot it enumerates the available actions and drives them breadth-first within a step budget, returning a closed report.
Each path is re-reached from a fresh session, the frontier ordered by path length; panics and refusals met on a step are caught and reported, and never stop the run.
Each explored path is left as an owner-only kept check, holding no screenshot step, that replays green.
The kept-check step format is versioned and can carry a verb with more than one target. Exploration reads only the verbs that the schema's verb table and the actions answer offer, so a later verb needs no reshaping.
Navigation and carriers are founder forks held at the most conservative reading: navigation is not followed, and the command runs in process only.
@task_item n=1 part=needs
The exploration command over a held app: starts from boot, drives every available action breadth-first until a step budget is spent, re-reaches each path from a fresh session, orders the frontier by path length, and returns a closed report.
@task_item n=2 part=needs
The available actions come only from the driver's own readers: the escher-session actions answer and the schema's verb table. A verb is driven only when the table holds it and the actions answer offers it on a node. Exploration's own code holds no closed list of verbs.
@task_item n=3 part=needs
Every step runs as a validated call through the one session run path, the way reach does. There is no second executor.
@task_item n=4 part=needs
A refusal met on a step is caught and reported by its closed cause, as in the escher-session refusals (unknown-verb, off-screen and the others). A panic raised inside a step is caught and reported as a closed word, and the path that led to it is kept. Neither ends the exploration.
@task_item n=5 part=needs
Each explored path is kept as a session recording that is a replayable check, in the existing recording format (check-{n}.json). It is owner-only: a 0600 file in a 0700 launcher-named directory. A group-open or other-open directory is refused as record-dir-not-private, and a failed write is reported as recording-failed. Every kept check replays green.
@task_item n=6 part=needs
An explored path holds no screenshot step, and the run leaves no pixel at rest.
@task_item n=7 part=needs
The kept-check step format is versioned: each kept check states its format version. A step with two targets is an addition to that format and does not redefine it. This chunk does not produce a two-target verb such as drag.
@task_item n=8 part=needs
Exploration reads the default seed alone, as every driver session does today, and takes no seed argument. A later seed argument is an addition to the command's arguments and needs no rework.
@task_item n=9 part=needs
Navigation is a founder fork, held provisionally at its most conservative reading. A step whose action would navigate the document (a link, or a form submission to another URL) is not driven, and exploration loads nothing new.
@task_item n=10 part=needs
Carriers are a founder fork, held provisionally at in-process only. The chunk adds no new verb to the escher-session command line, no new role like the coverage role, no new tool on the MCP server's tools list and nothing to the socket. Their help answer, exit codes and command form stay as they are.
@task_item n=11 part=needs
Untrusted input (the step budget and the recording directory) is refused as typed values and never reaches a panic path.
@task_item n=12 part=needs
Logging: no new log field carries content, and caller input is never written back. A span, if one is added, holds fixed words and counts only.
@task_item n=13 part=needs
The proof runs on the stand's served tasks (counter, cells, crud, flight-booker, load, timer) in both layout modes. Each session reads as a fresh boot of the held instance, and the session roles share the stand boot.
@task_item n=14 part=needs
New proof fixtures: one whose handler panics and one that holds a refusing control. Their controls are keyed by stable ids that are carried as attributes, and they lint clean at boot.
@task_standing n=1
convention: Each new test drives the stand only through its stable element ids, which are carried as attributes, and every control it reaches is keyed.
home: surface-desktop-native
@task_standing n=2
convention: Each new fixture or stand control states its name, role and states by attributes alone, and lints clean at boot.
home: surface-desktop-native
@task_standing n=3
convention: Each kept check a new test writes is a replayable session recording in an owner-only private recording directory, and it holds no screenshot.
home: surface-cli
@task_standing n=4
convention: Each new log line or span holds fixed words and counts only, and never echoes caller input.
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag_window, maximize, minimize, decorations, close) are window chrome. Exploration drives document actions, and drag_window is not the two-target drag verb.
@task_item n=2 part=not_needed
The cold-agent run pipe and the agent-run test contract. Their command lines, exits, record directory and Windows pass-throughs stay as they are, and the cold agent's MCP use is not touched.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines (arguments, environment knobs, backend, stdout) are benches. Exploration does not reach them.
@task_item n=4 part=not_needed
The stand accent colour and other visual styling. Exploration measures no pixel.
@task_item n=5 part=not_needed
The absence of arbitrary index.html templates. Exploration loads no new document.
@task_item n=6 part=not_needed
The mobile-native and web-spa surfaces are unmeasured, and the chunk reaches no layout of theirs.
