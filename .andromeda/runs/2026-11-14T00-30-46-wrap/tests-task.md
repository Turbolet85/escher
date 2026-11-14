@task_prose part=session
blitz-net's file: branch now serves a file: URL only to a request with no loading document or with a file: loading document; any other is refused with the fixed-text error FileForbidden, before any disk read.
Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) now carries its document's URL as the request's initiator; a navigation and an embedder's top-level load carry none.
The file: predicate moved to one body in blitz-traits, re-exported where vibey-script held it; blitz-dom builds a request only through one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect.
A new blitz-tests target local_files_for_local_documents checks, in both layout modes, that every engine request carries its document and that a remote page reads no local file through blitz-net's real provider while a file: page reads all four; unit tests in blitz-traits and blitz-net cover the predicate table (6 rows, count asserted) and the refusal offline.
blitz-tests gains blitz-net and tokio as dev-dependencies, and a check now encodes a PNG through the png dev-dependency; blitz-tests top-level files measure 171 to 172, targets 170 to 171; three controls held red-before-green; CI green 17 of 17.
@task_file n=1 path=packages/blitz-dom/clippy.toml
what: blitz-dom's crate-level clippy config, read by the workspace clippy leg and the blitz-dom all-targets clippy run; it disallows building a request outside the stamping constructor
@task_file n=2 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the new blitz-tests integration target, one behaviour: file: URLs loadable only from a file: document across engine sub-resource loads, with two hand-written network stand-ins and fixture files written under CARGO_TARGET_TMPDIR
@task_file n=3 path=packages/blitz-traits/src/net.rs
what: the shared network request type and the file: URL predicate (document_may_load), with its inline unit test module
@task_file n=4 path=packages/blitz-traits/src/navigation.rs
what: navigation options turned into a top-level request, which carries no loading document
@task_file n=5 path=packages/blitz-vibey-script/src/fetch.rs
what: vibey-script's script fetch, which re-exports the file: URL predicate for script loads
@task_file n=6 path=packages/blitz-net/src/lib.rs
what: blitz-net's network provider and its typed provider errors, the file: read branch, and its inline unit tests
@task_file n=7 path=packages/blitz-dom/src/net.rs
what: blitz-dom's sub-resource loading: stylesheet, @import and @font-face handlers, and the one request constructor that stamps the loading document
@task_file n=8 path=packages/blitz-dom/src/document.rs
what: the base document: its request builder, linked stylesheet and image loads, and stylesheet loader construction
@task_file n=9 path=packages/blitz-dom/src/iframe.rs
what: iframe sub-document loading from its parent document
@task_file n=10 path=packages/blitz-dom/src/mutator.rs
what: the DOM mutator, which constructs a stylesheet handler for a linked sheet
@task_file n=11 path=packages/blitz-dom/src/cssom.rs
what: the CSSOM, which builds a stylesheet loader and inserts @font-face rules that fetch fonts
@task_file n=12 path=packages/blitz-dom/src/layout/damage.rs
what: style damage handling, which fetches CSS background image layers
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests manifest and its dev-dependencies, the manifest's difference from upstream
@task_file n=14 path=Cargo.lock
what: the workspace lockfile the locked cargo legs read
@task_item n=1 part=touches
The decision that file: URLs are loadable only from a file: document: its rule states script loads alone, and now reaches every engine sub-resource load through blitz-net; its guard cell gains local_files_for_local_documents and blitz-net's offline unit test.
@task_item n=2 part=touches
The decision on PNG decoding in checks through the png dev-dependency: a blitz-tests check now also encodes a PNG through it (a fixture image).
@task_item n=3 part=touches
The decision on the blitz-tests manifest's difference from upstream: it gains two dev-dependencies, blitz-net and tokio with the rt feature.
@task_item n=4 part=touches
The rules of one test target per file and one integration target per behaviour, and the rule that the blitz-tests manifest holds one test target: a new auto-discovered target joins; a row holding a blitz-tests file or target count moves 171 to 172 files, 170 to 171 targets.
@task_item n=5 part=touches
The clippy legs over the workspace and blitz-dom: blitz-dom now carries a crate-level clippy config whose disallowed-methods entry fails the leg on any request built outside the one expect.
@task_item n=6 part=touches
Hand-written stand-ins, no mocking library: two new hand-written network stand-ins, one recording requests and leaving them in flight, one reading through blitz-net's real provider on a current-thread runtime it owns.
@task_item n=7 part=touches
Network stand-ins keep their loads counted, and the harness counts loads of fetching providers only: the new stand-ins answer inside fetch on the test's thread or leave requests in flight.
@task_item n=8 part=touches
Layout assertions in both layout modes: both new integration checks loop over the two layout modes.
@task_item n=9 part=touches
Table-driven tests assert their row count: the predicate table holds 6 rows with its count asserted.
@task_item n=10 part=touches
Unit tests in inline test modules in their source file: new inline test modules in blitz-traits' network module and in blitz-net.
@task_item n=11 part=touches
Tests controlled red-before-green: three controls held (refusal removed, stamp removed, expect removed), each removed firing showing its case run and failed.
@task_item n=12 part=touches
Test data stated in code, nothing seeded: the new check writes its fixture files (a stylesheet, an import, an image, an iframe page) from code into a test-only directory under CARGO_TARGET_TMPDIR read by build-time env!.
@task_item n=13 part=touches
A failed load ends the settle wait, and the harness settle's failed-load item: a refused file: sub-resource now reaches the handler's error path as a failed load.
@task_item n=14 part=touches
A file joining a leg keeps its legs, and the agent-run selections run all and run {file}: the new target is in workspace test, run all and run local_files_for_local_documents, and not in the stand-repeat list.
@task_item n=15 part=touches
Cargo legs run locked and the dependency audit: the lockfile moves in blitz-tests' own dependency list only, no new package enters, audit passed.
@task_item n=16 part=touches
Regression tests document and are named for their bug: the new target is named for the behaviour that closes a remote page reading local files.
@task_fact n=1 slug=integration-test-strategy
fact: Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL, checked in both layout modes; a navigation or embedder top-level load carries none.
@task_fact n=2 slug=ci-integration
fact: blitz-dom builds a request only through its one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect; the clippy leg fails on any other, and upstream's blitz-dom has no such config, a new merge surface.
@task_fact n=3 slug=test-data-fixtures
fact: The test-only directory local_files_for_local_documents under CARGO_TARGET_TMPDIR is occupied by the blitz-tests target of that name.
@task_fact n=4 slug=mocking-stubbing-discipline
fact: A blitz-tests check may read file: loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime it owns; blitz-net and tokio are blitz-tests dev-dependencies for it.
@task_item n=1 part=untouched
The session, driver, MCP and escher-session command-line contracts, exit codes, wire and recording rows: no session or driver surface moved.
@task_item n=2 part=untouched
Agent-run and cold-agent events, verbs, exit codes and verdict rows: no harness script or status shape changed.
@task_item n=3 part=untouched
WPT runner, conformance and reftest rows: no WPT surface moved.
@task_item n=4 part=untouched
Coverage floor values, mutation testing and complexity ceiling rows: no floor, threshold or budget moved.
@task_item n=5 part=untouched
Stand checks, stand repeat rounds and the zero-failure budget: the new target joins no stand list.
@task_item n=6 part=untouched
Handler coverage, lint defects, contrast and snapshot rows, and the harness construction, input and screenshot contracts: the harness and shared test modules stand unchanged.
@task_item n=7 part=untouched
CI artifacts, apt-install, action pinning, secrets, permissions and cache rows: no CI step, script or workflow changed.
