@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=18 class=omission
leaf-text: pages and held network stand-in `HeldNet` in `tests/blitz-tests/tests/fixtures/mod.rs`
source: dec-c7viqc7i «A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, with blitz-net and tokio taken as blitz-tests dev-dependencies for it.»
replace-with: pages and held network stand-in `HeldNet` in `tests/blitz-tests/tests/fixtures/mod.rs` (a check may also read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, blitz-net and tokio blitz-tests dev-dependencies for it)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: a bad `href` starting no load)
source: dec-bbvpdkrm «A `file:` URL is loaded only for a `file:` document: a script's `fetch()`, module `import` or `<script src>` is refused with a `FetchError` otherwise, and an engine sub-resource load (stylesheet, `@import`, font, image»
replace-with: a bad `href` starting no load; and `local_files_for_local_documents`, an engine sub-resource load — stylesheet, `@import`, font, image, iframe — through blitz-net reading no local file for a non-`file:` document)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=46 class=omission
leaf-text: must pass — fmt, clippy and the workspace rustdoc gate green
source: dec-mfqto2t4 «blitz-dom builds a network request only through its one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect, so the clippy leg fails on any other construction.»
replace-with: must pass — fmt, clippy and the workspace rustdoc gate green; blitz-dom's crate-level clippy disallowed-methods config and one expect failing the clippy leg on any network request built other than through its one stamping constructor
