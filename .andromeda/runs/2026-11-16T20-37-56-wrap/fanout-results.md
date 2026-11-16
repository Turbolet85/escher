# Fan-out results — 2026-11-16-error-text-in-the-answer

a11y-plan: add 0 · set 1 · retire 0 · none 0 · ops-a11y-plan.md
architecture: add 9 · set 5 · retire 0 · none 1 · ops-architecture.md
design-system: add 3 · set 0 · retire 0 · none 2 · ops-design-system.md
layout-templates: add 3 · set 0 · retire 0 · none 0 · ops-layout-templates.md
obs-plan: add 6 · set 9 · retire 1 · none 1 · ops-obs-plan.md
security-plan: add 6 · set 5 · retire 2 · none 1 · ops-security-plan.md
test-plan: add 6 · set 3 · retire 0 · none 1 · ops-test-plan.md
main: security-plan set 1 · ops-main-security-plan.md (check 5: `decision/no-screen-content-in-logs`, an expected amendment no proposer took)

Check 5, the plan's expected amendments no operation moves, each read whole: architecture `decision/direct-engine-changes-flagged-upstreamable` (a general rule, true of the six sites and `TRACING_COMPILED`) · `decision/driver-spans-only` and `decision/escher-driver-spans-ungated` (the driver still emits no event and installs no subscriber; the capture's hook is a panic hook, named on `decision/driver-library-side-effect-free`) · `decision/accessibility-feature-gating` (already names `boundary` ungated) · `contract/dioxus-native-dom-api/root-error-mark` (still true; `boundary_marks` added beside it) · security-plan `decision/engine-target-field-allowlist` (the sink's engine rule unchanged) · test-plan `contract/session-recording/exploration-checks` (an explored path still ends panicked with no payload; `explore.rs` unchanged) — no change owed on each.
Check 6: Spec claim disproved 1 (`password` not in `CONTENT_FIELDS`) — disposed by security-plan `decision/escher-target-content-scrub`, `decision/escher-target-content-fields-withheld` and obs-plan `decision/escher-target-content-named-set`; claim 2 (`escher_telemetry::subscriber` private) — disposed by architecture `contract/telemetry/subscriber`.
Citations: 1 row, a curation home — P3.

ops-a11y-plan.md
decision/password-never-revealed · reject — check 4: the set claims a typed password reaches no caused answer, while a caused message holds what its author wrote (security-plan `decision/mask-holds-on-every-carrier`, this wrap's set); the standing row stays true

ops-architecture.md
contract/acting-verb-results/caused · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the error text crossing every carrier; the founder's word inputs#I1 · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/driver-library-side-effect-free · apply — check 1: routine, the plan's expected amendment names the chaining hook as the stated exception
decision/no-node-id-across-session · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the list of what crosses gains the caused text · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/no-echo-of-typed-input · escalate → Q1 — check 1: boundary widening (rul-izni57qv), an author's message may echo typed text · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
contract/answer-json/outcome · apply — check 1: a closed field inside a ratified carrier (rul-cepujoy6)
contract/acting-verb-results/caused-past · apply — check 1: closed words and counts inside a ratified carrier (rul-cepujoy6); the home of the answer's bound
contract/telemetry-capture/sources · reject — check 2: the fact's home is obs-plan decision/capture-framework-targets-warn-up
contract/telemetry-capture/scrub · reject — check 2: the fact's home is obs-plan decision/scrub-by-field-name
contract/telemetry-capture/bounds · apply — check 2: the home of the capture's bounds (1024 bytes, 8 fields, 16 records a source); obs-plan's and security-plan's restatements rejected
contract/dioxus-native-dom-api/boundary-marks · apply — check 5: the plan's expected amendment beside root-error-mark; the home of the registry's growth
contract/telemetry/subscriber · apply — check 6: Spec claim disproved 2 disposed
contract/blitz-dom-crate-root/tracing-compiled · apply — check 5: the plan's expected amendment (`TRACING_COMPILED`)
decision/engine-log-values-in-named-fields · reject — check 2: the fact's home is obs-plan decision/engine-sites-fixed-messages
decision/mcp-result-text-decoded-whole · reject — check 2: the fact's home is test-plan decision/mcp-result-decoded-whole

ops-design-system.md
contract/acting-verb-answer/caused · reject — check 2: the fact's home is architecture contract/acting-verb-results/caused
contract/acting-verb-answer/caused-past · reject — check 2: the fact's home is architecture contract/acting-verb-results/caused-past
contract/acting-verb-answer/caused-redaction · reject — check 2: the fact's home is obs-plan decision/scrub-by-field-name

ops-layout-templates.md
contract/escher-session-acting-verbs/caused · reject — check 2: the fact's home is architecture contract/acting-verb-results/caused
contract/escher-session-acting-verbs/caused-past · reject — check 2: the fact's home is architecture contract/acting-verb-results/caused-past
decision/mcp-result-text-decoded-whole · reject — check 2: the fact's home is test-plan decision/mcp-result-decoded-whole

ops-obs-plan.md
decision/headless-stand-no-sink · apply — check 5: the plan's expected amendment, `stand_act_error_text`'s exception
decision/checks-silent · apply — check 1: routine, the same exception on the silent-checks row
decision/answers-echo-nothing-typed · escalate → Q1 — check 1: boundary widening (rul-izni57qv), an author's message crossing in caused · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/driver-library-installs-no-subscriber · apply — check 1: routine, the capture's hook named
decision/driver-emits-no-event · apply — check 1: routine, the count-only capture retired (report Retired)
decision/capture-framework-targets-warn-up · apply — check 5: the plan's expected amendment, extended to the engine
contract/telemetry-scrub-set/error · escalate → Q1 — check 1: boundary widening (rul-izni57qv), «no other carrier» restated for caused · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
contract/telemetry-scrub-set/url · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the field's crossing in caused · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/escher-target-content-named-set · apply — check 6: Spec claim disproved 1 disposed (`password`)
decision/scrub-at-sink-not-call-sites · apply — check 1: routine, retired with its heir; the report's Retired bullet and Schema / config
decision/scrub-by-field-name · apply — check 2: the home of the field-name scrub for the sink and the capture
decision/engine-sites-fixed-messages · apply — check 5: the plan's expected amendment, obs-plan §8's engine messages fixed text
decision/capture-keeps-bounded-records · reject — check 2: the fact's home is architecture contract/telemetry-capture/bounds (which states the cut in bytes, as `TEXT_CUT` does; this add says characters)
decision/capture-chaining-panic-hook · reject — check 7: not_master type 4 — the hook's build is the code's; its printing nothing stands on decision/driver-library-installs-no-subscriber (this wrap's set) and the exception on architecture decision/driver-library-side-effect-free
decision/caused-carries-error-text · reject — check 2: the fact's home is architecture contract/acting-verb-results/caused
decision/boundary-registry-document-life · reject — check 2: the fact's home is architecture contract/dioxus-native-dom-api/boundary-marks; the growth's owner is P5's residual

ops-security-plan.md
decision/caused-answer-crossing · escalate → Q1 — check 1: boundary widening (rul-izni57qv), retired for its heir · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/root-boundary-err-as-screen-content · escalate → Q1 — check 1: boundary widening (rul-izni57qv), retired for its heir · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/caused-text-crossing · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the plan's expected amendment widened to the text; the founder's word inputs#I1 · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/caused-answer-bounds · reject — check 2: the fact's home is architecture contract/acting-verb-results/caused-past and contract/telemetry-capture/bounds
decision/root-boundary-err-crossing · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the root Err's text crossing in caused · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/capture-scrub-by-field-name · reject — check 2: the fact's home is obs-plan decision/scrub-by-field-name
decision/capture-panic-hook-chaining · reject — check 2: the fact's home is obs-plan decision/capture-chaining-panic-hook
decision/engine-log-values-in-fields · reject — check 2: the fact's home is obs-plan decision/engine-sites-fixed-messages
decision/escher-target-content-scrub · apply — check 6: Spec claim disproved 1 disposed (`password`)
decision/escher-target-content-fields-withheld · apply — check 6: the count 11 → 12 (report Counts / qualifiers moved)
decision/escher-telemetry-allowlist-sink · apply — check 1: routine, the capture scrubs by the same set
decision/mask-holds-on-every-carrier · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the plan's expected amendment; a message holds what its author wrote · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/socket-carries-screen-reading · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the socket's answer gains the caused text · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»

ops-test-plan.md
contract/driver-session/acting-verbs · reject — check 2: the bound's home is architecture contract/acting-verb-results/caused-past; the standing row (caused where the step caused something) stays true
decision/socket-carries-commands · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the socket carries the caused text out · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
decision/root-boundary-err-on-own-session · escalate → Q1 — check 1: boundary widening (rul-izni57qv), the root Err's text read in caused · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-16, inputs#I5: «Применить все 14 (Recommended)»
contract/driver-session/payload-unread · apply — check 1: the program's own fixed text inside a ratified carrier (rul-fxlxu7mw)
decision/caused-fields-scrubbed-by-name · reject — check 2: the fact's home is obs-plan decision/scrub-by-field-name
decision/mcp-result-decoded-whole · apply — check 2: the home of the MCP comparison convention (a test convention); architecture's and layout-templates' restatements rejected
decision/caused-text-kept-off-sink-lines · reject — check 2: the fact's home is security-plan decision/no-screen-content-in-logs (this wrap's main set) and obs-plan decision/headless-stand-no-sink (the exception)
decision/engine-items-asserted-on-tracing-build · apply — check 5: the plan's expected amendment, test-plan §9's founder target per build through `TRACING_COMPILED`
decision/expected-value-pinned-as-literal · apply — check 1: routine, a measured control hazard (report Decisions & corrections)

ops-main-security-plan.md
decision/no-screen-content-in-logs · apply — check 5: the plan's expected amendment (`decision/no-screen-content-in-logs`), the caused text kept off every log line
