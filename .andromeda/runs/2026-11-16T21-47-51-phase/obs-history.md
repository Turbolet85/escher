@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — founded escher-telemetry's scrub: engine targets print only the safe fields, and `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` are redacted everywhere. This is where `CONTENT_FIELDS` came from. The chunk keeps it unchanged for the sink and drops `error` only from the answer's own set.

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made the sink an allowlist with three outcomes: an engine target prints only safe fields, an `escher_` target prints everything but the content-named set, and any other target is dropped whole. Its standing rule is that a target is admitted only together with a scrub rule for it. This is why an app record's `error` text never reaches the sink's lines, so the chunk's control can only be read on an engine record (`decision/sink-target-allowlist`).

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (both entries) — recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. It also said a closed span's pairs are judged by the same scrub rule, and a content-named span field's value is never stored. Bearing on this chunk: the `Resource load failed` control exists only in an engine-tracing build (`decision/engine-tracing-by-invocation`). The sink's span-field store (`format.rs:236`) must stay on `CONTENT_FIELDS` (`decision/content-span-fields-never-stored`).

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set that a command's answer, refusal or session-error object on stdout is an answer and never a log line, and that the next surface adds no span or log field holding what a call supplied or a screen reads. Opening `error` in `caused` widens the answer only; it must add no log field (`contract/escher-session-stdout/command-answer`).

@history_item n=5 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the published statement `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of them, so a change to the scrub's limits must be made there by hand (`decision/capability-statement-updated-by-hand`). Check whether the answer's narrower scrub set needs a hand edit there.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler, and where its error text stops) — blitz-net's error arm calls `handler.error(url, e.to_string())`. The error text stops in `ResourceLoadResponse` and on the `Resource load failed` line, where `url` and `error` print `[redacted]` and no other carrier takes the text. This is the engine record (`document.rs` `Resource load failed`, fields `url`, `waiting_nodes`, `error`) the chunk's control reads. Its trap: an engine target's `message` also prints `[redacted]`, and the report had overstated what the sink prints, so read the formatter rather than the report.

@history_item n=7 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/no-file-loads-from-non-file-documents`: an engine sub-resource fetch refused for a `file:` load fails with fixed text naming no URL or path. The founder's stated price for this chunk is that an error text can hold a failed fetch's URL. This entry is one place where the engine already keeps the URL out of its error text, and that text now crosses into `caused`.

@history_item n=8 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — introduced escher-telemetry's scoped capture around an acting step: `~ decision/headless-stand-no-sink`, `~ decision/driver-library-installs-no-subscriber`, `~ contract/telemetry-sink/rust-log`, `~ decision/driver-emits-no-event`, `~ contract/escher-sink-env/rust-log`, `+ decision/capture-framework-targets-warn-up`. The capture layer runs beside the sink and is unfiltered by `RUST_LOG`. It is the layer whose `Values::keep` the chunk changes.

@history_item n=9 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§3, 4 rows) — `~ decision/answers-echo-nothing-typed` gained the exception that a caused item's message, chain and kept fields hold what their author wrote, a content-named field's value reading `[redacted]`. `~ decision/headless-stand-no-sink` and `~ decision/checks-silent` name the error-text founder check, which installs escher's sink over an in-memory writer. The chunk extends that check, and the exception's wording must change once `error` is no longer read `[redacted]` in `caused`. The founder's ruling «Применить все 14 (Recommended)» was relayed by the overseer as the operator's Q1 answer.

@history_item n=10 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§4 and §6) — `~ decision/driver-emits-no-event`: the capture keeps the step's records and panics in process and prints nothing. `~ decision/capture-framework-targets-warn-up`: the capture admits engine targets at warn and above and `escher_` targets never. This is why the engine `Resource load failed` warn reaches a `caused` item, and so why its `error` field crosses once opened.

@history_item n=11 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§7, 2 rows) — `~ contract/telemetry-scrub-set/error`: the sink prints `error` as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item, and a failed fetch's error text reaches no carrier as a field value. `~ contract/telemetry-scrub-set/url`: the same for `url`. This chunk directly reverses the capture half of the `error` row while keeping the `url` row, so the `error` row's promise must be amended at this wrap as a boundary widening.

@history_item n=12 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§8, `password` added) — `~ decision/escher-target-content-named-set`: `CONTENT_FIELDS` gained `password`, making it twelve names. This is the set the chunk copies, minus `error`, for the answer, and that row must keep stating the sink's full twelve.

@history_item n=13 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (§8, 3 rows) — `− decision/scrub-at-sink-not-call-sites → decision/scrub-by-field-name`, `+ decision/scrub-by-field-name`, `+ decision/engine-sites-fixed-messages`. Scrubbing is by field name in the sink and in the capture, both using the same content-named set, and engine warn and error sites carry their values in named fields (including `error`). The chunk splits that shared set, so `scrub-by-field-name` needs amending: the capture gets its own set without `error`. It also means an engine site's `error` field, whose text may hold a URL, now crosses into the answer.
