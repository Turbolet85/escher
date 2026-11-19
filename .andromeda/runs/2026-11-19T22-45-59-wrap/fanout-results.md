# Fan-out results — 2026-11-19-upstream-sync-ahead-of-the-debts

architecture: add 15 · set 18 · retire 4 · none 0 · ops-architecture.md (re-spawned once, SET_IN_READ; the raw first file `.raw-ops-architecture.md`)
security-plan: add 11 · set 5 · retire 2 · none 2 · ops-security-plan.md (re-spawned once, SET_IN_READ; the raw first file `.raw-ops-security-plan.md`)
design-system: add 9 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 2 · set 0 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 2 · set 1 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 0 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 5 · set 5 · retire 0 · none 0 · ops-a11y-plan.md

Opening rule: 0 operations rejected for a source the report does not carry.

Check 5 (expected amendments): engine-dependency-pins → security-plan decision/engine-dependency-pins (proposer's set) · upstream-sync-dependency-set, the two-parent merge's form → its home is architecture decision/upstream-sync-by-two-parent-merge (proposer's add); the dependency-set row stays true as it stands · stylo-prefs-set-once → architecture retire + heir decision/stylo-prefs-upstream-set · the CDP listener and its env reads → security-plan decision/cdp-server-opt-in-loopback and its two env contracts, architecture's three listener excepts and three names · test-plan §3, the new blitz-tests targets → not carried: no master names a target list (sweep `tgt` 0 master sites, its control refused unfired); the manifest holds the targets · W5–W9 → security-plan's adds and set, a11y-plan's three sets (W9).
Check 6 (disproved claims): the plan's step 4 lock claim → disposed by the operator's ruling (inputs#I6), a deviation, no master site · «every runtime event has isTrusted = true» → no master site (sweep `istrusted` 0); its fact enters by security-plan decision/event-is-trusted-by-source, the leaf by the cascade · the W8 mask reach «not measurable» (research) → measured, security-plan decision/svg-mask-url-fetch · upstream's docs/licensing.md webpki-root-certs row → upstream's file, no master site; flagged upstreamable in the handoff.

ops-architecture.md
decision/parley-text-layout · apply — check 1: routine version re-statement of the merged pin (fct-wogrtpli)
decision/stylo-css-engine · apply — check 1: routine version re-statement of the merged pin (fct-wogrtpli)
decision/winit-exact-beta-pin · apply — check 1: routine version re-statement of the merged pin (fct-wogrtpli)
decision/blitz-net-reqwest-fetching · escalate → Q1 — check 1: boundary widening by upstream merge (W3, rustls) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W3 (card confirmed by the operator, 2026-11-20)
decision/anyrender-rendering-abstraction · reject — check 7: not_master type 5 — lengthens a still-true row with a backend inventory the code and manifests hold
decision/one-apt-install-script · apply — check 1: routine CI fact, the SHA-pinned setup-ndk step (report Schema / config)
decision/accessibility-tree-refresh-on-change · apply — check 1: routine re-statement of the moved refresh site (report Symbols / APIs)
contract/accessibility-tree/platform-refresh · apply — check 1: routine re-statement of the moved refresh site
decision/a11y-rebuild-on-nonempty-changed-set · apply — check 1: routine re-statement of the moved refresh site
decision/platform-tree-rebuild · reject — check 7: not_master type 4 — the standing row is false (poll site); main retires it in ops-main-architecture.md
decision/one-local-unix-listener · escalate → Q1 — check 1: boundary widening by upstream merge (W1, the CDP listener) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
decision/session-socket-one-listener · escalate → Q1 — check 1: boundary widening by upstream merge (W1) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
decision/driver-session-unix-socket · escalate → Q1 — check 1: boundary widening by upstream merge (W1) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
contract/event-dispatch/focus-change · escalate → Q1 — check 1: boundary widening by upstream merge (W5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W5 (card confirmed by the operator, 2026-11-20)
decision/default-action-event-order · escalate → Q1 — check 1: boundary widening by upstream merge (W7) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W7 (card confirmed by the operator, 2026-11-20)
decision/handlers-before-default-action · escalate → Q1 — check 1: boundary widening by upstream merge (W7) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W7 (card confirmed by the operator, 2026-11-20)
decision/user-scroll-chaining · apply — check 1: routine re-statement, upstream's overscroll-behavior taken (fct-wpucxmhz)
decision/user-scroll-chains-to-viewport · apply — check 1: routine re-statement, upstream's overscroll-behavior taken
decision/dispatch-event-callback · apply — check 1: routine retire; the event-queue model replaced the callback (report Symbols / APIs, Retired)
decision/stylo-prefs-set-once · reject — check 7: its heir decision/stylo-prefs-upstream-set read not_master; main retires the row with no heir in ops-main-architecture.md
decision/deferred-stylo-dioxus-bumps · apply — check 1: routine retire; Stylo taken, Dioxus held (P5 card row 1, inputs#I3)
decision/upstream-taken-by-cherry-pick · apply — check 5: the plan's two-parent merge form (P5-approved entry head), its heir the merge rule
decision/events-queued-on-document · apply — check 1: routine heir of the retired callback convention
decision/stylo-prefs-upstream-set · reject — check 7: not_master type 4 — upstream's prefs set is a mechanism the code holds
decision/dioxus-0-8-move-held · apply — check 1: the P5 card's row 1 (inputs#I3), the operator's word «yes»
decision/upstream-sync-by-two-parent-merge · apply — check 5: the plan's two-parent merge form; the home of the merge-form expected amendment
decision/cdp-devtools-server-opt-in · reject — check 2: the fact's home is security-plan decision/cdp-server-opt-in-loopback
name/other/blitz-cdp-port · escalate → Q1 — check 1: boundary widening by upstream merge (W1, an env read) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
name/other/blitz-devtools-frontend-dir · escalate → Q1 — check 1: boundary widening by upstream merge (W1, an env read) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
name/crate/blitz-vibey-cdp-server · escalate → Q1 — check 1: boundary widening by upstream merge (W1, the crate) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
decision/is-trusted-by-dispatch-source · reject — check 2: the fact's home is security-plan decision/event-is-trusted-by-source
decision/inline-box-placeholder-fallback-family · apply — check 1: no rule governs it (rul-47a6zcoz's precondition, a founder fork at P4, fails — none was put); the operator's recorded direction settles it, his technical ruling (inputs#I8, inputs#I9)
decision/cancelled-pointerup-still-clicks · reject — check 2: the fact's home is security-plan decision/cancelled-pointerup-still-clicks
decision/visibility-collapse-hidden-from-a11y · reject — check 2: the fact's home is a11y-plan decision/hidden-content-excluded and its two sibling rows
decision/svg-mask-url-stamped-fetch · reject — check 2: the fact's home is security-plan decision/svg-mask-url-fetch
decision/skia-built-by-default · reject — check 2: the fact's home is security-plan decision/skia-default-build-fetch
decision/wheel-gesture-axis-lock · reject — check 7: not_master type 4 — one mechanism's behaviour that the code and its wheel_axis_lock test hold

ops-security-plan.md
decision/session-socket-sole-listener · escalate → Q1 — check 1: boundary widening by upstream merge (W1) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
decision/accessibility-tree-refresh-on-change · apply — check 1: routine re-statement of the moved refresh site
decision/presentational-hints-typed-parse · reject — check 7: not_master type 5 — lengthens a still-true row with an attribute inventory the code holds; escalated (Q1), not asked
decision/engine-dependency-pins · apply — check 5: the plan's expected amendment (fct-wogrtpli)
decision/upstream-sync-dependency-set · reject — check 3: the four lock updates are the operator's technical ruling recorded as a deviation (inputs#I6, inputs#I9), not a change of the standing rule, which already admits a lock-only semver-compatible update
decision/tls-via-native-tls · escalate → Q1 — check 1: boundary widening by upstream merge (W3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W3 (card confirmed by the operator, 2026-11-20)
decision/clipboard-via-arboard · apply — check 1: routine retire; arboard left the graph (report Dependencies)
decision/cdp-server-opt-in-loopback · escalate → Q1 — check 1: boundary widening by upstream merge (W1) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
contract/environment-variables/blitz-cdp-port · escalate → Q1 — check 1: boundary widening by upstream merge (W1) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
contract/environment-variables/blitz-devtools-frontend-dir · escalate → Q1 — check 1: boundary widening by upstream merge (W1) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W1 (card confirmed by the operator, 2026-11-20)
decision/event-is-trusted-by-source · escalate → Q1 — check 1: boundary widening by upstream merge (W6) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W6 (card confirmed by the operator, 2026-11-20)
decision/programmatic-focus-events · escalate → Q1 — check 1: boundary widening by upstream merge (W5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W5 (card confirmed by the operator, 2026-11-20)
decision/cancelled-pointerup-still-clicks · escalate → Q1 — check 1: boundary widening by upstream merge (W7) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W7 (card confirmed by the operator, 2026-11-20)
decision/svg-mask-url-fetch · escalate → Q1 — check 1: boundary widening by upstream merge (W8) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W8 (card confirmed by the operator, 2026-11-20)
decision/visibility-collapse-hidden-from-accessibility · reject — check 2: the fact's home is a11y-plan decision/hidden-content-excluded and its two sibling rows
decision/tls-via-rustls · escalate → Q1 — check 1: boundary widening by upstream merge (W3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W3 (card confirmed by the operator, 2026-11-20)
decision/clipboard-via-winit · apply — check 1: routine heir of the arboard row (report Dependencies, Schema / config)
decision/skia-default-build-fetch · escalate → Q1 — check 1: boundary widening by upstream merge (W4) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W4 (card confirmed by the operator, 2026-11-20)

ops-design-system.md
decision/inline-box-placeholder-font-fallback · reject — check 2: the fact's home is architecture decision/inline-box-placeholder-fallback-family
name/other/blitz-inline-box-placeholder · reject — check 2: the fact's home is architecture decision/inline-box-placeholder-fallback-family, which names the private family
decision/chrome-devtools-opt-in · reject — check 2: the fact's home is security-plan decision/cdp-server-opt-in-loopback
decision/skia-default-renderer · reject — check 2: the fact's home is security-plan decision/skia-default-build-fetch
decision/wheel-gesture-axis-lock · reject — check 2: the fact's home is architecture decision/wheel-gesture-axis-lock (the chaining half: architecture's user-scroll-chaining excepts)
decision/cancelled-pointer-events-keep-gestures · reject — check 2: the pointerup half's home is security-plan decision/cancelled-pointerup-still-clicks; the pointerdown half is upstream's handling held by its test pointerdown_cancelled, no master question it answers
decision/visibility-collapse-hidden-from-a11y · reject — check 2: the fact's home is a11y-plan decision/hidden-content-excluded and its two sibling rows
decision/programmatic-focus-fires-focus-events · reject — check 2: the fact's home is security-plan decision/programmatic-focus-events
contract/dom-events/is-trusted · reject — check 2: the fact's home is security-plan decision/event-is-trusted-by-source

ops-layout-templates.md
decision/devtools-inspector-opt-in · reject — check 2: the fact's home is security-plan decision/cdp-server-opt-in-loopback; the inspector is in no default build
decision/programmatic-focus-fires-focus-events · reject — check 2: the fact's home is security-plan decision/programmatic-focus-events

ops-test-plan.md
decision/rust-tests-on-built-in-harness · apply — check 1: routine; upstream's merged checks of that form (fct-bhfcjucw)
decision/upstream-defect-fixed-with-own-witness · reject — check 3: the operator's ruling (inputs#I7) settled this one defect, not a standing rule; its «Ratified» names no ratification
decision/devtools-cdp-server-upstream-tests-only · reject — check 7: not_master type 5 — a coverage inventory, not a chosen strategy; the code and the sidecar hold it

ops-a11y-plan.md
decision/a11y-tree-refresh-on-change · apply — check 1: routine re-statement of the moved refresh site
decision/accessibility-tree-through-document-method · apply — check 1: routine re-statement of the moved refresh site
decision/hidden-content-excluded · escalate → Q1 — check 1: boundary widening by upstream merge (W9) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W9 (card confirmed by the operator, 2026-11-20)
contract/accessibility-tree/hidden-and-inert-nodes · escalate → Q1 — check 1: boundary widening by upstream merge (W9) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W9 (card confirmed by the operator, 2026-11-20)
decision/hidden-content-excluded-from-a11y-tree · escalate → Q1 — check 1: boundary widening by upstream merge (W9) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-19, inputs#I4: his answer to W9 (card confirmed by the operator, 2026-11-20)
contract/accessibility-tree-hidden-content/visibility-collapse · reject — check 2: the fact's home is a11y-plan decision/hidden-content-excluded and its two sibling rows
decision/programmatic-focus-fires-focus-events · reject — check 2: the fact's home is security-plan decision/programmatic-focus-events
decision/script-events-untrusted · reject — check 2: the fact's home is security-plan decision/event-is-trusted-by-source
decision/cancelled-pointerup-still-clicks · reject — check 2: the fact's home is security-plan decision/cancelled-pointerup-still-clicks
decision/cancelled-pointerdown-counts-for-double-click · reject — check 2: upstream's pointerdown handling, held by its test pointerdown_cancelled; no master question it answers (the pointerup half's home is security-plan decision/cancelled-pointerup-still-clicks)

ops-main-architecture.md
decision/platform-tree-rebuild · apply — check 7: the correction fit's not_master owes (the standing row false); a retire takes no verdict
decision/stylo-prefs-set-once · apply — check 5: the plan's expected amendment (stylo-prefs-set-once); a retire takes no verdict

ops-main-architecture.2.md
absence/served-network-port · apply — check 6 (cascade): the amended listener claim's stale site; inside W1, confirmed at Q1 (relayed verbatim by the overseer, 2026-11-19, inputs#I4)
absence/network-listener · apply — check 6 (cascade): the amended listener claim's stale site; inside W1, confirmed at Q1 (relayed verbatim by the overseer, 2026-11-19, inputs#I4)

ops-main-security-plan.md
absence/served-network-api · apply — check 6 (cascade): the amended listener claim's stale site; inside W1, confirmed at Q1 (relayed verbatim by the overseer, 2026-11-19, inputs#I4)
overview/api-security · apply — check 6 (cascade): the amended listener claim's stale site; inside W1, confirmed at Q1 (relayed verbatim by the overseer, 2026-11-19, inputs#I4)
