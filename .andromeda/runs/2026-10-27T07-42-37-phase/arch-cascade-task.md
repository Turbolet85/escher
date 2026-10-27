# Task — arch · 2026-10-27-upstream-sync-engine-debts

## The chunk
The fork's eighth upstream sync: 27 DioxusLabs/blitz commits past the merge base 23b9b26a9 (upstream/main measured at ebacd68bd) are taken or left by the take the founder's delegate picks at P4 (cherry-picks onto the registry pins, the same plus winit beta.4 and its clipboard move, a whole merge with git Stylo, git Taffy and Dioxus 0.8 alpha, or nothing).
Conflicts in the blitz-dom layout, mutator, node, scrolling and writing-mode files, the script runtime, the lock and WPT_COMMIT are resolved keeping both sides' intent: the changed set, the clock invariant, the select model, the hit stops and the scrolled-box bounds unchanged.
Upstream's new and edited test files join blitz-tests under the one-target-per-file rule where the take carries them.
It re-reads the merge surfaces of the fork's engine changes (residual lines 35 and 36), the clock invariant, the telemetry sink allowlist against upstream's new log sites, and the per-OS test tallies of the take's green CI run.
It re-runs every escher proof: the fast and doc legs, the stand checks in both layout modes, the a11y leg, the dependency audit and fork CI.

## Reached files
- Cargo.toml · merge surface
- Cargo.lock · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-vibey-script/src/runtime.rs · conflict surface
- wpt/WPT_COMMIT · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/events/pointer.rs · merge surface
- packages/blitz-dom/src/events/driver.rs · merge surface
- packages/blitz-dom/src/node/text.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-shell/src/convert_events.rs · merge surface
- packages/blitz-shell/Cargo.toml · merge surface
- packages/dioxus-native/Cargo.toml · merge surface
- packages/blitz-traits/src/events.rs · merge surface
- packages/blitz-traits/src/shell.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface
- tests/blitz-tests/tests/clipboard.rs · merge surface
- tests/blitz-tests/tests/inline_box_scrollable_overflow.rs · merge surface
- tests/blitz-tests/tests/display_contents.rs · merge surface
- tests/blitz-tests/tests/pre_overflow_scroll.rs · merge surface
- tests/blitz-tests/tests/all.rs · edits its mod lines
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target pin
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist
- packages/blitz-dom/src/clock.rs · re-reads the clock invariant
- .github/scripts/ci-leg.sh · re-reads the fast, doc, a11y and audit legs
- deny.toml · re-reads the dependency audit and its ignored IDs
- .andromeda/residuals.md · re-reads lines 35 and 36
- .andromeda/test-plan.md · re-reads the per-OS test tallies
- .andromeda/architecture.md · edits the Upstream sync line

