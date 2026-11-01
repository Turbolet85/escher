group: a11y-plan-2
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»
- leaf: .claude/rules/a11y.md:32   class: omission
  leaf-text: «`disabled` and `hidden` take no part in it»
  source: .andromeda/a11y-plan.md:107 «The fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys, with no panic;»
  replace-with: «`disabled` and `hidden` take no part in it — the fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys, a select's choice still moving past its dropped anonymous block, with no panic (`BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still in the slab naming the block), the limit owned by a `.andromeda/residuals.md` line, a line in `escher-0.4.0-incubator/deferred-from-0.3.0.md` and a `CARRY:` on the entry «A11y re-read»»
- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «and the windowed list is read and not run. Cells:»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:136 «it carries no `stand_` prefix and runs in neither the `a11y` nor the `stand-repeat` leg»
  replace-with: «and the windowed list is read and not run; `key_press_after_container_lost_its_box` (3 tests) runs in both layout modes — a button past its dropped anonymous block, a select's choice past it, and a control in no anonymous block reading the same — with no `stand_` prefix, in neither the `a11y` nor the `stand-repeat` leg. Cells:»
- leaf: .claude/rules/a11y.md:38   class: omission
  leaf-text: «so SC 1.4.3 is asserted on the list in the harness»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:97 «The open list paints last, past the blend layers `render_element` pushes for `mix-blend-mode` and `isolation`, `select_popup_painted` passing on plain and emboldening builds;»
  replace-with: «so SC 1.4.3 is asserted on the list in the harness, the list painting last, past the blend layers `render_element` pushes for `mix-blend-mode` and `isolation`, `select_popup_painted` passing on plain and emboldening builds, while no test paints a blended element to a frame, so the order under a blend is read and not measured»
- leaf: .claude/rules/a11y.md:38   class: omission
  leaf-text: «a highlighted row's contrast is unread by the lint and a closed select's own pair is unread.»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:18 «`Session::lint`'s `contrast` reads the same `None` past a blend, through the one reader, so a blended text's pair is unread by the lint»
  replace-with: «a highlighted row's contrast is unread by the lint and a closed select's own pair is unread; `text_contrast` reads `None` where an element on its walk has `opacity` below 1, a non-empty `filter`, a `background-image` layer or a `mix-blend-mode` other than `normal`, no stand row moving, and the lint's `contrast` reads the same `None` through the one reader, so a blended text's pair is unread by the lint.»
- leaf: .claude/rules/a11y.md:42   class: omission
  leaf-text: «none is among this leg's fourteen files, though `stand_keyboard`, one of them, carries the list's walk here.»
  source: .andromeda/a11y-plan-detail/ci-integration.md:13 «`select_popup_painted` also runs as `embolden`, the last step of the `fast` leg, built with `blitz-paint/font-embolden`, with no CI job, macOS exercising the feature in CI»
  replace-with: «none is among this leg's fourteen files, though `stand_keyboard`, one of them, carries the list's walk here. `select_popup_painted` also runs as `embolden`, the last step of the `fast` leg, built with `blitz-paint/font-embolden`, with no CI job, macOS exercising the feature in CI.»
