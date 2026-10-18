group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «since 2026-10-18-one-virtual-clock `stand_virtual_clock`, PROVISIONAL: a shot»
  source: .andromeda/test-plan.md:27 «repeating across three fresh sessions per mode (tests/blitz-tests/tests/stand_virtual_clock.rs:50-133) (ratified by the founder, 2026-10-18)»
  replace-with: «since 2026-10-18-one-virtual-clock `stand_virtual_clock`, ratified by the founder (2026-10-18): a shot»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «a double click selecting the whole value (PROVISIONAL)»
  source: .andromeda/test-plan.md:19 «and a double click selects the whole value (ratified by the founder, 2026-10-18) (tests/blitz-tests/tests/password_caret_click.rs:83-121)»
  replace-with: «a double click selecting the whole value (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/commands.md:63   class: stale
  leaf-text: «and, PROVISIONAL, at a `contain: paint` box's padding box»
  source: .andromeda/architecture.md:86 «or one whose computed `contain` includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18)»
  replace-with: «and, ratified by the founder (2026-10-18), at a `contain: paint` box's padding box»

- leaf: .claude/docs/commands.md:63   class: stale
  leaf-text: «only its ancestors' scroll offsets subtracted (PROVISIONAL)»
  source: .andromeda/architecture.md:86 «the second reader `Node::absolute_position` subtracts only its containing blocks' scroll offsets (ratified by the founder, 2026-10-18; §Standard Contracts → Scrolling, selection, tree)»
  replace-with: «only its ancestors' scroll offsets subtracted (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/commands.md:63   class: stale
  leaf-text: «the client rect read where an element's transforms move it (PROVISIONAL)»
  source: .andromeda/architecture.md:149 «- The client rect under transforms (ratified by the founder, 2026-10-18): `BaseDocument::get_client_bounding_rect` and `node_client_rects` read an element where its transforms move it»
  replace-with: «the client rect read where an element's transforms move it (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/commands.md:63   class: stale
  leaf-text: «the document's clock (PROVISIONAL, since 2026-10-18-one-virtual-clock)»
  source: .andromeda/architecture.md:139 «- The document's clock (ratified by the founder, 2026-10-18): `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` in `Default`»
  replace-with: «the document's clock (since 2026-10-18-one-virtual-clock, ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: stale
  leaf-text: «where one was handed (PROVISIONAL, since 2026-10-18-one-virtual-clock)»
  source: .andromeda/architecture.md:175 «a step's diff can be non-empty on any animated page (ratified by the founder, 2026-10-18)»
  replace-with: «where one was handed (since 2026-10-18-one-virtual-clock, ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: stale
  leaf-text: «The harness's time (PROVISIONAL, since 2026-10-18-one-virtual-clock)»
  source: .andromeda/architecture.md:169 «- The harness's time (ratified by the founder, 2026-10-18): the harness holds a `VirtualClock` its documents share»
  replace-with: «The harness's time (since 2026-10-18-one-virtual-clock, ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: stale
  leaf-text: «`scrolled_box_absolute_position` its check, PROVISIONAL)»
  source: .andromeda/test-plan.md:93 «as `BaseDocument::get_client_bounding_rect` and the snapshot's `bounds` do, `scrolled_box_absolute_position` its check (ratified by the founder, 2026-10-18)»
  replace-with: «`scrolled_box_absolute_position` its check, ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «reading `harness().time()`) (PROVISIONAL)»
  source: .andromeda/test-plan.md:27 «repeating across three fresh sessions per mode (tests/blitz-tests/tests/stand_virtual_clock.rs:50-133) (ratified by the founder, 2026-10-18)»
  replace-with: «reading `harness().time()`) (ratified by the founder, 2026-10-18)»
