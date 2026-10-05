# Fan-out results — 2026-10-05-as-built-baseline

Report: `escher-0.1.0/chunks/2026-10-05-as-built-baseline/report.md`. Detectors 15 (arch 2 · security 3 · design 1 ·
layout 1 · tests 3 · obs 3 · a11y 2 = drift-base's 15 single-doc entries). Keyed-contract renders: test-plan · obs-plan ·
a11y-plan (1 key each); architecture `NOT MIGRATED` (line dropped).

## Verdicts
- architecture — `proposals: []` (D-arch-resources, D-arch-decisions: no new resource, dependency or stack change). Commentary stripped: names the rustdoc-gate disproval (report Spec claims disproved #1) and expected amendments 1-2 as outside its detectors → the expected-amendment channel.
- security-plan — `proposals: []` (no input surface; no auth/secret flow; no dependency added).
- design-system — `proposals: []` (no UI element; no tokens flag to read).
- layout-templates — `proposals: []` (no UI surface or region).
- test-plan — `proposals: []` (no new path; standard cargo test harness matches §3/§2/§4; harness unchanged). Commentary stripped: names expected amendment 3 (test-plan §9) as plan-carried content for the expected-amendment channel.
- obs-plan — `proposals: []` (no operation, no telemetry dependency, no logging; keyed contract untouched).
- a11y-plan — `proposals: []` (no interactive element; no schema change; keyed contract untouched).

No raw twin: every return parsed; stripping removed commentary only (summarized above), never a proposal.

## Validate
Proposal set empty. Orchestrator-raised (check 5, the plan's `Expected amendments (wrap)` floor):

- **O1** — architecture · §Stack and Technologies (Code quality row) · §Conventions (Formatting and lints) · §Infrastructure
  Patterns (Build system · CI/CD) · §Inherited Defaults (Code quality): the rustdoc `-D warnings` "doc gate" claim corrected —
  CI's docs job runs bare `cargo doc`, which documents only the lib-less root package `blitz-examples`, so no library
  crate's rustdoc is gated; the workspace form is red at baseline. Basis: report Spec claims disproved #1 + Expected
  amendments 1. Disposition: **apply** — check 1: plan entry 1 names the change ("correct the claim") → direction settles
  it; playbook "Accurate this-chunk addition" (reconciles a spec mechanism to measured reality; the fact is this chunk's
  measurement). Not a boundary widening. Sites: architecture.md:67, :104, :155, :174, :217 (`grep -n
  'RUSTDOCFLAGS\|rustdoc\|doc gates\|docs, cross'`; no line over 2 000 chars among them — `splice.py index --min-chars
  2000` lists 89/100/114/115/121/122/123/136/175/191/253).
- **O2** — architecture · §Stack and Technologies (Build environment row): extend with the measured dev-host reading.
  Basis: report Dev-tool versions + Expected amendments 2. Disposition: **apply** — check 1: plan entry 2 names the
  change ("extend … verified dev-host reading").
- **O3** — test-plan · §9 CI Integration: extend with the local baseline reading. Basis: report Baseline figures +
  Outcome gates + Expected amendments 3. Disposition: **apply** — check 1: plan entry 3 names the change ("extend").

Checks: 2 cross-contradiction — none (O1/O2 edit different rows/lines; O3 another doc). 3 intent-consistency — report
matches the entry ("workspace build and blitz-tests green on this host, one run's wall-clock recorded") and the plan
acceptance; scope record empty (`gate.py scope` clean). 4 absence-needs-evidence — O1's site set from the grep above,
re-checked by the cascade sweep. 5 expected amendments — 3/3 raised (O1-O3). 6 disproved claims — #1 → O1 (DISPOSED).

Escalations: 0.
