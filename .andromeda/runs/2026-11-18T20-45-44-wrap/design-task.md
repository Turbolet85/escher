@task_prose part=session
The session ran 0.3.0's one live cold-agent run: a fresh agent drove the seven served stand apps, the order app among them, through the escher-session MCP role over stdio.
It measured 76 tool calls with 0 wrong against the founder's 5 % wrong-call bar (kept), fifteen tools, client 2.1.327, verdict passed, $1.03 of a $3 cap; every refusal cause read 0 and caused_by_kind read panic 1, error 1, log 1.
The measured instance replaced the earlier one (2026-10-25, six apps, twelve tools, 66 calls, 0 wrong, client 2.1.308), which stays named as the earlier run.
No code line moved: the cold-agent pipe changed in two comment lines only, and its bar, prompt, counted kinds, causes and verdict are byte-equal.
It retired the interim built-not-yet-run wording: the 0.3.0 heading now reads built, its cold-agent run passed, and all 17 of its capabilities are verified.
@task_file n=1 path=scripts/cold-agent.sh
what: the cold-agent pipe, the fresh-agent run over the served stand apps through the escher-session MCP role, with its 5 % wrong-call bar, its counted tool kinds, refusal causes and verdict; two comment lines edited
@task_file n=2 path=README.md
what: the project readme, its 0.3.0 version heading and the verified-capability count; edited
@task_file n=3 path=docs/capabilities.md
what: the capability page, its section on what an MCP client carries onward, the measured cold-agent instance and the capabilities out of a live run's reach with their stand witnesses; edited
@task_item n=1 part=touches
decision/escher-session-agent-read-surface: escher-session is read by an agent; the session measured a fresh agent reading it over seven apps at 76 calls, 0 wrong, the 5 % bar unchanged; a row holding the earlier run's counts (66 calls, six apps, twelve tools, client 2.1.308) is now stale.
@task_item n=2 part=touches
decision/driver-output-for-agents: driver output read by agents; the live run is the measurement of that output's readability, and any row cell holding the wrong-call bar (5 %) or the measured counts must read the new run.
@task_item n=3 part=touches
contract/escher-session-tasks/order: the order task is the seventh app the live run read for the first time; its error page controls panicked, returned an error and logged, one each.
@task_item n=4 part=touches
contract/escher-session-commands/graph-answer and contract/escher-session-commands/actions-answer: the founder ratified graph and actions answers crossing to the model provider on the live run; their content did not change.
@task_item n=5 part=touches
decision/actions-share-the-verbs-checks and contract/ui-graph-line/content and the actions-line contracts: the live run called graph, actions and layout once each and read them with 0 wrong; no item of them moved.
@task_item n=6 part=touches
contract/escher-session-mcp rows (stdout, tool results, Err ending, screenshot and snapshot results): the live run reached the MCP role over stdio with fifteen tools and 0 wrong calls; no item moved.
@task_item n=7 part=touches
contract/escher-session-session-error/remedy and the escher-session stderr and exit-status rows: every refusal cause of the run read 0; no item moved, but a row stating refusal counts of a measured run would be stale.
@task_item n=1 part=untouched
Color palette, typography, spacing, depth strategy, border radius and iconography rows: no colour, font, spacing, paint order, radius or cursor changed.
@task_item n=2 part=untouched
Motion rows and the driver verbs and virtual clock contracts: no clock, scroll, animation or advance behaviour changed; advance was only called once by the run.
@task_item n=3 part=untouched
Select list decisions and contracts (open gestures, list events, accessibility tree, overlay, hit and paint order): no engine change.
@task_item n=4 part=untouched
WPT runner output and result colour rows, shell provider, JS window, MountedData, dioxus-native configuration and coverage check file rows: none reached.
@task_item n=5 part=untouched
Web-spa surface and the WASM font context rows, mobile targets built not tested: no build or platform change.
