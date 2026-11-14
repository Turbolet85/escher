@history_item n=1 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script tests the take added) Script ↔ DOM gained the `file:` load rule for scripts: `fetch()`, a module `import` or `<script src>` load a `file:` URL only for a `file:` document (`document_may_load`), refused with a `FetchError` otherwise, guarded by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`, read on the macOS and Windows CI legs beside linux; `tests/dom.rs` 27 → 30. This is the origin of decision/file-urls-only-for-file-documents, which this chunk extends from script loads to the engine's own sub-resource fetches. The row's rule and guards have to be restated to cover both, as the scope's wrap clause says.

@history_item n=2 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script tests' shared page and files on disk) Added the "Files on disk (script tests)" row: `temp_dir_with_files` writes sibling files under `std::env::temp_dir()` at test time for the two `file:`-load tests, a test-only `TMPDIR` read (arch §Occupied Resources), with the test-only sites going 1 → 2. This is the precedent for this chunk's witness, which needs an existing local file. Reuse the pattern or record the new on-disk seed and TMPDIR site, because decision/test-data-in-code states test data in code with only one named exception.

@history_item n=3 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (the test inventory gains upstream's tests) Added to blitz-net's "no test" row that its `file:` read through `Url::to_file_path` has no test of its own. This chunk's blitz-net unit test (a `file:` request from an `https:` loader errors, one from a `file:` loader or with no loader delivers) closes that recorded absence, so the absence wording has to be retired.

@history_item n=4 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (blitz-net's unit tests and test-only loopback binds) blitz-net's `#[cfg(test)] mod tests` went from "no test" to 3: `delivers_successful_and_empty_responses`, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`. The last two each use a test-only `TcpListener::bind` on `127.0.0.1:0`. §8 gained the test-local `Handler` implementing `NetHandler` and the `fetch` helper returning `Result<Bytes, String>`. Kept: the `file:` read still has no test. This chunk edits `delivers_network_and_bad_url_errors` and the `file:` arm, so the 3-test count and the §1, §4, §7 and §8 records move. The scope's witness asks for offline with no listener, unlike the existing loopback binds.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed load ends the wait) The harness's crate-private load counter (`CountedHandler`) now forwards `NetHandler::error`, so a failed stylesheet reaches the document, stops counting as a pending render-blocking resource and ends the harness and driver settle wait. This is the origin of decision/failed-load-ends-settle-wait and contract/harness-settle/failed-load, with `settle.rs`'s unit test as witness. Trap recorded: `NetHandler::error`'s do-nothing default compiled silently through a wrapper, and no gate caught it, so a later change that extends a net trait must sweep every implementer. This chunk's refusal answers through `NetHandler::error` and may add a field to `Request`, so both points apply directly.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`render_blocking_stylesheet` on a stylesheet that stops blocking) Added `failed_stylesheet_unblocks_rendering`, `empty_or_non_css_stylesheet_unblocks_rendering` and `invalid_stylesheet_url_does_not_block_rendering`. A refused `file:` stylesheet on a non-`file:` document goes down the same failed-load path, so this chunk's "no style" witness relies on it to leave the document unblocked.

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — Defined `settle`: quiet = poll false, hover unchanged, no load finished. A load in flight is `NotSettled { busy: Loads }` at once, and `SETTLE_PASS_LIMIT` is 64. §7 records that a fetching `net_provider` is wrapped in the load counter, while a no-op or absent one carries none. This is the origin of the harness-settle contract rows (settle, busy-loads, side-effects, settle-pass-limit). A check booting a harness on blitz-net's real `Provider` gets the counted wrapper, so a refused load must end as an error and not stay pending.

@history_item n=8 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — Added the driver's `LOAD_WAIT` (5 s wall, settle every 10 ms, no virtual time), `NotSettled(Loads)` at the bound and `stand_load_wait`. It also added the second named exception to "no sleep and no wall-clock read in checks". This is the origin of decision/load-wait-bound, contract/driver-session/load-wait and the no-sleep rows' exceptions. A new check over real `file:` loads must take no sleep or wall-clock read outside those named exceptions.

