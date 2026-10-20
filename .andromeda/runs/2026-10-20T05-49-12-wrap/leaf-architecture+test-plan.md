group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_lint --test stand_load_wait` · `cargo test -p seven_guis»
  source: .andromeda/test-plan.md:27 «and `stand_act_select` 5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a page writing `selected` statically»
  replace-with: «--test stand_lint --test stand_load_wait --test stand_act_select` · `cargo test -p seven_guis»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «`harness().time()` unmoved in both; each picked up»
  source: .andromeda/test-plan.md:27 «and `stand_act_select` 5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a page writing `selected` statically»
  replace-with: «`harness().time()` unmoved in both; since 2026-10-20-select-by-id `stand_act_select`, 5 tests: a select driven by id — at boot its node `ComboBox` with its name and value and its options as its children named by their labels, a `type` with a label choosing that option (its diff naming the select with `value` and the two options with `selected` in `differs`), a click then a key choosing an option, the Tab walk skipping a disabled select, a disabled select and an unchosen option refused (a `click` on an option `covered`); each picked up»

- leaf: .claude/docs/commands.md:61   class: retired
  leaf-text: «`Session::lint` (PROVISIONAL: in process only, no verb)»
  source: .andromeda/test-plan.md:27 «on the in-process `Session::lint` (in process only)»
  replace-with: «`Session::lint` (in process only, no verb)»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «run by the workspace `test` leg and per package, not by `run stand`)»
  source: .andromeda/test-plan.md:27 «`select_interaction` 7, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix (run by the workspace `test` leg, by name and by `run all`, not by `run stand`)»
  replace-with: «run by the workspace `test` leg and per package, not by `run stand`); and `cargo test -p blitz-tests --locked --test select_interaction` — the engine's single-choice `select` model on a controlled Dioxus page, 7 tests, both layout modes (since 2026-10-20-select-by-id: the select laid out as a box holding its chosen option, the choice seeded from the markup, `ArrowDown` · `ArrowUp` · `Home` · `End` choosing an enabled option, typed text choosing the first enabled option its label starts with and kept for one second of the document's clock, one `input` and one `change` per changed choice, a disabled select taking no key and no click, an app's `selected` write re-seeding the choice; run by the workspace `test` leg, by name and by `run all`, not by `run stand`)»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`type --id <id> --text <text>` · `press»
  source: .andromeda/architecture.md:196 «says that on a select the text chooses the first option whose label starts with it (packages/escher-driver/src/schema.rs:208-209)»
  replace-with: «`type --id <id> --text <text>` (on a select, the text chooses the first option whose label starts with it) · `press»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: retired
  leaf-text: «(since 2026-10-19-visual-lint, PROVISIONAL)»
  source: .andromeda/architecture.md:202 «- The visual lint, in process only — its `lint` verb, on every carrier and recorded, owed to «Visual lint» (its gated part):»
  replace-with: «(since 2026-10-19-visual-lint, in process only)»
