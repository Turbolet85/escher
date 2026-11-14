@row_add n=1 row=decision/exploration-adds-no-span area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Auto-exploration adds no span of its own
rule: Auto-exploration opens no span and emits no event of its own; each call it drives, re-reach steps included, runs through the session's run and leaves its own command span.
basis: fct-xetejwtu, fct-oet33mkf, fct-4cjpr4wd
because: The chunk adds the in-process explore call, instrumented by decision only through the command spans of the calls it drives, and §4 must say so for later work that wraps it.
@row_add n=2 row=decision/exploration-report-holds-no-content area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Exploration report holds no content and is never logged
rule: The exploration report holds closed endings, indices, calls and bools only, never a panic payload, node id, screen text or screen digest, and is never logged or printed.
basis: fct-xetejwtu, fct-oet33mkf
because: The chunk adds a public report type of the driver library, and §8 must bind what it may hold and that it never reaches a log.
@row_add n=3 row=decision/exploration-catches-panics area=7-error-capture-reporting fact=F3 detector=none
name: Exploration catches a panicking call
rule: A panic in a call an exploration drives is caught and reported as the closed ending Panicked holding no payload, the process's panic hook still running, and the exploration goes on.
basis: fct-xetejwtu, fct-onsdyw3x
because: The chunk adds a panic catch around each explored call, and §7 must say that a caught panic is reported without its payload while the hook still runs.
@row_add n=4 row=contract/kept-check/format area=3-observability-harness-contract fact=F4 detector=none
boundary: kept check file
item: format
promise: The opening record carries format, the whole number 1; a check without it reads as format 1, and any other value refuses the check malformed holding nothing of its bytes.
basis: fct-xetejwtu, fct-vhm6vpzi
because: The kept check's opening record gains the key format, read strictly, a file format other tools read.
@row_set n=1 row=contract/escher-session-coverage/malformed cell=promise seen=1 detector=none
value: A file that cannot be read, is over 1,048,576 bytes, is no check, is a check of another app or carries a format other than 1 is refused `malformed`, status 1, holding nothing of the path or the file.
basis: fct-xetejwtu
because: The check reader now refuses any opening-record format other than 1 as malformed, so the closed list of the coverage role's malformed refusals is incomplete.
@row_none n=1 fact=F5 type=4
relied: An exploration keeps its checks through the same recording medium as every session, one check file per explored path's fresh session, and an open directory is refused record-dir-not-private before anything boots, as the standing recording-directory rows already state.
