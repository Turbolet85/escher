@row_add n=1 row=decision/layout-reason-reader-read-only area=input-validation fact=F1 detector=D-security-input
name: Engine layout reader read-only with none answers
rule: The engine's layout reader on every document is read-only and in process, answers none for an unresolved id, a non-element, an element with no computed styles or one with no box, and has no log site and no panic path.
basis: fct-x3q3dkxj
because: The chunk adds a new public reader on every document, and its none answers, read-only nature and absence of log site and panic path are the input-validation decision future readers rely on.
@row_add n=2 row=decision/session-layout-read-in-process area=input-validation fact=F2 detector=D-security-input
name: In-process layout read on a session
rule: The session's layout read takes its id from Rust code alone, resolves it through the run path's id resolution refusing stale or not-found, runs no step, pump, tick or settle, and answers stable id strings, closed words and numbers, never a NodeId, text or value.
basis: fct-x3q3dkxj, fct-igtskrq7, fct-5qpmkurf
because: A new public driver read takes a caller id, and its validation through the shared resolution and its closed answer are the boundary §Input Validation must state.
@row_add n=3 row=decision/layout-read-no-carrier area=input-validation fact=F3 detector=none
name: No carrier reaches the layout read
rule: No verb, MCP tool, socket call or kept check reaches the session's layout read; the read stays in process only.
why: Applied ahead of the founder's answer to the fork on putting the layout read on the carriers.
state: provisional
basis: fct-x3q3dkxj, fct-woiwjiy7, fct-u7f3beyp
because: The chunk keeps the layout read off every carrier as a provisional decision pending the founder's word, and security-plan must record that no exit was added.
@row_add n=4 row=decision/layout-answer-app-placer-reserved area=input-validation fact=F5 detector=none
name: Reserved app placer carries no app string
rule: The layout answer's app placer word, holding a canvas string, is reserved and built by no path outside the driver's unit tests, so no app-supplied string reaches the layout answer.
basis: fct-x3q3dkxj, fct-5qpmkurf
because: The new answer type reserves a placer word holding a string, and keeping it unbuilt is what keeps app text out of the answer.
@row_set n=1 row=decision/no-screen-content-in-logs cell=rule seen=1 fact=F6 detector=none
value: No driver call or in-process read puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer, a layout answer and a coverage check reach no log line at any level.
basis: fct-iqboitei, fct-igtskrq7
because: The new layout read opens no span, emits no event and writes no escher line at TRACE, so the row's list of answers kept out of logs must name the layout answer.
@row_none n=1 fact=F4 type=9
what: The unexplained element's closed word of its own before any carrier is an owed item, pinned as a CARRY by the route.
