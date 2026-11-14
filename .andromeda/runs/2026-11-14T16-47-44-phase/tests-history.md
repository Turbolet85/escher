@history_item n=1 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (amd-bn47lomj, `~ decision/file-urls-only-for-file-documents` rule) widened the rule from script loads alone to every engine sub-resource load through blitz-net: a non-`file:` document reads no local file. This chunk closes the two app-side paths, the favicon probe and navigation, that this widening left open. The rule's wording names engine and script loads only, so the chunk's two checks either extend this row or stand beside it.

@history_item n=2 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (amd-gso2bo7y, `~ decision/file-urls-only-for-file-documents` guard) added `local_files_for_local_documents` to the row's guard list beside the two script-load checks. It sets the precedent: a check that holds a new half of the `file:` rule joins the row's guard. The favicon and navigation checks are candidates for the same list.

@history_item n=3 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (amd-qi7vgvwx, `+ decision/real-provider-inside-stand-in`) the previous chunk's witness put blitz-net's real provider behind a hand-written stand-in on its own runtime. This chunk's scope departs from that on purpose: no `StdNetProvider`, temp file or `TMPDIR` read. The favicon gate is unit-tested on the pure `favicon_candidate`, and the navigation gate is reached through `navigate_to` on an in-VirtualDom `History`. The plan should state that departure.

@history_item n=4 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — (amd-lde2vbr3, `+ decision/blitz-dom-requests-through-stamping-constructor`) a crate-level clippy config makes blitz-dom build every request through one stamping constructor, which sets the initiator. `apps/browser` has no such guard: its `Request::get` without an initiator is the gap the favicon path exposes. The plan should not assume that clippy guard reaches the app.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (amd-6q2sqo5s) Script ↔ DOM gained the `file:` URL narrowing (`document_may_load`). Its tests are `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`, and the `file:` URLs are read on the macOS and Windows CI legs beside linux's. These are the origin of the row's guards. A test that reads files would need to hold on all three OSes; this chunk's checks avoid disk entirely.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (amd-cp6hvbjs) the two `file:`-load script tests write sibling files under `std::env::temp_dir()` through a test-only `TMPDIR` read (`temp_dir_with_files`), counted as an occupied resource. This is the precedent this chunk's scope steers away from; adding a disk-touching check would add a further test-only `TMPDIR` site.

@history_item n=7 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (amd-rf4xxwvm) recorded `apps/browser/src/tab.rs`'s test module, with one `#[test]`, `tab_lens_survives_webview_props_replacement`. It is the browser's sixth tested file, beside about_pages, browser_history, favicon, url_suggestions and persistence, and runs in the workspace `test` leg. This chunk extends `favicon.rs`'s existing test module and adds a new test module to `history.rs`, built on tab.rs's in-VirtualDom pattern. The §1 and §4 apps/browser inventories (the list of tested files and their counts) move with it.

@history_item n=8 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (amd-simkhsnt) the workspace tally's +8 passed was attributed by name, `apps/browser/src/tab.rs` 1 among them. This is the precedent that browser unit tests land in an existing result line of the workspace `test` leg, not in a new one, so the chunk's re-count moves passed and not result lines.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (amd-u6pbkyf3) recorded that blitz-net's `file:` read through `Url::to_file_path` has no test named for it. The navigation and favicon gates refuse before that read, so they still do not test it, and the absence stands.

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (amd-6o3k2s3y) blitz-net gained 3 unit tests, two of them binding a loopback `TcpListener`. It kept the record that the `file:` read has no test named for it. The chunk reads `blitz_traits::net::document_may_load` as it stands and adds no blitz-net test.

