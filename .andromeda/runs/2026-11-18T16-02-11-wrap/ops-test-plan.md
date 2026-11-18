@row_set n=1 row=decision/cross-os-readings-by-answer-digest cell=rule seen=1 detector=none
value: The seven served tasks are read on every CI OS by one stand check that compares the driver's answer digests for snapshot (every part), lint and actions, at boot and after one acting step in both layout modes, to one committed table, with no shot and no kept check file.
basis: fct-uduwzs7i
because: The served-task set moved from six to seven with the order page, and the cross-OS readings table now reads it, so the row's six is false.
@row_set n=2 row=decision/cold-agent-passes-on-positive-evidence cell=rule seen=1 detector=none
value: A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all seven apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
basis: fct-uduwzs7i
because: The cold-agent pipe now requires seven apps met, so the row's all six apps is false.
@row_set n=3 row=decision/cold-agent-prompt-names-no-ids cell=rule seen=1 detector=none
value: The fixed stand prompt asks for the seven apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped, then the check as the reply's last json block, and names no element id and no app slug.
basis: fct-wnmdhla7
because: The stand prompt now names seven apps and a seventh goal, so the row's six apps is false.
@row_set n=4 row=contract/cold-agent-tasks/stand cell=promise seen=1 detector=none
value: `stand` is the one task: seven apps (counter, flight booker, timer, CRUD, cells, the load page and the order page) in one client session; any other task, `run counter` included, is a usage error.
basis: fct-uduwzs7i
because: The pipe's stand task gained the order page as its seventh app, so the promise's six apps is false.
@row_set n=5 row=contract/cold-agent-verdict/capability-unused cell=promise seen=1 detector=none
value: A run that leaves a counted capability unused, or whose `caused_by_kind` reads 0 for any of panic, error and log, fails `capability-unused`.
basis: fct-vkogrj6o
because: The verdict's capability-unused reason now also fires when any caused kind reads 0, so the promise is incomplete.
@row_set n=6 row=contract/escher-session-command-line/start cell=promise seen=1 detector=none
value: `start <task> [--record <dir>]`, the task one of the seven served tasks; with `--record` the recording directory is checked before anything is spawned, a refusal status `3` with nothing spawned.
basis: fct-wnmdhla7
because: The served-task set the start command takes is now seven with order, so the promise's four-task list is false.
@row_set n=7 row=contract/session-run/layout-answer cell=promise seen=1 detector=none
value: The `layout` verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, holding ids, closed kebab-case words and numbers only, equal to the in-process layout read on every node of the seven served screens in both layout modes.
basis: fct-uduwzs7i
because: The stand layout check now reads seven served apps, so the promise's six served screens is false.
@row_set n=8 row=decision/count-and-rename-sweeps cell=except seen=1 fact=F4 detector=none
value: A test name holding a count word that another check pins as a witness string is kept by a count sweep, the name searched before any rename.
basis: fct-mxyck3jz
because: The chunk met a count-worded test name pinned as a requirement's witness and kept it, a sweep convention the row states only for renames.
@row_add n=1 row=contract/cold-agent-verdict/caused-by-kind area=3-test-harness-contract fact=F1 detector=none
boundary: cold-agent verdict
item: caused_by_kind
promise: An object of the three closed words panic, error and log to the count of accepted acting answers whose caused list holds an item of that kind, carried on the verdict and its run.end event; a non-JSON answer is skipped and nothing of an item but its kind is read.
basis: fct-vkogrj6o
because: The cold-agent verdict gained the caused_by_kind field, a new item of its outward shape that no row states.
@row_add n=2 row=contract/stand-boot/boot-order area=3-test-harness-contract fact=F2 detector=none
boundary: seven_guis headless stand
item: stand::boot_order
promise: stand::boot_order(HarnessOptions) boots the order page offline in TaskShell with the options given, the seventh served task's stand boot.
basis: fct-wnmdhla7
because: The stand gained a new public boot beside its other boots, each of which stands as a row of this boundary.
@row_add n=3 row=decision/caused-owner-pinned-as-graph-reads area=integration-test-strategy fact=F3 detector=none
name: Caused owner pinned as the UI graph reads it
rule: A caused item carries the owner the UI graph reads and no other, so a page keyed by HTML ids under no keyed element reads no owner on its caused items, and a check pins the graph's reading rather than an asserted owner.
why: A plan stating an owner for such a page states nothing to pin.
guard: stand_order_served
basis: fct-sedefl7e
because: The plan's owner clause was disproved by measurement and the check pins the graph's reading, a rule future checks of caused must follow.
@row_add n=4 row=decision/order-page-meets-every-caused-kind area=3-test-harness-contract fact=F5 detector=none
name: Served order page meets every caused kind
rule: The served order page is where a driver and a cold agent meet each caused kind: Submit with a quantity not a whole number from 10 to 20 answers one error item under its own boundary, a good Submit one info log item, and Export one panic item with its file and line.
guard: stand_order_served
basis: fct-wnmdhla7
because: The cold-agent pipe now requires each caused kind met, and the order page is the stand's one place that provides all three.
