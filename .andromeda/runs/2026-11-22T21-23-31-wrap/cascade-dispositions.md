# Cascade dispositions — 2026-11-21-dependency-and-ci-debts (gated wrap)

## Step 2 — the sweep
Skipped: no pattern file. The one landed operation (tx 3155, test-plan `decision/hosted-runner-cancel-hand-rerun` · `state` provisional → standing) wrote none: `patterns: none: 1 set(s) and retire(s) changed prose alone — no file; step 3's leaves read each changed row's delta`. security-plan's dry run landed nothing (`patterns: none — the call sets and retires no row`). The 13 ratification entries (tx 3156–3158) are sidecar entries, no master row. The pre-pass site sweep of P1 (`report-patterns.toml`: `hand-rerun`, `memmap2`) saw the claim's master sites (test-plan 1, security-plan 1) and two leaf rows (`.claude/rules/security.md:26`, `.claude/docs/security-summary.md:42`) stating the memmap2 limit, true as written — no change. Binds: test-plan §3 ↔ obs-plan §3/§6 and a11y ↔ log schema untouched (no harness or schema row moved).

## Leaves
Mode: WHOLE (`cascade.py leaves --all`): this wrap closes Epoch 2 «Debts of 0.1.0–0.3.0» (`epoch · «Epoch 2 — Debts of 0.1.0–0.3.0» · markerless 0 · gated 1 · open` — the one gated record is this chunk's, flipped `complete` at P7). `sweep leaves · none — no pass sweep of this marker`: no `swept` row was printed, so no comparer had a sweep line to answer, and no `@leaf_stands` record was written. `unmapped .claude/rules/host-linux.md (host)` — the host leaf, a statement no master states: no comparer's.

Total (the tool's line): `total · blocks 24 · applied 15 · held 9`; by hand 0.

| group | findings | applied | declined |
|---|---|---|---|
| a11y-plan-1 | 0 | 0 | 0 |
| a11y-plan-2 | 1 | 0 | 1 |
| architecture-1 | 2 | 1 | 1 |
| architecture-2 | 0 | 0 | 0 |
| architecture+layout-templates | 1 | 1 | 0 |
| architecture+obs-plan | 0 | 0 | 0 |
| architecture+security-plan+test-plan-1 | 3 | 3 | 0 |
| architecture+test-plan | 1 | 1 | 0 |
| design-system+layout-templates | 1 | 0 | 1 |
| masters | 3 | 1 | 2 |
| obs-plan-1 | 0 | 0 | 0 |
| obs-plan-2 | 1 | 1 | 0 |
| security-plan-1 | 1 | 1 | 0 |
| security-plan-2 | 5 | 3 | 2 |
| test-plan-1 | 2 | 1 | 1 |
| test-plan-2 | 3 | 2 | 1 |

Declined, each with why:
- `.claude/rules/a11y.md:33#1` — a row-height detail; no reader of the leaf acts against the source without it; it stays in its master.
- `.claude/docs/conventions.md:39#1` — the redraw scope of `DocumentMutator` is a detail that stays in its master.
- `.claude/docs/design-summary.md:27#1` — the order page's built-to-cause controls are a detail the master holds; the leaf points to it.
- `CLAUDE.md:92#1` — false as written: `refusal.rs` and `describe.rs` still hold their unit tests inline (`mod tests {`), so «each module's unit tests in a `#[path]` child file of its own» does not hold. The line's «unit tests inline» is stale for `command.rs` alone (`command_tests.rs`, its `#[path]` child): named and carried to «Upstream sync ahead of the canvas» (route-resolve CARRY), not fixed by hand.
- `CLAUDE.md:93#1` — false as written: `lint.rs` holds its unit tests inline (`mod tests {`); the leaf's «unit tests inline» stands true.
- `.claude/rules/security.md:17#1` — the selectedness algorithm is a detail that stays in its master.
- `.claude/rules/security.md:28#1` — the engine pins stand in CLAUDE.md's stack line; no reader of the security rule acts against them.
- `.claude/docs/tests-summary.md:24#1` — the `graph` part grammar is a contract detail the master's key holds.
- `.claude/rules/testing.md:48#1` — its replace-with adds a reason («a split moves the pinned target layout the CI legs read») that no source states; the omission of the kept-whole rule for over-800 integration targets is named and carried with the CLAUDE.md:92 line, not fixed by hand.
