@task_prose part=session
The session measured a canvas shot's bytes on CI's linux x86_64, windows x86_64 and macOS aarch64 hosts and found them equal for a shot holding canvas content alone.
It added one test-only stand check: three canvas-only fixtures (a solid fill, an antialiased edge, glyphs painted on a canvas from the document's font context), each a canvas at (8, 8, 200, 100) with no named node and no lint finding, shot whole and as an element in both layout modes, twelve digest rows in one table, no pixels committed.
CI run CI#45209188300 read all twelve rows equal on the three hosts; the founder ruled to require byte equality for canvas-only shots.
The capability statement's shot-bytes limit is now scoped: a shot holding document text still differs between linux x86_64 and macOS aarch64; a canvas-only shot is held equal on all three hosts, with SIMD level, scale other than 1.0 and gradients left unmeasured.
No product symbol, verb, argument, field, refusal, exit code, log field, element id of the served stand, dependency or committed check moved; nothing was retired and no capability is claimed.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: a new stand check of the headless session: canvas-only fixture components booted in both layout modes, each shot whole and as an element through the session's screenshot verb, its answer digest compared against one committed table of twelve rows
@task_file n=2 path=docs/capabilities.md
what: the capability statement: its limit on a shot's PNG bytes across hosts (now scoped to shots holding document text, canvas-only shots held equal) and its list of unmeasured canvas conditions
@task_item n=1 part=touches
The escher-session screenshot answer: its digest for a canvas-only shot is now held byte-equal across linux x86_64, windows x86_64 and macOS aarch64; any row stating that a shot's bytes differ by OS, or are equal on one host only, is stale for canvas-only content and holds only for a shot with document text.
@task_item n=2 part=touches
The escher-session MCP server's screenshot result: it carries the same shot bytes, so a row stating cross-host inequality of the screenshot result is scoped the same way.
@task_item n=3 part=touches
The stand canvas element circle-canvas (Circle Drawer): its element shot is directed, by a WATCH, to join the cross-host digest table later; its whole screen holds page text so it stays outside the byte-equality rule. No id changed.
@task_item n=4 part=touches
The headless stand pinned viewport: the new check relies on the pinned viewport and scale 1.0 to place each canvas at (8, 8, 200, 100); a row fixing the viewport or scale is the precondition the digest table rests on.
@task_item n=5 part=touches
Stand lints clean at boot: each new canvas fixture is booted and must read no lint finding before it is shot, an added instance of that rule.
@task_item n=6 part=touches
Stand ids as attributes alone and every stand control keyed: the new test-private canvas components carry their ids canvas-fill, canvas-edge and canvas-text as attributes; these are names taken inside a test target, not stand element ids of the served tasks.
@task_item n=7 part=touches
Session roles share the stand boot and held instance reads as a fresh boot: the check boots each fixture through the shared session boot in both layout modes (full and incremental) and found whole and element digests the same in each mode.
@task_item n=8 part=touches
Fonts-unavailable refusal and the document font context: the text canvas fixture paints glyphs from the document's own font context, and its digest is equal across hosts; a row saying glyph bytes always differ by host is scoped to document text, not canvas-painted glyphs.
@task_item n=9 part=touches
Session recording as a replayable check: a recorded or committed check still holds no screenshot step; the rule stands, but any reason row saying a shot is byte-identical on one host only now holds for page-text shots alone.
@task_item n=1 part=untouched
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): no window chrome behaviour or method moved.
@task_item n=2 part=untouched
Stand stable element ids of the served tasks (cells, counter, crud, flight, load, order, temp, timer, task cards, circle controls other than the canvas watch): none added, renamed or freed.
@task_item n=3 part=untouched
agent-run.sh and cold-agent.sh pipe contracts, their usage, exits, stdout and record directories, and the cold-agent decisions: untouched.
@task_item n=4 part=untouched
multi_doc and paint_bench command lines, their arguments, environment variables and backend: untouched.
@task_item n=5 part=untouched
escher-session exit codes, command form, help, layout, lint, snapshot, graph, coverage role, actions, host log service name, recording files and refusals other than fonts: no verb, argument, field, exit or refusal moved.
@task_item n=6 part=untouched
Stand page decisions on CRUD keyboard region, flight type select, load page network, order page causes, into-view scrolling, visible region clipping, open list row bounds and accent colour: no page or behaviour changed.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces and the arbitrary index.html templates absence: not reached.
