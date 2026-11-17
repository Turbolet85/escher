@row_set n=1 row=decision/socket-carries-screen-reading cell=rule seen=3 detector=D-security-input
value: An answer carries what the screen reads: ids, accessible names, control values, password and file input masked, snapshot text, diff, lint findings, UI graph, layout readings and an acting step's caused text scrubbed by its own field names, error kept, never a NodeId or an attribute value as typed.
basis: fct-xa2wojve, fct-stryxpwu
because: The layout answer now crosses the session socket as a fifteenth verb's answer, so the row's list of what an answer carries must name it.
@row_set n=2 row=decision/session-layout-read-in-process cell=rule seen=1 detector=D-security-input
value: The session's layout read, in process or through the layout verb on any ratified carrier, resolves its id through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
basis: fct-yx7iahh5, fct-stryxpwu
because: The read no longer takes its id from Rust code alone: the layout verb's run arm calls it with a carrier-supplied id, so the in-process-only statement is false.
@row_set n=3 row=decision/select-list-crossing cell=rule seen=1 detector=none
value: The select list crosses the ratified carriers only as a row's own bounds in the snapshot, the diff and a layout answer, as the layout answer's select-list placing word, and as pixels in a shot; node state holds no field of it and no expanded node key stands.
basis: fct-xa2wojve
because: The layout answer's closed placing words include select-list, so a list row now also crosses as a layout reading and the row's only-list is incomplete.
@row_set n=4 row=contract/actions-answer/verbs cell=promise seen=1 detector=none
value: A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause.
basis: fct-w5rznwmn, fct-gjow6tts
because: Every actions node now lists the layout verb with no cause, so the promise that each entry holds a cause no longer holds.
@row_set n=5 row=contract/recorded-check/tool cell=promise seen=1 detector=none
value: One of the eleven instance verbs.
basis: fct-w5rznwmn
because: The instance verbs moved from ten to eleven with layout, and the cell holds that count.
@row_set n=6 row=contract/command-span/verb cell=promise seen=1 detector=none
value: Records the instance-level verb; a graph, actions or layout call records the verb alone, plus cause when refused.
basis: fct-dxvnlriy
because: An accepted layout call's span records verb alone and a refused one verb and cause, so the list of non-acting verbs must name layout.
@row_add n=1 row=decision/layout-answer-crossing area=threat-model-summary fact=F1 detector=D-security-input
name: Layout answer crossing
rule: A layout answer crosses the session socket, the command line, MCP and a kept check at rest holding only stable ids, closed kebab-case words and numbers, never text, an attribute or a name.
basis: fct-xa2wojve, fct-gd5fq3ek
because: The layout verb puts a new answer on every ratified carrier, and the threat model keeps one crossing row per answer as for graph and actions.
@row_add n=2 row=contract/driver-verbs/layout area=input-validation fact=F2 detector=D-security-input
boundary: Driver verbs
item: layout
promise: Takes one required id, refused malformed without it or with a second argument and not-found or stale for an unknown id; changes nothing, leaves delivered Timer ticks unapplied, and answers the element's placement, shifts and sizes as ids, closed words and numbers.
basis: fct-yx7iahh5, fct-jgbhcxfs, fct-h7vqoq7u
because: A new instance verb with one validated id argument is a new external-input surface, and its promise stands beside the actions and graph verb rows.
@row_add n=3 row=decision/layout-answer-closed-words area=input-validation fact=F3 detector=D-security-input
name: Layout answer words from closed sets
rule: The layout answer's placing, shift and cause words are each a closed kebab-case set, and a later engine member is written as a fallback word, never a panic.
basis: fct-xa2wojve, fct-pmdi26bm
because: The layout answer's words are closed sets with fallback words for non-exhaustive engine sets, which keeps the answer inside the closed-value crossing.
@row_add n=4 row=decision/kept-check-layout-answer area=data-protection fact=F5 detector=none
name: Kept check holds the layout answer
rule: A kept check holds a layout step and its whole answer of ids, closed words and numbers, never a control value, typed text or NodeId, and its replay stops on a changed placed value.
guard: stand_replay a_kept_layout_step_replays_and_a_changed_one_stops_naming_placed
basis: fct-gd5fq3ek, fct-ikxbl3zs
because: A kept check now holds layout steps at rest, and the data-protection rows name each answer a kept check holds as for graph and actions.
@row_add n=5 row=decision/cold-agent-verbs-without-layout area=input-validation fact=F7 detector=none
name: Cold-agent pipe verb list without layout
rule: The cold-agent pipe keeps its verb list at fourteen without layout, so a live run reads not-isolated until the list is decided.
why: The conservative option of an open fork, applied ahead of its decision.
state: provisional
basis: fct-w5rznwmn, fct-uah4xbwy, fct-tnufsys6
because: The pipe's known-verb list deliberately leaves out the fifteenth verb as a provisional exception, which future work on the pipe must respect.
@row_none n=1 fact=F4 type=4
relied: An element the snapshot lists but the engine cannot explain answers the unexplained placing word at its snapshot bounds, inside the closed word set; the code and stand_layout hold it.
@row_none n=2 fact=F6 type=4
relied: Exploration selects target verbs by field kind, leaving out any with an image or layout reading, so layout is never explored; the code and explore_tests hold it.
