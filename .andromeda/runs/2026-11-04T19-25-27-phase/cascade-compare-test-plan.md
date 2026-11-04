@compare side=phase doc=test-plan marker=2026-11-04-dioxus-runtime-probe e_items=81 held=21 lost_classify=8 lost_strong=3 beside=- cascade_only=3 listed=3 body=41 unplaced=0 held_ids=ci-integration#1.1,ci-integration#13.1,ci-integration#14.1,ci-integration#15.1,ci-integration#16.1,e2e-test-strategy#2.1,integration-test-strategy#1.1,mocking-stubbing-discipline#1.16,mocking-stubbing-discipline#1.9,quality-gates-coverage-targets#2.1,quality-gates-coverage-targets#3.1,test-data-fixtures#3.27,test-scope-summary#23.1,test-scope-summary#23.3,test-scope-summary#23.4,test-scope-summary#25.1,test-scope-summary#41.1,test-scope-summary#41.3,test-strategy#1.1,test-strategy#9.1,unit-test-strategy#1.1 only_ids=test-scope-summary#10.1,test-scope-summary#37.1,unit-test-strategy#4.2
@compare_item n=1 part=lost how=strong entity=ci-integration#4.1 label=test-plan/ci-integration fact=fct-nrwelml5
@carried n=4 label=test-plan/ci-integration fact=fct-nrwelml5
why: this is the precedent for a probe check with no `stand_` prefix over the key-handler path. It reaches CI through the workspace `test` leg and `run all` with no script change, and its readings must cover both layout modes.
quote: the blitz-tests target `key_press_after_container_lost_its_box` runs in the workspace `test` leg and in `agent-run.sh`'s `run all`, each taking it as one more target with no line of its script naming it; its 3 tests run in both layout modes; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it
@compare_item n=2 part=lost how=classify entity=ci-integration#8.1 label=test-plan/ci-integration fact=fct-pmze6kz2
@carried n=9 label=test-plan/ci-integration fact=fct-pmze6kz2
why: a probe check that reproduces the MountedData panic must not be left as a red target in a leg. One red target hides every later target in that job's run.
quote: `cargo` stops at the first failing target, so a red job's later targets go unread on that run
@compare_item n=3 part=lost how=classify entity=e2e-test-strategy#4.2 label=test-plan/e2e-test-strategy fact=fct-h2aiavph
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-h2aiavph
why: the probe must run headlessly on the stand, so no check it adds launches a window; if a reading needs the windowed path, this absence line must be restated.
quote: still no test launches a window
@compare_item n=4 part=lost how=classify entity=e2e-test-strategy#1.1 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
why: the probe adds no verb, argument, node key, result field, cause, session error or exit, so the command-line e2e surface must still read exactly as stated, with its six served tasks and its closed set of causes unchanged.
quote: each of the six served tasks — the four lean tasks, `cells` and `load` — starts, answers and stops
@carried n=3 run=2 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
quote: every cause the binary provokes (`unknown-verb`, `malformed`, `disabled`, `not-found`, `off-screen`, `stale`) reads `remedy` equal to its cause's
@compare_item n=5 part=lost how=classify entity=e2e-test-strategy#1.3 label=test-plan/e2e-test-strategy fact=fct-gtzxj2kr
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-gtzxj2kr
why: since no verb crosses, the MCP role's listing must still offer exactly the twelve verbs in order, so `mcp_server` passes unchanged.
quote: `tools/list` offering the verbs in order — twelve —
@compare_item n=6 part=lost how=strong entity=integration-test-strategy#5.1 label=test-plan/integration-test-strategy fact=fct-t7rg232i
@carried n=8 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: If Cells is the probe's served task, a click after a `scroll` must come after an `advance` past the overlay scrollbar's fade, or the click dispatches nothing and the handler reading is false.
quote: a click on a Cells cell inside a scrolled box right after `scroll` lands on the shown overlay scrollbar, which takes it as a thumb drag and dispatches nothing, and a pointer left on the thumb keeps the scrollbar shown, so an `advance` past the fade comes before the click
@compare_item n=7 part=lost how=classify entity=mocking-stubbing-discipline#1.7 label=test-plan/mocking-stubbing-discipline fact=fct-c3fx7sq3
@carried n=4 label=test-plan/mocking-stubbing-discipline fact=fct-c3fx7sq3
why: a probe check in `tests/blitz-tests/` that needs a load held must reuse the shared `HeldNet` through `mod fixtures;`, never a fourth hand-written network stand-in.
quote: `HeldNet`, escher's one held network stand-in since 2026-10-19-shared-fixtures, in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider` and `stand_boot`'s `RecordingNetProvider`: `fetch` keeps each request's url and handler, `held()` and `urls()` (request order) read them, and `answer(body)` answers the last held request through `NetHandler::bytes` and returns its url
@compare_item n=8 part=lost how=strong entity=test-data-fixtures#3.6 label=test-plan/test-data-fixtures fact=fct-iee65fcu
@carried n=1 label=test-plan/test-data-fixtures fact=fct-iee65fcu
why: when the probe's served task beside the lean one is Cells, it must boot that screen through the stand's own boot and read it against this stated value, never a page of its own.
quote: the stand's own Cells screen, booted through `stand::boot_cells`: 47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary
@compare_item n=9 part=lost how=classify entity=test-data-fixtures#3.1 label=test-plan/test-data-fixtures fact=fct-hutsqqej
@carried n=4 label=test-plan/test-data-fixtures fact=fct-hutsqqej
why: any page the probe builds for itself (for example a key-handler page to re-verify the MountedData mechanism) must be test data stated in code, following the category's inline seed strategy.
quote: | Inline HTML | inline HTML string constants or `format!`-built pages |
@compare_item n=10 part=lost how=classify entity=test-scope-summary#2.1 label=test-plan/test-scope-summary fact=fct-k5y3m6cu
@carried n=7 label=test-plan/test-scope-summary fact=fct-k5y3m6cu
why: A probe check kept as a seven_guis integration target moves this count and must read the shared `tests/common/mod.rs`.
quote: `examples/seven_guis` holds eight integration-test targets (seven until 2026-10-19-session-recording added `tests/recording.rs`)
@compare_item n=11 part=lost how=classify entity=test-scope-summary#24.1 label=test-plan/test-scope-summary fact=fct-rjsmqegm
@carried n=8 label=test-plan/test-scope-summary fact=fct-rjsmqegm
why: The windowed shell's `poll` path has no test, so the reading must mark the windowed MountedData path as read and not run.
quote: **dioxus-native and stylo_taffy:** no tests — observed absent
@compare_item n=1 part=listed how=strong entity=test-data-fixtures#3.19 label=test-plan/test-data-fixtures fact=fct-afgwdd3i
@carried n=3 label=test-plan/test-data-fixtures fact=fct-afgwdd3i
why: the probe's headless stand runs must use the bundled font payload, so the readings measure the same on any runner.
quote: the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic
@compare_item n=2 part=listed how=strong entity=test-strategy#7.1 label=test-plan/test-strategy fact=fct-2vwamopj
@carried n=4 label=test-plan/test-strategy fact=fct-2vwamopj
why: a probe check that boots the stand (Counter or CRUD) must assert with no skip path, under the stand's bundled font with system fonts off.
quote: the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx` (examples/seven_guis/src/stand.rs:59-61; tests/blitz-tests/tests/stand_boot.rs:51-65)
@compare_item n=3 part=listed how=classify entity=unit-test-strategy#9.1 label=test-plan/unit-test-strategy fact=fct-l2ouwaqy
@carried n=4 label=test-plan/unit-test-strategy fact=fct-l2ouwaqy
why: a probe check placed in `tests/blitz-tests` must be a new file under `tests/` that auto-discovery picks up, with no `[[test]]` table added and `autotests` not turned off, so that `BlitzTestsTargetsTest` stays green.
quote: `BlitzTestsTargetsTest`, one test, reads `tests/blitz-tests/Cargo.toml` with the stdlib `tomllib` and asserts that `[package]` does not set `autotests` to false and that the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`: the standing check on the fork's two-line difference from upstream's manifest
