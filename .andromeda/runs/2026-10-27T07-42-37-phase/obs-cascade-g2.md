# obs · cascade read · group 2 of 2

## Carried
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) The sink allowlist is re-read against every target the taken commits add, and each new target must land in one of its three outcomes unchanged, `packages/escher-telemetry/` left unedited.
  > escher's own sink applies an allowlist of targets in its formatter (packages/escher-telemetry/src/format.rs:22-111) — two public prefix sets, three outcomes: an event whose target — or a bridged `log` record's `log.target` — starts with `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` (`ENGINE_TARGET_PREFIXES`) prints only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every other field (the message included) as `{name}=[redacted]`;
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20)
  > the five scrub sets and the rule are unchanged
  (.andromeda/obs-plan.md:257)
  > escher's own sink applies an allowlist of targets in its formatter, three outcomes: an engine target (`ENGINE_TARGET_PREFIXES`) prints only the safe fields, the rest `[redacted]`; an `escher_` target (`ESCHER_TARGET_PREFIXES`) prints every field but the content-named set, which is redacted; any other target is dropped whole before anything is written, at every level and whatever `RUST_LOG` names; a closed span's line is judged pair by pair by the same rule; no engine call site is edited.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) This take repeats the last take's allowlist re-check: the scrub sets, redaction shape and violation schema must not move, and `host_log` and `mcp_log` must pass in the per-package and the workspace build on the synced tree, the record restating which of the three crates the take leaves unchanged (dioxus-native-dom is edited by `7fc9ee3af`).
  > The sink allowlist re-checked after the upstream take read at `f9cbb2756` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/`, `packages/escher-driver/` and `packages/dioxus-native-dom/` unchanged; `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the taken commits' added engine lines hold one commented `dbg!` and no log site
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8)
  > Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` can still carry a host path, now only for a bridged `log` record under an engine target (`js_console`)
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10)
  > Typed text in a HOST's log is measured since 2026-10-10-driver-cli: a hosted `type` reaches the `escher-session` host's instance, and that host's stderr at `RUST_LOG=trace` holds 0 occurrences of the typed sentinel, 0 id needles and 0 name needles under both builds (`host_log` green as `-p seven_guis` and as `--workspace`)
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11)
  > A recording session's clients are read by a standing check: `host_log`'s third test (unix only) runs each client command through `common::run` at `RUST_LOG=trace` — `start crud --record`, `type` with a sentinel, `snapshot`, `snapshot --part 1`, `lint`, `stop` — in the state directories `hl-client` and `hl-client-rec`, guarded by `common::Started`, and over all the clients' own stderr reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences under both builds
  (.andromeda/obs-plan.md:98)
  > The upstream take read at `f9cbb2756` is silent: upstream's target `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing, its readings going into assertions only; the taken commits add no tracing site over `packages/` (the fixed-sha census reading one commented `dbg!`) and no wall-clock read in blitz-dom outside `clock.rs`
  (.andromeda/obs-plan.md:253)
  > Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` can still carry a host path only for a bridged `log` record under an engine target (`js_console`); a record from a target outside the sink's allowlist never reaches stderr; the sink's lines read 0 id, name and typed-text needles in process, in the `escher-session` host and in the windowed stand, and 0 bytes of a shot in the host's and `mcp` role's stderr.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:26) The capture checks are re-run under both builds because engine events reach the sink only in the workspace build, and the upstream apps' stdout subscribers stay outside the scrub where the take carries `d63ed2c8c`.
  > Its reach is that sink, installed by the two binaries of the seven_guis package (`seven_guis_native`, `escher-session`): in both the engine `tracing` features stay off in a package-alone build (`-p seven_guis`), so engine events reach it only when one is turned on — which a workspace-wide build does for blitz-dom (§2 → Feature wiring)
  (.andromeda/obs-plan.md:261)
  > the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed.
  (.andromeda/obs-plan.md:102)
  > The upstream apps install `tracing_subscriber::fmt::init()` under the `tracing` feature, writing to stdout (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:76-77; examples/todomvc/src/main.rs:16-17)
- (.andromeda/obs-plan-detail/log-coverage.md:8) A target the take adds must write a closed span's line only by this rule, every pair scrubbed under its target and an outside target's span writing no byte, as the re-run capture checks read it.
  > A closed span's line — a second record class since 2026-10-07-driver-command-spans, a boundary widening ratified by the founder (2026-10-09) as built — is written for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited
  (.andromeda/obs-plan.md:159)
  > every pair passes the §8 scrub; an event's line is unchanged inside a span.
- (.andromeda/obs-plan-detail/log-coverage.md:49) `host_log` and `mcp_log` must read 0 shot-byte, lint and part-read needles at `trace` under both builds on the synced tree, the driver's span lines unchanged.
  > one closed-span line per call to `Session::run`: INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`, then `message="close"`, `time.busy`, `time.idle`
  (.andromeda/obs-plan-detail/log-coverage.md:52)
  > over one CRUD flow at `info` the role's stderr reads 9 lines built alone and 10 built with the workspace — the tenth `INFO blitz_dom::document … message=[redacted]` — with 5 `tool_call` and 3 `command` lines in each
  (.andromeda/obs-plan.md:189)
  > one closed-span line per call to `Session::run`, `span="command"` with its recorded fields, and one per MCP tool call, `span="tool_call"` with `tool` and `ending`, each INFO at target `escher_driver`, closed words, bools and counts only — no shot byte, lint finding, part number or text; no line at the default `warn`
