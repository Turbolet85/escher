group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «six argument kinds with bounds (`id` 1 to 1024 bytes, `text` 0 to 4096, `key` one of twelve names, `flag`, `milliseconds` 1 to 60000, `name` 1 to 32 bytes of `a-z0-9-`)»
  source: .andromeda/security-plan.md:83 «and the seventh, `part`, a whole number from 1 to 1000 (`MAX_PART`) taken through `u32::try_from` (ten arguments across the table)»
  replace-with: «seven argument kinds with bounds (`id` 1 to 1024 bytes, `text` 0 to 4096, `key` one of twelve names, `flag`, `milliseconds` 1 to 60000, `name` 1 to 32 bytes of `a-z0-9-`, `part` a whole number from 1 to 1000, `MAX_PART`)»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «eight asked of the held instance, `scroll` the sixth»
  source: .andromeda/security-plan.md:83 «`snapshot` — one optional `part`, its result `text` and, when the screen is read in parts, the counts `part` and `parts`; a screen over the snapshot budget read from part 1 with no `part`, a part past»
  replace-with: «eight asked of the held instance, `snapshot` taking one optional `part` (its result `text` and, when the screen is read in parts, the counts `part` and `parts`, written `{"text":…,"part":n,"parts":m}` as `Outcome::Part`; a screen over the snapshot budget read from part 1 with no `part`, a part past the last the empty text), `scroll` the sixth»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «and of a shot never its image, its size or its dimensions, nor a lint's findings or their count (a `screenshot` or a `lint` records `verb` and, refused, `cause`)»
  source: .andromeda/security-plan.md:83 «a part's number or the part count, a shot's image, its size or its dimensions, a lint's findings or their count (a `screenshot`, a `lint` or a part read records `verb` and, refused, `cause`), the sess»
  replace-with: «and never the screen text or its length, a part's number or the part count, a shot's image, its size or its dimensions, nor a lint's findings or their count (a `screenshot`, a `lint` or a part read records `verb` and, refused, `cause`)»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «six argument kinds with bounds, one fixed refusal order»
  source: .andromeda/security-plan.md:83 «and the seventh, `part`, a whole number from 1 to 1000 (`MAX_PART`) taken through `u32::try_from` (ten arguments across the table)»
  replace-with: «seven argument kinds with bounds (`part`, the seventh since 2026-10-20-large-screen-in-parts, `snapshot`'s one optional argument, a whole number from 1 to 1000, `MAX_PART`), one fixed refusal order»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «and a lint's findings (`Outcome::Linted { findings }`: stable ids, closed defect words and numbers only), prints them under `Debug`»
  source: .andromeda/security-plan.md:83 «and a screen's part (`Outcome::Part { text, part, parts }`, its text the snapshot text's own lines, written by `to_json` alone as `{"text":…,"part":n,"parts":m}`), and the crate logs none of them and »
  replace-with: «and a lint's findings (`Outcome::Linted { findings }`: stable ids, closed defect words and numbers only) and a screen's part (`Outcome::Part { text, part, parts }`, the snapshot text's own lines, written `{"text":…,"part":n,"parts":m}`), prints them under `Debug`»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «a shot's image, its size or its dimensions, a lint's findings or their count included (a `screenshot` or a `lint` records `verb` and, refused, `cause`)»
  source: .andromeda/security-plan.md:83 «a part's number or the part count, a shot's image, its size or its dimensions, a lint's findings or their count (a `screenshot`, a `lint` or a part read records `verb` and, refused, `cause`), the sess»
  replace-with: «the screen text or its length, a part's number or the part count, a shot's image, its size or its dimensions, a lint's findings or their count included (a `screenshot`, a `lint` or a part read records `verb` and, refused, `cause`)»
