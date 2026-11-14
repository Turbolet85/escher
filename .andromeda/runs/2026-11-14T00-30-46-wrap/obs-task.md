@task_prose part=session
The session closed blitz-net's file: branch for a non-file: document: a file: URL is read from disk only for a request with no loading document (an embedder's top-level load) or with a file: loading document.
Every engine sub-resource request now carries its loading document's URL as an initiator: linked stylesheet, its reload, @import, @font-face, CSS image, img and iframe (0 to 7 paths).
A refused load fails with a new error variant whose text is fixed and names no URL or path, and reaches the document's handler through the existing error path, adding no log site, print, env read or panic path.
The file: load predicate moved to one body in blitz-traits and is re-exported from the script crate under its old name; blitz-dom may build a sub-resource request only through one stamping constructor, enforced by a crate clippy config.
A new silent blitz-tests check target proves a remote page reads no local stylesheet, @import, image or iframe while the same page as a file: document reads all four; the census of log sites and the witness silence probe both read 0.
Two paths stay open and are owed to a provisional route entry: the browser's favicon probe still reads a remote page's file: icon, and navigation from a remote page to a file: URL still loads it.
@task_file n=1 path=packages/blitz-net/src/lib.rs
what: blitz-net's provider, the engine's network fetch with its file: branch and its typed provider errors, now with the file: refusal for a non-file: document
@task_file n=2 path=packages/blitz-traits/src/net.rs
what: the engine's network request type and the file: load predicate (may a document load this URL), now carrying the request's loading document as initiator
@task_file n=3 path=packages/blitz-traits/src/navigation.rs
what: the navigation options that build a top-level load request, with no loading document
@task_file n=4 path=packages/blitz-vibey-script/src/fetch.rs
what: the script runtime's fetch and script load path, which re-exports the file: load predicate
@task_file n=5 path=packages/blitz-dom/src/net.rs
what: blitz-dom's sub-resource loaders: the stylesheet handler and loader, @import and @font-face fetches, and the one stamping request constructor
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: the engine document, which builds requests for linked stylesheets, their reload, images and fonts
@task_file n=7 path=packages/blitz-dom/src/iframe.rs
what: the iframe load of a sub-document from its parent document
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the document mutator, which builds the stylesheet handler for a linked stylesheet
@task_file n=9 path=packages/blitz-dom/src/cssom.rs
what: the CSSOM, its stylesheet loader and its @font-face insertion
@task_file n=10 path=packages/blitz-dom/src/layout/damage.rs
what: the layout damage pass, which fetches CSS background images
@task_file n=11 path=packages/blitz-dom/clippy.toml
what: blitz-dom's crate lint config, the guard that a sub-resource request is built only through the stamping constructor
@task_file n=12 path=tests/blitz-tests/tests/local_files_for_local_documents.rs
what: a silent blitz-tests check: the remote page reads no local file, the file: page reads them all, and every engine request carries its document as initiator
@task_file n=13 path=tests/blitz-tests/Cargo.toml
what: the blitz-tests manifest and its dev-dependencies, now blitz-net and tokio
@task_file n=14 path=Cargo.lock
what: the workspace lockfile, moved only in blitz-tests' own dependency list
@task_item n=1 part=touches
decision/no-file-loads-from-non-file-documents: its rule names script loads alone; the refusal now also covers stylesheet, @import, font, image and iframe loads through blitz-net, each refused with no log site of its own.
@task_item n=2 part=touches
decision/fixed-diagnostics: a new provider error, file: load refused for a non-file document, is a fixed string with no URL or path in it.
@task_item n=3 part=touches
decision/failed-stylesheet-leaves-pending-set: a refused file: stylesheet now fails through the handler's error path, so it leaves the pending critical set as a failed stylesheet does.
@task_item n=4 part=touches
decision/engine-tracing-feature-gated and decision/engine-tracing-by-invocation: blitz-net's feature-gated Error fetching line is the only log a refusal reaches, unchanged.
@task_item n=5 part=touches
decision/engine-target-safe-fields: the engine's fetch error line may now print the new refusal text, which holds no URL or path.
@task_item n=6 part=touches
contract/telemetry-scrub-set/url and contract/telemetry-scrub-set/error: the new refusal error carries no URL, so no url value enters an error field through it.
@task_item n=7 part=touches
decision/checks-silent: a new blitz-tests check target joins the silent checks, holding no print, log or runtime env read; it reads the test temp directory at build time only, and the shared modules stand unchanged.
@task_fact n=1 slug=7-error-capture-reporting
fact: A blitz-net file: load refused for a non-file: loading document fails with a fixed-text provider error naming no URL or path, delivered through the document handler's error path with no log site of its own.
@task_fact n=2 slug=7-error-capture-reporting
fact: Two local-file paths stay open: the browser's favicon probe reads a remote page's file: icon with no loading document, and navigation from a remote page to a file: URL loads it as a top-level document; owed to a provisional route entry.
@task_item n=1 part=untouched
escher-driver's command and tool_call spans, their fields and endings, and the refusal cause names: no driver code moved.
@task_item n=2 part=untouched
escher-telemetry's sink, its log line layout, resource keys, RUST_LOG handling and panic hook: unchanged.
@task_item n=3 part=untouched
The cold-agent and agent-run harness events, verdicts and kept checks: untouched.
@task_item n=4 part=untouched
escher-session's commands, exit codes, host wire, recording and state directories: untouched.
@task_item n=5 part=untouched
Metric emission and exporters, WPT score publishing and the telemetry backend absences: no metric, exporter or backend added.
@task_item n=6 part=untouched
CI workflows, their artifacts and the package-install diagnostic line: no CI step, script or status shape changed.
@task_item n=7 part=untouched
The window error event, window.onerror and the script error source labels: the script error surface is unchanged.
@task_item n=8 part=untouched
The snapshot, graph, actions and coverage line answers and the select and form entry rows: untouched.
