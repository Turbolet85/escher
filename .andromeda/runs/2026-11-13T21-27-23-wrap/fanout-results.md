# Fan-out results — 2026-11-13-cognitive-load-corrective

architecture: add 0 · set 1 · retire 0 · none 0 · ops-architecture.md
security-plan: add 0 · set 1 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: none · ops-layout-templates.md — reason: behaviour-preserving restructure, no surface, region, verb, field or provider added; every row stays true
test-plan: add 1 · set 2 · retire 0 · none 0 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: no span field, cause, verb, key, carrier, log field, env read or listener added; the new check is silent; every picked row stays true
a11y-plan: add 0 · set 1 · retire 0 · none 0 · ops-a11y-plan.md

Nothing removed by a fix: no file took a scripted fix.

## Dispositions

ops-architecture.md
decision/cognitive-complexity-ceiling · reject — check 5: the report dispositions architecture `dec-ndcu6j63` true, no-change (Expected amendments, `ceiling-left`); the row states no "left as they are" stance, so the exception has nothing to qualify there. It is test-plan `dec-akszmc27`'s clause, set in ops-test-plan.md, and one fact stands in one place.

ops-security-plan.md
decision/unresolvable-stylesheet-href-no-load · apply — check 6: the report's Spec claims disproved names the `why` false (`resolve_url` answers none and no resolver panics since the 2026-11-13 take), the rule still holding; the operator's standing word (relayed verbatim by the operator, 2026-11-13, inputs#I2) directs the shorter set or the retire, and the false clause is the whole cell, so the set empties it.

ops-test-plan.md
decision/complexity-ceiling-host-only · apply — check 5: the plan's Expected amendment (test-plan §10, `dec-akszmc27`'s clause «engine functions over it are left as they are») carried in the report's Changes; the operator chose «Every arm (Recommended)» at P4 (inputs#I1), so the exception is his recorded direction (rule `rul-ipb3atbm`, this chunk's change, the invariant holding for every other engine function).
decision/mutation-controlled-tests · reject — check 1: the loop-advance control form is an implement technique the report carries under Decisions & corrections for curation (P3), not a decision of the test plan; the rule (a named mutation turns the check red, the restored tree green) still holds as written.
contract/harness-inspect/focused · apply — check 4: the report's Decisions & corrections measured it (`get_focussed_node_id` reads `focus_node_id.or(root element)`, `document.rs:689-692`, Deviation 1), a promise of the harness's focused reading checks rely on, landing in the existing boundary `Harness inspection helpers`; no rule governs and nothing in it is structural.

ops-a11y-plan.md
decision/click-focuses-buttons · reject — check 2: the row's rule stays true as written (an unmatched click still clears focus — the chunk changed no behaviour); the root-element fallback is the focus reader's, not the click's, and it stands once, in test-plan `contract/harness-inspect/focused`.
