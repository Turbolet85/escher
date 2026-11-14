@task_prose part=chunk
Builds one exploration command over a held app: from boot it enumerates the actions available on the screen and drives them breadth-first, each path re-reached from a fresh session, the frontier ordered by path length, until a step budget is spent, then returns a closed report.
The actions come from the driver's own readers, the actions answer and the schema's verb table, never a hand list; each step runs as a validated call through the one session run path, with no second executor.
A refusal met on a step is reported by its closed cause, a panic inside a step is caught and reported as a closed word with its path kept; neither stops the exploration.
Each explored path is left as an owner-only kept check in the recording format, holding no screenshot step and replaying green; the recorded-check step format is versioned and can carry a verb with more than one target as an addition.
Exploration reads the default seed alone, follows no navigation and loads nothing new; it is proven on the stand's served tasks in both layout modes and on a fixture that panics in a handler and one holding a refusing control.
@task_item n=1 part=needs
The exploration command itself: a new driver-side command over a held app that takes a step budget, drives every available action breadth-first from a fresh session per path, and returns a closed report of the paths, refusals and panics it met.
@task_item n=2 part=needs
The actions answer and the closed driver verb schema as exploration's only source of drivable actions: a verb is driven only where the verb table holds it and the actions answer offers it on a node, so no closed verb list lives in exploration's own code and a future verb is driven with no reshaping.
@task_item n=3 part=needs
Every exploration step goes through the single command validation gate and the session's one run path, as reach does: one validation path for every carrier, unknown or unfit targets refused in the fixed refusal order, no second executor.
@task_item n=4 part=needs
Refusals met on a step are caught and reported by their closed cause name only, from the closed refusal cause set; a refusal and the report hold nothing a call supplied.
@task_item n=5 part=needs
A panic raised inside a handler during a step is caught and reported as a closed word with the path that led to it kept; exploration itself has no panic path on its own untrusted inputs, the step budget and the directory, which are refused as typed values.
@task_item n=6 part=needs
Each explored path is written as a kept check in the recorded check file format: owner-only, a 0600 file in a 0700 launcher-named recording directory, a group- or other-open directory refused, kept checks uncommitted.
@task_item n=7 part=needs
The recorder exploration uses removes only its own files and never replaces foreign ones, ends a failed or overlong recording, and never names or logs the recording directory.
@task_item n=8 part=needs
No screenshot step in an explored path and no pixel at rest: a kept check from exploration holds no screenshot answer.
@task_item n=9 part=needs
Each kept check replays green through the check replay, which runs each step through the session and reads the recorded check strictly as untrusted bytes.
@task_item n=10 part=needs
The recorded-check step format becomes versioned: a kept check states its format version, and a step whose verb carries more than one target is an addition to that format, not a redefinition; the strict reader keeps refusing what it does not know.
@task_item n=11 part=needs
Masked values stay masked in explored paths: a type step onto a password or file input records the masked stand-in, and the mask holds in the kept check and the report as on every carrier.
@task_item n=12 part=needs
Paths are re-reached by stable element id across fresh sessions: no NodeId or process-local handle is kept between calls, a caller-supplied id is looked up anew, and an unknown id is refused as stale or not-found.
@task_item n=13 part=needs
Navigation stays a boundary fork held at its most conservative provisional reading: an action that would navigate the document, a link or a form submission to another URL, is not driven, and exploration loads nothing new; no driver verb takes a URL.
@task_item n=14 part=needs
Carriers: the command reaches no new carrier provisionally; any new driver verb or role on the command line, MCP or the socket would be a widening under the boundary widening rule and is the founder's fork.
@task_item n=15 part=needs
Seed: exploration reads the default seed alone, with no seed argument on any carrier; the seeded source takes its seed from in-process code only, and a later seed argument is an addition with no rework.
@task_item n=16 part=needs
Logging: no new log field carrying content; any exploration span holds fixed words and counts only, driver outcomes and screen content stay out of logs, refusals are logged by cause name only, and the driver installs no subscriber.
@task_item n=17 part=needs
Proof: tests drive exploration over the stand's served tasks in both layout modes and over two new fixtures, one whose handler panics and one holding a refusing control, each path's kept check replaying green.
@task_standing n=1
convention: each new test file reaches the driver through the test harness, which takes no tracing and no style
home: dependency-security
@task_standing n=2
convention: each new fixture or fixture state a test serves is named in the closed fixture table, its name checked against it
home: input-validation
@task_standing n=3
convention: each new line of driver code stays under its crate's line coverage floor
home: input-validation
@task_standing n=4
convention: each new log line is one sink line per closed span on an escher target, content-named fields withheld
home: logging-monitoring
@task_item n=1 part=not_needed
Canvas virtual nodes and the virtual-node form of the stable element id: canvas is out of scope, only a direction for 0.4.0.
@task_item n=2 part=not_needed
The production of drag or any two-target verb's own behaviour: only the step format's room for one is built.
@task_item n=3 part=not_needed
Secrets, CI secrets and signing material, publish and release guards: the chunk touches no CI secret, release or publish job.
@task_item n=4 part=not_needed
Outbound HTTP, per-host limits, TLS, CORS and same-origin absences, response size and timeout absences: exploration loads nothing new.
@task_item n=5 part=not_needed
Browser-app storage and desktop integration: history in sqlite, cookie store, HTTP cache, clipboard, file dialog, browser JavaScript opt-in.
@task_item n=6 part=not_needed
Engine rendering internals: CSS parsing and property accessors, layout, border radii, inline boxes, fonts, select list painting and colours, serialization of SVG.
@task_item n=7 part=not_needed
The cold-agent harness, the agent-run script, the apt install script, the WPT runner and its results, and rdme: none is reached by the chunk.
@task_item n=8 part=not_needed
The Dioxus native bridge and its mounted-element, form-data and inner-HTML rows: the chunk drives the held app through the driver only.
