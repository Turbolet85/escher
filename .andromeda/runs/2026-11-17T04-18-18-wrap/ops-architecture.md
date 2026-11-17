@row_set n=1 row=contract/verb-table/order cell=promise seen=1 detector=D-arch-resources
value: The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.
basis: fct-yx7iahh5, fct-w5rznwmn
because: The verb table gained `layout` after `actions`, fifteen verbs, so the row's count and order no longer hold.
@row_set n=2 row=contract/driver-session/command-schema cell=promise seen=1 detector=D-arch-resources
value: The command and refusal schema is stated once as 'static data: fifteen verbs, eight closed causes.
basis: fct-w5rznwmn
because: The schema now states fifteen verbs, the row holding that count.
@row_set n=3 row=contract/mcp-server/tools-list cell=promise seen=1 detector=D-arch-resources
value: The fifteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
basis: fct-w5rznwmn
because: The MCP listing now names fifteen tools, `layout` after `actions`, the row holding that count.
@row_set n=4 row=contract/kept-check/kept-verbs cell=promise seen=1 detector=D-arch-resources
value: Only calls of the eleven instance verbs are kept, `lint`, `graph`, `actions` and `layout` among them with their answers whole; any other call is not kept.
basis: fct-w5rznwmn, fct-gd5fq3ek
because: `layout` joined the instance verbs, eleven now, and a kept check keeps its answer and replays it.
@row_set n=5 row=contract/kept-check-reader/strict-read cell=promise seen=1 detector=D-arch-resources
value: A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the eleven instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
basis: fct-w5rznwmn
because: The reader now admits the eleven instance verbs, `layout` among them, the row holding that count.
@row_set n=6 row=contract/driver-explore/explored-verbs cell=promise seen=1 detector=none
value: The verbs explored are the actions table's target verbs less any whose result holds an image or a layout reading field (png, rect, placement, shifts or size kind), selected by the field's kind and never its name; today click, type and scroll.
basis: fct-yx7iahh5
because: The explored-verbs rule now also leaves out a verb with a layout reading field, so `layout` is not explored.
@row_set n=7 row=contract/actions-verb/target-verbs cell=promise seen=1 detector=none
value: For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot` · `layout`, `layout` with no cause.
basis: fct-yx7iahh5, fct-gjow6tts
because: Every listed node now carries `layout` fifth with no cause, so the row's list of target verbs is incomplete.
@row_set n=8 row=contract/driver-session/layout cell=promise seen=1 detector=D-arch-resources
value: A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span; it resolves its id as the run path does, refusing stale or not-found, runs no step, pump, tick or settle and opens no span of its own.
basis: fct-yx7iahh5, fct-stryxpwu, fct-dxvnlriy
because: The read is now also reached by the `layout` verb through the run path, inside the call's command span, so «in-process read» no longer holds.
@row_set n=9 row=decision/no-node-id-across-session cell=rule seen=2 detector=none
value: Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
basis: fct-xa2wojve
because: The layout answer now crosses a session holding ids, closed words and numbers, which the row's list leaves out.
@row_set n=10 row=decision/select-list-crossing cell=rule seen=1 detector=none
value: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, the placing words select and select-list in a layout answer, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span.
basis: fct-xa2wojve
because: The layout answer writes the placing words `select` and `select-list` on every carrier, so «only» in the row no longer holds.
@row_retire n=1 row=decision/layout-read-in-process-only seen=1 heir=decision/layout-answer-crossing detector=D-arch-decisions
why: The `layout` verb now reaches the driver's layout read on the session socket, the command line, MCP and a kept check.
basis: fct-widxbd2b, fct-yx7iahh5, fct-stryxpwu
because: `Outcome::Layout` and its run arm make the in-process-only decision false, and the carriers' crossing replaces it.
@row_add n=1 row=decision/layout-answer-crossing area=standard-contracts detector=D-arch-resources
name: Layout answer crosses the carriers
rule: The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words and numbers only, never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, node key, span field or dependency.
basis: fct-zud3mrzc, fct-gd5fq3ek, fct-xa2wojve, fct-yx7iahh5
because: The fifteenth verb carries the layout read on every ratified carrier, the heir of the retired in-process-only decision.
@row_add n=2 row=contract/driver-verbs/layout area=standard-contracts fact=F1 detector=D-arch-resources
boundary: driver verbs
item: layout
promise: Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the element sits and why at its size, changing nothing and running no step.
basis: fct-yx7iahh5, fct-jgbhcxfs
because: The verb table gained `layout`, and the per-verb rows of driver verbs hold none for it.
@row_add n=3 row=contract/answer-json/layout area=standard-contracts detector=D-arch-resources
boundary: answer JSON
item: layout
promise: {"id","bounds","laid_out","placed":{"by","container"?,"canvas"?,"at","relative"?},"shifts":[{"by","holder"?,"dx","dy"}],"width","height"}, each size {"px","by","size","min","max","flex"?}; ids, closed words and numbers only.
basis: fct-xa2wojve
because: The layout answer's written form is a new answer shape every carrier writes, and answer JSON holds no row for it.
@row_add n=4 row=contract/answer-json/layout-placing area=standard-contracts fact=F3 detector=D-arch-resources
boundary: answer JSON
item: layout placing word
promise: placed's by is one of block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained or app, written kebab-case.
basis: fct-xa2wojve
because: The layout answer writes a closed set of placing words, a contract at the answer's boundary.
@row_add n=5 row=contract/answer-json/layout-shift area=standard-contracts fact=F3 detector=D-arch-resources
boundary: answer JSON
item: layout shift word
promise: A shift's by is scroll or transform.
basis: fct-xa2wojve
because: The layout answer writes a closed set of shift words, a contract at the answer's boundary.
@row_add n=6 row=contract/answer-json/layout-size-cause area=standard-contracts fact=F3 detector=D-arch-resources
boundary: answer JSON
item: layout size cause word
promise: A size's by is one of style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content or layout.
basis: fct-xa2wojve
because: The layout answer writes a closed set of size cause words, a contract at the answer's boundary.
@row_add n=7 row=contract/answer-json/layout-length area=standard-contracts fact=F3 detector=D-arch-resources
boundary: answer JSON
item: layout length
promise: A length is "auto", "none", "min-content", "max-content", "fit-content", "stretch", {"px":n}, {"percent":n} or {"calc":n}.
basis: fct-xa2wojve
because: The layout answer writes a closed set of length forms, a contract at the answer's boundary.
@row_add n=8 row=contract/answer-json/layout-fallback area=standard-contracts fact=F3 detector=D-arch-resources
boundary: answer JSON
item: layout fallback words
promise: A later member of an engine set is written unexplained, transform, layout or auto, never a panic.
basis: fct-xa2wojve, fct-pmdi26bm
because: The engine's layout sets are non-exhaustive, and the answer promises a fallback word for each in place of a panic.
@row_add n=9 row=contract/driver-session/layout-unexplained area=standard-contracts fact=F4 detector=D-arch-resources
boundary: escher-driver session
item: unexplained placer
promise: An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds.
basis: fct-yx7iahh5, fct-ighszuki
because: `Placer::Unexplained` replaced a block placer for such an element, a promise of the layout read no row states.
@row_add n=10 row=contract/escher-session/layout area=standard-contracts fact=F5 detector=D-arch-resources
boundary: escher-session command line
item: layout --id
promise: The verb layout takes --id with --session and answers the run's one JSON line, status 0 on an answer and 1 on a refusal; a missing --id is refused malformed.
basis: fct-gd5fq3ek, fct-suhuk4an
because: The command line gained `layout --id`, and its per-verb rows hold none for it.
@row_add n=11 row=decision/cold-agent-pipe-fourteen-verbs area=standard-contracts fact=F6 detector=none
name: Cold-agent pipe at fourteen verbs
rule: The cold-agent pipe's verb list holds fourteen verbs, leaving layout out, until the founder widens it.
why: The conservative option of a fork held for the founder's own word.
state: provisional
basis: fct-w5rznwmn, fct-tnufsys6, fct-dbhvnokp
because: The verb table grew to fifteen while the cold-agent pipe was kept at fourteen, a provisional decision future work must respect.
@row_none n=1 fact=F2 type=5
what: The count and names of the verb table's result field kinds are an inventory the code's FieldKind enum holds.
