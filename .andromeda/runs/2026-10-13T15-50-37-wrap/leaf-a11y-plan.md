group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «dioxus-native-dom removes a falsy value of its 27 boolean attributes (`disabled`, `checked`, `hidden` among them)»
  source: .andromeda/a11y-plan.md:183 «of one of the bridge's 28 boolean attributes — upstream's private `is_bool_attr` predicate, a closed `matches!` set, since 2026-10-13-upstream-sync-polish-and-ship»
  replace-with: «dioxus-native-dom removes a falsy value of its 28 boolean attributes — upstream's `is_bool_attr` predicate since 2026-10-13-upstream-sync-polish-and-ship (`disabled`, `checked`, `hidden`, `inert` among them; a framework-written `inert: false` is removed, so it no longer makes content inert)»

- leaf: .claude/rules/a11y.md:32   class: stale
  leaf-text: «dioxus-native-dom removes a falsy value of its 27 boolean attributes (`disabled`, `checked`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` among them)»
  source: .andromeda/a11y-plan.md:183 «`disabled`, `checked`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them — removes the attribute»
  replace-with: «dioxus-native-dom removes a falsy value of its 28 boolean attributes — upstream's private `is_bool_attr` predicate, a closed set, since 2026-10-13-upstream-sync-polish-and-ship (`disabled`, `checked`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus`, `inert` among them; a framework-written `inert: false` is removed, so it no longer makes content inert)»
