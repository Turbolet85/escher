# Cascade compare — a11y-plan · phase · 2026-10-25-capability-statement
E items 115 · held 26 · lost at classify 6 · lost at strong 2 · beside a placed fact - · cascade only 12 · body 59 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### a11y-strategy#3.3 · lost at classify · .andromeda/a11y-plan-detail/a11y-strategy.md:31
- (.andromeda/a11y-plan-detail/a11y-strategy.md:31) If the statement's a11y reach names platform adapters, it must state these per-platform adapter gaps and not claim full adapter parity.
  > Android adapter `set_focus` and `set_window_bounds` are empty; Windows `set_focus` and macOS `set_window_bounds` are no-ops

### aria-patterns-roles#2.5 · lost at strong · .andromeda/a11y-plan-detail/aria-patterns-roles.md:19
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) The select limit the statement gathers (a `multiple` select not modelled) must stay consistent with the role mapping, which still maps a `multiple` select to `ListBox` and every other to `ComboBox`.
  > **Select** — `<select multiple>` is ListBox, else ComboBox

### aria-patterns-roles#2.1 · lost at strong · .andromeda/a11y-plan-detail/aria-patterns-roles.md:15
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:15) A claim that an app's `role` attribute reaches the tree and the snapshot must name it as the engine's ARIA role-name mapping, not a second mapping.
  > **ARIA `role` attribute** — `role_from_name` maps ARIA role names (alert, button, checkbox, dialog, link, tab, textbox, landmark roles, etc.) to AccessKit roles

### keyboard-navigation#1.9 · lost at classify · .andromeda/a11y-plan-detail/keyboard-navigation.md:16
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:16) An engine focus limit a frontend consumer meets; where the statement lists engine limits it must not claim `:focus-visible` styling works.
  > while `:focus-visible` never matches

### keyboard-navigation#2.4 · lost at classify · .andromeda/a11y-plan-detail/keyboard-navigation.md:26
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:26) The statement must say that two driver clicks on one point with no `advance` between them read as a double click.
  > Two pointer clicks on one point within 500 ms and 2 px are a double click, read on the document's clock — on a harness-built document the harness's `VirtualClock`, so two driver `click`s on one point with no `advance` between them are a double click, selecting a word in a text input — while each still focuses its target as one click does

### keyboard-navigation#4.6 · lost at classify · .andromeda/a11y-plan-detail/keyboard-navigation.md:59
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:59) A framework limit a Dioxus consumer meets; where the statement lists Dioxus-bridge limits it must not claim queued focus events.
  > Mounted elements can take or drop focus with `set_focus`; focus/blur events are not queued (TODO)

### keyboard-navigation#4.8 · lost at classify · .andromeda/a11y-plan-detail/keyboard-navigation.md:61
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:61) A framework limit a Dioxus consumer meets; where the statement lists Dioxus-bridge limits it must not claim IME handling there.
  > IME events are not handled in dioxus-native-dom (TODO)

### visual-design-verification#1.6 · lost at classify · .andromeda/a11y-plan-detail/visual-design-verification.md:15
- (.andromeda/a11y-plan-detail/visual-design-verification.md:15) The select's unmeasured contrast pair must be among the stated limits beside the select's unmodelled parts.
  > its text/background pair is not measured — the stand holds no select, so `stand_contrast`'s tables are unmoved

## Held

- a11y-scope-summary#1.9
- a11y-strategy#1.1
- a11y-strategy#2.3
- a11y-strategy#2.4
- a11y-strategy#4.1
- aria-patterns-roles#2.2
- aria-patterns-roles#2.9
- ci-integration#1.0
- cognitive-accessibility#1.2
- keyboard-navigation#1.1
- keyboard-navigation#2.3
- keyboard-navigation#3.2
- keyboard-navigation#3.3
- keyboard-navigation#3.4
- keyboard-navigation#5.8
- keyboard-navigation#7.2
- screen-reader-support#1.4
- screen-reader-support#2.1
- screen-reader-support#3.1
- screen-reader-support#4.1
- screen-reader-support#5.1
- screen-reader-support#8.2
- visual-design-verification#1.4
- visual-design-verification#1.5
- visual-design-verification#3.9
- visual-design-verification#8.1

## Cascade only

- a11y-scope-summary#1.2
- a11y-scope-summary#1.3
- a11y-scope-summary#1.6
- keyboard-navigation#1.2
- keyboard-navigation#1.4
- keyboard-navigation#1.5
- keyboard-navigation#2.7
- keyboard-navigation#7.1
- keyboard-navigation#7.3
- keyboard-navigation#7.4
- screen-reader-support#1.2
- screen-reader-support#1.3