- (.andromeda/obs-plan-detail/log-coverage.md:125) The chunk re-reads escher-driver's span sites after the take: 2 `info_span!` sites, no event and no `#[instrument]`.
  > escher-driver holding 2 `info_span!` sites, no event and no `#[instrument]`
- (.andromeda/obs-plan-detail/log-coverage.md:58) Where the re-read moves the span set or one of the scrub's limits this document restates — `log.file` on a bridged `js_console` record sits in the runtime the take splits — `docs/capabilities.md` is changed by hand, its pin reading none of these claims.
  > The published statement `docs/capabilities.md` restates this emitter's reach: the `command` and `tool_call` spans with closed-word, bool and count fields, stderr only, no driver line at the default `warn`, and the scrub's limits (the raw panic message, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR, the unscrubbed upstream apps and WPT runner); its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the span set or to the scrub's limits is made in that document by hand
- (.andromeda/obs-plan-detail/log-coverage.md:101) The uncaught-JS-error site in the conflicted `runtime.rs` is one of the two new `tracing::error!` sites to read — whether `e6f4615ec`'s split moves it or adds it, and whether its message carries page content — and its recorded location is re-read on the take's tree.
  > Uncaught JS errors are logged as "Uncaught JS error in" plus the source description (packages/blitz-vibey-script/src/runtime.rs:1104-1105)
  (.andromeda/obs-plan-detail/log-coverage.md:100)
  > Recorded script errors are logged with a "blitz-vibey-script:" prefix under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:255-258)
  (.andromeda/obs-plan-detail/log-coverage.md:102)
  > JS console log, info, warn and error all map to one debug-level log call (packages/blitz-vibey-script/src/runtime.rs:1253-1269)
  (.andromeda/obs-plan.md:111)
  > Embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:233-252)
  (.andromeda/obs-plan.md:112)
  > At most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:103-105; packages/blitz-vibey-script/src/document.rs:247)
- (.andromeda/obs-plan-detail/log-coverage.md:96) blitz-shell's error sites in `convert_events.rs` and `window.rs` stay behind its optional `tracing` dependency through the take of `a09f46792` and `6587e7e79`, and the new clipboard-failure `tracing::error!` site of `527394724` is read for its target and fields beside them.
  > `tracing::error!` is logged for unsupported Ime, PointerSource, PointerKind, ButtonSource and MouseScrollDelta variants (packages/blitz-shell/src/convert_events.rs:41-45; packages/blitz-shell/src/convert_events.rs:78-82; packages/blitz-shell/src/convert_events.rs:94-98; packages/blitz-shell/src/convert_events.rs:112-116; packages/blitz-shell/src/convert_events.rs:140-144; packages/blitz-shell/src/window.rs:849-853)
