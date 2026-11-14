@task_prose part=session
The session closed blitz-net's file: branch to documents that are not themselves file: documents: a remote page's sub-resource fetch now reads no local file.
The shared request type gains an initiator, the URL of the document a sub-resource is loaded for; none means a top-level load no document asked for (an embedder's load or a navigation).
The engine stamps every one of its seven sub-resource paths with its document (linked stylesheet, its reload, CSS import, font-face, CSS image, img, iframe), through one constructor guarded by a crate clippy disallowed-methods rule with one expect.
blitz-net refuses a file: read for a non-file initiator with a new ProviderError FileForbidden, fixed text holding no URL or path, before any disk read; the refusal takes the existing error path and adds no log site.
The file-URL predicate document_may_load now has its one body in the shared traits crate, re-exported unchanged by the script fetcher; a new blitz-tests witness and blitz-net and traits unit tests prove it, CI green.
Two paths stay open and are owed to a provisional route entry: the browser's favicon probe still reads a remote page's file: icon, and navigation from a remote page to a file: URL still loads it top-level.
@task_file n=1 path=packages/blitz-traits/src/net.rs
what: the shared outbound request type (Request, non-exhaustive) with its new initiator field and builder, and the file-URL load predicate that decides which documents may load file: URLs
@task_file n=2 path=packages/blitz-traits/src/navigation.rs
what: navigation options turned into a top-level request, which carries no initiator
@task_file n=3 path=packages/blitz-vibey-script/src/fetch.rs
what: the script fetcher (default script fetcher and JS-facing fetch), which now re-exports the file-URL load predicate instead of holding it
@task_file n=4 path=packages/blitz-net/src/lib.rs
what: the blitz-net provider, outbound HTTP and file: reads, its typed ProviderError set gaining FileForbidden, and the file: read refusal for a non-file loading document
@task_file n=5 path=packages/blitz-dom/src/net.rs
what: the engine's sub-resource loaders (stylesheet handler and loader, CSS import, font-face fetch) and the one request constructor that stamps the loading document
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: the document's request builder and its linked stylesheet, image, font-face and resource reload loads
@task_file n=7 path=packages/blitz-dom/src/iframe.rs
what: the iframe load start, which stamps the parent document as initiator
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the DOM mutator building the stylesheet handler for a linked sheet
@task_file n=9 path=packages/blitz-dom/src/cssom.rs
what: the CSSOM script API's font-face rule insertion and stylesheet loader
@task_file n=10 path=packages/blitz-dom/src/layout/damage.rs
what: the CSS background image layer fetch from style
@task_file n=11 path=packages/blitz-dom/clippy.toml
what: blitz-dom's lint config, the construction guard that forbids building a request outside the stamping constructor; a new upstream merge surface
@task_file n=12 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: the witness check that a remote page reads no local file and a file page reads them all through blitz-net's real provider, and that every engine request carries its document
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests manifest, its dev-dependencies gaining blitz-net and tokio, workspace-inherited
@task_file n=14 path=Cargo.lock
what: the lockfile, moved only in blitz-tests' own dependency list, no new package
@task_item n=1 part=touches
decision/file-url-only-for-file-documents and decision/file-urls-only-from-file-documents state script loads alone; the rule now also covers every engine sub-resource fetch through blitz-net (stylesheet, its reload, CSS import, font-face, CSS image, img, iframe), and their guard set gains the new witness and blitz-net's refusal test.
@task_item n=2 part=touches
decision/default-script-fetcher-local-only, decision/script-fetcher-file-data-only and decision/default-fetcher-file-and-data-only rest on the file-URL predicate, which moved its one body to the shared traits crate and is re-exported by the script fetcher, text and behaviour unchanged.
@task_item n=3 part=touches
decision/outbound-http-through-blitz-net and decision/outbound-http-via-reqwest-or-blitz-net: blitz-net's provider now refuses a file: read for a request whose loading document is not a file: document, before any disk read.
@task_item n=4 part=touches
decision/subresource-requests-get-only and decision/document-abort-signal-cancels-fetches: every engine sub-resource request is now built through one constructor that takes the signal and stamps the loading document; still a GET.
@task_item n=5 part=touches
decision/net-handler-error-default-noop: a FileForbidden refusal reaches the handler's error method through the existing error path, so a handler that keeps the default does nothing on it.
@task_item n=6 part=touches
decision/failed-stylesheet-blocks-no-render, decision/failed-load-ends-wait and decision/bounded-load-wait: a refused file: stylesheet, image or frame is a failed load on the existing error path.
@task_item n=7 part=touches
decision/remote-content-nesting-limits, decision/iframe-nesting-depth-limit, decision/import-nesting-depth-limit and decision/stale-iframe-responses-discarded: iframe and CSS import loads now carry their parent document and a remote parent's file: frame or import is refused.
@task_item n=8 part=touches
decision/hintless-font-face-source-sniffed and decision/no-agent-urls-through-font-path: the font-face fetch path now takes the document URL and a remote document's file: font source is refused.
@task_item n=9 part=touches
contract/network-tracing-log/error, contract/network-tracing-log/url and contract/resource-load-failure-log/error: the refusal's only log line is the existing tracing-feature error line; the new error text is fixed and holds no URL or path.
@task_item n=10 part=touches
absence/request-timeout, absence/response-size-limit and absence/response-body-size-limit still hold: blitz-net's file: read keeps no path restriction, size cap or timeout, now narrowed to file: documents and top-level loads.
@task_item n=11 part=touches
absence/same-origin-cors-checks, absence/same-origin-and-cors-checks, absence/cors-checks, absence/same-origin-checks and absence/cors-same-origin-policy still hold: the new refusal is a scheme rule on file: alone, not an origin or CORS check.
@task_item n=12 part=touches
decision/error-pages-skip-history-and-favicon and the favicon probe: the browser's favicon probe still fetches a page's icon with no loading document, so a remote page's file: icon is still read from disk; owed to a provisional route entry.
@task_item n=13 part=touches
decision/external-navigation-schemes, decision/external-navigation-get-http-mailto and decision/iframe-navigation-to-parent: a navigation is a top-level load with no initiator, so navigation from a remote page to a file: URL still loads it.
@task_item n=14 part=touches
decision/png-encodes-screenshots and decision/png-shot-encoder: a blitz-tests check now also encodes a PNG fixture through the png dev-dependency, test-only.
@task_item n=15 part=touches
decision/workspace-inherited-dependency-versions, decision/locked-builds and decision/blitz-tests-not-published: blitz-tests gains blitz-net and tokio as workspace-inherited dev-dependencies; the lockfile moves only in blitz-tests' list, no new package, audit and licences pass.
@task_item n=16 part=touches
decision/upstream-sync-dependency-set and decision/engine-dependency-pins: blitz-dom gains a crate clippy config upstream lacks, a new upstream merge surface.
@task_item n=17 part=touches
contract/environment-variables/rust-log and decision/driver-reads-only-rust-log: the new witness reads CARGO_TARGET_TMPDIR at build time, test-only; no production runtime env read added.
@task_item n=18 part=touches
decision/dioxus-scheme-local-files: the dioxus scheme's local files load as top-level or file: document loads; check the file: rule's new reach does not contradict it.
@task_fact n=1 slug=api-security
fact: Every engine sub-resource request carries its loading document's URL as initiator, and blitz-net reads a file: URL only for a request with no initiator (a top-level load) or a file: initiator, else refuses it with FileForbidden before any disk read.
@task_fact n=2 slug=input-validation
fact: blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document, enforced by a crate clippy disallowed-methods rule on Request::get with a single expect exception at that constructor.
@task_fact n=3 slug=error-handling
fact: blitz-net's FileForbidden refusal has a fixed display text, file: load refused for a non-file document, holding no URL or path, and reaches the handler only through the existing error path with no log site of its own.
@task_fact n=4 slug=api-security
fact: No file: restriction yet on the browser's favicon probe or on navigation from a remote page to a file: URL; both still read local files, owed to a provisional route entry awaiting the founder.
@task_fact n=5 slug=secret-management
fact: One blitz-tests target reads CARGO_TARGET_TMPDIR at build time for a test-only fixture directory; no runtime env read.
@task_item n=1 part=untouched
Authentication and authorization rows: no auth, token or publish control moved.
@task_item n=2 part=untouched
Data protection rows: history store, cookies, HTTP cache, TLS, clipboard, file dialog and kept-check files are not reached.
@task_item n=3 part=untouched
The driver session socket, MCP server and escher-session CLI rows, their contracts, exit codes, refusal causes and bounds: no verb, wire or role changed.
@task_item n=4 part=untouched
Snapshot, stable element id, accessibility, select list, range, password mask and actions or graph answer rows: no reading, mask or carrier moved.
@task_item n=5 part=untouched
Cold-agent, recording, coverage floor, apt-install and agent-run tooling rows: no script or gate of theirs changed.
@task_item n=6 part=untouched
Logging sink allowlist, target scrub and stderr telemetry sink rows: no log field, target or scrub shape moved.
@task_item n=7 part=untouched
CI secrets, signing, actions pinning and WPT runner rows: no CI step, workflow or script changed.
