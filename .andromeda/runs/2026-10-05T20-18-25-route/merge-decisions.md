# Merge decisions — escher-0.1.0

_Phase 2 note: the design validator's first pass failed [REASONS_CONCISE] (one 3-sentence reason). It was retried once and passed._

a11y Insert "A11y CI leg reached" (Epoch 1) · adjusted · folded into the Foundation CI-legs chunk as "named a11y leg". The existing a11y tests already run on the base pipe, so this is a base-CI leg, not a separate grows-with-code gate.
a11y Insert "Stand contrast harness" (Epoch 6) · adjusted · placed first in Polish after the screenshot it reads pixels from; the SC id is dropped from the harness line (the assertions chunk names it); cites a11y-plan §6 (bootstrap contrast-verification-harness-setup)
a11y Insert "Stand keyboard harness" (Epoch 6) · applied · a11y-plan §3/§5 record keyboard tests absent; placed before the assertions
a11y Rewrite "Stand a11y assertions" (names SC ids, gating merges) · applied · the SC ids come from the intent's a11y tier answer; the per-plan citation is dropped to stay within 25 words
obs Insert "Telemetry bootstrap" (Epoch 1) · adjusted · merged with the obs scrub-layer Insert into one Foundation chunk; keeps logs off stdout (they would collide with CLI JSON / MCP stdio)
obs Insert "Log scrubbing layer" (Epoch 1) · adjusted · merged into Telemetry bootstrap ("scrub layer"); also covers security bootstrap logging-redaction-wire
obs Rewrite "Driver diagnostics" → per-command spans · applied · retitled "Driver command spans" because SDK and scrub now live in Foundation
obs Rewrite "MCP surface" (tool call parents driver trace) · adjusted · condensed to "tool call parents its trace" and merged with the security MCP rewrite
obs Rewrite "Fork CI reached" (failing-run logs/snapshots as artifacts) · adjusted · merged with the tests artifact rewrite as "failure artifacts uploaded"
security Rewrite "Fork CI reached" (signing/cross-repo-token jobs excluded) · adjusted · condensed to "signing-secret jobs excluded"
security Rewrite "Supply-chain and coverage legs" (pinned actions, least-privilege tokens) · applied · chunk retitled "CI gate legs" because it now also carries the a11y leg
security Rewrite "Snapshot state fidelity" (password + file-path values masked) · adjusted · kept "password values masked" only; no stand task has a file input, and 25 words is the limit
security Rewrite "Command and refusal schema" (malformed args refused at boundary) · applied · per security-plan §Input Validation
security Rewrite "MCP surface" (local to invoking user, no listener, no auth surface) · applied · replaces bootstrap auth-scaffolding-baseline (no served API per §API Security)
tests Insert "Stand test contract" (Epoch 1) · applied · test-plan §3 5-command contract is NOT YET MEASURED; placed after Headless stand, before the cold-agent pipe
tests Insert "Quality gates" (Epoch 6) · applied · test-plan §10; placed before the cold-agent test, which the intent puts last
tests Rewrite "Fork CI reached" (test results + WPT diff as artifacts) · adjusted · merged into "failure artifacts uploaded"
tests Rewrite "Headless stand" (fixed viewport, bundled fonts, no live network) · applied · merged with the design stand rewrite
tests Rewrite "Settle detection" (on virtual time, no real-time sleeps) · adjusted · kept the intent's outcome "passes with no sleep" and dropped "virtual time": it is mechanism, and intent §Out lists "virtual time everywhere" for a later version
tests Rewrite "Headless screenshot" (fail never skip without fonts) · applied · merged with the design bundled-font rewrite
design Rewrite "Headless stand" (TaskShell, fixed viewport/scale/light) · adjusted · merged with the tests rewrite; dropped "scale and light scheme" for word count (fixed viewport kept; layout-templates §IA notes carry the defaults)
design Rewrite "Headless screenshot" (bundled font, independent of host fonts) · applied · merged with the tests rewrite
design Rewrite "Driver CLI" (uncoloured JSON on stdout, diagnostics on stderr) · applied · per design-system §Surface: cli

Totals: applied 12 · adjusted 11 · rejected 0 · deferred 0
