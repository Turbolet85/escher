# Merge decisions — escher-0.2.0 route (Phase 3)

Sources: the five `validation-*.md` files and the founder's word in `founder-word-readme.md`.
Each suggestion is one line: `{validator} {kind} · {decision} · {reason / adjustment}`.

- founder Rewrite (Handler splits) · applied · The README update joins the first entry. It shows 0.1.0 done and 0.2.0 planned, and writes «Escher» capitalized in prose (crate and binary names stay lowercase). This is his word, relayed by the operator. To fit 25 words the line drops "escher's" and "audit M1", and the arms named are escher's range and activation arms.
- design — · no suggestions · confirmation populated.
- obs Rewrite (File splits) · adjusted · The line gains "spans unchanged" (the `tool_call` / `command` fields and nesting). The detail goes to the chunk's phase.
- obs Rewrite (4 upstream syncs) · adjusted · Merged with the tests and security sync rewrites into one wording: "CI and audit green, per-OS tallies re-read, sink allowlist re-checked". The `(intent §Principles)` cite is dropped for length.
- tests Rewrite (4 upstream syncs) · adjusted · Merged as above (per-OS tallies).
- security Rewrite (4 upstream syncs) · adjusted · Merged as above (audit and licence legs = "audit green"). Lock review and "no advisory ignore added" go to each sync's phase.
- obs Rewrite (Session recording) · adjusted · Merged with security's as "never logged".
- security Rewrite (Session recording) · adjusted · The line reads "owner-only" and "password handling per security-plan". Holding typed text as the mask is NOT decided here. A replay needs the typed text, so the recording is a new carrier (ids, names, typed text at rest) and a founder crossing at that chunk's phase. It is surfaced in Phase 4.
- tests Rewrite (Recorded-check replay) · adjusted · The line names agent-run, CI and stand-repeat.
- security Rewrite (Recorded-check replay) · adjusted · Merged as "steps re-validated". A malformed check refused unreplayed is the phase's acceptance.
- tests Insert (Shared fixture bootstrap) · adjusted · The direction is right: fixtures are infrastructure ahead of Load wait, Select and Large screen. It is placed at the head of Epoch 4 after its sync, NOT in Foundation, because the founder's order puts Engine debts "right after the improvement" («Один пункт «долги движка» (Рекомендую)»). The engine-debt fixtures stay per-file, as in 0.1.0. Title: "Shared fixtures".
- tests Rewrite (Visual lint) · adjusted · Merged as "fixture-proven". A clean stand reading none is the phase's acceptance.
- a11y Rewrite (Visual lint) · adjusted · Merged as "text contrast as `stand_contrast` reads it". The SC id is omitted as domain content.
- obs Rewrite (Visual lint) · rejected · Already a standing invariant: obs-plan §4 never fields an `Outcome`. It goes to the chunk's phase.
- security Rewrite (Visual lint) · rejected · Already a standing invariant: password and file values leave only as the mask (security-plan §Input Validation). It goes to the chunk's phase.
- obs Rewrite (Load wait) · rejected · The busy class `loads` and the cause set are already closed words on the command span (obs-plan §4). A new word past the bound is the phase's call within that rule.
- a11y Rewrite (Select by id) · adjusted · The line reads "role and name, Tab order kept". The role names and SC id are omitted as domain content.
- tests Rewrite (Stand requirement sweep) · adjusted · Merged as "inside stand-repeat; coverage floors held".
- obs Rewrite (Stand requirement sweep) · adjusted · Merged as "new verbs' stderr id-free".
- a11y Insert (Tier-0 a11y re-read) · adjusted · Inserted between the sweep and the Capability statement as "A11y re-read", with the SC ids omitted as domain content and a11y-plan §9 cited.
- a11y Rewrite (Handler splits) · rejected · Covered by v020-01's "every check stays green", and `stand_keyboard` is among those checks. The line has no room once it carries the founder's README. It goes to the chunk's phase.
- a11y Rewrite (Engine debts) · rejected · Naming the SC per debt is domain content, and the line is at 25 words. Each fork names its SC at the chunk's phase.
- a11y Rewrite (Capability statement) · adjusted · Merged with security's as "a11y reach".
- security Rewrite (Capability statement) · adjusted · Merged as "what MCP client carries onward".
- security Rewrite (Cold-agent bar) · adjusted · The line reads "via the cold-agent pipe, operator's login". "On the stand task allowlist" is NOT written, because it conflicts with tests' rewrite.
- tests Rewrite (Cold-agent bar) · deferred · The conflict: tests wants the pipe's task allowlist extended to 0.2.0's fixtures, while security wants it kept at `stand`. Extending it carries fixture screens to the model provider, beyond the stand the founder ratified («Один запуск, до $3 (Рекомендую)»). That is the founder's crossing call, so it is surfaced in Phase 4.

Tally: 27 validator suggestions on 24 lines (security's four sync rewrites share one line), plus the founder's word. By line: applied 1 (the founder) · adjusted 18 · rejected 5 · deferred 1 · design's confirmation 1. Two items are surfaced in Phase 4: the recording's typed-text carrier (security) and the cold-agent task reach (tests ↔ security).