- (.andromeda/obs-plan-detail/log-coverage.md:67) blitz-dom's sites in the conflicted `mutator.rs` and the reached `document.rs` stay behind the `tracing` feature through each resolution, their URL fields recorded as values logged as-is, and their recorded line locations re-read.
  > info: image cache hit, pending queue and fetch (packages/blitz-dom/src/mutator.rs:1329-1349); image loaded and node count (packages/blitz-dom/src/document.rs:1551-1555)
  (.andromeda/obs-plan-detail/log-coverage.md:68)
  > warn: iframe depth cap and unresolvable iframe URL (packages/blitz-dom/src/mutator.rs:1371-1375; packages/blitz-dom/src/mutator.rs:1398-1399)
  (.andromeda/obs-plan-detail/log-coverage.md:69)
  > warn: resource load failed with and without URL (packages/blitz-dom/src/document.rs:1441-1452)
  (.andromeda/obs-plan-detail/log-coverage.md:70)
  > info: focussed node (packages/blitz-dom/src/document.rs:1871-1872)
  (.andromeda/obs-plan-detail/log-coverage.md:65)
  > warn: no DOM on resolve (packages/blitz-dom/src/resolve.rs:45) and on hit test (packages/blitz-dom/src/document.rs:2035)
  (.andromeda/obs-plan.md:244)
  > blitz-dom log lines include resource URLs (packages/blitz-dom/src/document.rs:1442-1447; packages/blitz-dom/src/mutator.rs:1330; packages/blitz-dom/src/mutator.rs:1349)
- (.andromeda/obs-plan-detail/log-coverage.md:75) The href and attribute sites in `events/pointer.rs`, which `0980cce5f` reaches, stay feature-gated with their fields unchanged, their locations re-read.
  > `warn` logs an unparseable link href together with the document URL (packages/blitz-dom/src/events/pointer.rs:901)
  (.andromeda/obs-plan-detail/log-coverage.md:76)
  > `info` logs a click on a link without href together with the element's attributes (packages/blitz-dom/src/events/pointer.rs:906)
  (.andromeda/obs-plan.md:246)
  > Log lines include the raw href, the document URL and element attributes unfiltered (packages/blitz-dom/src/events/pointer.rs:901; packages/blitz-dom/src/events/pointer.rs:906)
- (.andromeda/obs-plan-detail/log-coverage.md:81) The SVG parse-failure event in the conflicted `construct.rs` keeps its outer-HTML field behind the `tracing` feature through the resolution of `7fc9ee3af` and `2963b65a2`, and the layout events of the files `0980cce5f` reaches are read in the same census.
  > `tracing::warn!` "SVG parse failed" with fields `node_id`, `html`, `error` (packages/blitz-dom/src/layout/construct.rs:471-477)
  (.andromeda/obs-plan-detail/log-coverage.md:80)
  > `tracing::error!` "Tried to lay out text node individually" with fields `node_id`, `data` (packages/blitz-dom/src/layout/mod.rs:247-252)
  (.andromeda/obs-plan-detail/log-coverage.md:82)
  > `tracing::info!` for image loading from cache, image already pending, and image fetch start, each including the image URL (packages/blitz-dom/src/layout/damage.rs:583-584; packages/blitz-dom/src/layout/damage.rs:592-593; packages/blitz-dom/src/layout/damage.rs:598-599)
  (.andromeda/obs-plan.md:247)
  > The SVG parse-failure event carries the element's full outer HTML in field `html` (packages/blitz-dom/src/layout/construct.rs:454; packages/blitz-dom/src/layout/construct.rs:472-477)
- (.andromeda/obs-plan-detail/log-coverage.md:85) `node/element.rs`'s property warnings, reached by `53e9d81c6`, stay gated with their value field recorded as logged as-is, and `node/node.rs`'s `print_tree` stays the one stdout dump of that conflicted file.
  > warn "Unsupported property" with field `property`, and warn "Invalid property value" with fields `property` and `value` (packages/blitz-dom/src/node/element.rs:797-812; packages/blitz-dom/src/node/element.rs:863-866)
  (.andromeda/obs-plan-detail/log-coverage.md:86)
  > `Node::print_tree` writes the tree to stdout with `println!` (packages/blitz-dom/src/node/node.rs:1012-1026)
  (.andromeda/obs-plan.md:250)
  > The CSS property value is logged in the "Invalid property value" warning (packages/blitz-dom/src/node/element.rs:810-811)
- (.andromeda/obs-plan-detail/log-coverage.md:104) dioxus-native-dom's mutation debug sites are re-read where `7fc9ee3af` edits the bridge, and where a take of `286874cc1` replaces upstream's side of it every site of the replaced writer is read again for its fields.
  > Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:141-227; packages/dioxus-native-dom/src/mutation_writer.rs:327; packages/dioxus-native-dom/src/mutation_writer.rs:410)
  (.andromeda/obs-plan-detail/log-coverage.md:105)
  > Asset fetch success is logged at trace and failure at warn; fetches without a net provider are logged at warn (packages/dioxus-native/src/assets.rs:47-60)
  (.andromeda/obs-plan.md:251)
  > dioxus-native debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:172; packages/dioxus-native-dom/src/mutation_writer.rs:224; packages/dioxus-native-dom/src/mutation_writer.rs:410; packages/dioxus-native/src/assets.rs:48)
