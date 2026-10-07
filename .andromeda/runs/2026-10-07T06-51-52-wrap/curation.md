# Curation — 2026-10-07-driver-session (wrap run 2026-10-07T06-51-52-wrap)

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  corrected in place: "Inside a Bash tool call `grep` is a shell function running Claude Code's embedded ugrep (`/usr/bin/grep` is GNU grep) …" (confidence 0.7, unchanged)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "`$TMPDIR` is unset in the Bash tool's shell on this host" (confidence 0.8)
                                              + "A per-crate `cargo clippy --all-targets` is not the CI lint leg" (confidence 0.8)
                                              corrected in place: "Inside a Bash tool call `grep` is ugrep, and a long bounded repetition can print nothing"
  Filters: 3 dup (a master or a rule body of this wrap already carries them) · 1 below threshold · 0 conflict · 0 deferred
  No-other-home: "`$TMPDIR` is unset in the Bash tool's shell on this host" · "A per-crate `cargo clippy --all-targets` is not the CI lint leg"
```

## Corrections (exempt from the cap)
- **Tier 1, the grep bullet (2026-10-07)** and **Tier 3, its detail entry (2026-10-06)** — both said "on this host `grep` is ugrep". The operator's technical correction (2026-10-07, `inputs#I3` direction 3 of the chunk): the host's grep is GNU; inside a Bash tool call `grep` is a shell function running Claude Code's embedded ugrep; use `command grep`.
  Proof: measured in this session's Bash tool, 2026-10-07T06:17Z — `type grep` reads `grep is a function`; bare `grep --version` reads `ugrep 7.8.4`; `command grep --version` and `/usr/bin/grep --version` both read `grep (GNU grep) 3.12-modified` (`escher-0.1.0/chunks/2026-10-07-driver-session/evidence/wrap-directions.md`).

## New entries
- **Tier 3 — `$TMPDIR` is unset in the Bash tool's shell on this host.** Signals: verified by a real failure +0.4 · specific technical detail +0.2 · no other durable home this wrap +0.2 = 0.8. Host/shell mechanics with no rendered host rule file; one sentence of directive plus its mechanism — Tier 3 by the lower-context-cost tiebreaker.
  Proof: implement's first build command of the chunk redirected to `$TMPDIR/build1.log`; the shell printed `/build1.log: Permission denied` and the pipeline's cargo stage read exit 101, while the same build with a scratchpad path exited 0.
- **Tier 3 — A per-crate `cargo clippy --all-targets` is not the CI lint leg.** Signals: verified by a real failure +0.4 · specific technical detail +0.2 · no other durable home +0.2 = 0.8. A gotcha about an invocation, spanning no one rule file's paths — Tier 3.
  Proof: `cargo clippy -p escher-driver -p seven_guis --locked --all-targets -- -D warnings` exited 101 on `needless_return` at `packages/blitz-dom/src/mutator.rs:1298`, while `bash .github/scripts/ci-leg.sh fast` (its clippy leg) was green on the same tree three times.

## Filtered
- dup — "a nothing-in-the-logs check over a host that installs no sink passes vacuously": this wrap's P2 wrote it into test-plan §5, obs-plan §3 and the body of `rules/observability.md`.
- dup — "a sink-installing host at `trace` overruns an undrained pipe": written into test-plan §2 (Process-lifecycle checks).
- dup — "an error message reading 'I/O' holds a `/`": written into test-plan §4.
- below threshold — "a pasted message stating no author is not acted on before the operator confirms it": a one-off of this session, 0.2; it is recorded in the chunk report's Decisions & corrections.
- other home, not a candidate — "PROVISIONAL items are discharged in a batch at each epoch boundary": minted as a playbook rule at P2.

Carried, still unreviewed (from earlier wraps, not this session's): "A chunk that moves cited source lines stales the masters' file:line citations" (Tier 3, 2026-10-05) · "A grep hit seen through a clipped view is not read" (Tier 3, 2026-10-06) · "Count from the listing you just read, never from the plan's forecast" (Tier 3, 2026-10-05).
