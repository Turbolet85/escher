@row_add n=1 row=decision/exploration-in-process-only area=surface fact=F1 detector=none
name: Exploration in process only
rule: Auto-exploration is one in-process driver call and reaches no carrier: no verb, role, argument, command or mcp tool of escher-session exposes it.
why: ratified
basis: fct-iipgu2rq
because: The founder ruled exploration in process only, so the carriers' surface must say no command or tool reaches it.
@row_add n=2 row=decision/exploration-follows-no-navigation area=surface fact=F2 detector=none
name: Exploration follows no navigation
rule: Auto-exploration follows no navigation; it explores the one app it boots.
why: ratified
basis: fct-zzs4273z
because: The founder answered the navigation fork with A, so an exploration stays within the one app it boots and that bound is now ratified.
@row_add n=3 row=contract/explore-call/step-budget area=surface fact=F3 detector=none
boundary: The driver's explore call
item: step budget
promise: A step budget is a whole number from 1 to 10,000 acting calls, re-reach steps counted; 0 or more than 10,000 is no budget.
basis: fct-xetejwtu
because: The chunk adds the exploration's step budget with a closed range, a promise its callers rely on.
@row_add n=4 row=contract/explore-call/path-ending area=surface fact=F4 detector=none
boundary: The driver's explore call
item: path ending
promise: An explored path ends in one closed word: accepted with whether it settled, refused with its cause, panicked, or diverged at a step index; the ending holds no panic payload, node id, screen text or digest.
basis: fct-xetejwtu
because: The chunk adds the closed path endings an exploration reports, and what they never hold.
@row_add n=5 row=contract/explore-call/explored-verbs area=surface fact=F5 detector=none
boundary: The driver's explore call
item: explored verbs
promise: The verbs explored are the action table's target verbs whose result holds no image field, chosen by the field's kind and not its name; screenshot is excluded.
basis: fct-na5l7a6y
because: The chunk fixes which verbs an exploration drives, by the result field's kind, so a new verb's inclusion follows from its table entry.
@row_add n=6 row=contract/kept-check/format area=surface fact=F6 detector=none
boundary: The kept check
item: format
promise: A kept check's opening record carries the format number 1; absent it reads as 1, and any other value, another number, 1.0, a string, null or a boolean, refuses the check as malformed.
basis: fct-vhm6vpzi
because: The kept check's form gains a format key read strictly, a promise every reader of a check now keeps.
@row_add n=7 row=contract/explore-call/recording-dir area=surface fact=F7 detector=none
boundary: The driver's explore call
item: recording directory
promise: An exploration that records keeps one check per path in an owner-only directory; a directory open to the group or to others is refused before anything boots.
basis: fct-xetejwtu
because: The chunk adds recording of explored paths under the owner-only medium, refused before boot when the directory is open.
