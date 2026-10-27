# Task — security · 2026-10-27-upstream-sync-engine-debts

## The chunk
This chunk is the fork's eighth upstream sync. It takes in DioxusLabs/blitz's work up to the measured sha `ebacd68bd` by the take chosen at P4. There are 27 commits to take or leave: the ten left out at the last sync (git Taffy, git Stylo 0.23-pre, Dioxus 0.8 alpha with the stable-id port) and seventeen new ones. The new ones cover engine code, winit beta.4 and its clipboard API in place of arboard, git Taffy bumps, the script-runtime module split, the `file:` narrowing in the script fetcher, CSS property accessors, and upstream-only browser, publish and WPT-pin surfaces.
Conflicts are resolved where the take meets them, keeping the fork's merge surfaces intact: the changed set, the clock invariant, the select model, the hit stops and the scrolled-box bounds. Upstream's new and edited test files join as blitz-tests targets.
It re-reads the sink allowlist against every new `tracing`/`log` call site, target, print and env read in upstream's delta, with the capture checks staying at 0 needles. It re-reads each merge-surface row of residual 36 against the take, and re-reads the clock invariant on the take's tree.
It re-asserts the fast, doc, a11y and dependency-audit legs, the stand checks in both layout modes and green fork CI. It re-reads the per-OS tallies on the take's CI run.

## Reached files
- Cargo.lock · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-vibey-script/src/runtime.rs · conflict surface
- wpt/WPT_COMMIT · conflict surface
- Cargo.toml
- packages/blitz-shell/Cargo.toml
- packages/dioxus-native/Cargo.toml
- packages/blitz-shell/src/lib.rs
- packages/blitz-shell/src/window.rs
- packages/blitz-shell/src/convert_events.rs
- packages/blitz-traits/src/events.rs
- packages/blitz-traits/src/shell.rs
- packages/blitz-dom/src/events/driver.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/node/element.rs
- packages/blitz-dom/src/node/text.rs
- packages/blitz-dom/src/stylo.rs
- packages/dioxus-native-dom/src/dioxus_document.rs
- packages/blitz-vibey-script/src/dom/element.rs
- packages/blitz-vibey-script/tests/dom.rs
- .github/workflows/publish-browser.yml
- tests/blitz-tests/tests/all.rs · edits its `mod` lines for upstream's new test files
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs
- tests/blitz-tests/tests/clipboard.rs
- tests/blitz-tests/tests/inline_box_scrollable_overflow.rs
- tests/blitz-tests/tests/display_contents.rs
- tests/blitz-tests/tests/pre_overflow_scroll.rs
- .github/scripts/ci-leg.sh · edits the coverage leg's two `keyboard_compat` ignore-filename lines, where the take carries `286874cc1`
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target list pin
- packages/escher-telemetry/src/format.rs · re-reads the sink target allowlist
- deny.toml · re-reads the dependency audit's ignored IDs and licence table

