# fanout results — 2026-11-21-single-select-sized-as-a-list-box

architecture: add 3 · set 21 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 8 · retire 0 · none 0 · ops-security-plan.md
design-system: add 4 · set 5 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 2 · ops-layout-templates.md
test-plan: add 3 · set 1 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 1 · set 2 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 5 · set 10 · retire 0 · none 1 · ops-a11y-plan.md

Check 5: every Expected-amendments entry is carried by an operation below; the test-plan entry «the single-select `size` limit retired» has no test-plan site (the pre-pass sweep's `q-size` reads test-plan 0) — disposed, no operation owed; `matrix#v040-04 notes` — P7.3.
Check 6: the lint-at-boot claim → test-plan `decision/scrolling-box-boot-lint-pinned`; the popup-gate control → test-plan `decision/mutation-controlled-tests`; the stale client rect → architecture `contract/geometry/get-client-bounding-rect` and test-plan `decision/box-presence-read-with-display`; the centre-row control claim → curation (P3), the proposer's `@row_none`.

ops-architecture.md
decision/single-select-list-box · escalate → Q1 — check 1: a boundary widening, the engine reading `size` on a single select in every document (the founder's F1 A, inputs#I3); the fact's one home · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/select-readers/is-list-box · apply — check 1: a new public read-only reader the report's Symbols / APIs names, no widening of its own
contract/accessibility-tree/select-role · reject — check 2: the fact's home is a11y-plan contract/accessibility-tree/select-role
contract/geometry/get-client-bounding-rect · apply — check 6: the measured stale client rect, Spec claims disproved, engine behaviour unchanged
decision/targeted-restyle-on-mutation · apply — check 1: routine, the report's Deviations 9
decision/select-size-hint · escalate → Q1 — check 1: the widening's own row, the «stays unread» clause retired · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/single-select-list-opening · escalate → Q1 — check 1: inside the widening, the list on a drop-down alone · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-list-painted-overlay · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-keyboard-model · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/hit-focus-form-engine-changes · escalate → Q1 — check 1: inside the widening, an engine change for every document · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/click-focuses-button · escalate → Q1 — check 1: inside the widening, a list box row's click · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/closed-select-options-covered · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/multiple-select-chosen-row-colours · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/accessibility-tree/select-expanded · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/accessibility-tree/aria-expanded · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/dioxus-native-dom/closed-select-of · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/driver-forms/closed-select-of · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/driver-forms/select-snapshot-options · escalate → Q1 — check 1: inside the widening, the HTML lean (5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/snapshot/closed-select-option · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/snapshot/multiple-select-bounds · escalate → Q1 — check 1: inside the widening, the HTML lean (5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/lint-verb/overlap-select-rows · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/driver-verbs/click · escalate → Q1 — check 1: inside the widening, the HTML lean (5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/select-list-default-actions/click-select · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/select-list-default-actions/open-keys · escalate → Q1 — check 1: inside the widening, the HTML lean (4) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»

ops-security-plan.md
decision/single-select-selectedness · escalate → Q1 — check 1: inside the widening, the HTML lean (2); the settle's one home · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/multiple-select-size-rows · escalate → Q1 — check 1: the widening, the «unread» clause retired · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-size-hint-cap · escalate → Q1 — check 1: the widening's own row · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-list-open-gate · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-list-keys-and-painted-labels · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-expanded-from-engine-state · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-readers-resolve-node-closed-select-none · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-list-closes-when-select-unfit · escalate → Q1 — check 1: inside the widening; the size write's close, its one home · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/single-select-list-box · reject — check 2: the fact's home is architecture decision/single-select-list-box
decision/single-list-box-keys-and-clicks · reject — check 2: the fact's home is a11y-plan contract/single-list-box/keys

ops-design-system.md
decision/select-chosen-row-chromium-pair · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/single-select-box-shows-chosen-label · reject — check 7: not_master type 4 — a drop-down's layout is one mechanism the code holds; escalated (Q1), not asked; the standing row is false, retired in ops-main-design-system.md
decision/single-select-open-gestures · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/selects-that-open-no-list · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/select-list-events/close-on-select-change · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/single-select-display-size · reject — check 2: the fact's home is architecture decision/single-select-list-box
decision/single-list-box-may-stand-unchosen · reject — check 2: the fact's home is security-plan decision/single-select-selectedness
decision/single-list-box-keys-and-clicks · reject — check 2: the fact's home is a11y-plan contract/single-list-box/keys
contract/select-accessibility/single-list-box · reject — check 2: the fact's home is a11y-plan contract/accessibility-tree/single-select

ops-main-design-system.md
decision/single-select-box-shows-chosen-label · escalate → Q1 — check 1: inside the widening, the correction fit's not_master owes (a retire, no verdict) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»

ops-main-architecture.2.md
decision/hit-focus-form-engine-changes · apply — check 1: routine, restores three clauses the landed set re-worded beside its list-box addition (inside Q1's confirmed change, the list box kept)

ops-layout-templates.md
decision/single-select-sized-list-box · reject — check 2: the fact's home is architecture decision/single-select-list-box

ops-test-plan.md
decision/mutation-controlled-tests · apply — check 6: the popup-gate control's disproved claim, a convention it states
decision/box-presence-read-with-display · apply — check 6: the stale client rect's measured consequence for checks
decision/scrolling-box-boot-lint-pinned · reject — check 7: not_master type 4 — the lint code and the one check hold it (the boot-lint claim stays disposed there)
decision/moved-fixture-select-controlled · apply — check 1: routine, a measured fixture convention (Deviations 8)

ops-obs-plan.md
decision/select-size-attribute-read · escalate → Q1 — check 1: the widening's own read, no log site added · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-chosen-set-never-logged · escalate → Q1 — check 1: inside the widening, the HTML lean (5) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/snapshot/single-list-box-value · reject — check 7: not_master type 4 — it retells the settle and the list-box reading, the code holds it; escalated (Q1), not asked

ops-a11y-plan.md
decision/single-select-engine-overlay-list · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/aria-property-reads/aria-expanded · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/single-select-expanded-from-list · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/accessibility-tree/single-select · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/accessibility-tree/select-expanded · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/accessibility-tree/select-role · escalate → Q1 — check 1: inside the widening, the HTML lean (3); the role's one home · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/select-closed-list/enter-space · escalate → Q1 — check 1: inside the widening, the HTML lean (4) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/hidden-content-excluded · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/snapshot/closed-select-option-bounds · escalate → Q1 — check 1: inside the widening · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/multiple-select-chosen-row-pair · escalate → Q1 — check 1: inside the widening, the HTML lean (3) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/select-list-box-or-drop-down · reject — check 2: the fact's home is architecture decision/single-select-list-box
contract/single-list-box/keys · escalate → Q1 — check 1: inside the widening, the HTML lean (4); keys' one home · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
contract/single-list-box/row-click · escalate → Q1 — check 1: inside the widening, the HTML lean (4) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-21, inputs#I4: «Подтвердить все 47 (Recommended)»
decision/only-drop-down-settles-choice · reject — check 2: the fact's home is security-plan decision/single-select-selectedness
decision/size-write-reclassifies-select · reject — check 2: the fact's home is security-plan decision/select-list-closes-when-select-unfit
