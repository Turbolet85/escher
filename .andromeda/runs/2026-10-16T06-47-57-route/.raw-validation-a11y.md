# A11y validation — route draft

## Insert
- Between `Stand requirement sweep` and `Capability statement`: **"Tier-0 a11y re-read — stand Tab order, Enter/Space activation and contrast re-asserted after engine changes; new a11y checks join the `a11y` leg (SC 2.1.1 · 2.4.3 · 1.4.3)"** (epoch: `Epoch 5 — Polish & ship`)
  Reason: The draft has no Polish verification chunk for the a11y tier set in a11y-plan §1. The engine debts change focusability for every document (§5 Focus order, `disabled` and `inert`), and §9's `a11y` leg runs a fixed list of six files, so new checks are not picked up on their own. The two bootstrap items in the plan's §Bootstrap contract (contrast harness, a11y CI gate) are already discharged, so only this verification is still owed.

## Rewrite
- `Handler splits`: "`parse_float` in steps (v020-01; audit M1)" → "`parse_float` in steps; Enter/Space activation and range keys unchanged, `stand_keyboard` green (v020-01; audit M1; SC 2.1.1)"
  Reason: The click, pointerdown and key arms being split are the Enter/Space and range-key paths that a11y-plan §5 (Keyboard handling) records as meeting SC 2.1.1 by measurement.
- `Engine debts`: "fixed and proven on stand or fixture, pinned as a limit, or kept" → "fixed and proven on stand or fixture, pinned as a limit, or kept; the four a11y debts each name their SC (2.1.1 · 2.4.3 · 4.1.2)"
  Reason: Four of the nine debts are a11y debts (`inert` and `disabled` focusability in a11y-plan §5, `aria-checked` in §4, `aria-activedescendant` in §5/§7), and per the focus guide a verification chunk names the SC IDs in its scope.
- `Visual lint`: "failed contrast" → "failed text contrast (SC 1.4.3), agreeing with `stand_contrast`'s measured rows"
  Reason: a11y-plan §6 (Color contrast) already holds the stand's SC 1.4.3 record (`Harness::text_contrast`, 4.5/3.0 thresholds), and the lint command must not become a second contrast reading that disagrees with it.
- `Select by id`: "read in the snapshot, proven on a fixture" → "read in the snapshot with its tree role (ComboBox / ListBox) and a non-empty name, Tab order kept, proven on a fixture (SC 2.4.3)"
  Reason: a11y-plan §4 maps `select` to ComboBox/ListBox and §7 makes the snapshot read role and name only from the accessibility tree, so the new control must arrive there named and keep the focus order.
- `Capability statement`: "what escher can do and its stated limits" → "what escher can do and its stated limits, a11y tier 0 and no platform adapter or windowed AT witness among them"
  Reason: a11y-plan §1 and §2 record that the stand builds no platform adapter (F1, re-carried past 0.2.0). Viola's frontend planning has to know that escher's a11y reading is headless and tree-only.
