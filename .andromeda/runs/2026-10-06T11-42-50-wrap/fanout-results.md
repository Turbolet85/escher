# Fan-out results — 2026-10-06-project-readme

Seven Explore doc-agents, one parallel batch, prompt from amendment-flow.md §Fan-out sent verbatim (contracts line for
test-plan, obs-plan, a11y-plan; architecture `NOT MIGRATED`, security-plan / design-system / layout-templates carry no
keyed contracts — line dropped). Returns were YAML with `#` comment lines only; stripping removed the comments (their
substance below); no entity escapes in any return (no `<` `>` `&` in a value). No raw twin warranted: every
`proposals: []` return parsed unchanged as YAML (the comments are YAML comments).

| doc | verdict | stripped substance |
|---|---|---|
| architecture | proposals: [] | D-arch-resources / D-arch-decisions: no new resource, dependency or decision; named the plan's two expected amendments as outside both detectors' reach (left to the orchestrator) |
| security-plan | proposals: [] | no new input surface; the contact address is not a secret under §Secret Management; no dependency |
| design-system | proposals: [] | README.md is not product UI; tokens n/a |
| layout-templates | proposals: [] | no user-facing surface; rdme's layout entry is a different subject |
| test-plan | proposals: [] | no new code path; runner unchanged; harness unchanged; 444 · 0 · 5 matches §9 |
| obs-plan | proposals: [] | no operation, logger or log field; the address is page content, not a log line |
| a11y-plan | proposals: [] | no interactive element; neither schema changed |

## Validate

- Check 1 (playbook) · 2 (cross-contradiction) · 4 (absence) — no detector proposal to judge.
- Check 3 (intent-consistency) — the report's deviations: the contact line (address as link text and mailto target, "open to AI-related work") is justified by the operator's word quoted in the report; the agent-run operator pass and the hygiene rewrite of a phase artifact are directed / contract-sanctioned. Scope record: none (`gate.py scope` clean). No unjustified divergence → no escalation.
- Check 5 (expected amendments) — raised by the orchestrator:
  - **E1** architecture §Conventions → Licensing exceptions: add `stylo_taffy` `license = "MIT OR Apache-2.0 OR MPL-2.0"` (packages/stylo_taffy/Cargo.toml:3). Report substantiates (Changes → The README's content, License; expected-amendments line with the sweep: `\bMPL` arch 1 hit, a different subject; 0 masters state stylo_taffy's licence). Playbook "Accurate this-chunk addition" (the README states the fact this chunk) + the P5-approved entry names the change itself → **apply** (routine).
  - **E2** architecture §Project Intent: the root `README.md` is escher's own front page; at an Upstream sync an upstream change to it resolves to escher's version. Report substantiates (Changes → Files, The README's content; sweep: `README` 0 hits in §Project Intent). Same rule + entry names the change → **apply** (routine).
- Check 6 (disproved claims) — the report's bullet reads none; nothing to dispose.

Escalations: 0.
