# Fan-out results — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas

architecture: add 10 · set 6 · retire 0 · none 1 · ops-architecture.md
security-plan: add 6 · set 4 · retire 0 · none 1 · ops-security-plan.md
design-system: add 6 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 2 · set 0 · retire 0 · none 1 · ops-layout-templates.md
test-plan: add 3 · set 0 · retire 0 · none 4 · ops-test-plan.md
obs-plan: add 2 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 2 · set 0 · retire 0 · none 1 · ops-a11y-plan.md

Check 5, the plan's entries with no operation: architecture §Project Intent (`29a439e05` as the next merge base) — no op: `decision/upstream-sync-merge-base` states the rule with no sha, true as it stands; the sha is the report's and `evidence/lock.md`'s. Architecture §Standard Contracts' `blitz-shell` `url` dependency — no op: no master row lists a crate's dependencies (`rec.py find --doc architecture --regex 'blitz-shell.{0,80}(depend|url)'`: 0 hits); the manifest holds it.
Check 6, the disproved claims: the left-out count 14 — stated in no master (LEFTOUT sweep, 29 standing sites, none a count), disposed to the report and `evidence/lock.md`; the refused previews — `decision/root-drop-examples` (security-plan) read `not_master` at fit, so the measured set stays the report's and `evidence/limits.md` §4's. The 8 `println!` sites likewise: `decision/root-example-prints-not-telemetry` read `not_master`, so the attribution stays `evidence/sink.md`'s.

ops-architecture.md
decision/taffy-layout · apply — check 1: no rule; the founder's own answer T1 A settles it, the plan's expected entry names the change · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
decision/no-git-dependencies · apply — check 1: no rule; T2 A retires the Taffy exception, inputs#I4 item 3 · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
contract/layout/stylo-taffy-exports · apply — check 5: the plan's §Standard Contracts entry names `CalcHandle`; the report's Symbols / APIs carry it · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
decision/one-axis-clip-readings-kept · apply — check 1: An engine behaviour change for every document, on the founder's own word C1 A (routine) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять все три (Recommended)»
decision/default-action-event-order · apply — check 1: An engine behaviour change for every document, on the founder's own word P1 A (routine); the plan's entry names this row · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять все три (Recommended)»
decision/handlers-before-default-action · reject — check 2: the fact's home is architecture decision/default-action-event-order
decision/os-drop-input · escalate → Q1 — check 1: Boundary widening by upstream merge (escalate): the OS drop input crosses into every windowed document; the founder answered D1 A himself (inputs#I2), so it goes on one answer card · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
decision/dropped-file-read-unrestricted · reject — check 2: the fact's home is security-plan decision/dropped-file-read-unrestricted
contract/blitz-traits-data-transfer/blitz-data-transfer · escalate → Q1 — check 1: Boundary widening by upstream merge: the drop input's data model, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
contract/blitz-traits-data-transfer/blitz-drag-event · escalate → Q1 — check 1: Boundary widening by upstream merge: the drop input's event payload, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
contract/ui-event/drag-over · escalate → Q1 — check 1: Boundary widening by upstream merge: the drop input's UI event, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
contract/ui-event/drag-leave · escalate → Q1 — check 1: Boundary widening by upstream merge: the drop input's UI event, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
contract/ui-event/drop · escalate → Q1 — check 1: Boundary widening by upstream merge: the drop input's UI event, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
contract/dioxus-drag-events/native-drag-data · escalate → Q1 — check 1: Boundary widening by upstream merge: a Dioxus handler's read of the drop, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
contract/nodes-widgets/widget-drop-acceptance · escalate → Q1 — check 1: Boundary widening by upstream merge: how a widget takes a drop, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
decision/br-in-flex-text-item · apply — check 1: An engine behaviour change for every document, on the founder's own word B1 A (routine) · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять все три (Recommended)»

ops-security-plan.md
decision/engine-dependency-pins · apply — check 1: no rule; T1 A, the plan's expected entry names this row · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
decision/no-git-dependencies · apply — check 1: no rule; T2 A retires the Taffy exception, inputs#I4 item 3 · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
decision/no-git-dependency · apply — check 1: no rule; T2 A retires the Taffy exception, inputs#I4 item 3 · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
decision/upstream-sync-dependency-set · apply — check 1: no rule; T2 A, the plan's expected entry names this row · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять и снять исключение (Recommended)»
decision/os-drop-input · reject — check 7: not_master type 4 — a coverage line retelling the drop source's dispatch and file-URI conversion, no reason stated; the code holds it; escalated (Q1), not asked
decision/dropped-file-read-unrestricted · escalate → Q1 — check 1: Boundary widening by upstream merge: a dropped file read with no path restriction, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2
decision/root-drop-examples · reject — check 7: not_master type 5 — an inventory of the taken examples and their previews; the code holds it, the refusal the standing file-URL rule's consequence; escalated (Q1), not asked (check 6: the refused previews as measured stay the report's and `evidence/limits.md` §4)
decision/custom-widget-drop-by-drop-effect · reject — check 2: the fact's home is architecture contract/nodes-widgets/widget-drop-acceptance
decision/prevent-default-non-cancelable-ignored · reject — check 2: the fact's home is architecture decision/default-action-event-order
decision/os-drag-and-drop-untrusted · escalate → Q1 — check 1: Boundary widening by upstream merge: the drop as an input source in the threat model, D1 A · fit: fits · resolved: apply (Q1) — delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)» — PROVISIONAL, the ratification the founder's D1/D2 answer, inputs#I2

