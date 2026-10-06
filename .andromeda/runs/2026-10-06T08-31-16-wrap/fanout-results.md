# Fan-out results — 2026-10-06-upstream-sync-element-identity

Seven Explore doc-agents, one parallel batch, prompt from amendment-flow.md sent verbatim (contracts lines for
test-plan, obs-plan, a11y-plan from `registry.py contracts`; architecture NOT MIGRATED; the other three carry no keyed
contract). Returns stripped of their `#` commentary; no entity escapes in any return (entities=0); no raw twin warranted.

## Verdicts
- architecture — 2 proposals (A1, A2). Stripped commentary: D-arch-resources no drift (no port/socket/env var/crate;
  new upstream APIs fall under the `__blitz_` prefix rule and the JS bootstrap summary); dependency rev bumps keep
  "pinned rev" true; left §Origin and the citation re-points to the orchestrator.
- security-plan — 0 proposals. Stripped: D-security-input no drift (document.children read-only; textContent plain
  text; CSSStyleSheet.disabled `to_boolean` coercion, owner id from the sheet's own data); D-security-auth n/a;
  D-security-deps no drift; flagged security-plan.md:321 (disproved #2) and :213 (disproved #3) to the orchestrator.
- design-system — 0 proposals. Stripped: no new UI element, every surface `tokens n/a`; 33 citation shifts left to the re-point.
- layout-templates — 0 proposals. Stripped: no new surface or region; 8 citation shifts left to the re-point.
- test-plan — 3 proposals (T1, T2, T3). Stripped: tier 0 mandates no tier for new paths; runner/framework and
  harness unchanged; 35 citation shifts and test-plan.md:311 (disproved #3) left to the orchestrator.
- obs-plan — 0 proposals. Stripped: no hot-path op, no telemetry dependency, no logging added (probe 0); 32 shifts
  and 6 root-Cargo.toml citations left to the orchestrator.
- a11y-plan — 0 proposals. Stripped: no interactive element; violation schema still not defined against obs §6;
  32 shifts and 1 root-Cargo.toml citation left to the orchestrator.

## Proposals and dispositions
- **A1** D-arch-decisions · §Established Decisions → [CSS approximations] · container `align-items`/`justify-items`
  `normal` (and `auto`, an unknown flag) map to Taffy's `AlignItems::NORMAL`, resolved by the aligned box's layout
  mode; item-level `normal` → `NORMAL` (was stretch); basis convert.rs:594-599 (carried by the report).
  → **apply** — check 1: playbook "Accurate this-chunk addition" (the merged change is in Changes → Symbols/APIs; the
  body reconciles to it); check 6: disposes disproved #1. Text re-derived from the report, not pasted.
- **A2** D-arch-decisions · §Established Decisions → [CSS approximations] · the `safe` positional content-alignment
  default also covers table cells; cite convert.rs:443-454 (carried by the report).
  → **apply** — check 1: "Accurate this-chunk addition".
- **T1** D-tests-coverage · §9 CI Integration → Local baseline · re-count 431 · 0 · 4 (120 result lines, +1
  `subtest_names_include_nonempty_root_titles`, upstream 3aa87bc1), earlier history kept.
  → **apply** — check 1: "Accurate this-chunk addition"; check 5: expected amendment 2 (the plan names the change).
- **T2** D-tests-coverage · §1 Coverage scope (wpt/runner unit-test inventory) · add "1 in attr_test.rs" with a
  citation `attr_test.rs:262-293`.
  → **reject** — re-derivation tell: `basis`/`change` cite a source location (`attr_test.rs:262-293`) the report
  does not carry. The fact is real and the report carries it (the test's module path and name) → re-raised by the
  orchestrator as O4.
- **T3** D-tests-coverage (dependent-of T2) · §4 What unit tests cover (wpt/runner) · add attr_test.rs's subtest
  naming with a citation `attr_test.rs:262-293`.
  → **reject** — same tell; dependent of a rejected primary. Folded into O4 without the uncarried locator.

## Orchestrator raises (check 5 · check 6)
- **O1** expected amendment 3 — every file:line citation into the 17 merged paths re-pointed by the measured line map
  (report Changes → Counts / qualifiers moved), all seven masters; registries 0. The 6 numbers inside rewritten hunks
  set by reading HEAD: architecture.md:85 `convert.rs:432-435` → `436-439`, `510-512` → `546-548`, `522-534` → replaced by
  A1's text; security-plan.md:321 `436-437` → `440-441`, `513-514` → `549-550` (with O3); architecture.md:73
  `Cargo.toml:114` — in-place line, handled by E1. → **apply**, first (before the semantic amendments, per the
  2026-10-05 Tier-3 learning). Check 1: the plan's expected amendment names the change itself.
- **O2** expected amendment 1 — architecture §Project Intent "Origin": record the last upstream sync
  `2335458530518cdf167c55ce635fae99323e0789` beside the adoption base. → **apply** (check 5; the plan names the change).
- **O3** disproved #2 — security-plan §Error Handling (line 321): unknown alignment flags still never panic; content
  alignment maps one to Taffy's `AlignContent::NORMAL` (convert.rs:440-441), item alignment to none (convert.rs:549-550).
  → **apply** — check 1: "Accurate this-chunk addition" (reconciles mechanism wording; invariant "no panic" holds); check 6.
- **O4** (re-raised from T2/T3) — test-plan §1 wpt/runner inventory gains "1 in attr_test.rs"; §4 wpt/runner coverage
  gains attr_test.rs's test that checkLayout subtest names include non-empty root titles (named by the report:
  `test_runners::attr_test::tests::subtest_names_include_nonempty_root_titles`, upstream 3aa87bc1). → **apply** —
  check 1: "Accurate this-chunk addition"; `#[test]` count per file re-read: fuzzy 8 · js_wrapper 3 · harness_test 2 ·
  attr_test 1 · mod 1.
- **E1** disproved #3 — 55 root `Cargo.toml:N≥61` citations in five masters; 53 predate `1503df2e`'s insert at line 61
  and are one line low (46 verified by content `Cargo.toml@1503df2e^:N == HEAD:N+1`, 7 on the pin lines the merge
  rewrote in place verified against `ab936a9e:N+1`); 2 written after the insert (architecture.md:105/:152
  `Cargo.toml:61`) are correct. Playbook "Not this chunk's drift" matches the subject (pre-existing, not in this
  chunk's Changes as its work), and its note routes such a family to an owned channel rather than a one-artifact
  amend; the fix itself is mechanical and verified. → **escalate** (the routing vs fixing-now choice is the operator's). **Resolved:** fix now — the operator, 2026-10-06, chose "Fix now (Recommended)" at this wrap's escalation: all 53 re-pointed +1 in this pass, verified by content, as a separate sidecar entry naming `1503df2e` as the cause; the 2 post-insert citations untouched.

## Validate checks
- check 2 cross-contradiction: none (A1/A2 edit one bullet in compatible directions; O1 runs first, then A1/A2/O3 re-cite).
- check 3 intent-consistency: no divergence — report Deviations none; scope record none; the merge is the intent.
- check 4 absence-needs-evidence: A1's "only at line 85" sweep (agent) re-run by the cascade sweep below.
- check 5 expected amendments: 1 → O2 · 2 → T1 · 3 → O1. All covered.
- check 6 disproved claims: #1 → A1 · #2 → O3 · #3 → E1 (escalated).
- Boundary widening considered: the three new script-reachable JS accessors are new realizations on the already
  registered script→DOM binding crossing (security detector: validated per §Input Validation's JS API rows); no new
  crossing, input class or channel → not the never-routine class.