@history_item n=11 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (amd-knbkpfwu, `~ decision/mutation-controlled-tests` rule) the controlled-tests rule now adds that a control removing a bound loosens it to twice the budget. The scope requires every new check to carry a control (inputs#I1). For example, a mutation that drops the `document_may_load` filter in `favicon_candidate`, or the early return in `navigate_to`, should turn the new tests red, and the restored tree should read green.

@history_item n=12 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — (amd-le75xddb) set the mutation tier's recipe: cargo-mutants with `--test-tool nextest` under the `mutants` profile and `--baseline=skip`, each unit's test command read green first, on the dev host only and never in CI. This is the origin of the picked rows `decision/mutation-baseline-read-per-unit`, `decision/mutation-testing-dev-host-only`, `decision/mutation-baseline-skipped`, `decision/mutation-runs-on-dev-host-only` and `decision/mutation-runs-under-nextest-profile`. Any mutation control run on the browser app follows this recipe.

@history_item n=13 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — (amd-lfmouigx, `~ decision/inline-cfg-test-modules` except) dropped the stated count of out-of-line `#[path]` test modules from the exception. The new `history.rs` test module is planned inline at the file's end, inside this row's rule, so no exception is needed.

@history_item n=14 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — (amd-ur7tfp4h) unit tests sit inline, with `bridge_tests` the one out-of-line `#[path]` module, a child of the Dioxus document because the event handler's fields are private to it. This is the origin of the `decision/unit-tests-in-source-file` exception. It also records a trap: a sed or grep address ending at a function's name also selects items whose names start with it, and a control built on the same address cannot catch that. This matters for controls on `favicon_candidate` and `navigate_to`.

@history_item n=15 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — (amd-dvdqody5) moved test modules out of line with `#[cfg(test)]` and `#[path]` at their parent, test paths unchanged. This is the precedent the `decision/unit-tests-in-source-file` row states for a module moved out of line. It applies only if a browser test module grows out of its source file.

@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (amd-s53lzeiq) the same check read green per package and red in the workspace leg. This is the origin of `decision/per-package-green-not-workspace-green`. The chunk's checks run in the workspace `test` leg, so a green `cargo test -p` on the browser app is not that leg's green.

@history_item n=17 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (amd-cqdq6a7d) found that the workspace and per-package builds resolve different engine features (`writing-mode`, `tracing`). This sharpened the per-package-green rule: the two builds can compile different code under the same test.

@history_item n=18 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (amd-dhibncze) made the workspace tests the fast `test` leg (`cargo test --workspace --locked`), with the matrix testing windows and macos and only building ios and android. This is the origin of `decision/workspace-tests-in-fast-ci-leg` and `decision/native-platform-ci-matrix`. The browser checks run where the scope says: linux in the `test` leg, windows and macos in the matrix. With no disk read, all three OSes should read them alike.

@history_item n=19 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (amd-jy34nxjf) set the rule for re-reading per-OS tallies: in a GitHub Actions job log, the `Running …` lines and the `test result:` lines do not interleave in order, so a tally move is attributed by grepping the test's own name, beside the ANSI strip. This applies when the chunk re-counts its browser unit tests on CI.

@history_item n=20 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — (amd-472linzq) added `test_capability_statement.py` (`CapabilityStatementTest`). It pins the verb and cause names and the counts in `docs/capabilities.md`, plus the README's MCP tool count, and adds the claims-to-checks map in which only names and counts are pinned. This is the origin of the capability-statement rows. The chunk edits `docs/capabilities.md` only to drop the CARRY's two «provisional» clauses. That edit must leave the pinned names and counts untouched so the ci-scripts leg stays green.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (amd-hf3zl3jw) the capability statement's claim map gained three stated limits, each with its owner, under `decision/capability-claims-name-their-checks`. This is the precedent for how a limit's wording in `docs/capabilities.md` changes when its status moves. The chunk's dropped «provisional» clauses are such a move, and the scope decides that the browser's closed paths are not added as claims.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (amd-krjdo5z6) the `docs/capabilities.md` select limits went from two bullets to four, each read by named checks. This is a precedent for editing the capabilities doc by hand under the claims-to-checks map. The map's names and counts are a hand edit when a check is renamed or moved.

@history_item n=23 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (amd-3ieo66hz, `+ contract/session-recording/exploration-checks`) added the exploration-checks contract row, a picked row. Exploration follows no navigation (the founder's F2 A), and that is one of the `docs/capabilities.md` clauses at lines 346–347 whose «provisional» this chunk's CARRY drops. The row itself is unchanged by this chunk.
