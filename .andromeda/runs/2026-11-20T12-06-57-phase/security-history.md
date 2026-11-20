@history_item n=1 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — (five advisory ignores: zip 2.4.2's three) recorded that zip reaches the whole-workspace graph only through `anyrender_serialize 0.8.0` ← `browser` (apps/browser's capture feature), which reads archive entries `by_name` and never extracts; the ignores are owned by a residual row. Matters here: this chunk works in `apps/browser/src/capture.rs`, the very module that ignore's reach argument rests on. A test or an edit there must add no `extract` call and no new dependency, or that written reason stops holding.

@history_item n=2 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the zip ignores gain RUSTSEC-2026-0429; advisories published after a green audit) restated the reach as `anyrender_serialize`'s `deserialize` reading `by_name` and never calling `extract`, and added a standing rule: the CI audit can go red over an unchanged lock, so run `bash .github/scripts/ci-leg.sh audit` locally before a push. Matters here: the capture module is that reach's only path in, and the pre-push audit rule applies to this chunk's push too.

@history_item n=3 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — (Dependency Security) `~ decision/upstream-sync-dependency-set` was amended for the AnyRender 0.15 coupled set entering the lock. Matters here: that anyrender bump (`4f61cffac`) is the one upstream change to `capture.rs` between the audit sha and HEAD, and the scope re-derives the mutant coordinate across it. Choosing to remove `render_scene`'s return value is an upstream divergence in that same file.

@history_item n=4 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — (API Security) `~ decision/session-socket-sole-listener`, which now reads "one listener of every default build", `+ decision/cdp-server-opt-in-loopback`, and `~ absence/served-network-api` / `~ overview/api-security`, which scope the absence to default features. (Dependency Security) `+ decision/skia-default-build-fetch`: Skia is a default renderer with a build-time download, plus the Stylo 0.24 / winit beta.4 pins. Matters here: the browser crate's test run under its default features builds this graph. A capture test must bind no listener and must not turn on devtools.

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (no served port, two test-only TCP binds) registered two `blitz-net` `#[cfg(test)]` loopback binds as test-only so the "no served port" claim holds, under the session rule that a new bind in a test is registered in the security plan. Matters here: a precedent for this chunk's `#[cfg(test)]` module in `capture.rs`. It binds nothing, and any test bind would owe the same registration against `decision/session-socket-sole-listener`.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the script fetch's `file:` refusal by document, a narrowing) added `document_may_load` and the Script fetch row (guard `remote_document_cannot_load_file_urls`). The default fetcher takes only `file:` and `data:`, which is where `decision/default-script-fetcher-local-only`, `decision/script-fetcher-file-data-only` and `decision/file-url-only-for-file-documents` come from. Recorded as a narrowing, not a widening. Matters here: these picked rows bind any browser-crate code path, and the chunk's "no change to what the browser paints or saves" keeps them intact.

@history_item n=7 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — `~ decision/file-url-only-for-file-documents` and `~ decision/file-urls-only-from-file-documents` widened the refusal to every engine sub-resource load, and `+ decision/one-stamping-request-constructor` (a clippy rule) followed. Matters here: these rows bind the browser crate's loads. A capture test that builds a `BaseDocument` must not route a `file:` load around the stamping constructor.

@history_item n=8 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — `~ decision/file-urls-only-from-file-documents` and `~ decision/file-url-only-for-file-documents` extended the rule into `apps/browser` itself: no `file:` favicon for a non-file document, and a `file:` link navigation is dropped silently. Matters here: this is the last security change made inside the browser app, the crate this chunk edits. It is a constraint the chunk must leave untouched.

@history_item n=9 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (input rows re-read on the merged engine) `file:` URLs are read through `Url::to_file_path`: a non-path URL is a typed `InvalidInput`, not a panic, and there is no path restriction. Matters here: this is background for the picked file-URL rows the browser app is bound by.

@history_item n=10 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (CI legs locked) brought `--locked` to every ci.yml cargo leg, the flake and the bundle, with `examples/wasm_hello` excepted. This is the origin of `decision/locked-builds` and its exception. Matters here: the browser crate's test run, and any operator-run `cargo mutants` invocation if the founder orders one, should pass `--locked`.

@history_item n=11 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — (Dependency Security) `~ decision/locked-builds` extended `--locked` to every cargo run of the build-directory sweep, so the rule reaches cargo runs outside CI. Matters here: this is the picked row's current wording. A dev-host mutation run is also a cargo run outside the CI legs.

@history_item n=12 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (`multi_doc`'s inputs, the traversal lock) added the process-wide `PARALLEL_TRAVERSAL_LOCK` with poison recovery, so two documents resolving `Parallel` at once are serialised rather than panicking. This is the source of `decision/parallel-style-traversal-serialised`. Matters here: a capture unit test that builds and resolves a `BaseDocument` while other tests run in parallel relies on this lock not to panic `already mutably borrowed`.

@history_item n=13 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (what the driver logs) fixed that the driver crate installs no subscriber and reads no env var or clock, and fields one `command` span. This is the origin of `decision/driver-installs-no-subscriber` / `decision/driver-no-subscriber-no-env`. Matters here: the chunk's scope rules out any new span, log field or env read.

@history_item n=14 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — (the acting step's bounded load wait) let the session read the wall clock for `LOAD_WAIT` only. This is one of the clock reads the picked `decision/driver-installs-no-subscriber` row enumerates. Matters here: the scope adds no clock or env read, so the row's list of clock readers must stay as it is.

@history_item n=15 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — (Input Validation) `~ decision/driver-installs-no-subscriber` and `~ decision/driver-no-subscriber-no-env` gained the exception that an acting step runs under escher-telemetry's scoped log capture, which prints nothing. Matters here: this is the picked rows' current exception cell. Nothing in this chunk should touch it.

@history_item n=16 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (Logging & Monitoring) `~ decision/no-screen-content-in-logs` widened the rule to in-process reads, the layout answer included. Matters here: this is the picked logging row. A capture test asserting `render_scene`'s returned size must add no log site that prints screen content.

@history_item n=17 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (Logging & Monitoring) `~ decision/no-screen-content-in-logs` added a step's caused text to what reaches no log, which is the row's current wording. Matters here: this is the picked row's latest amendment, and the chunk's "no new log field, span" boundary keeps to it.

@history_item n=18 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (the driver session socket: threat vector, access, input rows) introduced the Unix-domain session socket as the one local listener, ruling out a TCP or UDP port. This is the origin of `decision/session-socket-sole-listener`. Matters here: the scope adds no listener, consistent with this founder ruling.

@history_item n=19 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (the masters as rows) converted the security plan's text into rows, so every handle in the picked set dates from here and was verified against the code. Matters here: a row whose claim this chunk would make false (none is expected, given the scope's "no change to what the browser paints or saves") is amended by its handle at wrap.
