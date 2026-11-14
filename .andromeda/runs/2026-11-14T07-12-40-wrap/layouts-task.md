@task_prose part=session
The session added a seeded source of randomness: a run's randomness now comes from one seed, and two runs of one seed read the same state at every step.
The source is one shared stream per document, drawn in draw order by every component of that document, reached through a hook; outside any run (the windowed shell) it seeds from the host's per-process random keys.
The harness boot that every driver session goes through now provides a default seed of 0 where the builder of the document gave none, beside the one virtual clock it already supplies; the seed module is ungated, so the wasm build carries it.
No seed crosses the driver's carriers: no verb, argument, MCP tool field, kept-check key, env read, log site or exit was added; this is the operator's answer A to the founder's fork, PROVISIONAL until the founder confirms or replaces it.
A new stand check covers it in both layout modes, in a second process (the test binary re-executes itself) and through a kept-check replay; page-script Math.random stays unseeded, owned as a residual for next.
Nothing was retired and no dependency, config key, driver schema row or stand task changed.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the seeded source of randomness for a Dioxus document: the default run seed, the shared-stream generator and the hook a component draws from, with its host-seeded fallback outside a run
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus native document library's module list and public exports, where the ungated modules stand and the seeded source is now exported
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the test harness and its construction from a VirtualDom, the stand boot every session role (command line, host, mcp, coverage, replay) boots through, which supplies the virtual clock and now the default run seed
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: the stand check for seeded runs: one seed two runs read the same state, two seeds differ, no seed reads the default, a second process reads the same state, a kept check replays its seeded run, in both layout modes
@task_item n=1 part=touches
Session roles share the stand boot: the shared boot now also provides the default run seed to every document it builds where none was given, so every role reads the same seeded randomness.
@task_item n=2 part=touches
Held instance reads as a fresh boot: a held or rebooted instance now reads the default seed again at each boot, so its randomness restarts with the fresh boot as its clock does.
@task_item n=3 part=touches
Session recording as a replayable check: a kept check now replays a run that draws randomness to the same state, because the replay's boot reads the same default seed; no seed key was added to the check file.
@task_item n=4 part=touches
escher-session recording check file (check-{n}.json): it holds no seed key; a replay reaches the same randomness through the default seed alone.
@task_item n=5 part=touches
escher-session command line form and the actions: no seed argument or verb was added; a run's seed is not caller-settable, by the operator's PROVISIONAL answer A.
@task_item n=6 part=touches
escher-session MCP server tools/list: no tool and no tool field carries a seed.
@task_item n=7 part=touches
escher-session served tasks: no stand task reads randomness; the seeded source is exercised only by a test fixture, so each served task's answer is unchanged.
@task_item n=8 part=touches
escher-session coverage role: it boots through the shared stand boot and so now reads the default seed; its answer and its check-file bound are unchanged.
@task_item n=9 part=touches
Headless stand pinned viewport: the headless stand boot now supplies a pinned run seed as it supplies a pinned viewport and one virtual clock; the viewport itself is unchanged.
@task_item n=10 part=touches
Every stand control keyed and stand ids as attributes alone: the new check's fixture control (roll) is keyed by id as the convention asks; it is a test fixture, not a stand control, so no stand id was added.
@task_fact n=1 slug=surface-desktop-native
fact: The stand boot gives every harness-built document a run seed: where the builder gave none it provides the default seed 0 as a root context, so two runs of one task read the same randomness at every step, in both layout modes and across processes; outside any run the windowed shell seeds from the host instead.
@task_fact n=2 slug=surface-cli
fact: Absence, PROVISIONAL until the founder confirms or replaces it: no seed crosses the driver's carriers; the command line, the MCP tools, the actions and the kept-check file take no seed, and every session reads the default seed.
@task_fact n=3 slug=surface-desktop-native
fact: Absence: a page script's Math.random is not seeded; no driver run reaches the page-script engine, and seeding it is owned as a residual for the next release.
@task_item n=1 part=untouched
ShellProvider window chrome controls: the session changed no window chrome control.
@task_item n=2 part=untouched
Stand stable element ids: no stand element id was added, renamed or removed.
@task_item n=3 part=untouched
Stand layout decisions (fixed document skeleton, accent colour, open list row bounds, CRUD list keyboard region, scroll into view, clipping boxes, flight type select, stand lints clean at boot, a11y by attributes alone): no layout, style or accessibility behaviour moved.
@task_item n=4 part=untouched
Load page network provider and arbitrary index.html templates: no provider or document template changed.
@task_item n=5 part=untouched
agent-run.sh and cold-agent.sh contracts and the cold-agent decisions: no script, pipe, prompt or record dir changed.
@task_item n=6 part=untouched
escher-session exit codes, refusals, snapshot, screenshot, graph, lint, help, host log and the MCP transport, revisions and result bound: no answer shape, exit, refusal, log name or protocol moved.
@task_item n=7 part=untouched
multi_doc and paint_bench command lines: no argument, env read or output changed.
@task_item n=8 part=untouched
The mobile-native and web-spa surfaces and the decisions log: the session measured nothing there.
