@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=27 class=omission
leaf-text: a radio's `change` among them: a first click hearing `input,change` and a second adding `input` alone)
source: dec-bbvpdkrm «A `file:` URL is loaded only for a `file:` document: otherwise a script's `fetch()`, `import` or `<script src>` is refused with a `FetchError`, a blitz-net sub-resource load reads no local file»
replace-with: a radio's `change` among them: a first click hearing `input,change` and a second adding `input` alone; `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` among them too: a `file:` URL loaded only for a `file:` document, a script's `fetch()`, `import` or `<script src>` otherwise refused with a `FetchError`, a blitz-net sub-resource load reading no local file and user loads not gated)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=18 class=omission
leaf-text: pages and held network stand-in `HeldNet` in `tests/blitz-tests/tests/fixtures/mod.rs`
source: dec-c7viqc7i «A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns»
replace-with: pages and held network stand-in `HeldNet` in `tests/blitz-tests/tests/fixtures/mod.rs` (a check may also read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, blitz-net and tokio blitz-tests dev-dependencies for it — `a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all`)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=46 class=omission
leaf-text: must pass — fmt, clippy and the workspace rustdoc gate green
source: dec-mfqto2t4 «blitz-dom builds a network request only through its one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect»
replace-with: must pass — fmt, clippy and the workspace rustdoc gate green; blitz-dom builds a network request only through its one stamping constructor, a crate-level clippy disallowed-methods config and one expect failing the clippy leg on any other construction
