@row_set n=1 row=contract/kept-check/opening-record cell=promise seen=1 fact=F9 detector=D-arch-resources
value: A kept check opens with the record `{"tool":"start","arguments":{"app":<label>},"format":1}`, `format` the public check format number CHECK_FORMAT, now 1; a reader accepts only that whole number or its absence, read as 1, and refuses any other value malformed.
basis: fct-xetejwtu, fct-vhm6vpzi
because: The opening record gained the strictly read key `format`, so the row's two-key record is no longer what a kept check opens with.
@row_set n=2 row=decision/no-seed-on-carriers cell=state seen=1 detector=none
value: standing
basis: fct-ohhhzzn3
because: The founder confirmed answer A by his own word, so the decision stands ratified and no longer provisional.
@row_set n=3 row=decision/unit-tests-at-file-end cell=except seen=1 detector=none
value: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
basis: fct-w3z3di4h
because: The explore module gained a `#[path]` test child of its own, so the count the exception states no longer holds.
@row_add n=1 row=contract/driver-explore/explore area=standard-contracts fact=F1 detector=D-arch-resources
boundary: escher-driver explore
item: explore
promise: An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot re-reaching through its earlier calls; a boot error ends it.
basis: fct-xetejwtu
because: The driver gained the public in-process explore call, a new contract no row states.
@row_add n=2 row=contract/driver-explore/expansion area=standard-contracts fact=F1 detector=D-arch-resources
boundary: escher-driver explore
item: path expansion
promise: A path accepted on a screen no earlier path reached gains one child per actionable node and explored verb; a refused, panicked or diverged path is never expanded.
basis: fct-xetejwtu
because: The explore call's frontier grows by this rule, a new contract no row states.
@row_add n=3 row=contract/driver-explore/exhausted area=standard-contracts fact=F6 detector=D-arch-resources
boundary: escher-driver explore
item: exhausted
promise: An exploration stops exhausted when its frontier empties, which only a finite set of screens reaches; an app whose screens never repeat never exhausts it, and such an exploration stops when the next path would pass the step budget.
basis: fct-xetejwtu, fct-fxbk34p7
because: The exploration's two endings, and that the budget alone bounds an unbounded app, are new contract no row states.
@row_add n=4 row=contract/driver-explore/step-budget area=standard-contracts fact=F2 detector=D-arch-resources
boundary: escher-driver explore
item: StepBudget
promise: A step budget is a whole number from 1 to 10,000 (MAX_EXPLORE_STEPS), refused as none at 0 and above the bound, and counts every acting call an exploration runs, re-reach steps included.
basis: fct-xetejwtu
because: The public StepBudget type and its bound are new and no row states them.
@row_add n=5 row=contract/driver-explore/path-ending area=standard-contracts fact=F3 detector=D-arch-resources
boundary: escher-driver explore
item: PathEnding
promise: An explored path ends accepted (settled or not), refused with its cause, panicked, or diverged at a step; a handler panic is caught per path and the exploration goes on.
basis: fct-xetejwtu
because: The closed path endings and the per-path panic catch are a new public contract no row states.
@row_add n=6 row=contract/driver-explore/explored-verbs area=standard-contracts fact=F4 detector=D-arch-resources
boundary: escher-driver explore
item: explored verbs
promise: The verbs explored are the actions table's target verbs less any whose result holds an image-kind field, selected by the field's kind and never its name; today click, type and scroll.
basis: fct-xetejwtu, fct-na5l7a6y
because: The exploration's verb selection is a new rule no row states.
@row_add n=7 row=contract/driver-explore/exploration-report area=standard-contracts fact=F5 detector=D-arch-resources
boundary: escher-driver explore
item: Exploration
promise: The exploration report holds closed words, indices, calls and bools only, never a panic payload, NodeId, screen text or screen digest, and is never logged or printed.
basis: fct-xetejwtu, fct-oet33mkf
because: The public exploration report is new and what it may hold is a promise no row states.
@row_add n=8 row=absence/exploration-on-carriers area=standard-contracts fact=F7 detector=none
capability: Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
basis: fct-quogk6m5, fct-iipgu2rq
because: The founder answered A, so exploration is one in-process library call and its absence on every carrier is a whole capability the project does not have.
@row_add n=9 row=absence/exploration-follows-navigation area=standard-contracts fact=F8 detector=none
capability: Exploration that follows navigation to another document
basis: fct-zzs4273z, fct-zgf6cgl4
because: The founder answered F2 A, so exploration explores the current screens' actions only and never navigates, a whole capability's absence.