## Needs
- Dependency audit: after the take, cargo-deny advisories and licences must read green over the whole workspace's graph, with `--locked` and the committed `Cargo.lock`. Every ignored ID stands as recorded. A crate that enters is reviewed by hand and licence-checked. `arboard` leaves the graph and winit's lock entries move where the take carries them.
- Pinning: the recorded state is no git dependency in the root manifest, with taffy a registry "0.15" and registry Stylo, Dioxus and winit. Upstream's manifest at the sha names git Stylo, git Taffy and a Dioxus alpha, so the take fork decides whether this record holds or moves. The coupled pins move as one set or not at all.
- The pinning record of each upstream sync: it says how the sync took upstream's dependency set and how the lock was read name by name. This sync adds one more instance to that record.
- Script execution boundary: the default ScriptFetcher accepts only `file:` and `data:`. Upstream's `cee6b8731` lets only a `file:` document load a `file:` URL. That is a narrowing to read against the recorded fetcher rule.
- Script runtime split (`e6f4615ec`): runtime.rs moves into modules. The plan's citations into runtime.rs then move: JS console output to the `js_console` target at debug, uncaught JS errors through `tracing::error!`, eval/eval_module logging without propagating, and module resolve failures becoming TypeErrors.
- JS API arguments: the native APIs check or coerce their arguments, and fetch() takes GET or HEAD only. `efdc5472a` adds CSS property accessors on CSSStyleDeclaration, and `selectorText`/`cssText` assignment never throws.
- Sink allowlist (escher-telemetry): engine targets print only safe fields, `escher_*` targets have content-named fields redacted, and every other target is dropped whole. The two new `tracing::error!` sites must be read against this rule: the failed clipboard set in blitz-shell and the uncaught JS error in blitz-vibey-script. The standing capture checks stay at 0 needles.
- Engine telemetry stays behind each crate's `tracing` feature with a no-op fallback, and no new user-content log field is added. Upstream's delta is read for new call sites, targets, prints and env reads.
- Environment values read: the delta is read for any new env read. The recorded list must stay complete.
- Clipboard (`527394724`): the recorded local-user-data path is clipboard text read and written through arboard behind the `clipboard` feature in blitz-shell. Its error record is `ClipboardError`, a unit struct, with arboard's constructor unwrapped. Both change to winit's clipboard API, and the `clipboard` feature leaves the defaults. The copy path's discarded write result in keyboard.rs stays as it is.
- Shell errors and IME (`6587e7e79`, `a09f46792`, `7fc9ee3af`): unsupported winit variants fall back and log under `tracing`. The IME update and window drag results are discarded. Event-loop and window setup panic on failure. Each recorded line in the blitz-shell files moves.
- Hit testing (`ebacd68bd`): an element with no computed styles is hit-tested. This meets the recorded panic path `layout_style()` panics without computed styles, in node.rs. The fork's hit stops must keep their recorded reach: a box that clips, `contain: paint`, an `inert` pass-through and a dropped node skipped.
- Graceful degradation in blitz-dom: hit testing returns early with no DOM, stale node ids resolve to None through `get`, and stale layout children are skipped. NodeId stays a versioned slot id across the conflict resolutions.
- Scrolling (`0980cce5f`): scroll targets stay clamped to the scroll range, and a programmatic scroll to a missing node does nothing. `scroll_into_view` and the scrolled-box bounds keep their recorded behaviour.
- Layout values and layout panic paths: degenerate aspect ratios are discarded, inline-box heights are kept finite, and `text_extent` stays read-only. Several recorded panic paths cite construct.rs and inline layout: the `unreachable!()` sites, and a table root without styles.
- Markup attributes read from remote pages: these are the range and select controls held within their rules (the select's typeahead buffer is never carried out), `disabled` and `inert` by presence, and Enter/Space activation. The rules live in mutator.rs and the event files. A conflict resolution must leave them unchanged.
- The changed set and the document clock, held in mutator.rs and scrolling.rs: no `Instant::now()` or `SystemTime::now()` stands in blitz-dom outside clock.rs. That is re-read on the take's tree.
- File input path: a chosen file's path is stored in the `value` attribute (events/pointer.rs), and the snapshot reads it as `MASKED_VALUE`. The masking must hold after pointer.rs moves.
- Stable element id and masked snapshot values in dioxus-native-dom: `7fc9ee3af` edits dioxus_document.rs, and `286874cc1` would port the id derivation onto Dioxus 0.8's vdom. Ids must stay collision-free and leave only by the ratified carriers, with no `NodeId` crossing.
- The two committed stand-Counter checks hold their kept answers whole, and they are replayed in the fast leg. A take that moves layout numbers may move a kept answer. That is re-measured and re-recorded by its script, never loosened. A committed check stays free of typed text, host paths, names, secrets and `screenshot` steps.
- publish-browser.yml (`a7ea9631b`, `b5b163450`): its jobs that reach signing secrets stay repository-guarded to `DioxusLabs/blitz`. Signing material written to disk is still removed in an `always()` step, so the fork still produces no signed artifact.
- Lockfile verification: every ci.yml cargo leg and the leg script keep `--locked`. Where the take carries `286874cc1`, the coverage leg's ignore lines are edited.
- Surface registry: no new crate, listener, port or env var of ours. P3 re-reads the manifests and the lock for a `members` change or a new package, and the plan's surface statements stay as they read.

## Standing conventions
- S1 · A new log site in engine code sits behind the crate's `tracing` feature with a no-op fallback, prints only safe fields and adds no user-content field · .andromeda/security-plan-detail/logging-monitoring.md
- S2 · A new dependency is reviewed by hand and run through the audit's advisory and licence checks. An advisory is fixed by a semver-compatible update or ignored by ID with a written reason, never a blanket allow · .andromeda/security-plan-detail/dependency-security.md
- S3 · No secret in code, fixtures, logs or committed evidence · .andromeda/security-plan-detail/secret-management.md
- S4 · A new env read, a test-only one included, is registered in the environment-values list before it lands · .andromeda/security-plan-detail/secret-management.md
- S5 · A new input path keeps parse failures as typed `Result`s, never panics · .andromeda/security-plan-detail/input-validation.md
- S6 · A test name or assertion message holds no element id, accessible name or control value · .andromeda/security-plan-detail/input-validation.md

## Not needed
- The driver session socket, its wire, state directory, idle expiry and connection bounds. The chunk changes no escher-driver code or carrier.
- The MCP server's framing, request and result bounds, echoed id and protocol errors. These are unchanged.
- The driver command schema, its validation, refusal causes and remedies, and the command and tool_call spans. No verb, argument or cause moves.
- Session recording: the recording directory, a kept check's creation and bounds, and `Check::from_bytes`'s strict reader. These are unchanged.
- The cold-agent pipe: its agent-written check, its model-provider crossing and the operator login it uses. The chunk runs no live agent session.
- Authentication and authorization. None exists, and the take adds none.
- CLI argument handling of the agent-run, apt-install and cold-agent scripts, and of the screenshot, paint_bench, preact_script and multi_doc examples. These are untouched.
- rdme's markdown rendering with raw HTML passed through. rdme is untouched.
- Outbound request controls and TLS in blitz-net: per-host concurrency, headers, the cache and cookies. blitz-net is untouched.
- Fixture pages' subresource integrity and vendored Preact. These are untouched.
