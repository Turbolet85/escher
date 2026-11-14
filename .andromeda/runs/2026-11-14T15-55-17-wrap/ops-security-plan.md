@row_set n=1 row=contract/recorded-check/start-record cell=promise seen=1 fact=F2 detector=D-security-input
value: The array opens with `{"tool":"start","arguments":{"app":…},"format":1}`; a check without format reads as format 1, and any other format value, of whatever JSON type, is refused malformed, holding nothing of the bytes.
basis: fct-xetejwtu, fct-vhm6vpzi
because: The kept check's opening record gained the strictly read key format, so the two-key opening this promise states is no longer the whole record.
@row_set n=2 row=contract/kept-check-file/entry cell=promise seen=1 detector=none
value: A check is a JSON array: an opening record of the app label and the format, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
basis: fct-vhm6vpzi, fct-sjgsvbqg
because: The opening record's keys moved from two to three with format, so an entry that opens with the app label alone is incomplete.
@row_set n=3 row=contract/kept-check-file/medium cell=promise seen=1 fact=F5 detector=none
value: Under `--record <dir>`, `Session::recording_into` or an exploration's record directory, one `check-{n}.json` with mode `0600` per recorded session, each explored path its own, is written in a `0700` directory the launcher named.
basis: fct-xetejwtu, fct-kz6ciuhu
because: Exploration keeps each explored path as a kept check in the same owner-only medium, a third way into it the promise must name.
@row_set n=4 row=contract/command-span/line cell=promise seen=1 fact=F9 detector=none
value: A sink-installing host writes one closed command span line per call it runs, an exploration's driven calls included; an exploration writes no span, field or line of its own.
basis: fct-xetejwtu, fct-4cjpr4wd
because: Exploration drives calls in process and adds no span of its own, which the line promise must state so no later work adds one unawares.
@row_add n=1 row=decision/explore-step-budget-bounded area=input-validation fact=F1 detector=D-security-input
name: Exploration step budget bounded
rule: An exploration's step budget is a whole number from 1 to 10,000, 0 or more than the bound giving no budget, and it bounds every acting call the exploration runs, re-reach steps included; the first screen is read unrecorded and costs no step.
basis: fct-xetejwtu
because: Exploration is a new in-process surface whose budget is validated and bounds every call, a rule future work on exploration must keep.
@row_add n=2 row=decision/explored-verbs-no-image area=input-validation fact=F3 detector=D-security-input
name: Explored verbs leave out image results
rule: Exploration drives only the driver table's target verbs whose result holds no image field, chosen by the field's kind and never its name, and the text typed on a path is only the fixed in-bound value the call naming gives.
basis: fct-xetejwtu, fct-na5l7a6y
because: Exploration chooses its verbs and typed text itself, and the chosen limits keep pixels and caller text out of what it drives.
@row_add n=3 row=decision/exploration-report-closed-content area=data-protection fact=F4 detector=none
name: Exploration report holds closed content only
rule: An exploration's report holds only closed endings, indices, calls and booleans, never a panic payload, NodeId, screen text or screen digest, and it is never logged or printed.
basis: fct-xetejwtu, fct-oet33mkf
because: Exploration adds a new report of its own, whose content class must stay closed as every other answer's does.
@row_add n=4 row=decision/exploration-catches-handler-panic area=error-handling fact=F6 detector=none
name: Exploration catches a handler panic per path
rule: During exploration a handler panic is caught for each path, which ends as Panicked, the session is dropped under a second catch and the exploration goes on with the next path; a refused, panicked or diverged path is never extended.
basis: fct-xetejwtu, fct-mpfykyrj
because: Exploration adds a panic catch at its own boundary, a handling rule future work on exploration must keep.
@row_add n=5 row=absence/exploration-on-a-carrier area=threat-model-summary fact=F7 detector=none
capability: Exploration on any driver carrier: no verb, role, argument, socket request or MCP tool reaches it
basis: fct-iipgu2rq, fct-xetejwtu
because: The founder ratified exploration as one in-process call only, so no carrier is widened by it and the threat model must say so.
@row_add n=6 row=absence/exploration-following-navigation area=threat-model-summary fact=F8 detector=none
capability: Exploration that follows navigation beyond the in-page actions of the screen it boots on
basis: fct-zzs4273z, fct-zgf6cgl4
because: The founder answered that exploration follows no navigation, so it never leaves the screen it boots on and the threat model must say so.
