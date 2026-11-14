@task_prose part=session
A driver session can now start in a named fixture state of its app: a fixture is a name and a fixed list of acting steps (click, type, press, advance), reached from boot by running each step through the session's validated run path.
The stand gains closed tables of named states per task: crud has fourteen-people (twelve clicks on crud-create) and person-selected (a click on crud-person-0), flight-booker has return-flight (typing r into flight-type); counter and timer have none.
A state name is compared whole against the app's closed table before anything boots; any other name is refused as an unknown state whose text holds nothing the caller supplied, and a reach stops at the first refused or unsettled step, naming its index and cause, with the steps before it left standing.
A session started in a state reads, in both layout modes, as that state reached by hand from boot, and takes the next call alike; a recorded session that reached a state replays from boot with the check form unchanged.
No carrier changed: the command line, the MCP tools, the session socket, the spawned host's arguments, the recording form, the verbs, the session errors and the exit codes are all as before; a state is reached in process only, and that absence is ratified by the founder's own word.
One CI failure on Android only (a bare two-parameter Result under the dioxus_native prelude glob) was fixed by spelling the standard Result; CI is green on the final head.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the driver's named fixture state, its steps, its whole-name lookup in an app's closed table, the session's reach of a state through the validated run path, and the reach's refusals
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's unit checks of a fixture state: each step's call, the whole-name lookup, the label form of names, fixed refusal texts and the reach stopping at its first refused step
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the driver library's public surface and its crate documentation
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the Seven GUIs stand, its tasks' boot and the session start over it, now with each task's named fixture states and the start of a session in one of them
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared fixtures test module that states once the fixture boot and hold of a stand page, now with a hold in a named state beside the hold
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the check of a session started in a named state of the stand: read as its by-hand path from boot, the next call alike, an unknown name refused, a refused step stopping the reach, a held page in a state, a recording replayed from boot
@task_file n=7 path=docs/capabilities.md
what: the capabilities list of the driver and the stand, with its stated absences
@task_item n=1 part=touches
Held instance reads as a fresh boot: a session or a held page started in a named state now reads as that state reached by hand from boot, a new reading beside the fresh-boot one; the hold gains a hold-in-a-state beside it.
@task_item n=2 part=touches
Session roles share the stand boot: starting in a state boots the stand through the same session start, then reaches the state; the stand boot itself is unchanged, the start in a state is a new entry beside it.
@task_item n=3 part=touches
Stand stable element ids crud-create, crud-person-{person.id} and flight-type: the stand's named states are reached by acting on these ids (twelve clicks on crud-create, a click on crud-person-0, typing r into flight-type), so these ids now carry a further dependent; no id was added, renamed or removed.
@task_item n=4 part=touches
Every stand control keyed, stand ids as attributes alone, stand names roles and states by attributes alone: the named states act only through the existing keyed controls; no markup, id, class or style was added to any task.
@task_item n=5 part=touches
Flight type select reads its two options at boot: the return-flight state is reached by typing r into the flight type select, a state beside the boot reading the row names.
@task_item n=6 part=touches
CRUD list as one keyboard region: the person-selected state is reached by a click on the first person row of the crud list, and fourteen-people by twelve creates, states of the list beside its boot reading.
@task_item n=7 part=touches
escher-session served tasks crud, flight-booker, counter and timer: each served task now has a closed table of named states (crud two, flight-booker one, counter and timer none); the served task list itself is unchanged.
@task_item n=8 part=touches
Session recording as a replayable check, and the recording's check-{n}.json: a recorded session that reached a state replays from boot as its steps, with the check form unchanged; no new recording field.
@task_item n=9 part=touches
Caller input never written back: the unknown-state refusal and the reach's refusals read fixed texts naming no state name tried, the same rule extended to a new refusal.
@task_item n=10 part=touches
escher-session command line form, the MCP server's tools/list and the session's unknown-verb refusal: unchanged, but the new absence (no fixture state crosses the command line, MCP, the session socket or a kept recording) belongs beside these carrier rows.
@task_item n=11 part=touches
escher-session exit codes and refusals: the reach's refusals (unknown state, a refused step, an unsettled step) are in-process library errors, not session errors or exits; no exit code or session refusal moved.
@task_fact n=1 slug=surface-desktop-native
fact: The stand holds a closed table of named fixture states per task, each a label-form name reached from boot by a fixed list of acting calls on keyed controls: crud fourteen-people and person-selected, flight-booker return-flight, counter and timer none; each name is named once.
@task_fact n=2 slug=surface-desktop-native
fact: A session started in a named state, or a stand page held in one, reads in both layout modes as that state reached by hand from boot and takes the next call as the by-hand session does.
@task_fact n=3 slug=surface-cli
fact: A state name is compared whole against the app's closed table before anything boots; any other name is refused as an unknown state with nothing started, its text fixed and holding nothing the caller supplied.
@task_fact n=4 slug=surface-cli
fact: A reach runs each step through the session's validated run path in order and stops at the first refused or unsettled step, naming the step's index and its cause; the steps before it stand and are not rolled back.
@task_fact n=5 slug=surface-cli
fact: Absence, ratified by the founder: no fixture state crosses the command line, the MCP server, the session socket or a kept recording; a state is reached in process only.
@task_fact n=6 slug=surface-cli
fact: A recorded session that reached a named state replays from boot as its steps, the check form unchanged.
@task_item n=1 part=untouched
ShellProvider window chrome controls: drag, maximise, minimise, close and decorations are not reached by a start in a state.
@task_item n=2 part=untouched
agent-run.sh and cold-agent.sh pipes, their usage, stdout, exit codes, record directory and Windows pass-through, and the cold-agent prompt and MCP decisions: no pipe or prompt changed.
@task_item n=3 part=untouched
multi_doc and paint_bench command lines, their arguments, environment knobs, backend and stdout: untouched.
@task_item n=4 part=untouched
escher-session coverage, graph, lint, screenshot, snapshot and actions answers, the MCP protocol revisions, result bound, screenshot result and stdio, and the host log service name: no answer shape, bound or log field changed.
@task_item n=5 part=untouched
Headless stand pinned viewport, stand accent colour, fixed document skeleton with main mount, load page network provider, into-view scroll, visible region through clipping boxes and open list rows bounds: no layout, viewport, scroll or style behaviour changed.
@task_item n=6 part=untouched
The arbitrary index.html templates absence, and the mobile-native and web-spa surfaces: not reached.
