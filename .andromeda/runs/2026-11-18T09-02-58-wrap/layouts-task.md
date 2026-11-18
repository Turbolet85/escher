@task_prose part=session
The session widened the cold-agent pipe over 0.3.0's carried reads: it now counts graph, actions and layout calls beside lint, screenshot and advance (3 to 6 counted verbs).
The verdict and its run.end event gain five fields: graph_calls, actions_calls, layout_calls, list_opens (accepted click or press answers whose changed list holds a ListBoxOption differing in bounds alone) and refused_by_cause (refusals counted per closed cause word read from the command span's existing cause field).
The capability-unused reason now requires eight capabilities at above zero (adds graph, actions, layout and a list opening); the wrong-call bar stays 5 percent, now the founder's answer.
The stand prompt gains four asks: the counter's UI graph, the flight booker's actions and its flight type list opened and its option clicked, CRUD's people list position and size, and Cells told not to ask for graph or actions; it still names no element id or app slug.
No driver verb, carrier, argument, cause word, session error, log field or span was added; no live run fired; the pipe's contract tests went 38 to 41 and one layout-uncounted test was retired for a layout-counted one.
Measured from research, not re-measured: the Cells actions answer is 118,292 B at HEAD, not the 106,088 B a residual and the 2026-11-08 record state.
@task_file n=1 path=scripts/cold-agent.sh
what: the cold-agent run pipe (cold-agent.sh run), its fixed stand prompt, its counted verbs and its verdict with reasons such as capability-unused
@task_file n=2 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests, driving a fake escher-session and a claude shim through the stand flow
@task_item n=1 part=touches
decision/cold-agent-prompt-names-no-id: the prompt widened with four per-app asks (counter graph, flight booker actions and type list, CRUD list layout, Cells no graph or actions); it still names no element id or app slug, pinned by test_the_prompt_names_no_stand_id.
@task_item n=2 part=touches
decision/cold-agent-pipe-builds-nothing: the pipe gained counting and verdict fields but still builds nothing; check the row's claim holds unchanged.
@task_item n=3 part=touches
decision/cold-agent-uses-driver-mcp: the agent is now asked to use the MCP tools graph, actions and layout and to open a list, all through the same driver MCP server.
@task_item n=4 part=touches
contract/cold-agent-cli/stdout and contract/cold-agent-cli/record-dir: the verdict the run writes (target/cold-agent/verdict.json and its run.end event) gains five fields; any row naming the verdict's fields or reasons is stale where it lists them.
@task_item n=5 part=touches
contract/cold-agent-cli/exit-2 and contract/cold-agent-cli/exit-3: a failing verdict now also arises when graph, actions, layout or a list opening is left unused (capability-unused over eight capabilities, was four); the wrong-call bar stays 5 percent.
@task_item n=6 part=touches
decision/open-list-rows-own-bounds: the pipe now counts a list opening by that very shape, a ListBoxOption row whose only difference is bounds in an acting answer's changed list.
@task_item n=7 part=touches
decision/flight-type-select-boot-reading and contract/stand-element-ids/flight-type: the stand flow and prompt now open the flight type list and click its option to choose the return flight; the ids themselves are unchanged.
@task_item n=8 part=touches
contract/escher-session-graph/answer, contract/escher-session-actions/answer and contract/escher-session-layout/answer: these three verbs are now counted and required in the cold-agent run; their answers did not change.
@task_item n=9 part=touches
contract/escher-session-actions/answer and contract/escher-session-mcp/result-bound: where a row holds the Cells actions answer size, 106,088 B is disproved, it reads 118,292 B at HEAD (each node now lists layout).
@task_item n=10 part=touches
contract/escher-session-refusals/fonts-unavailable, off-screen and unknown-verb: the cold-agent verdict now counts refusals per closed cause word in refused_by_cause, read from the command span's existing cause field; no cause word was added.
@task_item n=11 part=touches
contract/escher-session-host/log-service-name: the pipe now reads every server.log line, its command span lines' cause field; no log field or span was added.
@task_item n=12 part=touches
decision/session-recording-replayable-check: the layout read is now counted in the run and replayed in the check (test renamed to test_a_layout_read_is_counted_in_the_run_and_replayed_in_the_check).
@task_fact n=1 slug=surface-cli
fact: The cold-agent verdict counts graph, actions and layout calls and list openings beside lint, screenshot and advance, and counts refusals per closed cause word in refused_by_cause; capability-unused fails a run that leaves any of the eight at zero.
@task_fact n=2 slug=surface-cli
fact: The cold-agent stand prompt asks for each app's 0.3.0 read (counter UI graph, flight booker actions and its type list opened by click, CRUD list position and size) and tells Cells not to ask for its UI graph or actions, naming no id or slug.
@task_item n=1 part=untouched
The desktop window chrome contracts (shell-provider window controls): no window or shell code moved.
@task_item n=2 part=untouched
The stand's element id list and its attribute-only id, role and accent rules: no id was added, renamed or removed.
@task_item n=3 part=untouched
The served tasks list and the stand boot rules (session roles, held instance, pinned viewport, load page network): no served app was added; the error page is only owed next.
@task_item n=4 part=untouched
escher-session's own command form, exit codes, help, snapshot, lint, screenshot and coverage answers, and its MCP protocol revisions and tools list: no verb, argument or answer changed.
@task_item n=5 part=untouched
The agent-run, multi_doc and paint_bench command line contracts: none of those tools moved.
@task_item n=6 part=untouched
The mobile-native, web-spa and decisions-log areas: nothing reaches them.
