@history_item n=1 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** (5. Integration Test Strategy, `~ decision/file-urls-only-for-file-documents`) — the rule gained the browser app's two gates: a `file:` favicon or link navigation from a non-`file:` document is dropped silently, user loads ungated. This is the hand-off (`make_doc_config` → `BrowserNavProvider::for_document`) this chunk proves; the new browser tests guard this row's browser half, which so far names no browser-app guard (guards are the script and blitz-net checks only), so the chunk may add its test name to the row's guard.

@history_item n=2 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (`~ decision/file-urls-only-for-file-documents` rule and guard; `+ decision/real-provider-inside-stand-in`) — widened the rule to blitz-net sub-resource loads, added `local_files_for_local_documents` as guard, and recorded the convention that a check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in on a current-thread tokio runtime it owns (blitz-net and tokio as dev-dependencies). Precedent for a browser test that asserts the config's `net_provider` reads no local file for a remote page while a `file:` page's does; dev-dependency additions there were scoped to blitz-tests.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script tests the take added; the script tests' files on disk) — origin of the `file:`-for-`file:`-documents rule (`document_may_load`, `remote_document_cannot_load_file_urls`, `file_document_can_load_sibling_file_urls`, read on macOS and Windows legs too) and of `temp_dir_with_files`, a test-only `TMPDIR` read through `std::env::temp_dir()` writing sibling files at test time. A browser test reading that a `file:` page's navigation loads needs a real file path and would be another test-only temp-dir site.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the browser's tab test module) — `apps/browser/src/tab.rs` gained upstream's test module (`tab_lens_survives_webview_props_replacement`), the browser's sixth tested file beside about_pages, browser_history, favicon, url_suggestions and persistence, run by the workspace `test` leg. The browser inventory this chunk extends (document_loader becomes a tested file) and the allow on `TabWebView` (`tab.rs:178`) arrived with this sync; §1/§4 browser lists will need the new file and count.

@history_item n=5 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — precedent for a corrective that adds unit tests to kill mutants a code audit left surviving (seven in dioxus-native-dom), and for the `bridge_tests` exception: a test module made a child of the module under test because the fields it needs are private with no constructor. This chunk does the same for `make_doc_config`'s three survivors; if its test needs private access it extends that exception's shape (`decision/unit-tests-in-source-file`).

@history_item n=6 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** (a textarea's value read on the fixture) — killed the Epoch 4 code audit's one survivor by a test placed inside the fixed test set the audit's mutation run uses for that unit, because a new file would sit outside the set the next audit runs. Trap for this chunk: the browser tests must sit where the cargo-mutants run on `document_loader.rs` (its unit's test command) reaches them.

@history_item n=7 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** (§10 Mutation tier) — the mutation recipe behind `decision/mutation-runs-under-nextest-profile`, `decision/mutation-baseline-skipped`/`mutation-baseline-read-per-unit` and `decision/mutation-runs-on-dev-host-only`: cargo-mutants `--test-tool nextest` under `.config/nextest.toml`'s `mutants` profile (`NEXTEST_PROFILE=mutants`, fail-fast, direct-test priority override), `--baseline=skip` with each unit's test command first read green by hand, dev host only. The kill read the scope names for `document_loader.rs` must follow it, or survivors read `timeout`; the priority override may need the new browser tests named.

@history_item n=8 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (`~ decision/mutation-controlled-tests`) — the controlled-test rule: each new check turns red under a named mutation and green on the restored tree; a control removing a bound loosens it to twice the budget. The three new browser tests each need their named mutation (config replaced by default, `net_provider` dropped, `navigation_provider` dropped).

@history_item n=9 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (`~ decision/inline-cfg-test-modules`) — the exception for `cfg(test)` modules held in their own files through a path attribute lost its count, so a new out-of-line test module no longer moves the row; the browser tests may sit inline in `document_loader.rs` or out of line without a row edit.

@history_item n=10 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — out-of-line `#[path]` test modules: test paths unchanged, and a `*_tests.rs` file is no coverage record, so moving tests out of line moves the crate's coverage reading. Trap if the browser tests are placed in a separate file.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — origin of `decision/per-package-green-not-workspace-green`: the same check read green per package and red in the workspace leg, the workspace build enabling different engine features (tracing, later writing-mode). The browser tests' green must be read in the workspace `test` leg, not by `cargo test -p` alone.

@history_item n=12 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — set the fast/slow split: the workspace tests are the fast `test` leg (`cargo test --workspace --locked`, ubuntu), and `ci-leg.sh fast` the local pre-push gate (`decision/workspace-tests-in-fast-ci-leg`, `decision/local-pre-push-gate`). Grounds the scope's inferred claim that browser app tests are CI-gated through the workspace leg.

@history_item n=13 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` reads fmt → clippy → test → ci-scripts → embolden, the row `decision/local-pre-push-gate` states; the chunk's gate is that full sequence, run only once the host is free.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`select_popup`'s `CountingShell`) — a hand-written `ShellProvider` stand-in counting redraws, the precedent for providing the `ShellProvider` context `make_doc_config` consumes (`consume_context::<Arc<dyn ShellProvider>>()`) under `decision/hand-written-stand-ins`, no mocking library.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the event-loop stand-in and the redraw route's check) — blitz-shell's test-local `CountedWakes` implementing `EventLoopProxyProvider`: another hand-written shell-side stand-in, beside the windowed path read and not run (`decision/ui-driven-through-session-binary`, no test launches a window).

@history_item n=16 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `HeldNet` replaced file-private network stand-ins and keeps no `is_noop` override so the harness counts its loads (`decision/network-stand-ins-counted`), and the test-data bootstrap was measured: nothing seeded, fixtures are code compiled by `boot` (`decision/test-data-is-code`). A net stand-in the browser test writes follows the same rule.

@history_item n=17 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blitz-net's unit tests and test-only loopback binds) — blitz-net gained its first unit tests with a test-local `NetHandler`; the `file:` read through `Url::to_file_path` still has no test named for it. The net provider the browser config carries is blitz-net's.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added the wall-time exceptions now in `decision/no-sleep-or-wall-clock-in-checks` / `decision/no-wall-time-in-session-checks` (`stand_load_wait`'s clock reads and stand-in sleep), allowed only where wall time is the check's subject. A browser test waiting on a navigation or load must not sleep or read the clock.

@history_item n=19 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (test-that named as the assertion crate) — the one exception to `decision/rust-tests-on-built-in-harness`: upstream's merged checks return `TestResult<()>` with `verify_that!`. New browser tests use plain `#[test]`/`assert!` (or `#[tokio::test]`).

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a named select's entries read by `form.rs`) — that chunk's added-lines census forbade `.unwrap()` and `.expect(` under `packages/`; the browser app is under `apps/`, where the test modules' `expect_used` allows this chunk justifies stand.

@history_item n=21 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (every area), each verified against the code; the picked rows this chunk reads and may amend (e.g. a guard added to `decision/file-urls-only-for-file-documents`) take their form from it.
