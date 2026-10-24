# security · Data Protection — category read

## Carried
- (.andromeda/security-plan-detail/data-protection.md:8) The stderr measurement of the `--record` recording must record through the binary's own launchers into a `0700` directory the launcher names, so each kept check lands `0600` per recorded session and nothing else.
  > under `--record <dir>` on `escher-session` (`start`, the host role, the `mcp` role) or `Session::recording_into` in process, one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named
- (.andromeda/security-plan-detail/data-protection.md:8) A kept check written by any sweep check holds call content and screen readings at rest and must never be logged: the sweep's 0-needle stderr reading under `--record` is the witness that no record of it reaches a log, and no shot pixel enters a kept check.
  > holding call content (the target ids, `press` keys, `advance` ms and text typed into unmasked controls; a masked or unresolved `type`'s text as `MASKED_STAND_IN`) and the screens' readings at rest — ids, names, values, snapshot text and diffs, never a pixel — never logged, and never removed by escher but for its own check when a recording ends
- (.andromeda/security-plan-detail/data-protection.md:14) The two committed Counter checks are the only public kept checks: a check the sweep records (its stderr measurement, a new fixture or stand check) must stay owner-only in its scratch directory and must never be committed.
  > a check a launcher records of any other session stays owner-only and is never committed
- (.andromeda/security-plan-detail/data-protection.md:14) Where the sweep's record names the recording and replay witnesses, the committed checks must stay read only through the strict reader and pinned byte for byte to a fresh recording — the `stand_replay` and seven_guis `recording` targets inside `stand-repeat`.
  > read only through the strict `Check::from_bytes` (`stand_replay`) and compared byte for byte with a fresh recording (seven_guis `recording::the_committed_checks_are_what_the_binary_records`)
- (.andromeda/security-plan-detail/data-protection.md:14) If the sweep regenerates or touches the committed checks, they must keep holding no typed text, host path, name or secret and no `screenshot` step.
  > holding the Counter's ids, names, values, snapshot text and diffs and the scripts' calls — no typed text, host path, name or secret (both read by the operator, 2026-10-19) and no `screenshot` step (a shot's PNG bytes differ by OS, as measured at CI#39225375577)
- (.andromeda/security-plan.md:185) The body's broad line for kept checks binds the sweep's recording runs: masked answers, never a pixel, owner-only, never logged.
  > - **Files (kept checks):** under `--record <dir>` on `escher-session` or `Session::recording_into`, one `check-{n}.json` (`0600`) per recorded session in a `0700` directory the launcher named — per call its verb, arguments, acceptance, digest and masked answer, never a pixel; owner-only, except the two committed checks (below); never logged, never removed by escher but for its own check when a recording ends.
- (.andromeda/security-plan.md:187) The body's broad line for committed checks binds: no check of any other session is committed by this chunk.
  > a check of any other session stays owner-only and is never committed.
- (.andromeda/security-plan.md:190) The "new verbs' stderr id-free" measurement reads the sink this line names: escher's sink must redact, and drop a record from a target outside its allowlist, on the new verbs' and the recording's runs.
  > escher's own log sink now redacts and, for a record from a target outside its allowlist, drops (escher-telemetry's target allowlist — see Logging & Monitoring).

## Bindings
- **Kept checks (data at rest) ↔ body §Input Validation (Driver recording directory)**: the recording directory a sweep check hands `--record` must be created `0700` (a group- or other-open one refused) and nothing of it logged, matching the at-rest medium (.andromeda/security-plan-detail/data-protection.md:8) and (.andromeda/security-plan.md:71).
- **Committed checks ↔ body §Input Validation (Recorded check)**: the committed checks are read only through the strict `Check::from_bytes`, each replayed step still passing `validate` (.andromeda/security-plan-detail/data-protection.md:14) and (.andromeda/security-plan.md:73).
- **Kept checks never logged ↔ body §Logging & Monitoring (escher's own sink) ↔ obs plan**: the stderr id-free measurement of `lint`, `part` and `--record` at `trace` under both builds reads the allowlist sink (.andromeda/security-plan.md:190) and (.andromeda/security-plan.md:396).
