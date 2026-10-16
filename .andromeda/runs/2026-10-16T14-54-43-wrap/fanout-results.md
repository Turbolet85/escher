# Fan-out results — 2026-10-16-handler-splits-and-readme (resumed window)

Path check: before `tree 7ef021cc… index 5e341031… head c71f7ed1…`, after `tree 3bd8341e… index 5e341031… head c71f7ed1…`;
`git diff-tree -r --name-status` printed exactly the seven `A {run_dir}/fanout-{doc}.md` lines; index and head unchanged.
No file needed a fix, so there is no raw twin; every receipt count equals its file's (0).

architecture: 0 proposals · fanout-architecture.md
  note: neither detector fires; the expected amendments (§Project Intent → Front page; [Scrolling and input] and Events citations, the moved-into-a-function ones needing a read) are outside the two invariants.
security-plan: 0 proposals · fanout-security-plan.md
  note: no invariant violated; the expected re-points, the moved-into-a-function ones included, are the citation pass's.
design-system: 0 proposals · fanout-design-system.md
  note: D-design-tokens holds — no new UI element, every tokens flag n/a.
layout-templates: 0 proposals · fanout-layout-templates.md
  note: D-layout-surface holds — no UI surface or region added; the README is architecture's Front page.
test-plan: 0 proposals · fanout-test-plan.md
  note: no drift; test-plan's one citation into a changed file (range.rs:303-529, lines 15 and 153) already reads the shifted range.
obs-plan: 0 proposals · fanout-obs-plan.md
  note: no detector fires; the link-href log-site citations already read pointer.rs:806-812 / :807 / :812 after the sweep.
a11y-plan: 0 proposals · fanout-a11y-plan.md
  note: neither detector fires; §5's citations are the citation pass's and its prose names mechanisms that stand.

## Validate
Rejected for a source the report does not carry: 0. Check 1 playbook · 2 cross-contradiction (none) · 3 intent-consistency
(scope record none, `gate.py scope` clean; the report matches the plan's acceptance) · 4 absence (no absence claim
proposed) · 5 expected amendments — every entry raised by the orchestrator, below · 6 disproved claims: the report's
bullet reads none — nothing to dispose.

Raised by the orchestrator (check 5, the plan's `Expected amendments (wrap)` list, P5-approved; playbook "Accurate
this-chunk addition" → routine, apply):
architecture 1 · §Project Intent → Front page — the README states its state under `## Status` (0.1.0 done and verified with its six stages and three facts; 0.2.0 planned in five stages), where it read "built vs planned", and spells «Escher» in prose and title on the founder's word (inputs#I2) → apply (check 5, routine)
architecture 2 · §Established Decisions [Scrolling and input] — the range keys' gate cited beside the mapping (keyboard.rs:103-108 · 141-158), `press_range_input` cited for the pointerdown (pointer.rs:579-594), `click_button` for the button arm (pointer.rs:890-907) → apply (check 5, routine)
architecture 3 · §Standard Contracts → Events — the button arm's citation gains `click_button` (pointer.rs:890-907); the signatures sentence names the private functions the arms' bodies became and `parse_float`'s steps (range.rs:272-301), behaviour unchanged → apply (check 5, routine)
security-plan 1 · §Input Validation, the range row — `parse_float`'s steps cited (range.rs:272-301), the keys' gate cited (keyboard.rs:103-108), `press_range_input` cited (pointer.rs:579-594) → apply (check 5, routine)
security-plan 2 · §Input Validation, the keyboard-activation row — the button arm's focus and submit in `click_button` cited (pointer.rs:890-907) → apply (check 5, routine)
security-plan · §Error Handling — the line map's re-points only, written by the citation sweep; gate entry 7 reads 0, the panic-path list gains nothing → no body change (carried by the sweep)
obs-plan · §6 / §8 — the link-href log-site citations re-pointed by the citation sweep (moved rows, written) → no body change (carried by the sweep)
a11y-plan 1 · §5 — the button click focus row cites `click_button` (pointer.rs:890-907); the range click row cites `press_range_input` (pointer.rs:579-594); the range keys row cites the gate (keyboard.rs:103-108) → apply (check 5, routine)

Escalations: 0. No boundary widening (no behaviour, carrier, input class or crossing moved; the README is prose).