@history_item n=9 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — Created blitz-tests' `tests/fixtures/mod.rs` with `HeldNet` and recorded that the module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`. It also measured the test-data bootstrap (no `run fixtures` selection), which is the origin of contract/agent-run-selections/all. This chunk's witness drives blitz-net's real `Provider` with files on disk, which the shared fixtures module states it never holds. Keep that in the check's own file, or amend the stated property.

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the served pages' boots and checks, `LoadNet`) The stand's load page answers through the in-process `LoadNet`, with no `blitz_net`, print, env read, socket or listener, and `net_provider: None` for every other stand boot. This is the fact behind the scope's boundary that the stand and the driver do not use blitz-net, so no driver or stand contract row moves.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (`tests/all.rs` declared and never built, pinned by `BlitzTestsTargetsTest`) The fork keeps one auto-discovered blitz-tests target per file, with upstream's `all` held at `test = false`, by the operator's decision of 2026-10-10. This is the origin of decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file and decision/one-test-target-per-behaviour. A new witness file is its own target with no manifest line, and only its `mod` line goes into `all.rs` if the upstream pattern is followed.

@history_item n=12 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — Changed decision/layout-tests-in-both-layout-modes: its `except` now keeps upstream integration targets taken in a sync as written, with the one-layout-mode ones standing as named exceptions. A new fork-written check is under the rule, not the exception, so this chunk's witness must loop `for incremental in [false, true]` wherever a document is laid out (as the scope also asks).

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`pseudo_element_flow_classification` runs both layout modes) Ended the earlier both-modes exception (2026-10-27's record) by making all three tests run the incremental loop, with `incremental_oracle` staying the layout-identity witness. Precedent: the fork closes both-modes gaps in its own checks and does not record new exceptions for them.

@history_item n=14 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the target, its legs and its pages) Precedent for recording a new non-`stand_` blitz-tests target: §9 records the legs that reach it (the workspace `test` leg, `run all` and `run {name}`, with no script line naming it, none of `stand-repeat`'s named words and none of the `a11y` leg's fourteen), §7 records its own pages and §1 records its tests. The same entry moved `run all` to 743 · 0 · 12, the newest `run all` reading this chunk's new target would move.

@history_item n=15 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the blitz-tests shared modules and targets) This is the latest count: blitz-tests holds 161 `.rs` files, 160 test targets and the unbuilt `all.rs`, with `mod common;` read by 24 checks and `mod session_common;` by 35. A new witness target moves these counts in test-strategy's Directory pattern.

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the legs, the baseline re-count and the coverage floors) This is the latest local workspace baseline: 223 result lines and 0 failed, with passed and ignored not read. Per-OS tallies were last re-read at 2026-11-07-mounteddata-buffered-render. These are the registers a new target and the edited blitz-net tests move at this chunk's wrap.

@history_item n=17 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the per-OS CI tallies re-read) The per-OS tallies were re-read on CI#42371051720 (linux 220 · 1310 · 0 · 12). Rule set here: in a GitHub Actions job log, `Running …` and `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name after stripping ANSI codes.

@history_item n=18 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (engine features by runner) Recorded that `cargo test --workspace` and `cargo test -p blitz-tests` build blitz-dom with different features (2026-10-10 found a second feature split), so a green per-package run is not the workspace leg's green. A witness touching blitz-dom's `stamped_request` and blitz-net should be read under both builds.

@history_item n=19 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — WPT and its PR-results posting run only on upstream `DioxusLabs/blitz` (repository guard), and the fork runs WPT on the host. This is the origin of decision/wpt-run-on-host-on-fork. The scope's boundary that the WPT runner keeps reading its local `file:` fixtures green can only be witnessed by a host run on the fork.

@history_item n=20 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (the first conformance reading) The first WPT conformance reading on the fork's dev host was `css/cssom-view` at the pinned WPT commit, report-only. This is the origin of decision/fork-wpt-reading-report-only and decision/wpt-conformance-report-only. A host WPT reading taken to show that the runner's `file:` fixtures still load grades no criterion.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (line citations re-pointed after the take) Re-pointed citations into blitz-dom's `net.rs`, blitz-traits' `net.rs` and blitz-test-harness's `settle.rs`, among others. This chunk edits `packages/blitz-traits/src/net.rs`, `packages/blitz-dom/src/net.rs` and `packages/blitz-net/src/lib.rs`, so the citations into them need the same re-point sweep at wrap.

@history_item n=22 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — The test plan's text became rows (601 rows, each verified against the code at edd0cfaea7f0), and every picked row comes from this pass. This chunk amends rows by handle (`~ decision/file-urls-only-for-file-documents`, plus any new blitz-net row), not prose sections.
