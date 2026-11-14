@row_set n=1 row=decision/mutation-controlled-tests cell=rule seen=1 fact=F9 detector=none
value: A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
basis: fct-dbgbmdwy
because: The chunk's budget-bound control loosened the bound instead of removing it so the mutated run stayed bounded, a convention the controlled-tests rule now carries.
@row_add n=1 row=contract/explore-call/paths area=integration-test-strategy fact=F1 detector=none
boundary: the explore call
item: paths
promise: An exploration boots once to read the first screen, unrecorded and at no step, then runs paths off a breadth-first frontier, each on a fresh session, expanding only an accepted path reaching a screen no earlier path reached; it stops when the frontier empties or the next path would pass the budget.
basis: fct-xetejwtu
because: The chunk adds the in-process explore call, whose path order and stop condition its integration checks hold.
@row_add n=2 row=contract/explore-call/step-budget area=integration-test-strategy fact=F2 detector=none
boundary: the explore call
item: step budget
promise: A step budget is a whole number from 1 to 10,000 acting calls, re-reach steps counted; 0 or a number past 10,000 is no budget.
basis: fct-xetejwtu
because: The chunk adds the exploration's step budget and its bound, pinned by the budget-bound table and the budget check.
@row_add n=3 row=contract/exploration-path-ending/accepted area=integration-test-strategy fact=F3 detector=none
boundary: exploration path endings
item: Accepted
promise: A path whose last call was accepted ends Accepted, reading whether it settled; only an Accepted path is ever expanded.
basis: fct-xetejwtu
because: The chunk adds the exploration report's closed path endings.
@row_add n=4 row=contract/exploration-path-ending/refused area=integration-test-strategy fact=F3 detector=none
boundary: exploration path endings
item: Refused
promise: A path whose last call was refused ends Refused with its cause, and is never expanded.
basis: fct-xetejwtu
because: The chunk adds the exploration report's closed path endings.
@row_add n=5 row=contract/exploration-path-ending/panicked area=integration-test-strategy fact=F3 detector=none
boundary: exploration path endings
item: Panicked
promise: A path whose last call panicked ends Panicked, and is never expanded.
basis: fct-xetejwtu
because: The chunk adds the exploration report's closed path endings.
@row_add n=6 row=contract/exploration-path-ending/diverged area=integration-test-strategy fact=F3 detector=none
boundary: exploration path endings
item: Diverged
promise: A path whose re-reach did not hold ends Diverged at the step where it parted, and is never expanded.
basis: fct-xetejwtu
because: The chunk adds the exploration report's closed path endings.
@row_add n=7 row=decision/exploration-report-holds-no-content area=integration-test-strategy fact=F3 detector=none
name: Exploration report holds no screen content
rule: An exploration report carries closed words, indices, calls and booleans only, never a panic payload, an internal node id, screen text or a digest.
basis: fct-xetejwtu
because: The chunk adds the exploration report, whose closed content is a privacy promise its checks hold.
@row_add n=8 row=contract/explore-call/panic area=integration-test-strategy fact=F4 detector=none
boundary: the explore call
item: panic
promise: A panic in a handler during exploration is caught, the panicked session dropped under a second catch, and the exploration goes on; the next boot on the thread answers.
basis: fct-xetejwtu
because: The chunk adds panic catching to exploration, held by its panicking-handler check.
@row_add n=9 row=decision/explored-verbs-chosen-by-field-kind area=integration-test-strategy fact=F5 detector=none
name: Explored verbs chosen by result field kind
rule: Exploration explores the verb table's target verbs whose result holds no image field, chosen by the field's kind and never by its name, so screenshot is never explored.
guard: a_verb_whose_result_holds_an_image_is_left_out_by_its_field_not_its_name
basis: fct-xetejwtu
because: The chunk chooses the explored verbs by result field kind, a selection rule future verbs fall under.
@row_add n=10 row=contract/session-recording/exploration-checks area=session-lifecycle fact=F6 detector=none
boundary: session recording
item: exploration checks
promise: With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a panicked path's check holds the steps before the panic.
basis: fct-xetejwtu
because: The chunk records each explored path as a kept check, a promise its recording checks hold.
@row_add n=11 row=contract/session-recording/format area=session-lifecycle fact=F7 detector=none
boundary: session recording
item: format
promise: A kept check's opening record carries format, written as the integer 1; a check without format reads as format 1, and any other value refuses the check as malformed holding nothing of its bytes.
basis: fct-vhm6vpzi
because: The kept check's form gains the opening record's format key, written and read strictly.
@row_add n=12 row=decision/exhaustion-asserted-on-finite-page area=integration-test-strategy fact=F8 detector=none
name: Exploration exhaustion asserted on a finite page
rule: Exhaustion of an exploration's frontier is asserted only on a finite fixture page; an exploration of the Counter asserts exhausted false.
why: An unbounded app never exhausts a breadth-first frontier.
guard: a_refusing_control_is_met_and_reported_by_its_cause, the_counter_is_explored_and_each_path_replays_green
basis: fct-fxbk34p7
because: The plan's premise that the Counter exhausts was measured false, so the test strategy moved exhaustion to a finite page.
@row_add n=13 row=absence/exploration-on-driver-carriers area=integration-test-strategy fact=F10 detector=none
capability: Exploration reached through a driver verb, role, argument or carrier
basis: fct-iipgu2rq
because: The founder ruled exploration in process only, so no carrier reaches it.
@row_add n=14 row=absence/exploration-following-navigation area=integration-test-strategy fact=F10 detector=none
capability: Exploration that follows navigation
basis: fct-zzs4273z
because: The founder answered that exploration follows no navigation, ratified rather than provisional.
