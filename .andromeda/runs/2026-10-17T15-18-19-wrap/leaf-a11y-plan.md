group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:26   class: omission
  leaf-text: «Names come from text children, a trimmed-non-empty `aria-label` and `<label>` association (`for`-target or nested `<input>`).»
  source: .andromeda/a11y-plan.md:280 «since 2026-10-17-upstream-sync-engine-debts an `aria-labelledby` whose targets hold text outranks it (accname 2B), the `aria-label` staying the fallback when they hold none»
  replace-with: «Names come from text children, a trimmed-non-empty `aria-label` and `<label>` association (`for`-target or nested `<input>`); since 2026-10-17-upstream-sync-engine-debts an `aria-labelledby` whose targets hold text outranks both (accname 2B) — its loop runs after the `<label>` → input loop, clears `labelled_by` and pushes the referenced nodes, the `aria-label` staying the fallback when the targets hold none — and that text reaches the snapshot `name`.»

- leaf: .claude/docs/a11y-summary.md:21   class: omission
  leaf-text: «Check: `accessibility_states` (4, both layout modes; outside the `a11y` leg).»
  source: .andromeda/a11y-plan.md:47 «the snapshot reads no description and its `enabled` reads the `disabled` attribute, so `aria-disabled` moves no snapshot reading»
  replace-with: «Check: `accessibility_states` (4, both layout modes; outside the `a11y` leg). Since 2026-10-17-upstream-sync-engine-debts the builder also holds upstream's six reads, as upstream wrote them: `aria-labelledby` (Names, below), `aria-describedby` (the referenced nodes as `described_by`), `aria-description` (its text into AccessKit's description), `aria-expanded` (a boolean), `aria-disabled` (`true` → `set_disabled`) and `aria-level` (a number, with the tag and role defaults); upstream's own `parse::<bool>` `aria-selected` and unfiltered `aria-label` reads were dropped in the merge, so there is still one read per attribute; the snapshot reads no description and its `enabled` reads the `disabled` attribute, so `aria-disabled` moves no snapshot reading; no stand file states any of the six, and the description and the three new closed values leave only to the platform adapter, which no stand binary builds — what an assistive technology reads of them is not measured (ratified as upstream's behaviour by the founder, 2026-10-17).»

- leaf: .claude/docs/a11y-summary.md:12   class: omission
  leaf-text: «`assert_role(html, id, expected)` and `unknown_tags(html)` helpers.»
  source: .andromeda/a11y-plan.md:70 «two upstream files assert the builder's name and description reads off the built tree with `verify_that!`»
  replace-with: «`assert_role(html, id, expected)` and `unknown_tags(html)` helpers; since 2026-10-17-upstream-sync-engine-debts two upstream files assert the builder's name and description reads off the built tree with `verify_that!` — `accessibility_label` (10 tests: `aria-label`, `aria-labelledby` and its precedence, fallback and hidden targets) and `accessibility_description` (3: `aria-description`, `aria-describedby` in order, targets outside the tree excluded) — in the workspace test leg and per package, outside the a11y leg's six files.»

- leaf: .claude/rules/a11y.md:25   class: omission
  leaf-text: «The snapshot takes the three from the tree node, never from the element;»
  source: .andromeda/a11y-plan.md:47 «upstream's own `parse::<bool>` `aria-selected` and unfiltered `aria-label` reads dropped in the merge, so there is still one read per attribute»
  replace-with: «Since 2026-10-17-upstream-sync-engine-debts the builder also holds upstream's `aria-describedby` (→ `described_by`), `aria-description` (→ description), `aria-expanded` (a boolean), `aria-disabled` (`true` → `set_disabled`) and `aria-level` (a number, with the tag and role defaults) reads beside the three, as upstream wrote them; upstream's own `parse::<bool>` `aria-selected` and unfiltered `aria-label` reads were dropped in the merge, so there is still one read per attribute — keep it so. The snapshot takes the three from the tree node, never from the element, reads no description, and reads `enabled` from the `disabled` attribute, so `aria-disabled` moves no snapshot reading;»

- leaf: .claude/rules/a11y.md:26   class: omission
  leaf-text: «an `aria-label` non-empty after trimming sets `label`; a `<label>` is pushed onto its bound `<input>`'s `labelled_by` (`for`-target, else first nested input) after the whole tree is built.»
  source: .andromeda/a11y-plan.md:281 «since 2026-10-17-upstream-sync-engine-debts an input that also carries `aria-labelledby` is named by its targets instead»
  replace-with: «an `aria-label` non-empty after trimming sets `label`; a `<label>` is pushed onto its bound `<input>`'s `labelled_by` (`for`-target, else first nested input) after the whole tree is built; since 2026-10-17-upstream-sync-engine-debts an `aria-labelledby` loop runs after that `<label>` → input loop, clears `labelled_by`, pushes the referenced nodes and sets their computed text as the `label`, so it outranks a `<label>` and an `aria-label` (accname 2B) — an id naming no built node ignored, hidden targets read for the label, the `aria-label` the fallback when the targets hold no text.»

- leaf: .claude/rules/a11y.md:42   class: omission
  leaf-text: «`accessibility_states` (the three state attributes, one stated table per attribute with its row count asserted) and `accessibility_names` ride the workspace test leg, not this one.»
  source: .andromeda/a11y-plan.md:70 «they run in the workspace test leg and per package, outside the a11y leg's six files (§9)»
  replace-with: «`accessibility_states` (the three state attributes, one stated table per attribute with its row count asserted), `accessibility_names` and, since 2026-10-17-upstream-sync-engine-debts, upstream's `accessibility_label` (10) and `accessibility_description` (3), which assert the name and description reads with `verify_that!`, ride the workspace test leg, not this one.»
