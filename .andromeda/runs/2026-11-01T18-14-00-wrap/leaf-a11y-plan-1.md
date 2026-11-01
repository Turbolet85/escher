group: a11y-plan-1
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
- leaf: .claude/docs/a11y-summary.md:14   class: omission
  leaf-text: «and the windowed list is read and not run.»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:136 «`key_press_after_container_lost_its_box` (3 tests) runs in both layout modes: a button past its dropped anonymous block, a select's choice past it, and a control in no anonymous block reading the same»
  replace-with: «and the windowed list is read and not run; `key_press_after_container_lost_its_box` (3 tests) runs in both layout modes — a button past its dropped anonymous block, a select's choice past it, and a control in no anonymous block reading the same — with no `stand_` prefix, in neither the `a11y` nor the `stand-repeat` leg.»
- leaf: .claude/docs/a11y-summary.md:15   class: omission
  leaf-text: «a highlighted row's contrast is unread by the lint»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:18 «`Session::lint`'s `contrast` reads the same `None` past a blend, through the one reader, so a blended text's pair is unread by the lint»
  replace-with: «a highlighted row's contrast and a blended text's pair are unread by the lint (`Harness::text_contrast` reading `None` past an element on its walk with a `mix-blend-mode` other than `normal`, pinned by `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`, no stand row moving)»
- leaf: .claude/docs/a11y-summary.md:15   class: omission
  leaf-text: «`select_popup_painted` (6 tests) reads»
  source: .andromeda/a11y-plan-detail/ci-integration.md:13 «`select_popup_painted` also runs as `embolden`, the last step of the `fast` leg, built with `blitz-paint/font-embolden`, with no CI job, macOS exercising the feature in CI»
  replace-with: «`select_popup_painted` (6 tests; also run as `embolden`, the last step of the `fast` leg, built with `blitz-paint/font-embolden`, with no CI job, macOS exercising the feature in CI) reads»
- leaf: .claude/docs/a11y-summary.md:15   class: omission
  leaf-text: «so SC 1.4.3 is asserted on the list in the harness;»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:15 «no fork or upstream test paints a blended element to a frame; that absence is owned by a `CARRY:` on «Upstream sync ahead of determinism and exploration»»
  replace-with: «so SC 1.4.3 is asserted on the list in the harness; `mix-blend-mode` and `isolation` paint on every document, and no fork or upstream test paints a blended element to a frame, that absence owned by a `CARRY:` on «Upstream sync ahead of determinism and exploration»;»
- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «the engine paints it over the page»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:97 «The open list paints last, past the blend layers `render_element` pushes for `mix-blend-mode` and `isolation`»
  replace-with: «the engine paints it over the page, last, past the blend layers `render_element` pushes for `mix-blend-mode` and `isolation`, no test painting a blended element to a frame, so the order under a blend is read and not measured,»
- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «gives focus up as the mutator flushes, no DOM `blur` or `focusout` firing)»
  source: .andromeda/a11y-plan.md:107 «so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys, with no panic; the limit is owned by a `CARRY:` on the entry «A11y re-read»»
  replace-with: «gives focus up as the mutator flushes, no DOM `blur` or `focusout` firing; the fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys, with no panic, the limit owned by a `CARRY:` on the entry «A11y re-read»)»
