@task_prose part=session
In the browser app (upstream's reference app), a page that is not a file: page can no longer reach a local file through its favicon or through a navigation it starts.
The favicon candidate is now kept only where document_may_load allows it, so a remote page names no file: favicon and never reaches the favicon probe or the shared favicon cache; a file: page's file: favicon and a remote page's own /favicon.ico are unchanged.
The navigation provider is now built for its document by a constructor, for_document. It silently drops a non-file: document's navigation to a file: URL, the way Chromium and Firefox do, loading nothing and logging nothing; the error page counts as no file: document.
Both refusals are silent: no tracing event, print, env read, log field, span, cause, exit or error was added, Cargo.lock is unchanged, and the user's own loads (URL bar, history, home, about pages, first tab) pass through no gate.
The capability statement lost its two clauses marked provisional pending the founder's choice (2 to 0), and the browser crate's unit tests went from 29 to 35. CI was green, 17 of 17 checks.
The Epoch 4 boundary batch moves the obs-plan sidecar's delegate-confirm PROVISIONAL entries to ratified on the founder's own recorded word, with no row text changed.
@task_file n=1 path=apps/browser/src/favicon.rs
what: the browser app's favicon candidate: which favicon URL a loaded document names, before the probe and the shared favicon cache
@task_file n=2 path=apps/browser/src/history.rs
what: the browser app's navigation provider and its history store: the link-click navigations a document starts
@task_file n=3 path=apps/browser/src/document_loader.rs
what: the browser app's document loader: the document config each loaded page or error page is built with
@task_file n=4 path=docs/capabilities.md
what: the capability statement, updated by hand
@task_item n=1 part=touches
decision/no-file-loads-from-non-file-documents: two refusals in the browser app now stand beside the typed forbidden errors this rule names. These are a remote page's file: favicon candidate and a non-file: document's navigation to a file: URL. Both are dropped silently, raise no error and add no log site, so the row's reach (typed errors only) no longer covers every refusal.
@task_item n=2 part=touches
decision/checks-silent: the browser app's two new file: refusals follow the silent rule, logging nothing and printing nothing. They are a new instance under this rule.
@task_item n=3 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand, and its two clauses marked provisional pending the founder's choice were removed (a count of 2 down to 0).
@task_item n=4 part=touches
The obs-plan row the capability sweep reached (dec-5bxeg64t, whose claim shares the capability-statement token) was read as true with no change. Its pinned names or sections may still need a look against the edited statement.
@task_item n=5 part=touches
decision/failed-stylesheet-leaves-pending-set and the other §7 error-capture rows over the browser's document loading: the document loader's config now carries a navigation provider built for the document's base URL. An error page (no base URL) counts as no file: document.
@task_item n=6 part=touches
contract/telemetry-scrub-set/url: a refused file: URL goes to no log or span, so no url field is written for it. The scrub set's url entry is not reached by the refusals.
@task_item n=7 part=touches
The obs-plan rows backed by the sidecar's delegate-confirm entries now rest on the founder's own recorded word, ratified at the epoch-close consolidation with no row text changed. These are the recording directory row of §9 (decision/recording-dir-private), the graph answer item of §8 (contract/graph-answer/content), the escher-session install row of §3, the coverage role example of §6, the command span's reach in §6, §4 and §2, and the coverage line's carried fields in §8 (contract/coverage-line/content, contract/coverage-line/shape).
@task_item n=8 part=touches
decision/session-logs-carry-no-page-content and decision/scrub-at-sink-not-call-sites: the browser app's new refusals write no line that could carry a page's URL. They hold under these rules with no new call site.
@task_item n=1 part=untouched
§2 telemetry strategy and the tracing-feature rows (engine tracing gated, escher crates ungated, tracing as the telemetry crate): no tracing call or dependency was added, and the lockfile is unchanged.
@task_item n=2 part=untouched
§4 span and trace coverage: the driver's command span, the tool_call span, its endings and the refusal cause names. No span, field, verb or cause was added.
@task_item n=3 part=untouched
§3 and §6 sink and log-line contracts: escher's telemetry sink, its log line layout, escaping, scrub and stream, the sink target allowlist, RUST_LOG and the escher-telemetry events and panic hook.
@task_item n=4 part=untouched
escher-session's roles, commands, exit codes, stdout, stderr, host wire, error object, coverage role and session lifecycle. No CLI surface changed.
@task_item n=5 part=untouched
The agent-run and cold-agent harness rows (events, verdict fields, reasons, counted verbs, transcripts, kept checks, cleanup). No harness changed.
@task_item n=6 part=untouched
§5 metric coverage and the absences of a telemetry backend, metrics exporters, OTel export and an error-reporting service. Nothing was emitted or exported.
@task_item n=7 part=untouched
§9 CI integration rows (GitHub workflows, artifacts on failure or success, WPT publishing upstream only, package-install diagnostics). No workflow changed, and the CI run was only read.
@task_item n=8 part=untouched
§8 snapshot, select, optgroup, page-events, actions answer and state-registry rows, and the script error and window error event contracts. No engine crate or driver answer changed.
