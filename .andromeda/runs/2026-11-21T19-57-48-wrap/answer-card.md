# Answer card Q1 — 2026-11-21-single-select-sized-as-a-list-box

## His answers, as recorded (inputs#I3, relayed by the overseer, each label verbatim)
- F1 — how much of his F8 falls: «Падает только последняя часть (Recommended)» — the forks file's option A: a single select with `size` above 1 becomes a list box `size` rows tall, no popup, keys and clicks choosing a row; a `multiple` select stays as F8 built it. Under it the engine reads `size` on a single select in every document.
- The five things taken from HTML: «Всё по HTML, возражений нет (Recommended)» — (1) HTML's display size; (2) a list box may start with nothing chosen; (3) `ListBox`, no `expanded`, the chosen row in the `multiple` pair; (4) arrows, Home, End, typeahead choose, Enter and Space open nothing, a click chooses one row; (5) for an agent a row `click` is accepted and `type` chooses by the text's start, nothing new on the driver.

## The operations lying wholly inside them (47)
ops-architecture.md · decision/single-select-list-box · established-decisions · fit: fits type 1
ops-architecture.md · decision/select-size-hint · fit: fits type 1
ops-architecture.md · decision/single-select-list-opening · fit: fits type 1
ops-architecture.md · decision/select-list-painted-overlay · fit: fits type 1
ops-architecture.md · decision/select-keyboard-model · fit: fits type 1
ops-architecture.md · decision/hit-focus-form-engine-changes · fit: fits type 1
ops-architecture.md · decision/click-focuses-button · fit: fits type 2
ops-architecture.md · decision/closed-select-options-covered · fit: fits type 1
ops-architecture.md · decision/multiple-select-chosen-row-colours · fit: fits type 1
ops-architecture.md · contract/accessibility-tree/select-expanded · fit: fits type 3
ops-architecture.md · contract/accessibility-tree/aria-expanded · fit: fits type 3
ops-architecture.md · contract/dioxus-native-dom/closed-select-of · fit: fits type 3
ops-architecture.md · contract/driver-forms/closed-select-of · fit: fits type 3
ops-architecture.md · contract/driver-forms/select-snapshot-options · fit: fits type 3
ops-architecture.md · contract/snapshot/closed-select-option · fit: fits type 3
ops-architecture.md · contract/snapshot/multiple-select-bounds · fit: fits type 3
ops-architecture.md · contract/lint-verb/overlap-select-rows · fit: fits type 3
ops-architecture.md · contract/driver-verbs/click · fit: fits type 3
ops-architecture.md · contract/select-list-default-actions/click-select · fit: fits type 3
ops-architecture.md · contract/select-list-default-actions/open-keys · fit: fits type 3
ops-security-plan.md · decision/single-select-selectedness · fit: fits type 2
ops-security-plan.md · decision/multiple-select-size-rows · fit: fits type 1
ops-security-plan.md · decision/select-size-hint-cap · fit: fits type 1
ops-security-plan.md · decision/select-list-open-gate · fit: fits type 1
ops-security-plan.md · decision/select-list-keys-and-painted-labels · fit: fits type 1
ops-security-plan.md · decision/select-expanded-from-engine-state · fit: fits type 1
ops-security-plan.md · decision/select-readers-resolve-node-closed-select-none · fit: fits type 1
ops-security-plan.md · decision/select-list-closes-when-select-unfit · fit: fits type 1
ops-design-system.md · decision/select-chosen-row-chromium-pair · fit: fits type 1
ops-design-system.md · decision/single-select-open-gestures · fit: fits type 1
ops-design-system.md · decision/selects-that-open-no-list · fit: fits type 1
ops-design-system.md · contract/select-list-events/close-on-select-change · fit: fits type 3
ops-main-design-system.md · decision/single-select-box-shows-chosen-label (retire) · fit: —
ops-obs-plan.md · decision/select-size-attribute-read · fit: fits type 1
ops-obs-plan.md · decision/select-chosen-set-never-logged · fit: fits type 1
ops-a11y-plan.md · decision/single-select-engine-overlay-list · fit: fits type 1
ops-a11y-plan.md · contract/aria-property-reads/aria-expanded · fit: fits type 3
ops-a11y-plan.md · decision/single-select-expanded-from-list · fit: fits type 1
ops-a11y-plan.md · contract/accessibility-tree/single-select · fit: fits type 3
ops-a11y-plan.md · contract/accessibility-tree/select-expanded · fit: fits type 3
ops-a11y-plan.md · contract/accessibility-tree/select-role · fit: fits type 3
ops-a11y-plan.md · contract/select-closed-list/enter-space · fit: fits type 3
ops-a11y-plan.md · decision/hidden-content-excluded · fit: fits type 1
ops-a11y-plan.md · contract/snapshot/closed-select-option-bounds · fit: fits type 3
ops-a11y-plan.md · decision/multiple-select-chosen-row-pair · fit: fits type 1
ops-a11y-plan.md · contract/single-list-box/keys · keyboard-navigation · fit: fits type 3
ops-a11y-plan.md · contract/single-list-box/row-click · keyboard-navigation · fit: fits type 3

## Named beside the card, not asked (fit not_master, escalated)
- ops-design-system.md · decision/single-select-box-shows-chosen-label (the narrowing set) — not_master type 4; corrected by the retire above
- ops-obs-plan.md · contract/snapshot/single-list-box-value — not_master type 4

## Answer
The card's dialog timed out in the wrap window. The founder answered it himself by dialog in the overseer session (by 2026-11-21T21:33Z), relayed by the overseer and recorded verbatim in `../additional/escher-overseer/relays/single-select-list-box-founder-2026-11-21.md` (inputs#I4):

«Подтвердить все 47 (Recommended)»
