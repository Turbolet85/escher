# Cascade dispositions — 0-pending wrap, 2026-10-06T16-16-18 (the founder's PROVISIONAL batch)

**Pass:** four rulings by the founder, 2026-10-06, given here (the operator's `/andromeda-wrap-session` invocation applying the relay `founder-provisional-batch`, with option (c) defined in the invocation's own words):
1. the `coverage-report` upload widening — ratified;
2. the falsy-`disabled` engine fix — ratified;
3. the cold-agent run pipe's three widenings (the spawned `claude` client with its stdio MCP stub · the outbound model path · the operator's own Claude Code login) — ratified;
4. opt-in OTel export — option (c): no OTel export in escher 0.1.0; neither the export transport nor the credential path is decided.

**Body edits (step 1):**
- `architecture.md` lines 66 (§Stack, CI/CD row), 139 (CI contracts), 145 (Outbound hosts), 148 (Process-wide state and threads): `(PROVISIONAL, overseer under the founder's standing delegation, pending the founder's word` → `(ratified by the founder, 2026-10-06`.
- `security-plan.md` lines 68 (Input Validation, cold-agent stub row) and 260 (Secrets, Development): the same replacement.
- `registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md:3` (obs-plan §3 → Bootstrap phases, `otel-sdk-install`): "the opt-in export was deferred … decision for the founder … carried to "Driver command spans"" → escher 0.1.0 ships no OTel export (founder, 2026-10-06), transport and credential path undecided, a residual in `.andromeda/residuals.md`. `registry.py check` 0 defects.
- Rulings 1 and 2: no body edit. Every master statement of the `coverage-report` upload (arch 139 · 147 · 177, test-plan 138 · 304, obs-plan 321) and of the falsy-`disabled` clearing (arch 134 · 259, test-plan 25 · 314, a11y-plan 179) already states the behaviour as current truth with no PROVISIONAL clause; the PROVISIONAL status lived only in the sidecars, so the ratification is a sidecar record.

**The search (step 2):** `cascade-patterns.toml` — 10 patterns: `provisional`, `pending-word`, `standing-deleg`, `otel-carried`, `otel-headers`, `optin-export`, `founder-decide`, `otel-sdk`, `cov-report`, `falsy`; each control fired on the pre-pass masters/registries (baseline 47bbf38f). Listing before leaf edits: `sweep.txt`; after: `sweep-after.txt`. Not looked for: the chunk folders' prose records (plans, reports) — frozen chunk history, not master text or a leaf.

**Rows:**
- `provisional` / `pending-word` — masters 0 (all six body clauses replaced). Leaves → **re-derived**: `rules/security.md:8`, `rules/security.md:19`, `rules/verification-harness.md:26`, `docs/security-summary.md:18`, `docs/stack.md:43` — the PROVISIONAL clause now reads ratified by the founder, 2026-10-06. After: 0 rows.
- `standing-deleg` — 0 rows after the body edits (control fired).
- `otel-carried` / `founder-decide` — leaves → **re-derived**: `docs/obs-summary.md:15`, `docs/obs-summary.md:49`, `docs/security-summary.md:38`. After: 0 rows.
- `otel-headers` / `optin-export` / `otel-sdk` — the key file (`standing edited`, the new text) and `obs-plan-contracts.toml:9` (the row's label, true — the key keeps its label) · **no change**; leaves `rules/observability.md:36` → **re-derived** (split: the driver spans stay owned by "Driver command spans"; the OTel export is out of 0.1.0 → residuals), `docs/security-summary.md:38`, `docs/obs-summary.md:15/49` re-derived above. After: every remaining row is this pass's new text.
- `cov-report` — 6 master rows + 3 leaves (`docs/obs-summary.md:32`, `docs/stack.md:41`, `docs/tests-summary.md:40`): true claims carrying no provisional status · **no change**.
- `falsy` — 5 master rows + 6 leaves (`rules/a11y.md:30`, `docs/a11y-summary.md:22`, `docs/services/dioxus-native-dom.md:19/31/37`, `docs/tests-summary.md:21`): true claims carrying no provisional status · **no change**.
- curation 0 · base 0 across every pattern.

**Leaves (step 3):** re-derived only where a statement derives from an amended passage (the nine lines above). CLAUDE.md `GENERATED:setup:*` blocks: the warnings block names the operator's Claude Code login with no PROVISIONAL clause and no OTel claim — no change. `docs/commands.md`, `docs/gotchas.md`: no row for any pattern — no change.

**Route (P5, the operator-requested adaptation):** `working-route.md:60` ("Driver command spans") loses its `CARRY: otel-sdk-install …` block, now a residual; `working-route.md:86` ("Cold-agent test") — its CARRY's "PROVISIONAL pending the founder's own word (FOR DISCUSSION 7)" clause rewritten to the ratified state (factual, the premise measured false by this ruling).
