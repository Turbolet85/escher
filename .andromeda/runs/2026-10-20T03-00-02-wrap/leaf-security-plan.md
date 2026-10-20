group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:17   class: stale
  leaf-text: «The driver's command schema keeps this: no verb has an argument or a field of a wait, and no driver command waits on a load (the operator, 2026-10-07).»
  source: .andromeda/security-plan.md:222 «an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot` and `screenshot` wait on none»
  replace-with: «The driver's command schema keeps this: no verb has an argument or a field of a wait; since 2026-10-20-load-wait the driver's acting step (`Session::act` and the executor's settled step, so `click` · `type` · `press` · `advance` · `scroll` on every carrier) settles again every 10 ms while the answer is `Loads`, for at most `LOAD_WAIT` (5 s of wall time, under the 30 s answer read), moving no time on the held instance, then returns the load still in flight as `settled: false` with its busy class — so the absent timeout cannot hang a command either — and `snapshot` and `screenshot` wait on none.»

- leaf: .claude/rules/security.md:22   class: stale
  leaf-text: «never a log line; no driver command waits on a load.»
  source: .andromeda/security-plan.md:222 «an acting driver command waits on a load in flight for at most `LOAD_WAIT`, the bound the `settled` field's help states, and `snapshot` and `screenshot` wait on none»
  replace-with: «never a log line; an acting driver command waits on a load in flight for at most `LOAD_WAIT` (5 s of wall time, re-settling every 10 ms, moving no time on the held instance, then `settled: false` with its busy class; since 2026-10-20-load-wait), and `snapshot` and `screenshot` wait on none.»
