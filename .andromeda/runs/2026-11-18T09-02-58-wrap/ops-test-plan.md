@row_set n=1 row=decision/cold-agent-server-log-counted-only cell=rule seen=1 detector=none
value: The server's log is read only to count its tool-call span lines, their refused and error endings, each counted verb's accepted lines and its command span lines' refusal causes; it is never printed or copied.
basis: fct-wyr2e6ga
because: span_counts() now reads every server.log line and counts the command span lines' cause per closed word, so a rule naming only tool-call span lines no longer states all the log is read for.
@row_set n=2 row=decision/cold-agent-prompt-names-no-ids cell=rule seen=1 fact=F4 detector=none
value: The fixed stand prompt takes six apps in turn, each started, led to its goal, snapshotted, stopped, asking the counter's UI graph, the flight booker's actions and its return flight by the flight type list, CRUD's layout, Cells neither, then the check as last json block; it names no id or app slug.
basis: fct-wyr2e6ga
because: STAND_PROMPT gains four asks (the counter's UI graph, the flight booker's actions and list-opened return flight, CRUD's layout, Cells asking for neither), and the row stating what the prompt asks must name them.
@row_set n=3 row=contract/cold-agent-verdict/capability-unused cell=promise seen=1 detector=none
value: A run where any of lint_calls, screenshot_calls, advance_calls, graph_calls, actions_calls, layout_calls, part_reads or list_opens reads 0 fails capability-unused.
basis: fct-wyr2e6ga
because: The capability-unused requirement set widens from four to eight fields, adding graph_calls, actions_calls, layout_calls and list_opens, so the reason's promise must name the set it reads.
@row_add n=1 row=contract/cold-agent-verdict/calls area=3-test-harness-contract fact=F1 detector=none
boundary: cold-agent verdict
item: calls fields
promise: The verdict counts six verbs, lint, screenshot, advance, graph, actions and layout, each as its own calls field.
basis: fct-wyr2e6ga
because: COUNTED becomes six verbs and the verdict gains graph_calls, actions_calls and layout_calls, a verdict field set no row of the area states.
@row_add n=2 row=contract/cold-agent-verdict/refused-by-cause area=3-test-harness-contract fact=F2 detector=none
boundary: cold-agent verdict
item: refused_by_cause
promise: An object of the eight closed refusal cause words, each a count of the command span lines refused with that cause; a word outside the eight is not counted.
basis: fct-wyr2e6ga
because: The verdict gains refused_by_cause, a new field of closed words to counts, which no row of the area states.
@row_add n=3 row=contract/cold-agent-verdict/list-opens area=3-test-harness-contract fact=F3 detector=none
boundary: cold-agent verdict
item: list_opens
promise: Counts the accepted click or press answers whose changed list holds a ListBoxOption differing in bounds alone; an answer that does not decode is skipped.
basis: fct-wyr2e6ga
because: The verdict gains list_opens, a new counted field read from the acting answers, which no row of the area states.
@row_add n=4 row=decision/prompt-goal-control-phrase-once area=unit-test-strategy fact=F6 detector=none
name: Prompt goal controls pin a phrase standing once
rule: A deletion control on a prompt goal pins the goal by a phrase that stands once in the prompt.
why: A phrase that also stands elsewhere in the prompt makes the control vacuous.
basis: fct-mwv3azaa
because: The chunk's control on the prompt test read DOES NOT HOLD on words shared with the Cells sentence and held on unique phrases, a convention for future controls that no row states.
@row_add n=5 row=decision/ci-scripts-controls-read-by-hand area=unit-test-strategy fact=F7 detector=none
name: ci-scripts leg controls read by hand
rule: A control on the ci-scripts leg is read by hand from its removed log.
why: The gate's control reader reads only nextest, libtest and pytest case lines and reads unittest INDETERMINATE.
basis: fct-ligswnjn
because: Every control on the ci-scripts leg read INDETERMINATE under the gate's reader and was read by hand, a convention binding future controls of that leg that no row states.
@row_none n=1 fact=F5 type=4
relied: The contract tests count refused_by_cause from the fake escher-session's command span lines, one per instance-verb call after its tool_call line, none for session verbs, help or an error ending.
