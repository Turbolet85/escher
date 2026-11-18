@task_prose part=chunk
Widens the cold-agent pipe's task (the stand prompt, or a second task beside it) to ask a cold agent for 0.3.0's reads on the driver's MCP carrier: graph, actions, layout, caused on an acting answer, and the select list opened on the flight booker.
Widens the pipe's counted span set to those verbs, each a verdict count field and, where required, a capability-unused reason; the verdict gains closed-word counts of refusals by cause, read from the command span's cause.
Keeps the wrong-call bar as one named constant the founder's answer sets, with the contract tests pinning the judgement at the bar's edge whatever its value.
Extends the offline contract tests (claude shim and fake escher-session) red before green; adds no driver verb, carrier, argument, cause, log field or span, and the live run fires only on the founder's word.
@task_file n=1 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe — the driver's MCP tool list given a fresh agent, its task prompt and the verdict that counts tool calls and judges wrong calls against the bar — edits: the task prompt or TASKS list, the COUNTED verb set, the verdict's count fields and capability-unused reasons, the refusal-by-cause counts, the wrong-call bar constant
@task_file n=2 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's offline contract tests — a claude shim and a fake escher-session answering each verb — edits: the fake server's answers to graph, actions, layout and the select list, the shim's tool calls, the tests of the widened task, counts, cause counts, capability-unused reasons and the bar's edge
@task_item n=1 part=needs
The escher-session mcp role's tool results as a cold agent reads them: the graph, actions and layout answers and the Err ending of a refused call, whose shape the pipe's fake server must answer in and whose refusals the verdict counts.
@task_item n=2 part=needs
The UI graph line, the actions line and the layout answer's content as the instance reads the widened task asks a cold agent for; the pipe reads them as written and changes none of them.
@task_item n=3 part=needs
The escher-session commands' graph, actions and layout arguments: the widened task must ask for each with arguments the driver already takes, adding none.
@task_item n=4 part=needs
The flight-booker stand task: the widened task has the agent click its flight-type select to open the select list, so the single-select open gestures and the select list's open event bind what the prompt asks.
@task_item n=5 part=needs
The snapshot and accessibility tree of an open select and its rows (open select, open attribute, highlighted row, chosen option, a row's bounds): what a cold agent reads back after opening the list, and where a call on a covered option of a closed select ends refused.
@task_item n=6 part=needs
The session error object and stderr session error with their closed cause words: the verdict's refusal-by-cause counts are closed words read from the command span's cause, carrying no text.
@task_item n=7 part=needs
The escher-session exit status and exit code tables: a refused or erroring call the verdict counts as wrong is read by its ending, never by free text.
@task_item n=8 part=needs
escher-session is read by an agent, not a person, with no colour: the pipe's prompt and verdict read its plain output lines and add no rendering of their own.
@task_item n=9 part=needs
Actions decided by the verbs' own checks: a cold agent's call to a verb the actions line did not offer may end refused, and the verdict must count that ending by its cause rather than as a pipe fault.
@task_item n=10 part=needs
The stand's other tasks (counter, timer, crud, cells, load) and the unknown-task refusal: a second task beside stand must name only served tasks, and the fake server answers an unknown one as the driver does.
@task_item n=11 part=needs
The wrong-call bar as one named input: the verdict judges wrong calls against a single constant whose number is the founder's (0.2.0's is 5 % over 66 calls, 0 wrong); tests pin pass and fail at the bar's edge, not at a hard-coded value.
@task_item n=12 part=needs
Boundary: no driver verb, carrier, argument, cause, session error, log field or span is added or changed, no listener, env read or secret; the pipe never fires in CI or a loop, and its tests stay offline.
@task_standing n=1
convention: each new contract test of an agent-facing driver read answers through a fake escher-session in the mcp role's own result shapes and closed words, offline, red before green
home: surface
@task_item n=1 part=not_needed
Colour, typography, spacing, depth, border radius and iconography: no paint value, font, colour token, cursor or stacking rule changes; the pipe reads answers, it paints nothing.
@task_item n=2 part=not_needed
Motion and the virtual clock (advance, scroll, animation, select list motion): the pipe already counts advance and changes no clock or scroll behaviour.
@task_item n=3 part=not_needed
The WPT runner's output and result colours: a different tool with its own terminal output, untouched by the cold-agent run.
@task_item n=4 part=not_needed
The shell provider, the JS window object, MountedData during a render, the dioxus-native config and font contexts on WASM: engine and window internals no cold agent reaches.
@task_item n=5 part=not_needed
The web-spa surface and mobile targets: the cold-agent run drives the desktop driver's MCP carrier only.
@task_item n=6 part=not_needed
The select list's engine overlay placement, painting, hit order, wheel and key scroll and its colours: the run only opens the list through an existing verb and reads what the driver already answers.
