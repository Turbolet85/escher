group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:17   class: omission
  leaf-text: «and `snapshot` and `screenshot` wait on none.»
  source: .andromeda/security-plan.md:223 «an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot`, `screenshot` and `lint` wait on none»
  replace-with: «and `snapshot`, `screenshot` and `lint` wait on none.»

- leaf: .claude/docs/security-summary.md:19   class: stale
  leaf-text: «`tool` one of the eleven verbs»
  source: .andromeda/security-plan.md:77 «any step `valid_step` rejects (closed keys; `tool` one of the twelve verbs; `arguments` an object; `expect` a list of strings)»
  replace-with: «`tool` one of the twelve verbs»

- leaf: .claude/docs/security-summary.md:22   class: stale
  leaf-text: «`tool` one of the seven instance verbs»
  source: .andromeda/security-plan.md:81 «records with the closed keys `tool` · `arguments` · `masked` · `accepted` · `digest` · `answer` only, `tool` one of the eight instance verbs»
  replace-with: «`tool` one of the eight instance verbs»

- leaf: .claude/docs/security-summary.md:23   class: stale
  leaf-text: «a closed table of eleven verbs in three levels — seven asked of the held instance»
  source: .andromeda/security-plan.md:83 «`validate` checks a call against a closed `'static` table of twelve verbs in three levels — the seven asked of the held instance (`snapshot`, `click`, `type`, `press`, `advance`, `scroll`»
  replace-with: «a closed table of twelve verbs in three levels — eight asked of the held instance»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «the screen's pixels in band, ratified by the founder, 2026-10-12), and three asked of the session»
  source: .andromeda/security-plan.md:83 «an eighth, `lint` (no argument, any argument refused `malformed` with `Fault::Unnamed`; one result field, `findings`, `FieldKind::Entries`, always present; refused `fonts-unavailable` before anything»
  replace-with: «the screen's pixels in band, ratified by the founder, 2026-10-12), and `lint` the eighth since 2026-10-19-visual-lint (no argument, any argument refused `malformed`; one result field, `findings`; running no step — a widening of this validated surface onto every ratified carrier, ratified), and three asked of the session»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «is refused `fonts-unavailable` before anything is painted,»
  source: .andromeda/security-plan.md:83 «refusing a `screenshot` of an app with no font it can paint text with before anything is painted, and a `lint` of one before anything is read»
  replace-with: «is refused `fonts-unavailable` before anything is painted, and a `lint` of one before anything is read,»

- leaf: .claude/docs/security-summary.md:23   class: retired
  leaf-text: «on the in-process `Session::lint` (in process only), which takes no argument, returns `Finding`s holding ids, closed defect words and numbers only and crosses nothing (no verb, written form, MCP tool, recorded step, log site, span or field; `VERBS` staying eleven)»
  source: .andromeda/security-plan.md:125 «crosses as the `lint` verb's answer (`Command::Lint` → `Outcome::Linted`, written `{"findings":[…]}`) on every ratified carrier — the socket's `call`, the command line, MCP and a kept check»
  replace-with: «on `Session::lint`, which takes no argument, returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value) and crosses as the `lint` verb's answer (`Command::Lint` → `Outcome::Linted`, written `{"findings":[…]}`) on every ratified carrier — the socket's `call`, the command line, MCP and a kept check, kept whole — a ratified boundary widening, with no log site, print, span or field carrying a finding and `VERBS` holding twelve»

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «and of a shot never its image, its size or its dimensions (a `screenshot` records `verb` and, refused, `cause`).»
  source: .andromeda/security-plan.md:83 «a shot's image, its size or its dimensions, a lint's findings or their count (a `screenshot` or a `lint` records `verb` and, refused, `cause`)»
  replace-with: «and of a shot never its image, its size or its dimensions, nor a lint's findings or their count (a `screenshot` or a `lint` records `verb` and, refused, `cause`).»

- leaf: .claude/rules/security.md:21   class: stale
  leaf-text: «`tool` one of the seven instance verbs»
  source: .andromeda/security-plan.md:81 «records with the closed keys `tool` · `arguments` · `masked` · `accepted` · `digest` · `answer` only, `tool` one of the eight instance verbs»
  replace-with: «`tool` one of the eight instance verbs»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «a closed verb table of eleven verbs in three levels (`screenshot`, the seventh asked of the held instance, since 2026-10-12-headless-screenshot)»
  source: .andromeda/security-plan.md:83 «`validate` checks a call against a closed `'static` table of twelve verbs in three levels — the seven asked of the held instance (`snapshot`, `click`, `type`, `press`, `advance`, `scroll`»
  replace-with: «a closed verb table of twelve verbs in three levels (`screenshot`, the seventh asked of the held instance, since 2026-10-12-headless-screenshot, and `lint`, the eighth, since 2026-10-19-visual-lint: no argument, one result field `findings`, running no step)»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «and `snapshot` and `screenshot` wait on none.»
  source: .andromeda/security-plan.md:223 «an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot`, `screenshot` and `lint` wait on none»
  replace-with: «and `snapshot`, `screenshot` and `lint` wait on none.»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «as `fonts-unavailable` before anything is painted,»
  source: .andromeda/security-plan.md:83 «refusing a `screenshot` of an app with no font it can paint text with before anything is painted, and a `lint` of one before anything is read»
  replace-with: «as `fonts-unavailable` before anything is painted, and a `lint` of one before anything is read,»

- leaf: .claude/rules/security.md:22   class: retired
  leaf-text: «Since 2026-10-19-visual-lint the in-process `Session::lint` (in process only) also feeds it from its own snapshot; it takes no argument and returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value) and crosses nothing — no verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, socket, CLI arm, log site, span or field, `VERBS` staying eleven.»
  source: .andromeda/security-plan.md:125 «crosses as the `lint` verb's answer (`Command::Lint` → `Outcome::Linted`, written `{"findings":[…]}`) on every ratified carrier — the socket's `call`, the command line, MCP and a kept check»
  replace-with: «Since 2026-10-19-visual-lint `Session::lint` also feeds it from its own snapshot; it takes no argument and returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value) and crosses as the `lint` verb's answer (`Command::Lint` → `Outcome::Linted`, written `{"findings":[…]}`) on every ratified carrier — the socket's `call`, the command line, MCP and a kept check, kept whole — a ratified boundary widening, with no log site, print, span or field carrying a finding and `VERBS` holding twelve.»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «a shot's PNG bytes (`Outcome::Shot`), prints them»
  source: .andromeda/security-plan.md:83 «a shot's PNG bytes (`Outcome::Shot`) and a lint's findings (`Outcome::Linted { findings }`, written by `to_json` alone as `{"findings":[…]}`»
  replace-with: «a shot's PNG bytes (`Outcome::Shot`) and a lint's findings (`Outcome::Linted { findings }`: stable ids, closed defect words and numbers only), prints them»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «a shot's image, its size or its dimensions included (a `screenshot` records `verb` and, refused, `cause`).»
  source: .andromeda/security-plan.md:83 «a shot's image, its size or its dimensions, a lint's findings or their count (a `screenshot` or a `lint` records `verb` and, refused, `cause`)»
  replace-with: «a shot's image, its size or its dimensions, a lint's findings or their count included (a `screenshot` or a `lint` records `verb` and, refused, `cause`).»

- leaf: .claude/rules/security.md:23   class: stale
  leaf-text: «`tool` one of the eleven verbs»
  source: .andromeda/security-plan.md:77 «any step `valid_step` rejects (closed keys; `tool` one of the twelve verbs; `arguments` an object; `expect` a list of strings)»
  replace-with: «`tool` one of the twelve verbs»