## Needs
- The dependency-pinning decision: taffy and parley registry versions with no git dependency in the root manifest, the parser, skrifa and svgtypes couplings, winit pinned to one exact beta; a take carries upstream's pin set at the measured sha as one set or none, and a set that does not resolve is a halt, never a one-sided fix.
- The inherited framework pins the take may move or keep: Stylo 0.22.0, taffy 0.15, Dioxus 0.7.3, winit =0.31.0-beta.3, rust-version 1.91.0, against upstream's git Stylo, git Taffy, Dioxus 0.8.0-alpha.3 and winit =0.31.0-beta.4.
- The Upstream sync line of Project Intent: merge base 23b9b26a9, upstream last read at f9cbb2756, thirteen commits taken as no-commit cherry-picks with the base unmoved, the left-out Stylo 0.23-pre, Taffy-main and Dioxus 0.8 commits; it moves to the new pin and commit at the wrap where the take moves the pin.
- The merge surfaces of the fork's direct engine changes, each flagged upstreamable with no upstream pull request: the seven engine fixes, the document clock and the select model (residual line 36), re-read per row against the take, recording whether upstream moved each one's ground; no upstream pull request is opened.
- The changed-set contract over the mutator.rs conflict and upstream's new mutator paths (details :open, the no-computed-style hit): written at its four kinds of site and by no node creation, hover, scroll, resize, load, animation or layout; has_changes true until take_changed_nodes drains it.
- The document clock invariant over upstream's rewritten scrolling code: every engine time read goes through DocumentConfig.clock, and no Instant::now() or SystemTime::now() stands in blitz-dom outside clock.rs.
- The hit-test contract over the node/node.rs conflict and upstream's hit test of an element with no computed styles: a hit stops at the padding box of a box paint clips (overflow or contain: paint), passes through inert and skips a node dropped since the last resolve; Node::absolute_position subtracts only its containing blocks' scroll offsets.
- The scrolling contract over upstream's reversed scroll origins through Taffy's ScrollableOverflow: scroll_into_view writing every scrolling box on the chain then the viewport, visible_region read-only, a scrolled box's client rect never subtracting its own offset, the scrolled-box bounds in writing_mode.rs; whether the take moves the recorded transformed-scrolling-box limit is recorded for the next entry.
- The select and range-input arms and the disabled reading through the conflicts: apply_select_choice as the one choice path, RangeInputData sanitized to its grid, ElementData::is_disabled the one disabled reading, Enter and Space activation and a button click's focus.
- Box construction and layout over the construct.rs and inline/build.rs conflicts: display: contents hoisting, pseudo-elements voting in flow classification, text inputs getting control data at construction (upstream adds IME enabling there), LayoutPassState as the one implementer of Taffy's traits.
- The one incremental pipeline: incremental and non-incremental layout stay identical, every pipeline scenario re-run in both layout modes on the take's tree.
- Shell and windowing over upstream's redraw-through-event-loop and winit moves: winit's IME event copied so lower crates depend on no winit, redraws requested after mutations and skipped while critical resources are pending, the shell provider carrying clipboard writes.
- Default features and the clipboard move, where a take carries it: arboard leaves the workspace's manifests and the clipboard feature leaves blitz-shell's and dioxus-native's default lists.
- The Dioxus DOM bridge over upstream's 36 lines in dioxus_document.rs and, under a Dioxus 0.8 take, the vdom port: stable ids in four tiers written to neither DOM nor vdom, the accessibility_tree override carrying author_id, the snapshot, its text, parts and diff, unkeyed_actionable; the stable-id checks decide.
- Under a take of the Dioxus 0.8 commit, the retirement of keyboard_compat.rs and the coverage leg's two keyboard_compat ignore lines dropping with it.
- The JavaScript contract over the runtime.rs conflict and its split into modules: ScriptFetcher accepting only file: and data: (upstream narrows file: loads to file: documents, a narrowing read against that rule), report_js_error's error path, the bootstrap's CSSOM classes gaining property accessors.
- The accessibility-tree builder and the a11y target through upstream's IME, :open and hit-test commits: state attributes read by closed set, names from aria-labelledby, label and aria-label, inert subtrees left out; focus order, Enter and Space activation and contrast re-proven by their standing checks and the a11y leg.
- The telemetry sink rule read against upstream's two new tracing::error! sites (the clipboard failure in blitz-shell, the uncaught JS error in blitz-vibey-script): engine targets print only safe fields, content fields redacted, every other target dropped whole; capture checks stay at 0 needles and no new user-content log field enters.
- Stand and harness numbers that a moved Taffy, Stylo or winit can shift — exact bounds, snapshot text byte sizes against SNAPSHOT_TEXT_BUDGET, screenshot pixels, contrast rows — re-measured and recorded, never loosened to pass.
- The blitz-tests target rule: upstream's new files become targets by auto-discovery, all.rs takes their mod lines and stays test = false, and each new target joins the list the target pin asserts.
- The dependency audit over the whole workspace's graph with advisories and licences: arboard leaving, winit's lock entries moving, any entering crate reviewed by hand and licence-checked, the ignored IDs standing as recorded.
- Fork CI green on the pushed commit with every job settled, every cargo leg --locked, and the per-OS tallies re-read from the test-bearing job logs (the linux test leg, the windows and macos matrix), each moved count explained.
- Workspace membership and occupied resources: no new crate, port, env var or listener of ours; the manifests and lock re-read for a members change or new package; any one would be an Occupied Resources registration.
- Upstream-only surfaces a take may carry (apps/browser, publish-browser.yml, WPT_COMMIT): the publish, WPT and post-results jobs keep their github.repository guard to DioxusLabs/blitz so the fork never reaches the signed-build environment.
- Additivity under the design philosophy: no edit by us to upstream-owned code beyond what a conflict or a gate failure requires, each named, minimal and recorded as non-additive; an upstream change to README.md resolves to escher's version.

## Standing conventions
- S1 · each integration test file covers one behaviour, opens with a //! doc naming it, and is one blitz-tests target, upstream's all.rs declared and held at test = false · .andromeda/architecture-detail/conventions.md
- S2 · scenarios run in both layout modes · .andromeda/architecture-detail/conventions.md
- S3 · cargo fmt --all --check and clippy --workspace --locked -D warnings through ci-leg.sh · .andromeda/architecture-detail/conventions.md
- S4 · rustdoc warnings are errors; modules open with //! docs and public items carry /// · .andromeda/architecture-detail/conventions.md
- S5 · engine-crate tracing gated per call site with a cfg(not(feature = "tracing")) fallback · .andromeda/architecture-detail/conventions.md
- S6 · dependencies declared once in [workspace.dependencies] and consumed as workspace = true, in-repo crates with default-features = false · .andromeda/architecture-detail/conventions.md
- S7 · every CI leg runs as bash .github/scripts/ci-leg.sh {leg}, --locked on every cargo leg · Infrastructure Patterns (CI/CD) in the body

## Not needed
- The driver session's own contracts — the v3 wire, the socket and idle expiry, the MCP framing and bounds, the twelve-verb schema, causes and exit statuses, the kept-check format and replay: the take reaches no escher-driver file and adds no verb, carrier, field or exit.
- The cold-agent pipe and the agent-run test contract: no script under scripts/ is reached.
- The session state and recording directories on the filesystem and their owner-only modes: nothing here creates or names one.
- The browser app's history database and persistence (rusqlite, migrations, history_entries) and the inherited database default.
- Example fixture forms (the google and graphite pages) and the HTML fixture conventions.
- Nix flake packaging and the cross installation from a git rev.
- Release versioning and the named build profiles.
- The WPT runner's execution, outputs, kinds and bucketing: WPT conformance numbers are out of scope and its workflows upstream-only.
- Font loading: @font-face registration, local() and format skipping, generic family mapping.
- blitz-net networking: the fixed User-Agent, the per-host semaphore, the HTTP cache.
- The accesskit_xplat adapter API and its platform adapters.
- The headless loading loops of the screenshot and paint_bench examples.
