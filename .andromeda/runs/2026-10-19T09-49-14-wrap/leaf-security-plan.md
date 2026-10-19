group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/rules/security.md:21   class: stale
  leaf-text: «acceptance and an FNV-1a 64 digest of its answer, no screen reading;»
  source: .andromeda/security-plan.md:192 «and, since 2026-10-19-recorded-check-replay, the answer itself as `answer` — the line the driver wrote, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`), a shot's as `{"width","height"}` with no `png`»
  replace-with: «acceptance, an FNV-1a 64 digest of its answer and, since 2026-10-19-recorded-check-replay, the answer itself as `answer` — the line the driver wrote, masked as on every carrier (a password's or file input's value as `MASKED_VALUE`), a shot's as `{"width","height"}` with no `png` — so the screens' readings (ids, names, values, snapshot text, diffs) at rest, never a pixel (each answer kept whole, ratified by the founder, 2026-10-19, F3 B);»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «never logged, no tool call, verb, argument or field names the directory;»
  source: .andromeda/security-plan.md:193 «since 2026-10-19-recorded-check-replay two kept checks are committed to the public repository, `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` (git mode `100644`) — the one exception to a kept check's owner-only medium»
  replace-with: «never logged, no tool call, verb, argument or field names the directory; the one exception to a kept check's owner-only medium is the two committed stand-Counter checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` (git mode `100644`, written by `examples/seven_guis/tests/flows/record_checks.sh`; no typed text, host path, name or secret and no `screenshot` step), a boundary widening ratified by the founder (2026-10-19, F1 A) — a check of any other session stays owner-only and is never committed;»

- leaf: .claude/rules/security.md:21   class: omission
  leaf-text: «(closed keys, `tool` one of the seven instance verbs, else `malformed` holding nothing of the file, no panic path), each step still validated inside `Session::run`.»
  source: .andromeda/security-plan.md:81 «`answer` required and a JSON object (missing, a string, a number, an array or `null` refused; key order not enforced, read as a map); anything else refused `Cause::Malformed` with no fault, holding nothing of the file»
  replace-with: «(closed keys, `tool` one of the seven instance verbs, since 2026-10-19-recorded-check-replay `answer` required and a JSON object, else `malformed` holding nothing of the file, no panic path), each step still validated inside `Session::run`; `Check::replay` runs each kept step's call through `Session::run` alone, nothing on a session of another app, and stops at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding bools, counts and closed schema words only.»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «`tool` one of the seven instance verbs, anything else refused `malformed` holding nothing of the file, no panic path, each replayed step still passing `validate` inside `Session::run`), and writes nothing»
  source: .andromeda/security-plan.md:255 «of a kept check's bytes (`Check::from_bytes`) and, since 2026-10-19-recorded-check-replay, of the driver's own answer line the replay compares (`from_str` in `Check::replay`), writing nothing»
  replace-with: «`tool` one of the seven instance verbs, since 2026-10-19-recorded-check-replay `answer` required and a JSON object, anything else refused `malformed` holding nothing of the file, no panic path, each replayed step still passing `validate` inside `Session::run`) and, since 2026-10-19-recorded-check-replay, the driver's own answer line `Check::replay` compares — `Check::replay` running each kept step's call through `Session::run` alone and stopping at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding bools, counts and closed schema words only — and writes nothing»

- leaf: .claude/docs/security-summary.md:32   class: stale
  leaf-text: «no log, event or file of escher's carries any of the three»
  source: .andromeda/security-plan.md:125 «no log or event carries the snapshot or its text — the one file that does is a kept check: since 2026-10-19-session-recording a recorded session's calls, the ids they named among them, and since 2026-10-19-recorded-check-replay each answer kept whole»
  replace-with: «no log or event of escher's carries any of the three, and a file only as a kept check»

- leaf: .claude/docs/security-summary.md:32   class: stale
  leaf-text: «and an FNV-1a 64 digest of each answer, never a screen reading (no snapshot text, name, value, diff or pixel)»
  source: .andromeda/security-plan.md:192 «holding call content (the target ids, `press` keys, `advance` ms and text typed into unmasked controls; a masked or unresolved `type`'s text as `MASKED_STAND_IN`) and the screens' readings at rest — ids, names, values, snapshot text and diffs, never a pixel»
  replace-with: «and an FNV-1a 64 digest of each answer and, since 2026-10-19-recorded-check-replay, each answer kept whole as `answer` — ids, names, values (a password's or file input's as `MASKED_VALUE`), snapshot text and diffs, a shot's as `{"width","height"}` with no `png`, never a pixel (ratified by the founder, 2026-10-19, F3 B) — owner-only but for the two committed stand-Counter checks `tests/blitz-tests/checks/counter-cli.json` and `counter-mcp.json` in git (no typed text, host path or secret, no `screenshot` step; ratified by the founder, 2026-10-19, F1 A), a check of any other session never committed»
