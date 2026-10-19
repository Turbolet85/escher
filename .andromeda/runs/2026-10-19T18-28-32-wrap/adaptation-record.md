# Adaptation record — the 2026-10-19T18-28-32-wrap 0-pending wrap

**Path:** Setup step 6, no-op path · route-resolve §Operator-requested adaptation. No chunk in flight
(`route.py cursor`: records 52 · complete 52 · pending 0 · gated 0).

**Authority:** the founder by question dialog in the escher overseer session, 2026-10-19 about 18:25Z, relayed by the
overseer through the operator — «Небольшой чанк сейчас (Рекомендую)» — snapped as inputs#I1
(`inputs/I1-epoch3-audit-corrective-2026-10-19.md.txt`, kind relay, copy no-repo). The operator's invocation dictated
the placement, the form (one markerless entry, WHAT-not-HOW, ≤25 words) and the citation of the audit run.

**Basis:** run `2026-10-19T11-16-25-code-audit` (Epoch 3, trend), proposals M1 and M2 (`proposals.md` there), as the
relay states them:
- M2 — files over 800 code lines 14 → 15 → 17; the two new ones escher-driver's `cli.rs` (876) and `recording.rs`
  (831), about half of each an inline test module.
- M1 — duplication 3.679 → 3.86 → 3.88 %, almost all in tests; `host_log` and `mcp_log` share fragments, 95 lines in
  all. `stand_replay`'s re-declared Counter (59 lines) is a deliberate variant and not in the entry's scope.

## Items

1. **Route insertion** — one markerless entry in `escher-0.2.0/working-route.md`, Epoch 4, at line 34, right after
   «Upstream sync ahead of visual lint and real-app readiness» (line 32), before «Shared fixtures»:

   > Test splits — inline test modules out of escher-driver's `cli.rs` and `recording.rs`; `host_log`/`mcp_log`'s
   > shared fragment stated once; behaviour unchanged (audit 2026-10-19T11-16-25-code-audit M1, M2)

   22 words, modelled on 0.2.0's frozen «File splits» line. Disposition: **applied**. The entry is not ahead of the
   first markerless entry, so no PREREQ or WATCH re-pins; the Epoch 4 head's four CARRY blocks stay on it.
2. **Requirement** — none: the corrective is behaviour-neutral and adds no capability; `requirements.md` and the
   verification matrix untouched.
3. **Gated re-check** — no `gated` record; nothing to re-verify.
