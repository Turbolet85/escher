@history_item n=1 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — Threat Model Summary: `~ decision/file-url-only-for-file-documents`. The rule widened from script loads alone to "a file: document or a top-level load", with blitz-net refusing the engine's stylesheet, @import, font, image and iframe loads before any disk read. This chunk rests on that "top-level load" carve-out. The favicon probe (`Request::get` with no initiator) and `navigate_to` (`initiator: None`) both currently read as top-level loads, which is the gap this chunk closes in the app. The row's wording stays as it stands: the scope keeps the engine rule unwidened and unnarrowed.

@history_item n=2 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Input Validation sub-entry) — `~ decision/file-urls-only-from-file-documents` gave every engine sub-resource request its loading document, so blitz-net reads a file: URL only when there is no loading document or a file: one. It also added `+ decision/one-stamping-request-constructor`, a blitz-dom clippy rule that allows only the stamping constructor. This chunk calls the public `document_may_load` as it stands. Any new request in the app either carries an initiator or is never started. The stamping rule covers blitz-dom only and does not reach `apps/browser`, which is why the two app paths were left open.

@history_item n=3 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Error Handling sub-entry) — `+ contract/blitz-net-provider-error/file-forbidden`: the refusal has fixed, path-free display text and reaches the handler only through the existing error path, with no log site of its own. If the favicon gate carried an initiator instead of filtering, a refused probe would hit this contract. The chosen design is a filter in `favicon_candidate` and an early return in `navigate_to`, so no `FileForbidden` and no error path is reached at all. The contract must stay unchanged.

@history_item n=4 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the script fetch's `file:` refusal by document, a narrowing) — the origin of `document_may_load(document_url, url)`, taken with upstream `cee6b8731`. It is recorded as a narrowing, not a boundary widening: a refusal added on an existing input surface, with no scheme, carrier or crossing added. Witnesses are `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`. This is the precedent for classing this chunk's two app refusals as a narrowing that needs no founder ratification.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` read at a second test-only site) — the `file:`-load tests brought a test-only `TMPDIR` read through `temp_dir_with_files`, and the environment-values list names every read. This chunk's checks are designed to read no temp file or `TMPDIR` (a pure `favicon_candidate` unit test and a `navigate_to` test on an in-memory `History`). If a test does read one, the env list owes a row.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (input rows re-read on the merged engine: the `file:` read) — the net provider reads a `file:` URL through `Url::to_file_path` (a typed `InvalidInput` error, no panic) and applies no path restriction. That is why any `file:` URL a remote page names (favicon href, link or form target) reads arbitrary local files once it reaches blitz-net as a top-level load. This is the hazard this chunk closes.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the hint-less `@font-face` source: the PROVISIONAL mark comes off) — kept the standing rule that escher routes no agent- or user-supplied URL through `blitz-net`. This chunk adds no URL-taking verb or input (absence `absence/url-taking-driver-verb` stands), and the user's own top-level loads in the browser app stay unchanged.

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches the handler and ends the wait) — `NetHandler::error` is a do-nothing default on the public trait, and blitz-net's error arm calls it. This is the "existing error path" the `FileForbidden` contract names. Recorded trap: a trait's do-nothing default can hide a fix behind a wrapper, so the implementers of a trait a change extends need sweeping. It matters if the favicon probe were refused through blitz-net instead of never started.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (where a failed load's error text stops) — a failed load's error text stops in `ResourceLoadResponse` and the `Resource load failed` line, and the sink redacts `url` and `error`. `NetHandler::error` holds no `tracing` site. This supports the scope's boundary that a refused favicon or navigation adds no log line or field.

@history_item n=10 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — the origin of `decision/enter-activates-focused-control` and `decision/button-click-focuses`, an engine change for every document and a boundary widening ratified by the founder. A pressed Enter on an `a` with `href`, or on a submit button, dispatches a Click that reaches link navigation and form submission. This is why keyboard-activated links and submits from a remote page also reach `BrowserNavProvider::navigate_to`, and the navigation gate must cover them. The rows themselves are not to move.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls, the password caret by the bullets) — the activation arms' gate became `ElementData::is_disabled` on form controls alone, so a `disabled` attribute on an `a` gates neither key nor click walk. A remote page's link to a `file:` URL is therefore always activatable, and the refusal has to sit at `navigate_to`, not in the engine's activation gates.

@history_item n=12 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (a Markup attributes row for the three control states) — the precedent behind `decision/attribute-readers-no-log-or-panic`: attributes read from parsed content, remote HTML included, by in-process readers with no log site or panic path, carrying no attribute text as typed. The favicon candidate read of a `<link rel=icon>` href from a remote page must keep that shape: no log and no panic on an unresolvable or `file:` href.

@history_item n=13 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows, the source of every picked row and its why/guard cells (among them `decision/error-pages-skip-history-and-favicon`, `decision/failed-fetch-error-page-shows-debug-text` and `decision/external-targets-to-system-browser`). An amendment from this chunk names its change by row handle (`+`/`~`/`−`). The scope's error-page case (a document with no URL counting as no `file:` document) has to stay consistent with the row that error pages skip favicon probing.
