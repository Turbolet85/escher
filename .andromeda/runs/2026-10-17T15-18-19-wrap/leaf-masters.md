group: masters
read: CLAUDE.md · 148 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:15   class: retired
  leaf-text: «- `apps/` — reference browser (`blitz`), markdown viewer (`rdme`), release `bump`»
  source: .andromeda/architecture.md:75 «no in-repo release tool remains since upstream deleted the bump crate (`23b9b26a9`, merged at the 2026-10-17 upstream sync)»
  replace-with: «- `apps/` — reference browser (`blitz`), markdown viewer (`rdme`)»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «accessibility tree (its builder writes a node's `selected` · `toggled` · `invalid` from `aria-selected` · `aria-pressed` · `aria-invalid`, read by value on every element; `aria-checked` unread)»
  source: .andromeda/architecture.md:121 «`aria-labelledby`: after the `<label>` → input loop it clears the node's `labelled_by`, pushes the referenced nodes that have built nodes»
  replace-with: «accessibility tree (its builder writes a node's `selected` · `toggled` · `invalid` from `aria-selected` · `aria-pressed` · `aria-invalid`, read by value on every element; `aria-checked` unread; and, upstream's reads merged at the 2026-10-17 upstream sync, `aria-labelledby`'s targets' text as the computed `label`, outranking a `<label>` and an `aria-label` (the `aria-label` the fallback when the targets hold no text), `aria-describedby` as `described_by`, `aria-description` as the description, `aria-expanded`, `aria-disabled` (`true` sets disabled) and `aria-level`)»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «`selected` · `pressed` · `invalid` are taken from the accessibility node, a mixed toggle reading not pressed and a grammar or spelling error invalid)»
  source: .andromeda/architecture.md:135 «while the snapshot reads no description and its `enabled` still reads the `disabled` attribute, so `aria-disabled` moves no snapshot reading»
  replace-with: «`selected` · `pressed` · `invalid` are taken from the accessibility node, a mixed toggle reading not pressed and a grammar or spelling error invalid; the `name` takes the node's `label`, since the 2026-10-17 upstream sync `aria-labelledby`'s computed text outranking a `<label>`, while no description is read and `enabled` still reads the `disabled` attribute, so `aria-disabled` moves no reading)»

- leaf: CLAUDE.md:76   class: omission
  leaf-text: «| Accessibility tree · its three state attributes | `packages/blitz-dom/src/accessibility.rs` · check `tests/blitz-tests/tests/accessibility_states.rs` |»
  source: .andromeda/test-plan.md:26 «two checks of the builder's merged name and description reads on a parsed page, asserted through test-that's `verify_that!`: `accessibility_label` 10»
  replace-with: «| Accessibility tree · its three state attributes · its merged name and description reads | `packages/blitz-dom/src/accessibility.rs` · checks `tests/blitz-tests/tests/accessibility_{states,label,description}.rs` |»
