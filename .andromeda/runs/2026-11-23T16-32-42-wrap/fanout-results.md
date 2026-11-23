# Fan-out results — 2026-11-23-canvas-shot-across-hosts (wrap 2026-11-23T16-32-42-wrap)

architecture: add 0 · set 2 · retire 0 · none 0 · ops-architecture.md
security-plan: no proposer
design-system: add 1 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: none · ops-layout-templates.md (reason: the chunk adds no user-facing surface or region; its canvas fixtures are private to the test target; every row read stays true)
test-plan: add 1 · set 4 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 1 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md (reason: no interactive UI element, no a11y violation or log schema moved; every a11y-plan row stays true)

Expected amendments floor (check 5): test-plan §2 / §3 → the three no-shot rows' `except` (ops-test-plan.md) and their restated `why` (ops-main-test-plan.2.md) · test-plan §2 → `+ decision/canvas-shots-cross-os-by-digest` (ops-test-plan.md) · architecture §Existing Scopes → `decision/no-screenshot-in-cross-os-byte-check`'s `except` (ops-architecture.md) and restated `why` (ops-main-architecture.2.md) · test-plan §1 → `decision/no-screenshot-step-in-committed-checks`'s `why` restated (ops-test-plan.md n=4), the rule unmoved. Every entry is met by an operation; the four Expected amendments entries are all carried.
No boundary widening (check 1, `rul-izni57qv`): the exception adds no carrier, input class, verb, field or crossing and commits no pixel — a shot's pixels cross only as they already did (architecture decision/canvas-crosses-as-pixels, security-plan decision/canvas-content-crosses-as-shot-pixels, both unmoved) — as the plan's fork named it; each change applies on the plan's P5-approved expected amendments, which name it, with the founder's own word F1, relayed verbatim (inputs#I3). No answer card.
Disproved claims (check 6): the report states none this run; the probe's disproof of «a shot's bytes differ between OSes» for canvas content is disposed by the rows above.
Citation dispositions: 1 row, a curation home (P3) — no master operation.

ops-architecture.md
decision/no-screenshot-in-cross-os-byte-check · apply — check 1: the plan's expected amendment names this row's canvas-only except (fct-o4fpla67), held on the three hosts by CI#45209188300 (fct-y2t2bg5i) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/committed-stand-checks · apply — check 4: the row's reason «a shot's PNG bytes differ by OS» is now true of a shot holding document text alone (fct-rpckbvls); the rule, no screenshot step in the committed checks, stands unmoved · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»

ops-design-system.md
contract/screenshot/canvas-only-bytes-across-hosts · reject — check 2: the fact's home is test-plan decision/canvas-shots-cross-os-by-digest; the design system holds no rule of a shot's bytes across hosts

ops-obs-plan.md
decision/canvas-shot-digest-equal-across-hosts · reject — check 2: the fact's home is test-plan decision/canvas-shots-cross-os-by-digest; obs-plan's harness contract adds no field, span or log line for it

ops-test-plan.md
decision/cross-os-checks-hold-no-shot · apply — check 1: the plan's expected amendment names this row's canvas-only except (fct-o4fpla67), held on the three hosts (fct-y2t2bg5i) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/no-shot-in-cross-os-byte-checks · apply — check 1: the plan's expected amendment names this row's canvas-only except (fct-o4fpla67), held on the three hosts (fct-y2t2bg5i) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/no-screenshot-in-cross-os-checks · apply — check 1: the plan's expected amendment names this row's canvas-only except (fct-o4fpla67), held on the three hosts (fct-y2t2bg5i) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/no-screenshot-step-in-committed-checks · apply — check 5: the plan's expected amendment leaves this row's `why` to the wrap's fit; the reason now holds for a shot of document text alone (fct-rpckbvls), the rule unmoved · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/canvas-shots-cross-os-by-digest · apply — check 1: the plan's expected amendment names this decision (three canvas-only fixtures, whole and element shots, both modes, one committed digest table, no pixels, guard stand_canvas_cross_os; fct-o4fpla67) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»

ops-main-test-plan.2.md
decision/cross-os-checks-hold-no-shot · apply — check 5: the plan's expected amendment restates this row's `why` to the measured cause (fct-o4fpla67, fct-rpckbvls) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/no-shot-in-cross-os-byte-checks · apply — check 5: the plan's expected amendment restates this row's `why` to the measured cause (fct-o4fpla67, fct-rpckbvls) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/no-screenshot-in-cross-os-checks · apply — check 5: the plan's expected amendment restates this row's `why` to the measured cause (fct-o4fpla67, fct-rpckbvls) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»

ops-main-test-plan.md
decision/checks-hold-no-screenshot-step · apply — check 4: cascade step 2's sweep (sweep-pass-1.1.txt, m-bytesdiffer) reads this row's reason as the amended claim stated flat; scoped to a shot holding document text (fct-rpckbvls), the rule unmoved · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
decision/no-screenshot-in-recorded-check-scripts · apply — check 4: cascade step 2's sweep (sweep-pass-1.1.txt, m-bytesdiffer) reads this row's reason as the amended claim stated flat; scoped to a shot holding document text (fct-rpckbvls), the rule unmoved · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»

ops-main-architecture.2.md
decision/no-screenshot-in-cross-os-byte-check · apply — check 5: the plan's expected amendment restates this row's `why` to the measured cause (fct-o4fpla67, fct-rpckbvls) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-23, inputs#I3: F1 «A: требовать побайтовое равенство (Recommended)»
