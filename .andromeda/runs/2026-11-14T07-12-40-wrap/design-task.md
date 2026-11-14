@task_prose part=session
The session added a seeded random source to dioxus-native: one splitmix64 stream per document, shared by every component in draw order, read through a hook.
A run's randomness now comes from one seed, so two runs of one seed read the same state at every step, in both layout modes, in a second process and in a replay.
The test harness that builds a Dioxus-backed document now provides a default run seed of 0 as a root context where the builder gave none, the way it supplies the one virtual clock.
Outside any run (the windowed shell) the source seeds itself from the host's per-process random keys.
No seed crosses the driver's carriers: no verb, argument, task, MCP field, kept-check key, exit status, log line or print was added; that answer is PROVISIONAL for the founder.
Page-script Math.random stays unseeded, a residual owned for the next release; no dependency, config key, colour, font, motion or UI changed.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the seeded random source of a dioxus-native document — the generator, its default seed, and the hook a component reads it through, with its host-seeded fallback outside a run
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native DOM crate's root — its modules and what it exports, the ungated modules beside the accessibility-gated ones, compiled into the wasm build too
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the test harness that builds a Dioxus-backed document and pumps it, supplying the one virtual clock and now the default run seed; every escher-session driver session boots through it
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: a stand check that drives a dice fixture through a session and holds that one seed reads one state at every step, in both layout modes, in a re-executed test process and in a kept-check replay
@task_item n=1 part=touches
Rows that say the harness or driver boot supplies one virtual clock (animation driven by the virtual clock, driver advance moves the one virtual clock): the boot now also supplies a default run seed as a root context beside it — no contradiction, a sibling precedent.
@task_item n=2 part=touches
The escher-session tasks rows (counter, timer, crud, flight-booker, cells, load): every task now boots reading the default seed 0, yet no task reads randomness, so each reads the same state as before.
@task_item n=3 part=touches
The escher-session commands, stdout, mcp-role results and exit status/code rows: no seed argument, field or status was added; a row stating the full set of arguments or tool fields stays true as written.
@task_item n=4 part=touches
Kept-check and replay behaviour behind the driver's surface: a kept check replays a seeded run to the same state with no seed key in the check file.
@task_item n=5 part=touches
The web-spa surface overview (WASM pages rendering into a canvas): the new seeded source is ungated and compiles into the wasm build, falling back to host-seeded randomness there.
@task_item n=6 part=touches
The surface decision that escher-session is read by an agent and its output read by agents: the absence of any seed on the driver's carriers is PROVISIONAL, and on the founder's other answer a seed would join the start command.
@task_item n=7 part=touches
The stylist device seeded with the root's values (a decision using the word seed in another sense): unchanged — the token seed there is not this source.
@task_item n=1 part=untouched
Colour palette, typography, spacing, depth strategy, border radius and iconography rows: no colour, font, spacing, paint order, hit-testing or cursor changed.
@task_item n=2 part=untouched
The select list rows (its overlay, events, colours, accessibility, screenshot and snapshot bounds): no select behaviour moved.
@task_item n=3 part=untouched
The scroll and motion rows on instant scroll, smooth scroll, overlay scrollbar fades and settle not waiting on animation: no scroll or animation timing moved.
@task_item n=4 part=untouched
The WPT runner output and result colour rows, the shell provider rows and the dioxus-native config rows (base colour, alpha mode, font context): none was reached.
@task_item n=5 part=untouched
The actions, coverage and UI graph line content rows and the coverage check file bounds: no line shape or bound changed.
@task_item n=6 part=untouched
The JS window object rows and MountedData-during-render rows: the page script's Math.random was deliberately left unseeded, and no script runtime API changed.
