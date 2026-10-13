group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: »
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «a password or file-input value reads the fixed mask `MASKED_VALUE`;»
  source: .andromeda/a11y-plan.md:275 «and, since 2026-10-12-headless-screenshot, in no painted frame either: blitz-paint paints a password input's value as bullets, laid out with the input's editor styles, scale and direction»
  replace-with: «a password or file-input value reads the fixed mask `MASKED_VALUE` — and, since 2026-10-12-headless-screenshot, a password's typed text is in no painted frame either: blitz-paint paints its value as bullets, its caret at the bullet of its caret index, in every document and through every renderer that paints with blitz-paint, windowed included (ratified by the founder, 2026-10-12), two values of one length painting byte-identical frames, focused or not, in both layout modes (`password_painted_masked.rs`); which bullet a click lands on can depend on the real characters' widths — not measured, owned by a `CARRY:` on "Stand requirement sweep"; the windowed paint is not run;»

- leaf: .claude/docs/a11y-summary.md:24   class: omission
  leaf-text: «each `differs` = `[bounds]` (`stand_diff.rs`).»
  source: .andromeda/a11y-plan.md:275 «since 2026-10-12-headless-screenshot the driver's `screenshot` is one more reader of the model's bounds and of nothing else in it — an element shot, addressed by stable id and refused as every id-addressed»
  replace-with: «each `differs` = `[bounds]` (`stand_diff.rs`). Since 2026-10-12-headless-screenshot the driver's `screenshot` is one more reader of the model's bounds and of nothing else in it — an element shot, addressed by stable id and refused as every id-addressed call is, is the node's bounds rounded outward and cropped to the part that can be seen — and a shot changes nothing: no focus moves, `has_changes` stays false and the next diff is empty, in both layout modes (`stand_act_screenshot.rs`).»
