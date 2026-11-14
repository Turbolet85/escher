@history_item n=1 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Established Decisions: 3 rows) — changed `decision/url-scheme-routing` so a `file:` URL is read from disk only for a top-level load or a `file:` loading document. It also added `decision/sub-resource-requests-carry-initiator`: a request with no initiator is a top-level load. This is the gap the chunk closes: the favicon probe's `Request::get` and `NavigationOptions::into_request` carry no initiator, so blitz-net reads them as top-level loads. That chunk's report left both app paths open (Insufficient fixes).

@history_item n=2 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Standard Contracts: 3 rows) — widened `decision/file-url-load-rule` to every engine request, top-level loads excepted. blitz-net refuses with FileForbidden before any disk read. It also registered `contract/blitz-traits-net/request-initiator` (the public `initiator` field and its builder, None for top-level) and `contract/stylesheet-handler/document-url`. The chunk uses this rule as it stands and does not widen or narrow it; gating the browser app is the only change.

@history_item n=3 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Conventions: 2 rows) — gave `decision/builder-method-naming` its exception: blitz-traits' Request builders are named by field (`signal`, `initiator`). It also added `decision/blitz-dom-stamped-requests`, so blitz-dom builds requests only through the stamping constructor. If the plan makes the favicon probe carry an initiator instead of gating `favicon_candidate`, it uses the `initiator` builder. The stamping guard covers blitz-dom, not `apps/browser`.

@history_item n=4 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Occupied Resources: 1 row) — registered `name/other/cargo-target-tmpdir` for a blitz-tests target's test-only fixture directory. This is the precedent for registering a test-only directory. This chunk's checks read no disk and no temp dir (the scope's premise correction), so it should register nothing new.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`document_may_load`: a `file:` script load only for a `file:` document) — introduced the public `document_may_load(document_url: Option<&Url>, url: &Url) -> bool` and `FetchError::Forbidden`. It was recorded as a narrowing, not a boundary widening. Both of this chunk's gates call this predicate: `favicon_candidate` answers none, and `navigate_to` returns early, wherever it refuses. The narrowing classification is the precedent for this chunk's refusals.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six upstream behaviour changes for every document, ratified for this take) — named the `file:` script-load refusal a narrowing, ratified under the engine-behaviour-change pattern. This precedent classes the app-side `file:` refusals as narrowings: no carrier and no founder widening question.

@history_item n=7 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` at a second test-only site, and its test directory) — registered `temp_dir_with_files` and the `file:` load tests `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`, under `TMPDIR`. This is the prior shape of a `file:`-refusal test. This chunk avoids that resource: its gates are pure predicates, unit-tested with no disk read.

@history_item n=8 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the browser's tab lenses across a webview props replacement) — recorded upstream's `tab_lens_survives_webview_props_replacement` in the upstream-only browser app. The scope models the new `history.rs` test on it: it builds a `History` store inside a `VirtualDom` (`apps/browser/src/tab.rs:301`). Since the browser is upstream-only, no escher contract row covers its favicon or navigation path.

@history_item n=9 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the taken files re-pointed) — stretched the browser's straight-line chrome-write block to `tab.rs:126-154` (`decision/chrome-writes-one-block`). The favicon candidate is chosen at `document_loader.rs:142`, upstream of that block. A gate added near a chrome write must not split the block.

@history_item n=10 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Conventions: 1 row) — reworded `decision/unit-tests-at-file-end`'s exception to "Unit-test modules in files of their own", with no count. The new navigation tests go in a test module at the end of `history.rs`, and the favicon tests in `favicon.rs`'s existing module. Either form satisfies the convention, and no count needs updating.

@history_item n=11 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** (a shared stand-check module, an out-of-line unit-test module) — set the convention that unit tests sit in `#[cfg(test)] mod tests` at the end of the source file. A file of its own is the exception (`decision/unit-test-placement`, `decision/unit-tests-at-file-end`). This governs where the two new browser test modules go.

@history_item n=12 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — added the `fixtures` shared module with `HeldNet`; a page with one reader stays in its check's file (`decision/fixtures-test-module`). This chunk's checks live in the browser crate's own unit tests, not blitz-tests, so no shared fixture is needed.

@history_item n=13 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (keyboard activation and a click's focus) — made Enter and Space dispatch a click on a link with href or a submit control. Enter and Space "now reach form submission, link navigation ... in every document, remote pages included" (`decision/keyboard-activation-enter`, `decision/keyboard-activation-space`). This is why keyboard-activated links and submits also reach `BrowserNavProvider::navigate_to` and must be refused there.

@history_item n=14 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Established Decisions: 4 rows) — added `decision/unresolvable-url-skipped`: URL resolution is fallible and every caller skips, never panics. `favicon_candidate(base_url, href)` resolves a candidate against its base, and the navigation gate reads a target, so both must keep the skip rule. The error page has no URL, which counts as no `file:` document.

@history_item n=15 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals serialised, mimalloc and the `multi_doc` env reads) — recorded that the browser's document loader and toolbar build their documents `Parallel` (`decision/windowed-documents-parallel`). Note the `PARALLEL_TRAVERSAL_LOCK`. `make_doc_config` in `document_loader.rs`, which this chunk edits to hand the document URL to `BrowserNavProvider`, is that loader's config site.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the bridge drops a removed node; the default sheet from the config) — `DioxusDocument::new` adds no sheet. The default UA sheet arrives once through `DocumentConfig::default()`, and a config that sets `ua_stylesheets` is used as given (`decision/ua-stylesheets-used-as-given`, `decision/dioxus-no-own-stylesheet`). Editing `make_doc_config` must leave the UA sheet behaviour untouched.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core: the document's clock) — added `clock` to `DocumentConfig`'s fields (`contract/document-core/documentconfig`). The navigation gate needs the document URL, which `make_doc_config` holds; `NavigationOptions.source_document` is only a `usize` id. The provider must take that URL at construction; DocumentConfig stays as listed.

