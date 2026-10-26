# Merge decisions — escher-0.3.0 route Phase 3

28 suggestions, all Rewrites; no Insert, so the master-route already-delivered check had nothing to judge. Each
chunk stays at 25 words or fewer, so same-direction rewrites of one chunk are folded into one compact phrase.

- security Rewrite (The UI graph: masked state, no `NodeId`) · adjusted · folded with obs's "never logged" into "values masked, never logged"; no-`NodeId` holds by the driver session contract (arch §Standard Contracts) and is left out for length
- security Rewrite (What can I do here: the actionable-key crossing) · adjusted · "an operator-asked crossing", per residual line 12; the `NodeId` clause left to the same contract
- security Rewrite (Start in a state: a bounded `validate` argument) · adjusted · "a bounded validated argument"
- security Rewrite (Auto-exploration: owner-only medium, navigation and network a boundary fork) · adjusted · "an owner-only replayable check … navigation a boundary fork"; network folded into navigation for length
- security Rewrite (Errors with the action: a ratified widening, redacted fields stay out) · adjusted · folded with obs into "scrub held; widening the founder's fork"
- security Rewrite (Layout explained: new fields per the widening rule) · applied · "new fields per the widening rule"
- security Rewrite (A component in isolation: props bounded through `validate`, a widening) · adjusted · "a validated widening", folded with design's viewport and font
- security Rewrite (Stand requirement sweep: masks in every new carrier) · adjusted · "masks … held", folded with obs, design and tests
- design Rewrite (Bundled fonts: the stand's DejaVu Sans, `fonts-unavailable` kept) · adjusted · "bundled from the stand's DejaVu Sans; `fonts-unavailable` kept"; "every generic" left to the phase (which generics is a design decision for that chunk), folded with tests
- design Rewrite (Engine debts: the select-option overlap re-read against the stand reading) · rejected · the re-pin of the stand's measured lint reading is that chunk's own work, and the sweep's "stand layout held" carries the re-read
- design Rewrite (Stand requirement sweep: stand layout held) · adjusted · "stand layout held", folded
- design Rewrite (A component in isolation: the stand's pinned viewport and bundled font) · adjusted · "at the stand's viewport and font", folded with security
- tests Rewrite (Upstream sync ahead of engine debts: per-OS tallies re-read) · applied · in the compact form that also holds v030-03 and a11y's leg
- tests Rewrite (What can I do here: capability-statement pin kept green) · rejected · the fast `ci-scripts` leg already turns the verb-adding chunk red until its pin is updated (test-plan §4); a gate that enforces itself needs no route text, and the chunk is at the word limit
- tests Rewrite (Handler coverage: capability-statement pin kept green) · rejected · same reason as above
- tests Rewrite (Auto-exploration: no screenshot step, every CI OS, pin) · adjusted · "no shot" kept (test-plan §2 Pixel tests); the pin rejected as above; every-OS replay follows from no shot step and is left to the phase for length
- tests Rewrite (Bundled fonts: linux and matrix legs read one state, never shot bytes) · adjusted · "every CI OS reads one state, never compared by shot bytes"
- tests Rewrite (Stand requirement sweep: agent-run `run stand` and `run all` re-counted) · adjusted · "agent-run re-counted"
- tests Rewrite (Cold-agent bar: the pipe widened past its twelve verbs, contract tests kept) · adjusted · "cold-agent pipe widened past its twelve verbs", folded with obs's span counts; "operator's login" dropped for length (security-plan §Secret Management states it)
- obs Rewrite (Errors with the action: the sink's allowlist and content scrub) · adjusted · "scrub held", folded with security
- obs Rewrite (Auto-exploration: one `command` span and one redacted panic line per step) · rejected · the `command` span per `Session::run` and the redacted panic line are standing contracts (obs-plan §4, §7) every step already keeps; restating them is mechanism, and the chunk is at the word limit
- obs Rewrite (The UI graph: no component, handler or state value logged or fielded) · adjusted · "never logged", folded with security
- obs Rewrite (Stand requirement sweep: stderr free of ids, names and values; one `tool_call` and `command` span per new verb) · adjusted · "stderr content-free"; the per-verb span pin left to the phase as mechanism
- obs Rewrite (Cold-agent bar: `server.log` span counts extended) · adjusted · "span counts with it", folded with tests
- a11y Rewrite (the five upstream syncs: the `a11y` leg green) · applied · "CI, a11y leg and audit green" in all five (Epoch 2's in its compact form)
- a11y Rewrite (Engine debts: served pages' keyboard and contrast rows) · applied
- a11y Rewrite (What can I do here: from the graph and the snapshot's roles and `enabled`, no second role mapping) · adjusted · "from graph and snapshot"; "no second role mapping" left to the phase as mechanism
- a11y Rewrite (A11y re-read: SC IDs, served pages, after engine debts and bundled fonts) · adjusted · applied with "new" dropped for length

Tally: 3 applied · 21 adjusted · 4 rejected · 0 deferred.
