
## 2026-10-13-upstream-sync-polish-and-ship — the merge's five targets join the no-subscriber census; the founder's word respelled
**Section:** §3 Observability Harness Contract → Logging stack (subscriber installation) · §4 Span / Trace Coverage (the `tool_call` span)
**Change:**
- §3: the census gains the five upstream integration targets of the `434e0661` merge — `anonymous_block_restyle`, `button_alignment`, `checked_attribute`, `pointer_leave`, `ua_stylesheet_order`. Each installs no subscriber, reads no env var and prints nothing; none installs escher's sink; `host_log` and `mcp_log` pass under both builds after the merge.
- §4: the `tool_call` span's ruling now reads "relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's" beside «Span с tool и ending», the form of the entry it copies.
**Why:** the merge added no log site, span, env read or print (the fixed-sha census reads 0), so the census grows by five rows and nothing else moves. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's); the respelling was folded at the cascade sweep under playbook rule A, which he added at the same card.
**Kept:** §6's sample closed-span line still reads `service.version=0.3.0-beta.2` — a capture dated to 2026-10-07-driver-command-spans, true of that run.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/
