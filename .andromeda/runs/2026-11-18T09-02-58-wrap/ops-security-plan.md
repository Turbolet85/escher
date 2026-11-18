@row_set n=1 row=decision/cold-agent-screen-to-provider cell=rule seen=2 fact=F3 detector=none
value: A cold-agent live run carries the stand's screen, ids, accessible names and control values in snapshot texts and diffs, layout, graph and actions answers, and an acting step's caused text, through the claude client to the model provider; the reply returns as a check the pipe parses and replays.
basis: fct-vly345k6, fct-xfqd7f22, fct-pkm3ir2p
because: The founder approved graph and actions answers crossing to the model provider on the live run (F4) and placed a served error page before that run (F5), so caused text is now in the run's reach and the row must name both.
@row_set n=2 row=decision/cold-agent-events-metadata-only cell=rule seen=1 fact=F1 detector=none
value: The cold-agent pipe's events and verdict carry counts, booleans and closed words, refusals as counts per closed cause word with a word outside the closed set not counted, never transcript text, a tool argument or answer, a reply or the check.
basis: fct-wyr2e6ga, fct-w53lgzmr, fct-pf5rizdr
because: The verdict and its run.end event now carry refused_by_cause, read from the server log's command span lines against the eight closed cause words, and list_opens, a count, so the row must say a log-read word reaches the verdict only as a closed word's count.
@row_add n=1 row=decision/cold-agent-transcript-answers-counted-only area=input-validation fact=F2 detector=D-security-input
name: Cold-agent transcript answers decoded only to be counted
rule: The cold-agent pipe decodes the transcript's acting answers only to count list openings; an answer that does not decode is skipped, and the decoded answer leaves the pipe only as that count.
basis: fct-wyr2e6ga, fct-pf5rizdr
because: The chunk adds a new read of the transcript's acting answers, a JSON decode whose undecodable input is skipped and whose content leaves only as the list_opens count, and no row of Input Validation states that boundary.