@history_item n=18 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the test-only binds and no served port) — narrowed the register to "no served network port": only blitz-net's tests bind TCP on loopback (`absence/served-network-port`, `absence/network-listener`). The chunk's boundary adds no listener; its checks need no HTTP stand-in.

@history_item n=19 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (fork CI on the build branch through one leg script) — the matrix tests windows and macos, and linux is tested by the fast `test` leg (`decision/linux-tested-by-fast-leg`, `contract/ci-leg-legs/test`). The new browser unit tests run in the workspace `test` leg and must read the same on all three CI OSes. That works because they read no disk.

@history_item n=20 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — set up the host-run mutation tier: cargo-mutants with nextest under the `mutants` profile, each unit's test command read green first (`decision/mutation-tier-host-only`). The operator's directive requires every new check to carry a control, the mutation that turns it red. This tier is where those controls are read.

@history_item n=21 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host cognitive-complexity reading) — added the dev-host-only `rust-code-analysis-cli` reading against a ceiling of 15 (`decision/cognitive-complexity-ceiling`). The new gate in `navigate_to` and the filter in `favicon_candidate` must stay under it.

@history_item n=22 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Conventions: 1 row) — added `decision/std-result-under-dioxus-prelude` after a red Android CI run: a glob of `dioxus::prelude::*` shadows a two-parameter `Result`. The new `history.rs` test builds a `VirtualDom` and may glob the Dioxus prelude, so spell std `Result` there.

@history_item n=23 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Standard Contracts: 4 rows) — added `absence/fixture-state-on-carriers`, the founder's C. The CARRY folded into this chunk drops `docs/capabilities.md` line 344's "This is provisional, pending the founder's choice." for that absence. This row is what that edit restates.

@history_item n=24 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Standard Contracts: 9 rows) — added `absence/exploration-on-carriers` and `absence/exploration-follows-navigation` (the founder's A to F1 and F2). The CARRY drops `docs/capabilities.md` lines 346–347 ("Both are provisional, pending the founder's choice."). That clause removal is the file's only change.

@history_item n=25 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — created `docs/capabilities.md` as escher's own capability statement (`decision/capability-statement`), its counts held by the `CapabilityStatementTest` pin. The scope keeps the browser's closed paths out of it: the browser is upstream's, and the statement's "Loading pages" limit speaks of the engine. Editing the file for the CARRY must keep the pinned counts and README phrase intact.

@history_item n=26 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** (Standard Contracts: 1 row) — added `decision/no-seed-on-carriers`, provisional for the founder. Its discharge to standing, with `CLAUDE.md`'s seed clause and the entry's CONTEXT mark, waits for this chunk's wrap: the Epoch 4 boundary batch. That is a wrap batch, not a plan task.

@history_item n=27 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder (Epoch 2 close)) — the precedent for the epoch-boundary Provisional discharge. Every PROVISIONAL mark was reworded to "ratified by the founder (date)", each clause unchanged, in one batch at the wrap. This chunk's wrap is the Epoch 4 boundary and discharges the held marks the same way.
