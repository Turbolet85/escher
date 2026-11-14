@row_add n=1 row=contract/escher-driver-explore/step-budget area=surface-cli fact=F1 detector=none
boundary: escher_driver explore call
item: step budget
promise: A whole number of acting calls from 1 to 10,000, re-reach steps counted and the first screen free; the walk is breadth-first from the first screen, each path on a fresh session, its last call under a panic catch, stopping when the frontier empties (exhausted) or the next path would pass the budget.
basis: fct-xetejwtu
because: The chunk adds the in-process explore call with a bounded step budget, a promise callers rely on that no row states.
@row_add n=2 row=contract/escher-driver-explore/path-ending area=surface-cli fact=F2 detector=none
boundary: escher_driver explore call
item: path ending
promise: Each explored path ends in one closed word: accepted (settled or not), refused with its cause, panicked, or diverged at a step; only an accepted path reaching a screen no earlier path reached is expanded, and refused, panicked and diverged paths never are.
basis: fct-xetejwtu
because: The exploration report's closed path endings are a new outward promise of the explore call, stated by no row.
@row_add n=3 row=contract/escher-driver-explore/explored-verbs area=surface-cli fact=F3 detector=none
boundary: escher_driver explore call
item: explored verbs
promise: The verbs explored are the actions table's target verbs whose result holds no image field, selected by the field's kind and never its name; screenshot is left out.
basis: fct-na5l7a6y
because: The report disposes the plan's name-based wording into a new contract row stating the selection by the field's kind.
@row_add n=4 row=contract/escher-driver-explore/record-dir area=surface-cli fact=F7 detector=none
boundary: escher_driver explore call
item: record directory
promise: Given a record directory, the exploration keeps one replayable check per explored path in that owner-only directory, a panicked path's check holding the steps before the panic.
basis: fct-xetejwtu
because: Exploration now writes kept checks per path into the owner-only record directory, a promise no recording row covers.
@row_add n=5 row=contract/escher-session-record/format area=surface-cli fact=F6 detector=none
boundary: escher-session recording
item: format
promise: A kept check's opening record states the check format, the whole number 1; a check without it reads as format 1, and any other value (another number, 1.0, a string, null, a bool) refuses the check as malformed.
basis: fct-vhm6vpzi
because: The kept check's form gains the opening record's key format, read strictly, a file-format promise its readers rely on.
@row_add n=6 row=absence/exploration-on-carriers area=surface-cli fact=F4 detector=none
capability: Auto-exploration on any driver carrier: no verb, role, argument, command-line form or MCP tool runs it
basis: fct-iipgu2rq
because: The founder ruled exploration one in-process call, so the command line and the MCP server lack the whole capability.
@row_add n=7 row=absence/exploration-following-navigation area=surface-cli fact=F5 detector=none
capability: Exploration that follows navigation beyond the one screen tree it starts on
basis: fct-zzs4273z
because: The founder answered that exploration follows no navigation, a whole capability the project does not have.
