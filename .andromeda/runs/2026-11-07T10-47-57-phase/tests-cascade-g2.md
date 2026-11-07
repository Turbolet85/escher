@carried n=1 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the stand's element ids across a re-render, a remount and a fresh process, in both layout modes, must stay as recorded once both render sites write through the buffer, the stand checks the scope's witness.
quote: the seven_guis headless stand — each lean task mounted in TaskShell offline at the pinned viewport with the bundled font, fresh-boot and incremental `dom_string` identity, one driven interaction per task, timer ticks, one stable element id per element in both layout modes, the same id across a re-render, a remount (fresh `NodeId`s bar the four document-skeleton elements) and a fresh process
@carried n=1 run=2 label=test-plan fact=fct-aitehs62
quote: coverage spans the engine's behaviours, the seven_guis headless stand, the driver session over the stand and process telemetry
@carried n=1 run=3 label=test-plan fact=fct-jydhmppz
quote: **dioxus-native-dom:** unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id, the actionable-key check, the bridge, the mutation writer and the document's events
@carried n=1 run=4 label=test-plan fact=fct-7exig2pt
quote: `cargo test --workspace --locked` runs on ubuntu with default features as the fast `test` leg, `bash .github/scripts/ci-leg.sh test`
@carried n=1 run=5 label=test-plan fact=fct-7m2wyfwg
quote: the CI matrix runs `test --all --tests --locked` on windows and macos and `build --all --locked` on ios and android; linux is tested by the fast `test` leg, not the matrix
@carried n=1 run=6 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change
@carried n=2 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the buffered replay must keep the bridge's attribute writes, the falsy boolean attributes cleared on both attribute paths and in both layout modes, as the Dioxus integration checks read them.
quote: Dioxus integration (including the clearing of a falsy Dioxus `disabled`, tests/blitz-tests/tests/dioxus_falsy_disabled.rs:1, and of each of the 28 boolean attributes the check names
@carried n=3 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the buffered poll must return the answer the direct poll gave, so stand_settle's readings over a held instance do not move.
quote: and settle on a held instance (`stand_settle`, 8, each in both layout modes and calling no sleep, no clock and no pump or tick of its own): a Timer step run through `Session::act` returns reading `Elapsed: 0.3s`
@carried n=3 run=2 label=test-plan fact=fct-e36kow75
quote: `settle` repeats that pass and keeps poll's answer — a pass is quiet when poll answered `false`, the hover node did not change and no load finished in it
@carried n=4 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the queued mounted events must still flush after the buffer is applied, so a write made in a mounted handler stays present after the step.
quote: and on in-file fixtures a write made in a `mounted` handler is present after the step, a load in flight reads `NotSettled(Loads)` until the check answers it
@carried n=5 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the buffer must add no log line, so stand_act_spans' exactly-one driver line per call holds over the buffered render.
quote: in both layout modes each of 27 calls run through `Session::run` — the six instance-level verbs and a refused call for each of the eight causes, as that chunk measured — leaves exactly one `escher_driver` line at INFO, its `span` reading `command`, any other line of a call under an engine target;
@carried n=6 label=test-plan/test-scope-summary fact=fct-lwk4jfam
why: a stand_* witness the chunk re-runs over the buffered render, required to stay green in both layout modes.
quote: `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`, in both layout modes, the driver reaching a control in a scrolling box under a zoomed world where it shows
@carried n=7 label=test-plan/test-scope-summary fact=fct-57a7c2li
why: a stand_* witness run by run stand and the stand-repeat leg over the buffered poll, required to stay green in both layout modes.
quote: 12 tests, the sixteenth `stand_act_*` file, each in both layout modes, driving the select list through a held driver session
@carried n=8 label=test-plan/test-scope-summary fact=fct-h2tpz3mp
why: a stand_* witness the chunk re-runs over the buffered render, its readings required unchanged.
quote: an open list's rows read no overlap on the stand, in both layout modes
@carried n=9 label=test-plan/test-scope-summary fact=fct-bzkoiqdf
why: a stand_* witness over the same Tab and key path the inverted pins drive, required to stay green over the buffered poll.
quote: each test in both layout modes: Cells' Tab sequence, its arrows clamped at the grid's ends and bringing a cell into view, Enter opening the cell's editor and committing by keyboard
@carried n=10 label=test-plan/test-scope-summary fact=fct-5bb5lxig
why: a stand_* witness the chunk re-runs over the buffered render, its ratios required unchanged.
quote: `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`, each in both layout modes
@binding n=1
**test-scope-summary ↔ quality-gates-coverage-targets**: the stand checks carried here (n=1, n=6–n=10) are the set the stand-repeat leg runs three times with no retry over the new render path, a zero-failure budget (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-temy7g7i)
@binding n=2
**test-scope-summary ↔ the body §3 harness contract**: stand_settle's readings hold only if the buffered poll keeps the answer settle reads off each pass (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-e36kow75)
@binding n=3
**test-scope-summary ↔ integration-test-strategy**: every carried witness runs in both layout modes, and the inverted probe pins must too, the buffer leaving incremental and non-incremental layout identical (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-hzkxjxb5)
