# Apply progress — 2026-10-25-cold-agent-bar wrap, P2 Apply (window 3, stopped at the 85 % context alarm)

The resume of window 3 (P2 Apply, Validate finished — fanout-results.md and answer-card.md stand) reached this point.
Nothing here is a sidecar entry; the entries are still to be written.

## Done in window 3
- **Spec bodies applied, all seven masters**, each read back: `detail.py check` 0 defects (six extracted docs),
  `registry.py check` 0 (obs, test, a11y), `classify.py check --mode standing --base auto` PASS on every edited body and
  category file (layout-templates gained its first labels: one `values · desktop-native-screens` group and its subject
  line; design-system's subject line gained `desktop-native-stand-load-page`). The diff against HEAD is the record of
  what was written; long-line edits went through `cascade.py replace` (`replace-2.md` layout-templates :10 · :75,
  `replace-3.md` test-plan ci-integration :8), every other edit an exact-once scripted replace that refused any new
  line over 2,000 chars.
- **`detail.py tally`** run for the six extracted docs (`--no-fit 0`), rows in `cascade-tally.tsv`.
- **Cascade step-2 sweep fired** with `cascade-patterns.toml` (19 patterns, old wording of every amended claim); its
  output was not yet read when the window stopped. Re-fire it at resume (its trail `cascade-2026-10-25-cold-agent-bar.json`).

## Applied as re-derived, not as the proposal's change line (name each in the sidecar or cascade-dispositions)
- architecture g5 18 · test-plan g2 10: `WITNESSES_020` stays **nine rows**, 46 → **49 witnesses** (read in
  `stand_requirement_sweep.rs`: `[(&str, &[Witness]); 9]`, 49 witness tuples; 46 at base `e84dc818`) — the chunk added
  three witnesses and the `Served` subject, not three rows.
- obs-plan cascade 11 (`cells_needles` silent): the chunk's silence census (plan.md:277) read `stand.rs`,
  `flight_booker.rs`, `stand_cells_served.rs`, `stand_load_page.rs`, blitz-tests `common/mod.rs` and `fixtures/mod.rs`
  — not seven_guis `tests/common/mod.rs`; the census line written into obs-plan §3's absence block names those files
  only, and no silence claim was written for `cells_needles`.
- Provenance kept out of every body (classify writing rule 7): no `since 2026-10-25-cold-agent-bar`, no founder
  quote, no date; each ratification goes to its sidecar entry's Why.
- Newest reading only (amendment-flow §Apply): architecture stack-and-technologies :73 `claude` 2.1.288 · 2.1.296 →
  2.1.308 alone; test-plan §3 Cold-agent **Proof** now holds the 2026-10-25 run alone — the 2026-10-15 run (43 calls,
  eleven tools, 0.3197 USD, 2.1.296) and the stub run (0.0418 USD) leave the body (`was …` in the sidecar Change);
  a11y ci-integration :65 and test-plan ci-integration :65 hold the 85-test reading alone; quality-gates :16 the
  2741/3202 · 952/1008 reading alone.
- Q3 (a11y-plan 3): an `OPEN — … · owner: the served pages' keyboard and contrast coverage, a .andromeda/residuals.md
  line` gap line written after a11y-plan.md's §5 `stand_keyboard` bullet; the lint overlap defect is stated as
  «owned by a `.andromeda/residuals.md` line» in layout-templates :46, standard-contracts (new driver-session group),
  test-scope-summary (new group and :123), visual-design-verification. **Both residuals lines must be written at P5**
  (and mirrored into `escher-0.3.0-incubator/deferred-from-0.2.0.md`, the operator's directive at invocation).
- Q4: test-plan.md §5 Wall time names the stand-side sleep (examples/seven_guis/src/stand.rs:171); test-plan 10 not
  applied (rejected).
- A sibling site the proposals missed, fixed in the same pass: test-plan.md:334 «a no-op or absent provider — the
  stand's —» → «every stand boot's but `boot_load`'s».

## The rulings each sidecar Why must name (authority as recorded in answer-card.md)
- Q1 — the founder's own word «Подтверждаю (Рекомендую)» (relayed by the operator at the P2 halt, 2026-10-25), on his
  F3 «Один запуск, до $3 (Рекомендую)» (inputs#I3) and I6 «Запускать сейчас (Рекомендую)» (inputs#I6): the live run's
  carriage of the new screens — security-plan (threat-model-summary :35/:37, body :27), architecture
  occupied-resources (outbound-hosts group, process-wide :40), obs-plan §8 body and pii-scrubbing :32.
- Q2 — «Внутри, записать (Рекомендую)», the founder: the pipe as a second reader of its live child's kept checks,
  inside the 2026-10-19 recording ratification, holding bools and counts only — security-plan body :63 · :185,
  input-validation (Recorded check row), data-protection.
- Q3 — «Записать пробел, владелец на P5 (Рекомендую)», the founder: the load page's open keyboard/contrast gap.
- Q4 — the overseer's technical call, verified in the tree.
- I5 — «Перезакрепить как измерено (Рекомендую)», the founder: `stand_lint` re-pinned as measured.

## Done in window 4
- Sweep re-fired (132 rows over 40 files); seven stale master sites folded (`replace-4.md`, `replace-5.md`, the snapshot
  sizes re-measured in `snapshot-sizes.md`); `cascade-dispositions.md` written; checks 0 defects / clean / PASS.
- Sidecar entries: 18 appended (architecture 4, security-plan 3, design-system 1, layout-templates 1, test-plan 5,
  obs-plan 2, a11y-plan 2), each `sidecar.py check` on-form, `splice.py append`, last line and block read back.
- Leaves: whole mode (Epoch 5 closes), 13 comparers, check 13 of 13 pass; 152 findings — 148 applied, 1 by hand
  (blitz-dom.md), 3 declined; `## Leaves` in cascade-dispositions.md. Reconcile evolve checkpoint appended (5 records).
- P2 summary: 226 amendments applied + 7 sweep folds · 4 escalations (12 proposals) resolved · 0 rejected for source ·
  citations 297 re-pointed · 22 changed · 104 stretched.
- P3: T2 testing.md 2026-10-10 entry extended; `curation.md` written; checkpoint appended.
- P4: code-graph `.refresh-done` rust ok 8866/54433 (17:41:11Z).
- P5: residuals.md:38 premise-corrected; :39 lint overlap · :40 served-pages gap · :41 capabilities.md (target 0.3.0,
  the founder's «Строка в residuals (Рекомендую)», inputs#I7) appended; mirrored into the 0.3.0 incubator;
  checkpoint appended. Next: P6 state + handoff, then P7.
- (resolved at P5) P5 card row 3 (inputs#I4) — `docs/capabilities.md:288-289` restatement never made at P1; untouched,
  still reads «not re-fired since `lint`». A wrap takes a directed source edit only before P1 → ask the operator.

## Next, in order (SKILL.md P2 Apply step 2 onward)
1. Re-fire the sweep; write `cascade-dispositions.md` from its listing (rows by class; the true lean-four claims,
   the engine's `aria-pressed` read and the frozen archives are no-change), fold any stale master row into this pass.
2. Sidecar entries, one per doc (several where over 3,000 B), each `sidecar.py check --entry-file` then `splice.py
   append`, read back; `Ref: .andromeda/runs/2026-10-25T17-25-04-wrap/`.
3. `cascade.py leaves` (pass mode; this chunk ends Epoch 5 — `route.py epoch` decides `--all`), the leaf batch, the
   decisions, `## Leaves` in cascade-dispositions; then the P2 evolve checkpoint (references/evolve/reconcile.md).
4. P3 curation · P4 code-graph · P5 route-resolve (residuals: rewrite :38 `[premise-corrected: …]` per inputs#I4 row 1;
   append the two open lines above; mirror into the incubator) · P6 state + handoff · P7 gates, flip, compaction,
   epoch-close consolidation, hygiene, commit, push.
