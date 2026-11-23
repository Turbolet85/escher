@task_prose part=session
The session measured a canvas shot's PNG bytes across CI's linux x86_64, windows x86_64 and macOS aarch64 hosts and found them equal; a shot holding document text still differs between linux and macOS.
It added one stand check target: three canvas fixtures (a solid fill, an antialiased edge, canvas glyphs), each shot as a whole shot and an element shot, in both layout modes, read against one table of twelve digest rows; no pixels committed.
Each fixture's precondition is a canvas painted alone at a fixed box, with no named node and no lint finding at boot.
The capability statement's shot-bytes limit is now scoped to a shot holding document text, with a canvas-only shot held equal on the three hosts, and it names the unmeasured canvas conditions (another SIMD level, a scale other than 1.0, gradients).
The founder ratified byte equality for canvas-only shots; no product symbol, driver verb, field, refusal cause, CI leg or gate script moved, and no capability was claimed.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: a new stand check target: canvas-only shots (whole and element) of three canvas fixtures held to one table of digests across hosts, in both layout modes, each fixture lint clean at boot with no named node
@task_file n=2 path=docs/capabilities.md
what: the capability statement, edited by hand: its stated limits, among them a shot's PNG bytes across OSes, and its unmeasured conditions
@task_item n=1 part=touches
A rule on the driver's screenshot and its element shot: an element shot of a canvas is now held byte-equal across linux, windows and macOS by a committed stand check, while a shot with page text still differs by OS.
@task_item n=2 part=touches
A rule that a screenshot changes nothing: the new check reads screenshot answers' digests only, in both layout modes, and relies on that rule.
@task_item n=3 part=touches
A rule that a stand reads no lint finding at boot: each new canvas fixture asserts no lint finding as its precondition.
@task_item n=4 part=touches
A rule that content leaves only as pixels (as option labels of an open list do): a canvas's content likewise crosses only as pixels, now with byte-equal pixels across hosts for canvas-only shots.
@task_item n=5 part=touches
The stand checks repeated three rounds: the repeat leg takes the new stand target by its name prefix, three of three rounds green; no threshold or round count moved.
@task_item n=6 part=touches
Accessibility reach in the capability statement, edited by hand: the statement was edited this session (the shot-bytes limit only); no accessibility reach line moved.
@task_item n=7 part=touches
Any row whose reason rests on a shot's bytes differing between OSes: that reason now holds for a shot of page text alone, a canvas-only shot being equal across the three hosts.
@task_item n=8 part=touches
The a11y CI leg and accessibility checks gating CI: no leg or gate line moved; the new target runs in the existing test jobs on all three hosts.
@task_item n=1 part=untouched
Keyboard navigation, focus, Tab order and activation rows: no key, focus or click behaviour moved.
@task_item n=2 part=untouched
ARIA role mapping, accessible name, ARIA property and state reads, and hidden or inert content rows: no role, name or state rule moved.
@task_item n=3 part=untouched
The accessibility tree, the AccessKit adapter, the accessibility feature defaults and the platform adapter absence: no tree, adapter or feature change.
@task_item n=4 part=untouched
Snapshot, snapshot text form, snapshot diff, actions answer and UI graph contracts: no field, cause or reader moved.
@task_item n=5 part=untouched
Contrast rows, the lint's contrast finding and its thresholds: no ratio, threshold or formula moved.
@task_item n=6 part=untouched
Select list rows (open list colours, motion, row heights, typeahead): untouched.
@task_item n=7 part=untouched
Cognitive accessibility rows: untouched.
