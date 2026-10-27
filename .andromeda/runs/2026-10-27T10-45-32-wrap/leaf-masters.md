group: masters
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:48   class: stale
  leaf-text: «pipeline tests run `for incremental in [false, true]`.»
  source: .andromeda/architecture-detail/conventions.md:53 «except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it»
  replace-with: «pipeline tests run `for incremental in [false, true]`, except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it.»

- leaf: CLAUDE.md:65   class: omission
  leaf-text: «the seven_guis stand's colours, depth, spacing and radius in»
  source: .andromeda/layout-templates.md:21 «seven_guis stand type — sans-serif, the home title 700 at 36px over a 14px base (examples/seven_guis/src/app.rs:179-205)»
  replace-with: «the seven_guis stand's colours, type, depth, spacing and radius in»
