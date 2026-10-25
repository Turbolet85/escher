group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/dioxus-native-dom.md:18   class: stale
  leaf-text: «the lean stand screens read 755 · 1287 · 1259 · 2049 bytes»
  source: .andromeda/architecture-detail/standard-contracts.md:230 «on the stand each lean task's screen at boot reads Counter 755 · FlightBooker 1295 · Timer 1259 · Crud 2055 bytes on the dev host (Linux), read through the host in its default layout mode»
  replace-with: «the lean stand screens read 755 · 1295 · 1259 · 2055 bytes»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «the three stated readings each asserted equal to the accessibility node's»
  source: .andromeda/architecture-detail/standard-contracts.md:226 «exactly the chosen flight type's option reads selected, exactly the selected CRUD row reads selected, and a date field reads invalid while what it holds is no date — the stand states no `aria-pressed`»
  replace-with: «the selected and invalid readings each asserted equal to the accessibility node's — the flight type's chosen option, the selected CRUD row, a date field holding no date — the stand stating no `aria-pressed`, so `accessibility_states` pins the pressed reading on a fixture»

- leaf: .claude/docs/services/dioxus-native-dom.md:16   class: omission
  leaf-text: «on Temperature Converter, Circle Drawer (its dialog open included) and Cells too (2 · 3 · 676 until then)»
  source: .andromeda/architecture-detail/standard-contracts.md:235 «it returns nothing on the load page `stand::boot_load` mounts too, its two elements keyed `load-fetch` · `load-done`, so it reads empty on all six served tasks (tests/blitz-tests/tests/stand_load_page.rs:70-84)»
  replace-with: «on Temperature Converter, Circle Drawer (its dialog open included) and Cells too (2 · 3 · 676 until then), and on the load page `stand::boot_load` mounts, its two elements keyed `load-fetch` · `load-done` — empty on all six served tasks (`stand_load_page`)»

- leaf: .claude/docs/services/blitz-dom.md:26   class: omission
  leaf-text: «- Interaction state references DOM nodes only and is retargeted before a node is freed.»
  source: .andromeda/architecture-detail/established-decisions.md:84 «right after a `scroll`, a main-button pointerdown at a point in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`events/pointer.rs`, `hit_with_scrollbar` while `scrollbar_opacity > 0`)»
  replace-with: «- Interaction state references DOM nodes only and is retargeted before a node is freed. - Right after a `scroll`, a main-button pointerdown in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`events/pointer.rs`, `hit_with_scrollbar` while `scrollbar_opacity > 0`, a measured hazard, no engine change); on the virtual clock the bar stays shown until an `advance` passes its fade, so a driver `click` inside a just-scrolled box needs an `advance` past the fade first (`stand_cells_served`).»
