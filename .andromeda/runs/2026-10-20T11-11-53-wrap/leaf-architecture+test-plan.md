group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «then `snapshot` · `click --id <id>`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «`snapshot`'s `--part` a whole number 1 to 1000, a paged answer one line `{"text":…,"part":n,"parts":m}`, status `0`, a whole screen's still `{"text":…}` alone»
  replace-with: «then `snapshot [--part <part>]` (`--part` a whole number 1 to 1000; a screen over the snapshot text budget, or a `--part` asked, answers one part as one line `{"text":…,"part":n,"parts":m}`, status `0`, a part past the last the empty text, a whole screen's still `{"text":…}` alone) · `click --id <id>`»

- leaf: .claude/docs/commands.md:59   class: omission
  leaf-text: «its tools the twelve driver verbs (`screenshot` after `scroll`»
  source: .andromeda/architecture.md:196 «an accepted `snapshot` one text block, the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one»
  replace-with: «its tools the twelve driver verbs (`snapshot` taking an optional integer `part` from 1 to 1000, a whole screen answered as the screen text itself and a paged one as its JSON line `{"text","part","parts"}`; `screenshot` after `scroll`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_load_wait --test stand_act_select`»
  source: .andromeda/test-plan.md:27 «and `stand_snapshot_parts` 3, declaring `mod fixtures;` and `mod session_common;`, in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «--test stand_load_wait --test stand_act_select --test stand_snapshot_parts`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «an unchosen option refused (a `click` on an option `covered`); each picked up»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_snapshot_parts` 3 (`Session::run` in both layout modes on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: `snapshot` with no argument answering part 1 of `parts` ≥ 2»
  replace-with: «an unchosen option refused (a `click` on an option `covered`); since 2026-10-20-large-screen-in-parts `stand_snapshot_parts`, 3 tests, on a fixture page whose screen passes `SNAPSHOT_TEXT_BUDGET`: `snapshot` with no argument answering part 1 of `parts` ≥ 2, each part within the budget, the parts concatenating to `to_text()`, `parts + 1` the empty text with the same `parts` and the screen unchanged, an id read only in the last part taking `scroll` then `click`, a password reading `MASKED_VALUE` in every part; the same walk through `serve_mcp`; and a screen inside the budget read whole as before; each picked up»
