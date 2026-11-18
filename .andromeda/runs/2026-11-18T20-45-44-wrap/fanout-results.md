# Fan-out results — 2026-11-18-cold-agent-live-run

architecture: add 1 · set 0 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 0 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: none · ops-layout-templates.md — reason: no code line moves and no surface is added; every row stands true
test-plan: add 4 · set 0 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 1 · set 1 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md — reason: no UI element and no a11y or obs schema touched

ops-architecture.md
decision/cold-agent-result-stated-measured · apply — check 1: routine (Accurate this-chunk addition); the founder's F2 «Измеренное, с досягаемостью (Recommended)» (relayed verbatim by the overseer, inputs#I1, named again by the wrap directive inputs#I6) settles it — the measured instance with its reach

ops-security-plan.md
decision/credential-probe-reads-valued-keys · apply — check 1: routine (Accurate this-chunk addition); the operator's ruling (inputs#I4, «Stop here, surfaced») and the revision of the probe (inputs#I5) settle it — a credential probe reads a key carrying a value
decision/cold-agent-live-run-fired-once · reject — check 2: the fact's home is test-plan decision/cold-agent-live-run-fired-once (ops-main-test-plan.md), the harness's live run

ops-test-plan.md
decision/cold-agent-live-run-once-per-version · reject — check 3: «once per version» reaches past the founder's F3 «Один раз, лимит $3 (Recommended)», ruled for this version's one run; main's ops-main-test-plan.md states it as ruled
decision/credential-probe-reads-key-with-value · reject — check 2: the fact's home is security-plan decision/credential-probe-reads-valued-keys
decision/live-run-evidence-byte-faithful · apply — check 1: routine (Accurate this-chunk addition); the operator's ruling settles it (inputs#I4: «keep live-transcript.jsonl byte-faithful, change nothing in it»)
decision/capability-statement-states-measured-live-run · reject — check 2: the fact's home is architecture decision/cold-agent-result-stated-measured

ops-obs-plan.md
decision/cold-agent-transcript-unscrubbed · reject — check 2: the fact's home is test-plan decision/live-run-evidence-byte-faithful; this row is the raw transcript under target/, not the kept masked evidence
decision/credential-probe-reads-valued-key · reject — check 2: the fact's home is security-plan decision/credential-probe-reads-valued-keys

ops-main-test-plan.md
decision/cold-agent-live-run-fired-once · apply — check 1: routine (Accurate this-chunk addition); the founder's F3 «Один раз, лимит $3 (Recommended)» (relayed verbatim by the overseer, inputs#I1, named again by inputs#I6) and the operator's take-up directive (inputs#I2) settle it
