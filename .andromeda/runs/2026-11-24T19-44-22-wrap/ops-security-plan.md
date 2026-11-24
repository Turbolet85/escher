@row_retire n=1 row=decision/canvas-content-crosses-as-shot-pixels seen=1 heir=decision/canvas-content-crossing detector=none
why: A canvas's content no longer crosses only as shot pixels: the canvas verb carries its draw commands as closed words, numbers and digests on the socket, the command line, MCP and a kept check.
basis: fct-x7cb4p36, fct-f4jjdetj
because: The chunk adds the canvas verb on every carrier, so the row's name, rule and key, all built on pixels alone, are false and an heir states the widened crossing.
@row_add n=1 row=decision/canvas-content-crossing area=threat-model-summary fact=F1 detector=none
name: Canvas content crossing
rule: A canvas's app-painted content crosses as pixels in a shot and, on the socket, the command line, MCP and a kept check, as a canvas answer of closed words, numbers and 64-bit digests, fonts and images as indexes with digests, never a glyph id, glyph position, text, font bytes or family name.
why: A boundary widening, ratified.
guard: stand_canvas_carriers the_fixtures_answers_hold_no_glyph_id_and_no_text
basis: fct-x7cb4p36, fct-f4jjdetj
because: The canvas verb carries a canvas's draw commands on every ratified carrier as closed values and digests, replacing the pixels-only crossing.
@row_add n=2 row=decision/cold-agent-canvas-to-provider area=threat-model-summary detector=none
name: Cold-agent runs carry canvas answers to the provider
rule: The cold-agent pipe's call allowlist takes the canvas verb, so a live run carries canvas answers through the claude client to the model provider.
basis: fct-rrwh4urw
because: The cold-agent pipe's allowlist takes canvas, and the standing provider row cannot take the canvas answer within its bound.
@row_set n=1 row=contract/actions-answer/verbs cell=promise seen=2 detector=none
value: A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed node carries layout with no cause, and canvas with no cause on an app that can paint text.
basis: fct-snllpxfi, fct-f37kboln
because: The actions answer now lists canvas among each node's target verbs, with no cause where the app can paint text.
@row_set n=2 row=contract/recorded-check/tool cell=promise seen=2 detector=none
value: One of the twelve instance verbs.
basis: fct-snllpxfi
because: The instance verbs moved from eleven to twelve with canvas, and a kept check's tool may name it.
@row_set n=3 row=decision/time-moves-only-on-advance cell=rule seen=1 detector=none
value: Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph, actions and canvas wait on no load.
basis: fct-gekogvs7, fct-orfcp2mx
because: The canvas read moves no clock and leaves timer ticks unapplied, so it joins the verbs that wait on no load.
@row_set n=4 row=decision/no-screen-content-in-logs cell=rule seen=3 fact=F6 detector=none
value: No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer, a canvas answer, a coverage check and a step's caused text reach no log line at any level.
basis: fct-ez6ffs55
because: No canvas answer content reaches the host or MCP role log at trace, so the canvas answer joins the content kept out of logs.
@row_set n=5 row=contract/command-span/verb cell=promise seen=3 detector=none
value: Records the instance-level verb; a graph call, a graph part's included, an actions, layout or canvas call records the verb alone, plus cause when refused, never the part asked or the part count.
basis: fct-ez6ffs55, fct-uhfo5wt4
because: The command span records the canvas verb alone and, refused, its cause.
@row_add n=3 row=contract/driver-verbs/canvas area=input-validation fact=F2 detector=D-security-input
boundary: Driver verbs
item: canvas
promise: Takes a required id within the id bound, an optional full flag and an optional part from 1 to 1000; any other argument, full twice, or part 0 or 1001 is refused malformed with nothing run, and the verb adds no cause, argument kind, session error or exit status.
basis: fct-hecr3z5l, fct-uhfo5wt4
because: The chunk adds the canvas verb as a new external-input surface validated by the closed schema, and its argument contract stands nowhere.
@row_add n=4 row=decision/session-canvas-read-in-process area=input-validation fact=F3 detector=D-security-input
name: Canvas read on a session changes nothing
rule: The session's canvas read, in process or through the canvas verb on any ratified carrier, refuses an unknown id as not-found or stale and an app that cannot paint text as fonts-unavailable, and moves no snapshot, focus reading, diff or virtual clock, leaving timer ticks unapplied.
guard: stand_canvas_commands a_canvas_read_changes_nothing_with_timer_ticks_pending
basis: fct-gekogvs7, fct-x7cb4p36
because: The new canvas read is a read-only verb with its own refusals, which no standing row states.
@row_add n=5 row=name/command/canvas area=input-validation fact=F7 detector=none
class: command
name: canvas
basis: fct-x7cb4p36
because: The chunk occupies canvas as the driver verb and the MCP tool name.
@row_add n=6 row=contract/cli-flags/full area=input-validation detector=none
boundary: escher-session flags
item: --full
promise: Bare, taking no value.
basis: fct-x7cb4p36
because: The escher-session command line gains the bare --full flag of the canvas command.
@row_add n=7 row=decision/kept-check-canvas-answer area=data-protection fact=F5 detector=none
name: Kept check holds the canvas answer
rule: A kept check holds a canvas step and its whole answer of closed words, numbers and 64-bit digests, never a glyph id, text or NodeId, and its replay stops naming digest when a canvas digest differs.
why: A boundary widening, ratified.
guard: stand_canvas_carriers a_kept_canvas_check_replays_green_and_a_flipped_digest_stops_it
basis: fct-fzvmboq3, fct-f4jjdetj
because: A kept check now records canvas calls whole, a new content class beside the layout, graph and actions answers it keeps.
@row_add n=8 row=decision/canvas-answer-paged area=api-security fact=F4 detector=none
name: The canvas dump is paged
rule: The canvas summary is unpaged; a full dump over the 10,000-byte text budget is cut into parts of whole lines each within it, the font and image indexes in part 1 alone and a part past the last empty.
guard: stand_canvas_commands a_full_dump_over_the_budget_reads_in_parts_each_within_it
state: provisional
basis: fct-gr2bwf56, fct-orfcp2mx
because: The canvas answer adds a paged dump under the answer budget, applied on a delegate's word ahead of its ratification.
