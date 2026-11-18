@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=14 class=omission
leaf-text: `stand::boot_load(options)` the load page (TaskShell titled "Load")
source: ctr-pxggfjvw «stand::boot_order(HarnessOptions) boots the order page offline in TaskShell with the options given, the seventh served task's stand boot.»
replace-with: `stand::boot_load(options)` the load page (TaskShell titled "Load"), `stand::boot_order(options)` the order page offline in TaskShell, the seventh served task's boot
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=16 class=stale
leaf-text: asking for six apps one at a time
source: ctr-etenfqik «`stand` is the one task: seven apps (counter, flight booker, timer, CRUD, cells, the load page and the order page) in one client session; any other task, `run counter` included, is a usage error.»
replace-with: asking for seven apps one at a time
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=16 class=stale
leaf-text: (plus one visual lint) and the load page —
source: ctr-etenfqik «`stand` is the one task: seven apps (counter, flight booker, timer, CRUD, cells, the load page and the order page) in one client session; any other task, `run counter` included, is a usage error.»
replace-with: (plus one visual lint), the load page and the order page —
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=16 class=stale
leaf-text: all six apps in `tasks_met`
source: dec-czga7niw «A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all seven apps are met, every counted capability was used, recordings were kept,»
replace-with: all seven apps in `tasks_met`
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=16 class=omission
leaf-text: `list_opens` above 0 and `recording_kept`
source: ctr-c2yaicpl «A run that leaves a counted capability unused, or whose `caused_by_kind` reads 0 for any of panic, error and log, fails `capability-unused`.»
replace-with: `list_opens` above 0, `caused_by_kind` above 0 for each of panic, error and log, and `recording_kept`
@leaf_edit n=6 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: every node of the six served screens in both layout modes
source: ctr-k7ugyh26 «The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process»
replace-with: every node of the seven served screens in both layout modes
@leaf_edit n=7 leaf=.claude/docs/tests-summary.md line=26 class=stale
leaf-text: and the six served tasks are read on every CI OS
source: dec-fcya4gfl «The seven served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both»
replace-with: and the seven served tasks are read on every CI OS
