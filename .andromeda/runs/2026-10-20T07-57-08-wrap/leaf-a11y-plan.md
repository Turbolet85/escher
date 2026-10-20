group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «escher-driver's in-process visual lint (`Session::lint`, in process only, no verb carries it)»
  source: .andromeda/a11y-plan.md:80 «its first non-test caller is escher-driver's visual lint (`Session::lint`, in process and as the `lint` verb on every carrier), whose `contrast` reading keeps the same exempt rule (§6).»
  replace-with: «escher-driver's visual lint (`Session::lint` in process and the `lint` verb on every carrier, recorded)»

- leaf: .claude/rules/a11y.md:38   class: stale
  leaf-text: «escher-driver's in-process visual lint (`Session::lint`, in process only, no verb carries it)»
  source: .andromeda/a11y-plan.md:236 «the `contrast` reading of escher-driver's visual lint (`Session::lint` in process and the `lint` verb on every carrier, recorded; packages/escher-driver/src/execute.rs:261-263)»
  replace-with: «escher-driver's visual lint (`Session::lint` in process and the `lint` verb on every carrier, recorded; a `contrast` finding written `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only)»
