@task_prose part=session
blitz-net now reads a file: URL only for a request with no loading document (a top-level load) or with a file: loading document; any other request is refused with a new typed error, FileForbidden, whose text is fixed and carries no URL or path.
A network Request now carries an initiator, the URL of the document it loads a sub-resource for; None means a top-level load an embedder asked for, and navigation requests set None.
blitz-dom stamps its document's URL on every sub-resource request it builds (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe: 7 paths, from 0), through one crate-private constructor, and a crate-level clippy disallowed-methods rule plus one expect make any other Request construction in blitz-dom a lint failure.
The document_may_load predicate now has its one body in blitz-traits' net module; blitz-vibey-script re-exports it unchanged, so its public name still resolves.
A new blitz-tests target proves, through blitz-net's real provider in both layout modes, that a remote page reads no local file and a file: page reads them all; blitz-tests gains blitz-net and tokio as dev-dependencies.
Two paths stay open and are not remedied here: the browser's favicon probe still reads a remote page's file: icon, and navigation from a remote page to a file: URL still loads it as a top-level document.
@task_file n=1 path=packages/blitz-traits/src/net.rs
what: the shared traits crate's network types: Request (non-exhaustive, now with an initiator field and builder), NetProvider, NetHandler, and now the document_may_load predicate's one body
@task_file n=2 path=packages/blitz-traits/src/navigation.rs
what: navigation options and their conversion into a top-level Request (NavigationOptions::into_request)
@task_file n=3 path=packages/blitz-vibey-script/src/fetch.rs
what: the script runtime's default script fetcher and its scheme rules; now re-exports document_may_load from blitz-traits
@task_file n=4 path=packages/blitz-net/src/lib.rs
what: the blitz-net Provider, its fetch by URL scheme (the file: branch reading from disk) and its typed ProviderError enum
@task_file n=5 path=packages/blitz-dom/src/net.rs
what: blitz-dom's sub-resource loading: the stylesheet handler and loader, @import and @font-face fetching, and the new crate-private request constructor that stamps the loading document
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: BaseDocument, the core document: its request builder, linked stylesheet, image and resource reload loads
@task_file n=7 path=packages/blitz-dom/src/iframe.rs
what: iframe sub-document loading from the parent document
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: DocumentMutator, the DOM mutation path, which builds a stylesheet handler when a link element is inserted
@task_file n=9 path=packages/blitz-dom/src/cssom.rs
what: the CSSOM implementation, including inserted @font-face rules and its stylesheet loader
@task_file n=10 path=packages/blitz-dom/src/layout/damage.rs
what: layout damage handling, which also requests CSS background image layers from style
@task_file n=11 path=packages/blitz-dom/clippy.toml
what: new crate-level clippy configuration for blitz-dom: a disallowed-methods rule forbidding Request::get outside the one stamped constructor
@task_file n=12 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: new blitz-tests integration target, one behaviour per file, both layout modes: every engine request carries its document as initiator, and a remote page reads no local file while a file: page reads them all
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests manifest and its dev-dependencies, now with blitz-net (workspace, default features off) and tokio (rt)
@task_file n=14 path=Cargo.lock
what: the workspace lockfile; only blitz-tests' own dependency list moves, no new package enters the graph
@task_item n=1 part=touches
decision/file-script-load-refused: the clause so a remote page cannot read local files is owed to this row (the operator's word and the route relay); the refusal now reaches every engine sub-resource fetch through blitz-net, not script loads alone.
@task_item n=2 part=touches
decision/file-url-load-rule: its rule names script loads only; now a file: URL is read from disk only for a request with no loading document or a file: loading document, enforced in blitz-net's provider.
@task_item n=3 part=touches
decision/url-scheme-routing: file: URLs read from disk is now conditional on the request's initiator; a non-file initiator gets FileForbidden before any disk read.
@task_item n=4 part=touches
contract/blitz-net-provider/fetch: the provider's fetch gains a refusal path, ProviderError::FileForbidden with fixed text, taken through the existing error path to NetHandler::error.
@task_item n=5 part=touches
contract/html-net/provider: the typed ProviderErrors of blitz-net gain the unit variant FileForbidden.
@task_item n=6 part=touches
contract/script/document-may-load and contract/blitz-vibey-script-api/document-may-load: the predicate's home moved to blitz-traits' net module; blitz-vibey-script now re-exports it, text and doc unchanged, so cited evidence locations are stale.
@task_item n=7 part=touches
decision/non-exhaustive-request-types and decision/non-exhaustive-option-types: Request gains the public field initiator and stays non-exhaustive; navigation's into_request sets it None.
@task_item n=8 part=touches
contract/net-handler/error: a file: refusal now reaches NetHandler::error for stylesheet, @import, font, image and iframe loads, with no log site of its own.
@task_item n=9 part=touches
decision/fmt-and-clippy-gate, decision/fmt-clippy-gates, decision/fmt-and-clippy-gates and contract/ci-leg-legs/clippy: the workspace clippy leg (-D warnings) now reads a crate-level clippy configuration in blitz-dom and fails on any unstamped Request construction there.
@task_item n=10 part=touches
decision/clippy-allow-scoping and decision/local-lint-allows: the one sanctioned construction stands under a local expect of clippy::disallowed_methods with a reason, a new local lint exception.
@task_item n=11 part=touches
decision/upstream-sync-merge-base, decision/upstream-taken-by-cherry-pick and decision/direct-engine-changes-flagged-upstreamable: blitz-dom gains a configuration file upstream has no counterpart for, and its sub-resource request paths diverge from upstream; a new upstream merge surface.
@task_item n=12 part=touches
decision/script-fetch-via-net-provider and contract/default-script-fetcher/schemes: indirect, the script fetcher's file: rule now shares one predicate with blitz-net's provider through blitz-traits.
@task_item n=13 part=touches
decision/failed-stylesheet-unblocks and decision/failed-stylesheet-stops-blocking: indirect, a refused file: stylesheet from a remote document is a failed load and stops blocking render through the existing error path.
@task_item n=14 part=touches
decision/image-cache-by-url, decision/font-face-css-descriptors, decision/import-nesting-cap and decision/iframe-nesting-cap: indirect, the image, font, @import and iframe load paths now carry their document's URL as initiator.
@task_item n=15 part=touches
decision/injected-embedder-services and decision/blitz-net-networking: indirect, embedders' top-level loads build requests with no initiator and read file: URLs as before; the provider now distinguishes loads by their initiator.
@task_item n=16 part=touches
decision/error-enums and decision/typed-results: ProviderError gains a unit variant with a fixed Display text, no URL or path.
@task_item n=17 part=touches
decision/builder-method-naming and decision/consuming-builder-methods: Request gains the consuming builder initiator(self, url) beside signal.
@task_item n=18 part=touches
decision/crate-internal-visibility and decision/restricted-crate-internals: the stamping constructor is crate-private and the stylesheet loader gains a pub(crate) document_url, while the public StylesheetHandler gains a pub document_url field, a public API change.
@task_item n=19 part=touches
decision/workspace-dependencies, decision/in-repo-crates-no-default-features and decision/in-repo-default-features-off: blitz-tests takes blitz-net and tokio as workspace dev-dependencies, blitz-net with default features off.
@task_item n=20 part=touches
decision/integration-tests-in-blitz-tests, decision/blitz-tests-one-target-per-file, decision/both-layout-modes and decision/scenarios-both-layout-modes: a new auto-discovered blitz-tests target follows them, runs both layout modes, and is not in stand-repeat's list.
@task_item n=21 part=touches
decision/favicon-success-by-decoding and decision/favicon-cache-positive-only: indirect, the browser's favicon probe builds its request with no loading document, so a remote page's file: icon is still read from disk; an open path left for a provisional route entry.
@task_item n=22 part=touches
decision/net-feature-noop-on-wasm and decision/net-noop-on-wasm: indirect, the file: refusal check runs before blitz-net's wasm32 branch and before any disk read.
@task_fact n=1 slug=conventions
fact: blitz-dom builds a network Request only through its one crate-private stamping constructor, which stamps the loading document as initiator; a crate-level clippy disallowed-methods rule on Request::get enforces it, with one local expect at that constructor as the sole exception.
@task_fact n=2 slug=established-decisions
fact: every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL as initiator; a request with no initiator is a top-level load an embedder or a navigation asked for.
@task_fact n=3 slug=established-decisions
fact: blitz-net reads a file: URL only for a top-level load or a file: loading document; any other request is refused with FileForbidden before any disk read, so a remote page cannot read local files through engine sub-resources.
@task_fact n=4 slug=established-decisions
fact: the browser's favicon probe and top-level navigation from a remote page to a file: URL are not refused: a remote page's file: icon is still read for the tab strip and a file: URL still loads as a top-level document, owed to a provisional route entry awaiting the founder.
@task_fact n=5 slug=standard-contracts
fact: blitz-traits' Request carries a public initiator, the URL of the document a sub-resource is loaded for, None for a top-level load, set by the consuming builder initiator; the document_may_load predicate lives in blitz-traits' net module and blitz-vibey-script re-exports it.
@task_fact n=6 slug=standard-contracts
fact: blitz-dom's public StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.
@task_fact n=7 slug=occupied-resources
fact: the blitz-tests target for local files writes its fixture directory under CARGO_TARGET_TMPDIR, read at build time, test-only, crossing no boundary.
@task_fact n=8 slug=infrastructure-patterns
fact: blitz-dom holds a crate-level clippy configuration read by the workspace clippy leg; upstream's blitz-dom has none, so it is an upstream merge surface.
@task_item n=1 part=untouched
The escher driver, session socket, MCP role, command line and their verbs, answers, refusals and exit codes: no driver or session surface moved.
@task_item n=2 part=untouched
Snapshot, diff, UI graph, stable element ids and actionable-key contracts: unchanged.
@task_item n=3 part=untouched
Select list, forms, focus, keyboard, pointer and DOM event dispatch decisions and contracts: no interaction behaviour changed.
@task_item n=4 part=untouched
Paint, layout algorithm, scrolling, hit testing and geometry readers: no rendering or layout rule changed, only the image request built from style now carries an initiator.
@task_item n=5 part=untouched
Accessibility tree and AccessKit contracts: untouched.
@task_item n=6 part=untouched
Telemetry, tracing and log scrubbing: no log site, log field, env read at runtime or scrub shape was added or moved.
@task_item n=7 part=untouched
CI workflow jobs, steps, permissions, caches, status shapes and coverage floors: no workflow or script changed, and no coverage floor or gate threshold moved.
@task_item n=8 part=untouched
WPT runner, its network provider, script fetcher, outputs and diff tooling: untouched.
@task_item n=9 part=untouched
Browser app history, persistence, about pages and chrome UI: unchanged; the browser's favicon probe code was not edited.
@task_item n=10 part=untouched
Cold-agent, agent-run, stand checks and recording directory contracts: untouched.
@task_item n=11 part=untouched
Names sheet: no crate, binary, server or other name was taken or freed, and no crate was added or removed.
@task_item n=12 part=untouched
absence/script-same-origin-cors: no same-origin or CORS check was added; the refusal is a scheme rule on the loading document only.