ops-design-system.md
decision/one-axis-clip-paints-clipped-axis · reject — check 7: not_master type 4 — an upstream paint function taken as written, its provenance alone; the code holds it (check 5: the plan's design-system §Depth entry ends here; architecture decision/one-axis-clip-readings-kept states the one-axis paint)
contract/os-drag-and-drop/drag-events · reject — check 2: the fact's home is architecture contract/ui-event/drag-over
contract/os-drag-and-drop/drop-effect · reject — check 2: the fact's home is architecture contract/ui-event/drop
contract/os-drag-and-drop/custom-widget-drop · reject — check 2: the fact's home is architecture contract/nodes-widgets/widget-drop-acceptance
decision/non-cancelable-ignores-prevent-default · reject — check 2: the fact's home is architecture decision/default-action-event-order
decision/br-after-text-joins-flex-item · reject — check 2: the fact's home is architecture decision/br-in-flex-text-item

ops-layout-templates.md
decision/desktop-window-os-drag-and-drop · reject — check 2: the fact's home is architecture decision/os-drop-input
decision/custom-widget-accepts-drop-by-drop-effect · reject — check 2: the fact's home is architecture contract/nodes-widgets/widget-drop-acceptance

ops-test-plan.md
decision/drag-and-drop-tested-headless · reject — check 7: not_master type 5 — one surface's line of a coverage inventory; the code and the sidecar hold it (check 5: the plan's test-plan §Integration Test Strategy entry ends here)
name/other/drag-and-drop · reject — check 1: no test-plan row names a test target as an occupied name (`rec.py find --doc test-plan --regex 'name/other/'`: 0); `decision/blitz-tests-one-target-per-file` governs targets, the manifest holds them
name/other/flex-br · reject — check 1: as name/other/drag-and-drop

ops-obs-plan.md
decision/root-example-prints-not-telemetry · reject — check 7: not_master type 4 — a description of example code with no choice or reason; the code holds it (check 5: the plan's obs-plan §Log Coverage entry ends here; check 6: the prints as measured stay the report's and `evidence/sink.md`'s)
decision/dropped-data-never-logged · apply — check 1: no rule; the report's coverage rows read PII n/a, nothing logged, for the drop input and the dropped-file reader · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять вместе с примером (Recommended)»

ops-a11y-plan.md
absence/keyboard-drag-and-drop · apply — check 1: no rule; the report's coverage row reads the drop input a11y n/a · word: relayed verbatim by overseer1 and the escher overseer, 2026-11-24, inputs#I2: «Взять вместе с примером (Recommended)»
decision/adapter-probe-on-sync-lock-move · reject — check 7: not_master type 8 — one acceptance check met in this run, no standing rule; the run's record holds it (check 5: the plan's a11y-plan §CI Integration entry ends here, gate entry 15)

ops-main-architecture.md
decision/os-drop-input · apply — check 1: an add on a delegate's confirm carries state provisional (Q1) · word: delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)»

ops-main-security-plan.md
decision/dropped-file-read-unrestricted · apply — check 1: an add on a delegate's confirm carries state provisional (Q1) · word: delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)»
decision/os-drag-and-drop-untrusted · apply — check 1: an add on a delegate's confirm carries state provisional (Q1) · word: delegate escher-overseer, 2026-11-24: «Confirm all 10 (Recommended)»
