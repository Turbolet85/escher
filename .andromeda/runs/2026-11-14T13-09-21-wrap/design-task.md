@task_prose part=session
The session added starting a driver session in a named fixture state of its app: a bounded, validated name chosen from the app's closed table, its first screen reading as that state reached by hand from boot.
A fixture state is a fixed list of the four acting calls (click, type, press, advance), each run in order through the session's own validated run path; the reach stops at the first refused or unsettled step, naming its index and cause, and the steps before it stand.
The stand gained three states, in process only: crud fourteen-people (twelve creates) and person-selected, flight-booker return-flight; counter and timer have none. A name outside the table is refused UnknownState before anything boots, with a fixed text holding nothing the caller supplied.
No carrier changed: the start command, the command line, MCP's start tool, the session socket, the recording form, the session errors, verbs, causes, exits, spans and log fields are all unchanged. The founder ruled the state stays in process only (option C), so the absence of a state on the carriers is ratified.
A recorded session started in a state replays from boot with the check form unchanged. One Android-only compile failure (the dioxus-native prelude's one-parameter Result alias on Android) was fixed by spelling the standard Result in full.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the driver's fixture state: the four acting steps a state is reached by, the named state, the whole-name lookup against an app's closed table, the reach error and the session's reach through the validated run path
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's unit checks of the fixture state: step to call conversion in schema order, whole-name lookup, label-form names, fixed error texts, stop at the first refused step, busy read only off an unsettled acting outcome
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the escher-driver crate root: its module list, public exports and crate documentation
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the seven_guis stand: the tasks escher-session serves (counter, crud, flight-booker, timer), now with each task's named fixture states and a start in a state that checks the name before it boots
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixtures module: the fixture boot and hold, now with a hold in a named state
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the integration check of a start in a state: each stand state read as its by-hand path from boot in both layout modes, the next call taken alike, an unknown name refused, a refused step stopping the reach, a recorded session replaying from boot
@task_file n=7 path=docs/capabilities.md
what: the capabilities document: the driver's capability list with its pinned counts, now stating a start in a state and that a state is in process only
@task_item n=1 part=touches
The escher-session tasks contracts for crud and flight-booker: each task now has named fixture states (crud fourteen-people and person-selected, flight-booker return-flight) reachable in process; counter and timer have none.
@task_item n=2 part=touches
The escher-session tasks unknown-task contract: beside an unknown task, an unknown fixture state name is now refused UnknownState before anything boots, with fixed text naming nothing supplied.
@task_item n=3 part=touches
The driver verbs and virtual clock contracts for click, type, press and advance: a fixture state is a fixed list of exactly these four acting calls, each spelled as the verb's call with arguments by schema name in schema order.
@task_item n=4 part=touches
Indirect, through the reach running every step through the session's validated run path: the decision that actions are decided by the verbs' own checks; a fixture step gets no check of its own.
@task_item n=5 part=touches
Indirect, through the reach reading settled off each acting or advance outcome: the decisions that settle does not wait on animation and that driver advance moves the one virtual clock; a step that leaves the app unsettled stops the reach as NotSettled.
@task_item n=6 part=touches
The escher-session commands contracts and the mcp role contracts: start, its arguments and the MCP start tool are unchanged; a state is not reachable through them, which is now a ratified absence rather than an unstated gap.
@task_item n=7 part=touches
The escher-session session error contracts (remedy, stderr session error): the reach error is a separate in-process error, not a new session error; the session error set is unchanged.
@task_item n=8 part=touches
The decision that mobile targets are built, not tested: the Android build alone caught a compile failure here (the dioxus-native prelude brings a one-parameter Result alias on Android), so the Android build leg is the only witness of that hazard.
@task_item n=9 part=touches
The decision that escher-session output is read by an agent: no output line, field or text was added; the reach error's texts are fixed and hold nothing a caller supplied.
@task_fact n=1 slug=surface
fact: A driver session can start in a named fixture state of its app, in process only: the name is compared whole against the app's closed table, and the first screen reads as that state reached by hand from boot, in both layout modes; the stand's states are crud fourteen-people and person-selected and flight-booker return-flight, counter and timer none.
@task_fact n=2 slug=surface
fact: Absence, ratified by the founder: no fixture state crosses the command line, MCP, the session socket or a kept recording; start, its schema and the session errors carry no state.
@task_fact n=3 slug=surface
fact: A fixture state name outside the app's table is refused UnknownState before anything boots, so nothing is started, and the refusal's text is fixed and holds nothing the caller supplied.
@task_fact n=4 slug=surface
fact: A reach runs its steps in order through the validated run path and stops at the first refused or unsettled step, naming the step's index and its closed cause or busy state; the steps before it stand and are not rolled back.
@task_fact n=5 slug=surface
fact: A recorded session started in a state replays from boot; the recording keeps the reach's steps as ordinary calls and its check form is unchanged.
@task_fact n=6 slug=surface
fact: Convention: in a file that globs the dioxus-native prelude, the standard two-parameter Result is spelled in full, because on Android the prelude brings a one-parameter Result alias that fails only the Android build.
@task_item n=1 part=untouched
Colour palette, typography, spacing, depth, border radius and iconography rows: no colour, font, spacing, paint order, radius or cursor changed.
@task_item n=2 part=untouched
The select list rows (overlay, events, accessibility, screenshot and snapshot of its rows): no select behaviour changed.
@task_item n=3 part=untouched
The escher-session exit status and exit code rows and the stdout line shapes: no exit, output line or MCP result shape was added or changed.
@task_item n=4 part=untouched
The actions, coverage and UI graph line contracts and their command arguments and answers: unchanged.
@task_item n=5 part=untouched
The WPT runner output and result colour rows, the JS window rows, the shell provider rows and the MountedData rows: not reached.
@task_item n=6 part=untouched
The web-spa surface and WASM font rows: the start in a state is native only.
@task_item n=7 part=untouched
Scroll motion rows (driver scroll, into-view scroll, smooth scroll, overlay scrollbar fade): no scroll or motion behaviour changed.