- (.andromeda/obs-plan-detail/log-coverage.md:29) Where a take carries `d63ed2c8c`, the browser app's logged events are re-read and their record moves with it, the app staying outside escher's sink.
  > Document loading logs info on success and error on failure (apps/browser/src/document_loader.rs:121; apps/browser/src/document_loader.rs:152)
  (.andromeda/obs-plan-detail/log-coverage.md:30)
  > Script prefetch failures and JS errors log at error (apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244)
  (.andromeda/obs-plan.md:243)
  > Visited URLs are logged at info and urlbar text at warn (apps/browser/src/document_loader.rs:121; apps/browser/src/toolbar.rs:130)
- (.andromeda/obs-plan-detail/log-coverage.md:115) Where a take carries `a7ea9631b` and `b5b163450`, the publish builds' logging and its repository guard are re-read on the moved workflow.
  > Publish builds log at `CARGO_LOG: info` with `--verbose --trace`, on upstream `DioxusLabs/blitz` only — the `release-cli` job is repository-guarded (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:153)
  (.andromeda/obs-plan.md:285)
  > Publish builds log at `CARGO_LOG: info` with `--verbose --trace`, on upstream `DioxusLabs/blitz` only (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:153)
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:41) The conflicted layout files and the script runtime's split must take in no call-site scrub silently, each taken site's content judged by the sink alone, as the slices' readings record.
  > observed absent — scrubbing or redaction · searched: `scrub|redact|sanitize` over the 8 slice files (layout slice)
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:44)
  > observed absent — redaction or scrubbing · searched: `redact|scrub|sanitiz|mask` over the 32 slice files (blitz-vibey-script / blitz slice)
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40)
  > observed absent — redaction or scrubbing of logged values · searched: `redact|scrub` over the 17 listed s06 files
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:38)
  > observed absent — redaction or scrubbing · searched: `redact|scrub|mask` (case-insensitive) over the 86 slice files; hits are a comment and CSS `mask-image` only (apps slice)
- (.andromeda/obs-plan-detail/ci-integration.md:19) If the take moves a reading the Counter's kept answers hold, the committed checks are rewritten by `record_checks.sh` alone and keep the row's limits — no typed text, host path, name, secret or `screenshot` step — never loosened to pass `stand_replay`.
  > `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json`, two kept checks in git (mode `100644`, a public repository) — the repository's own scripted stand-Counter sessions, written by `examples/seven_guis/tests/flows/record_checks.sh` (which prints only `step N`), holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls, no typed text, host path, name, secret or `screenshot` step — read by `stand_replay` and seven_guis' freshness check; no log or event line; uploaded by no CI leg
  (.andromeda/obs-plan.md:283)
  > two kept checks are committed in git, holding no typed text, host path or secret.

## Bindings
- **log-coverage's sites in the reached files ↔ pii-scrubbing-compliance's allowlist**: every log site the take carries or moves (blitz-dom, blitz-shell, blitz-vibey-script, dioxus-native-dom) is judged by the engine-target rule, only the safe fields printing, and a new target outside both prefix sets is dropped whole (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19)
- **log-coverage's escher-driver lines ↔ pii-scrubbing-compliance's capture readings**: the span counts and the 0-needle readings are one re-measurement, `host_log` and `mcp_log` at `trace` under both builds (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10)
- **log-coverage's `docs/capabilities.md` restatement ↔ pii-scrubbing-compliance's past-scrub limits**: a moved limit (`log.file` on a bridged `js_console` record, the unscrubbed upstream apps) moves both records together, the published statement by hand (.andromeda/obs-plan-detail/log-coverage.md:58)
- **log-coverage's blitz-vibey-script sites ↔ the body's embedder drain and §7's script error surface**: the error and console sites, `take_js_errors` and `take_messages` move together with the runtime's split (.andromeda/obs-plan.md:111)
- **log-coverage's apps/browser and publish-build emitters ↔ pii-scrubbing-compliance's reach**: both stay upstream-only and outside escher's scrub where a take carries them (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:26)
- **ci-integration's committed checks ↔ the test plan's `stand_replay`**: a reading moved by a Taffy, Stylo or winit take reaches the kept answers the replay compares (.andromeda/obs-plan.md:283)
