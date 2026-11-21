# Fan-out results — 2026-11-21-engine-debts

architecture: add 2 · set 13 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 4 · retire 0 · none 1 · ops-security-plan.md
design-system: add 2 · set 2 · retire 1 · none 0 · ops-design-system.md
layout-templates: none · ops-layout-templates.md — reason: no user-facing surface or region added; no row made false or incomplete
test-plan: add 4 · set 1 · retire 1 · none 0 · ops-test-plan.md
obs-plan: add 2 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 2 · set 2 · retire 1 · none 0 · ops-a11y-plan.md
main: design-system retire 1 · ops-main-design-system.md · test-plan retire 1 · ops-main-test-plan.md (each proposer's retire named a heir that was rejected)

## Dispositions

ops-architecture.md
decision/tab-walk-skips-unshown · reject — check 2: the fact's home is a11y-plan decision/tab-and-shift-tab-move-focus
decision/focus-within-hidden-control-takes-tab · reject — check 2: the fact's home is a11y-plan decision/focus-within-hidden-holder-limit
decision/focus-fixup-without-events · escalate → Q1 — check 1: records the post-layout fixup, an engine change for every document on the founder's F1 A with F2 A (inputs#I3); the operator's directive puts it on the one card · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
contract/dom-events/focus-change · escalate → Q1 — check 1: the post-layout fixup's silent focus change, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
contract/event-dispatch/focus-change · escalate → Q1 — check 1: the post-layout fixup's silent focus change, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/changed-set-write-sites · escalate → Q1 — check 1: the post-layout focus clear marks its node, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/hit-focus-form-engine-changes · escalate → Q1 — check 1: the tally of engine changes for every document gains the fixup (F1 A, inputs#I3) and the Tab skip (inputs#I6) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/resolve-reconciles-select-list · reject — check 7: not_master type 4 — the order of steps inside resolve is the code's; the standing row stays true (the fixup runs after the reconcile, both before hover); escalated (Q1), not asked
decision/bridge-buffered-render · apply — check 1: accurate this-chunk addition, the measured busy-yield limit (report Reverted; operator inputs#I5); check 6 disposes the plan's step 5 (a) claim
decision/one-run-seed · apply — check 1: accurate this-chunk addition, F4 A widens nothing (inputs#I3)
decision/no-seed-on-carriers · apply — check 1: accurate this-chunk addition, F4 A widens nothing (inputs#I3)
contract/blitz-vibey-script-api/scriptdocument · apply — check 1: accurate this-chunk addition, `with_seed` in the report's Symbols
contract/dioxus-native-dom-api/boundary-marks · apply — check 1: accurate this-chunk addition, F5 A widens nothing (inputs#I3); check 6 disposes the step 8 prediction
decision/anonymous-block-drop-clears-layout-parent · escalate → Q1 — check 1: a key on a boxless control acts only re-focused, by the post-layout fixup, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/integration-tests-in-blitz-tests · apply — check 1: the chunk added `packages/blitz-vibey-script/tests/seeded_random.rs` beside the crate's standing tests (report Files), so the row's rule needs its exception

ops-security-plan.md
decision/inert-focus-clear-silent · escalate → Q1 — check 1: the post-layout clear is silent too, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/programmatic-focus-events · escalate → Q1 — check 1: its except list gains the post-layout clear, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/seeded-source-in-process-seed-only · apply — check 1: accurate this-chunk addition, F4 A widens nothing (inputs#I3)
decision/dropped-anonymous-block-clears-parent · escalate → Q1 — check 1: a key reaches a boxless control only re-focused, by the post-layout fixup, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/tab-walk-skips-unshown · reject — check 2: the fact's home is a11y-plan decision/tab-and-shift-tab-move-focus
decision/boundary-registry-pruned-before-render · reject — check 2: the fact's home is architecture contract/dioxus-native-dom-api/boundary-marks

ops-design-system.md
decision/boxless-focused-select-keeps-focus · reject — check 2: its heir is rejected below; the retire is re-written heirless in ops-main-design-system.md
decision/unshown-focused-control-gives-up-focus · reject — check 2: the fact's home is a11y-plan decision/focus-fixup-gives-up-unshown-focus
decision/tab-walk-skips-unshown · reject — check 2: the fact's home is a11y-plan decision/tab-and-shift-tab-move-focus
contract/mounteddata-during-render/document-state · apply — check 1: accurate this-chunk addition, F3 A re-states the limit to the reachable case (inputs#I3)
decision/buffered-render-replay · reject — check 2: the fact's home is architecture decision/bridge-buffered-render

ops-main-design-system.md
decision/boxless-focused-select-keeps-focus · escalate → Q1 — check 1: the row falls by the post-layout fixup, F1 A (inputs#I3); the fact's home is a11y-plan · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»

ops-test-plan.md
decision/count-and-rename-sweeps · apply — check 1: accurate this-chunk addition, the report's Decisions (a CI leg's file count pinned as a literal); check 6 disposes research's sweep claim
decision/key-after-dropped-anonymous-block · reject — check 2: its heir is rejected by fit below; the retire is re-written heirless in ops-main-test-plan.md
decision/key-after-lost-box-acts-after-refocus · reject — check 7: not_master type 4 — an acceptance line the code and its test hold; escalated (Q1), not asked
decision/focus-test-pins-where-focus-lands · apply — check 1: accurate this-chunk addition, the report's measured test hazard
decision/render-wait-by-stored-waker · apply — check 1: accurate this-chunk addition, the operator's stored-waker ruling (inputs#I5)
decision/script-document-tests-seeded · reject — check 7: not_master type 4 — an acceptance line of the seeded mechanism the code and its test hold

ops-main-test-plan.md
decision/key-after-dropped-anonymous-block · escalate → Q1 — check 1: focus is given up after the layout, so the key acts only re-focused, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»

ops-obs-plan.md
decision/script-seed-never-logged · reject — check 2: the fact's home is security-plan decision/seeded-source-in-process-seed-only (a seed is no content; the report adds no log site)
decision/boundary-registry-memory-only · reject — check 1: not this chunk's drift — the registry and its error context stood before; the chunk extended it internally and added no log site

ops-a11y-plan.md
decision/changed-set-marks · escalate → Q1 — check 1: the post-layout focus loss marks its node, F1 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/tab-and-shift-tab-move-focus · escalate → Q1 — check 1: the Tab walk skips what does not show, an engine change for every document on the founder's «Tab пропускает скрытое (Recommended)» (inputs#I6); check 6 disposes the plan's step 4 claim · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/focus-fixup-reads-inert-alone · escalate → Q1 — check 1: the fixup reads more than inertness, F1 A with F2 A (inputs#I3); check 6 disposes the masters' «reads is_inert() alone» · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/focus-fixup-gives-up-unshown-focus · escalate → Q1 — check 1: the post-layout fixup, F1 A with F2 A (inputs#I3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»
decision/focus-within-hidden-holder-limit · escalate → Q1 — check 1: the stated limit of the Tab skip, «Tab пропускает скрытое (Recommended)» as read to him (inputs#I6) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I7: «Подтвердить все 16 (Recommended)»

ops-main-architecture.md
decision/hit-focus-form-engine-changes · apply — check 1: restores the row's «direct … flagged upstreamable», which the landed Q1 set dropped to fit the rule's bound; it moves no claim the card confirmed
