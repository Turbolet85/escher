# Evolve Diagnosis — escher-0.2.0 · Epoch 4 — Visual lint and real-app readiness · 2026-10-20T13:35:30Z

Run dir `.andromeda/runs/2026-10-20T13-35-30-evolve-diagnose/`. Evidence twins: `q-retractions.json` ·
`q-health.json` · `q-typed.json` · `q-untyped-raw.json` · `q-untyped.json` · `q-chains.json` · `q-facts-raw.json` ·
`q-level.json` · `q-lookback.json` (a read-only cross-epoch lookback). Scripts: `pass.py` · `lookback.py`.
Chunk markers below drop the `2026-10-1x-`/`2026-10-20-` date where unambiguous: **upsync** =
2026-10-19-upstream-sync-visual-lint-and-real-app-readiness · **test-splits** · **shared-fixtures** · **visual-lint** ·
**load-wait** · **select-by-id** · **large-screen** = 2026-10-20-large-screen-in-parts. Run-dir pointers name the run by
its timestamp under `.andromeda/runs/`.

## Mechanism health
- **Records:** 172 in the epoch label, 171 after the retraction pre-pass (106 step / 65 friction). All carry
  `version: escher-0.2.0`; the label is unique to this version, so no other version's records were split off.
- **Coverage:** 7 of 7 chunks fired every expected checkpoint (phase 5 · implement 3 · wrap 5). visual-lint fired
  implement and wrap twice (6 and 10) because of its gated acceptance pass; that is not a gap. There are 7 new-session
  `orientation` records with `chunk: null`, one per session start. Gaps: none.
- **Unparseable:** 0 lines in the whole ledger (1,534) · **malformed `ts`:** 0.
- **Untyped rate per step:** phase/plan 1/4 · phase/validate 1/8 · wrap/reconcile 2/21 · wrap/report 1/6 ·
  implement/fix-loop 1/4 · every other step 0. In total 6 of 65 (9%).
- **Problem-fact fill:** 20 of 106 step records carry facts (23 facts: 19 workaround · 2 overridden · 1 removed-cause ·
  1 unresolved; no `prohibition` and no `deferred`). **id fill:** 171 of 171.
- **Signal line:** `guard-refused 26 over 106 step records · exact-hit 10 over 8 curation records`. No signal string is
  malformed.
- **Retractions:** retracted 1 (`2026-10-20T01:37:33Z-c`, a visual-lint gates `contract.skill-reference-drift`). The
  load-wait take-up retracted it on the operator's correction, and that retraction record stays in as friction.
  clause-retracted 1 in the epoch (`2026-10-19T21:31:30Z-b`, note rendered in P6) and 4 ledger-wide.
  Unresolvable: 1, ledger-wide and not in this epoch: `2026-10-07T04:21:25Z-a` carries an `id: null` entry («in step
  record 2026-10-07T04:21:04Z-a (a step record, so n…»). It is reported verbatim for manual discount.
  Retraction-of-retraction: none.
- **Non-ok outcomes:** `halted-resolved` 5 (reconcile in visual-lint ×2, select-by-id and large-screen; gates in
  shared-fixtures) · `ok-degraded` 1 (select-by-id fix-loop).
- **Calibration boundaries in range:** the epoch runs from 2026-10-19T18:47Z to 2026-10-20T12:22Z, so every boundary
  in diagnosis-pass is live for it: the deviation scan, the universal types, `contract.in-pass-correction`, the
  `operator-pass` entry, the implement-outcome `ok` rule, the `new-text-*` words, `guard-refused`/`exact-hit`, and
  the 2026-10-19 consumed-grade rule (an `input.*` friction beside an `ok` grade is impossible). No reading in this
  document is an era reading.

## Proposals (typed patterns)

### P1 — */* (universal: recall) · `recall.corpus-recurrence` — 12 cases · weight 22
**Pattern:** 12 times, at the curation step of 6 of 7 chunks, a failure recurred although a curated learning already
stated its remedy. Two themes account for 8 of the 12: a count sweep that misses a count's other forms (5) and an
in-line edit beside a provenance parenthesis (3).
**Evidence:** ALL 12 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upsync | P1 site sweep keyed a count on a bare `\b575\b`, matched four line-range citations and none of the figure's sites, though the entry «A sweep pattern for a bare count also matches a citation's digits» states the remedy | extra_reads 4 | 2026-10-19T19-54-24-wrap/curation.md |
| upsync | two wraps left test-plan's per-OS tallies, baseline and run-all series unwritten; the report keyed its run-all sweep on the before-take value, missing the series' last written figure, though «A count the plan names may stand in no master» names the remedy | extra_reads 2 | 2026-10-19T19-54-24-wrap/curation.md |
| test-splits | shell grep refused a bounded repetition and printed nothing; re-read by Python. The T1 learning of 2026-10-07 and the global CLAUDE.md Shell rule both state it | retries 1 | 2026-10-19T21-24-54-wrap/curation.md |
| visual-lint | the 2026-10-19 Tier-3 provenance-parenthesis entry states the hazard; obs-plan.md:70's replace reproduced it (classify FAIL on «where eighteen stood»), fixed by moving the insertion | retries 1 | 2026-10-20T00-54-29-wrap/curation.md |
| load-wait | a phase P5 control script landed the repo root, a scratchpad path and a home path in its run dir, past the 2026-10-13 Tier-3 host-path entry | retries 1 | — |
| load-wait | a redirect through the unset `$TMPDIR` failed Permission denied, past the 2026-10-07 Tier-3 entry naming exactly that | retries 1 | — |
| load-wait | rustc read an import used only by an intra-doc link as unused, past the 2026-10-16 Tier-3 entry giving the path-form remedy | retries 1 | — |
| load-wait | P1 site sweep keyed reader counts on digits and missed «twenty-six checks» and arch's «by 26 … by 20», past the 2026-10-20 entry curated one wrap earlier | extra_reads 3 | — |
| select-by-id | the provenance-parenthesis entry states the rule; P2 apply reproduced it on test-plan.md:22, :68, :104 and session-lifecycle.md:11 (22 classify rows) | retries 3 | 2026-10-20T05-49-12-wrap/curation.md |
| select-by-id | the count-forms entry states the rule; P1 sweep still missed «twenty-seven checks» at obs-plan.md:70, caught by the obs-plan detector | extra_reads 1 | 2026-10-20T05-49-12-wrap/fanout-obs-plan.md |
| visual-lint (gated) | provenance arm flagged count edits inside the 8 tokens before a dated parenthesis at arch :196 and test-plan :22; the entry was not read before the apply | retries 2 | 2026-10-20T07-57-08-wrap/replace-3.md · replace-9.md |
| large-screen | the `readers` pattern missed «by 28 checks», «twenty-eight checks» and «23 readers», the forms the count-forms entry says every pattern must carry | extra_reads 2 | 2026-10-20T11-11-53-wrap/curation.md |

**Lookback (read-only):** this type was recorded 3 · 2 · 3 times in 0.2.0 Epochs 1–3 and 12 times here. The
large-screen wrap deferred a learning as «recurrence-despite-learning». The count-forms subset is the same event P5
and X1 see from the reconcile side.
**Proposal:** a learning stated as text has not stopped either of the two leading themes. A direction for the founder
is to move each remedy into the tool that the failure passes through:
- Count forms: the report's sweep could generate a moved count's patterns from the count itself (digits, number word,
  `by N`, `N readers`, both the before and after values) rather than take hand-written TOML per chunk.
- Provenance: classify's provenance check could run as a preview in the replace or apply tool before a write lands
  (see L1).

At the mechanism level, curation could route a learning that recurs after being curated to a tool-change candidate
instead of re-curating its text.

### P2 — wrap-session/reconcile · `contract.in-pass-correction` — 9 cases · weight 27 · rate 9/8
**Pattern:** every chunk's reconcile corrected at least one of its own first writes before commit. Four of the nine
are classify's provenance arm and three are hand-copied `citation-dispositions.md` rows.
**Evidence:** ALL 9 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upsync | architecture.md Upstream-sync line first written with «ratified by the founder (2026-10-18)»; classify FAIL provenance (a new date); rewritten, ratification left to the sidecar | retries 1 | 2026-10-19T19-54-24-wrap/classify-…json |
| upsync | the [DOM semantics] bullet first registered three Node methods Validate had rejected as registry over-reach; caught on re-read | retries 1 | 2026-10-19T19-54-24-wrap/fanout-results.md |
| test-splits | cascade-patterns.toml first written with doubled single quotes in TOML literals; cascade.py refused (exit 2); rewritten triple-quoted | retries 1 | 2026-10-19T21-24-54-wrap/cascade-2026-10-19-test-splits.json |
| shared-fixtures | CLAUDE.md:13 first called tests/fixtures/ «the fixture checks»; corrected on re-read | reformulations 1 | 2026-10-19T23-02-04-wrap/cascade-dispositions.md |
| visual-lint | obs-plan.md:70 insertion beside «where eighteen stood» read as new provenance (moved); test-plan.md:351 pointer named evidence/ci.md, which holds no such reading (re-pointed) | retries 2 | 2026-10-20T00-54-29-wrap/fanout-results.md |
| load-wait | citation-dispositions.md copied the stretched row as «execute.rs:95-254» instead of the printed «95-253 → 95-254» | retries 1 | — |
| select-by-id | test-plan amendments first written with chunk-marker provenance (17 classify rows) plus session-lifecycle.md:11 (5 rows); replace-5/6/9 | retries 3 | 2026-10-20T05-49-12-wrap/classify-2026-10-20-select-by-id.json |
| visual-lint (gated) | arch :118 :196 :202 :227 first wrote «ratified by the founder (2026-10-20)» and relay provenance (13 rows), then two anchored markers (2), then a test-plan :22 insertion in a dated parenthesis's window (3); citation-dispositions.md also dispositioned the tool's summary line as a row | retries 3 · iterations 4 | 2026-10-20T07-57-08-wrap/replace-2.md · replace-3.md · replace-9.md |
| large-screen | citation-dispositions.md carried a disposition on the sweep's second summary line; caught by a row count of 67 against 66 printed rows | retries 1 | — |

**Lookback:** 4 · 4 in Epochs 2–3 and 9 here. The type sits at 3–10 per epoch across both versions.
**Proposal:**
- Provenance subset (4): see L1.
- Disposition subset (3): `cites.py` could emit a `citation-dispositions.md` skeleton with one row per printed row
  and summary lines excluded. The hand copy, which is where both the mis-copied range and the summary-line row came
  from, would then disappear.
- The remaining two are content slips with nothing in common.

### P3 — phase/validate · `contract.mechanical-check` — 7 cases · weight 16 · rate 7/7
**Pattern:** a validate check or control fired on the plan in 5 of 7 chunks; one chunk fired three times. The checks
are working as designed, and the friction is the iterations they cost. Two cases are hand-built guard entries (see
X2), and one is a predicate that fires by construction.
**Evidence:** ALL 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upsync | check 10 read 4 UNPARSED citations (inputs#I3, I4, I6, I7) inside the verbatim CARRY 3 and 4 quotes folded at P1 | iterations 1 | scope.md:29–30 |
| visual-lint | check 4 (6): the gated criterion named `lint_answers_one_line_of_findings` with no entry running cli_commands; an entry was added | iterations 1 | plan.md |
| visual-lint | check 4 (9): the lint unit entry baselined green with 0 selected (92 filtered out); test-line atoms added, re-baselined red | iterations 1 · retries 1 | plan.md |
| load-wait | entry 9's changed-files guard filtered by a `grep -v` alternation that omitted the new check; it would read red after the pre-CI commit («the shared-fixtures wrap hit the same shape»); planlint 4 (16) reads pathspecs, not a grep -v filter | iterations 1 | 2026-10-20T01-41-31-phase/p5-controls.md |
| select-by-id | planlint 4 (13) HIT on the per-OS tally entry (a `gh run view` job-log read) fires by construction on every plan carrying it; resolved by a known-positive control «as in the previous chunk» | extra_reads 1 | 2026-10-20T03-46-58-phase/planlint-…json |
| large-screen | check 4 (9)'s must-pass control read the silence-census entry red: `grep -c` over two silent files exits 1, and session_common holds registered test-only reads the pattern matched; split and re-controlled | iterations 1 · extra_reads 1 | 2026-10-20T09-07-29-phase/p5-controls.md |
| large-screen | the plan shipped a hand preservation guard (`git diff \| grep -v` whitelist) where plan-template makes such a guard one `gate.py guard` call; no planlint predicate flags it; the operator's review caught it | dialogue_rounds 1 · iterations 1 | 2026-10-20T09-07-29-phase/p5-card-2.md |

**Lookback:** 2 · 2 · 3 in Epochs 1–3 and 7 here.
**Proposal:**
- A planlint predicate that flags a hand changed-files or preservation guard (`git diff`/`git status` piped through
  `grep -v`) where plan-template's rule names `gate.py guard`. Both guard cases record that no predicate attempted
  them.
- Check 4 (13) could recognise the per-OS tally entry's `gh run view` form, which it currently hits on every plan
  that carries the entry.
- A plan-template note that a `grep -c` zero-census exits 1.

### P4 — implement/code · `input.plan-step-ambiguous` — 5 cases · weight 9 · rate 5/8
**Pattern:** implement read a plan step as ambiguous or wrong in 5 of 7 chunks. Two cases are the plan disagreeing
with itself. Three are the plan disagreeing with the code: a crate dependency, a template entry path, and a call
site's shape.
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| shared-fixtures | step 6 lists stand_mcp_tools :532 under `fixtures::hold`, but the site is serve_mcp's boot closure; step 5's form taken | reformulations 1 | — |
| visual-lint | steps 3–4 assume escher-driver can name `blitz_dom::BoundingRect` and read computed contain/overflow; it depends on neither, and the git guard forbids the manifest edit | extra_reads 4 | — |
| load-wait | step 1's two stopping clauses for the never-answered wait disagree; the second taken, as the arm-2 criterion requires | reformulations 1 | — |
| select-by-id | step 4 named the mutator's entry points, but a static template enters through `process_added_subtree`; step 14's dynamic `selected` reverted every key choice | iterations 2 · extra_reads 2 | select_interaction.rs first run: 6 of 7 red |
| large-screen | the Codebase-touchpoints row says «the record fed from the part»; step 7 and Provenance say the whole screen's ids; step 7 followed | — | — |

**Lookback:** 1 · 1 · 3 in Epochs 1–3 and 5 here. 0.1.0 ran 3–9 per epoch.
**Proposal:**
- A validate cross-read of the Codebase-touchpoints table against the Implementation steps would cover the two
  self-disagreements (large-screen and load-wait).
- A research or plan check that each type the plan names is reachable from the crate it lands in (manifest
  dependencies) would cover visual-lint.

### P5 — wrap-session/reconcile · `input.report-insufficient` — 6 cases · weight 6 · rate 6/8
**Pattern:** in 6 of 7 chunks, the report's pre-pass site sweep missed stale count sites, and the reconcile
detectors then found them. All six are count sweeps: digit-only patterns, word forms, `by N` forms, a series' latest
figure, and two misread dispositions.
**Evidence:** ALL 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upsync | Counts said no master site states the run-all figure; `\b575\b` could not meet a series last written at 546 (test-plan.md:118) | extra_reads 2 | 2026-10-19T19-54-24-wrap/fanout-results.md |
| test-splits | the pre-pass dispositions misread architecture.md:309 (listed stale) and :223 (listed citation-only though content-stale) | extra_reads 2 | 2026-10-19T21-24-54-wrap/fanout-results.md |
| visual-lint | Counts carried no escher-driver unit-count fact (92 → 94) and dispositioned test-plan.md:68 and obs-plan.md:70, :79 as path citations, though each states a moved count | extra_reads 4 | 2026-10-20T00-54-29-wrap/fanout-results.md |
| load-wait | digit-keyed sweep missed a count in words, a «19 stand_*» restatement and arch's «by 26 … by 20» | extra_reads 3 | 2026-10-20T03-00-02-wrap/fanout-results.md |
| select-by-id | missed obs-plan.md:70 «twenty-seven checks», obs-plan.md:79 «21 readers», test-plan.md:132 «run stand 185 · 0 · 7» | extra_reads 3 | 2026-10-20T05-49-12-wrap/fanout-obs-plan.md |
| large-screen | `unitcount` and `readers` missed six standing sites (test-plan.md:22, :25, :132; session-lifecycle.md:11; obs-plan.md:70, :79) | extra_reads 3 | 2026-10-20T11-11-53-wrap/fanout-results.md |

**Lookback:** 1 · 2 · 1 in Epochs 1–3 and 6 here. 0.1.0 ran 1–6 per epoch.
**Proposal:** this is the same cause as P1's count-forms subset and X1. The sites recur: obs-plan.md:70 and :79, and
test-plan.md:22 and :132, appear in 3–4 of the six. Two directions:
- Generate a count's sweep forms in the tool (see P1).
- Keep a standing per-project list of the sites that restate the reader and test counts, which the report's sweep
  reads every time. These sites are where the detectors keep landing.

### P6 — */* (universal) · `tooling.host-shell` — 3 cases · weight 6
**Pattern:** in two plan steps, a TOML triple-single-quoted run string was pasted into `bash -c` and failed to parse.
In one report step, the embedded ugrep refused a bounded repetition.
**Evidence:** ALL 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upsync (plan) | a TOML-literal triple-single-quoted run string pasted into `bash -c` failed as a syntax error near `(`; one re-issue through a script file | retries 1 | plan.md gate 34 |
| test-splits (report) | the embedded ugrep refused a bounded-repetition context pattern (`.{200}`) and printed no rows, the third standing-learning recurrence of this hazard (T1 2026-10-07, global CLAUDE.md Shell); re-issued through python [clause retracted: discount «the third standing-learning recurrence of this hazard»: no count of prior recurrences was taken; the refusal and its one re-issue stand] | retries 1 | report.md |
| load-wait (plan) | the gate-20 log-tally control typed as `bash -c` with the TOML triple-quote delimiters copied in failed to parse; re-run from a scratchpad script | retries 1 | plan.md |

**Lookback:** recorded in 6 of the 10 epochs ledger-wide (1 · 1 · 1 · 2 in 0.1.0, 1 in 0.2.0 Epoch 2, 3 here) and has
never halted, which is the chronic-degrade shape (see L2). Its problem-fact twin (L2) adds shared-fixtures and
select-by-id.
**Proposal:**
- TOML subset: a `gate.py` subcommand that fires one entry's run text, or a control against it, straight from
  `plan.md`. The re-typing into `bash -c`, where the triple-quote form breaks, would then disappear.
- ugrep subset: the rule is already in the global CLAUDE.md and in a T1 learning. The remaining direction is
  environment-level, for example the bash guard flagging a `{n,m}` repetition in a `grep` call before it runs.

## Cross-step chains (starting heuristics)

### X1 — wrap-session/report →`report`→ wrap-session/reconcile — 6 chunks
Each of the six reconcile records grades `report` `thin`: upsync `2026-10-19T20:17:00Z-a`, test-splits `…21:48:43Z-a`,
visual-lint `2026-10-20T01:26:37Z-a`, load-wait `…03:32:15Z-a`, select-by-id `…07:00:02Z-a`, large-screen
`…12:20:13Z-a`. Each producer `report` record is formally `ok` and carries `new-text-generated` plus row counts, with
nothing about sweep completeness (one carries `sweep-control-refused-3`, test-splits). **Hypothesis:** the report's
signals don't measure whether its count sweep covered every form, and the detectors downstream recover every miss at
the cost of extra reads. This is the same cause as P1's count subset and P5. **Direction:** a report-record signal
for sweep form coverage, or the generated forms of P1 and P5.

### X2 — phase/plan · phase/validate →`plan`→ implement/code · implement/fix-loop · wrap-session/gates · wrap-session/report — 5 chunks
There are 13 consumer anchors grading `plan` `thin`:
- code: shared-fixtures, visual-lint, load-wait, select-by-id, large-screen
- fix-loop: select-by-id, visual-lint, large-screen
- gates: shared-fixtures
- report: select-by-id, visual-lint
- validate: large-screen

All producers are formally `ok`. select-by-id's validate carries `baseline-caught-defect` and `guard-refused:1`.
**Sub-theme, joined by hand on the artifact:** the plan's write-set and no-move guard entries account for 9 records
in 5 chunks:
- shared-fixtures: gates entry 12 went red at the wrap after the pre-CI commit tracked the new module. It halted,
  with `2026-10-19T23:43:31Z-b` `tooling.light-gate-red` and an overridden fact.
- select-by-id: entry 12 omitted `events/mod.rs` (`…04:41:26Z-b` spec-reality-gap, `ok-degraded`; `…05:56:43Z-b`
  outcome-unsettled).
- visual-lint: entry 7's no-move guard contradicted step 12 (`…07:26:35Z-b` jointly-contradictory). The manifest
  guard blocked the dependency steps 3–4 needed (`…00:24:41Z-b`, with a product-logic workaround fact).
- load-wait: a `grep -v` alternation omitted the new check, caught at P5 (`…02:00:45Z-b`). A control was aimed
  outside the entry's pathspecs (`…02:00:45Z-c`, untyped).
- large-screen: a hand guard was written where `gate.py guard` applies (`…09:52:35Z-c`).

Lookback: the same probe shape went red or was overridden at gates in 0.2.0 Epoch 1 (file-splits entry 13, after the
pre-CI commit), 0.2.0 Epoch 2 (engine-debts entry 21, source-list probe skipped) and 0.1.0 Epoch 2 (project-readme
entry 17). **Hypothesis:** hand-built write-set probes keep breaking for the same reasons: tracked versus untracked
around the pre-CI commit, a missed companion file, and a freeze before the work they guard. plan-template already
names `gate.py guard` for this, but nothing enforces it. **Direction:** the planlint predicate of P3, and a
`gate.py guard` that reads tracked and untracked files together against a declared write set, so the pre-CI commit
cannot flip it.

Other shapes are single-chunk or covered above: implement/smoke →`implement-outcome`→ report (2: select-by-id and
visual-lint, the operator's plan edits; `input.implement-outcome-unsettled`, below threshold) · phase/research
→`research`→ code or plan (1 each).

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — classify.py's provenance anchor window — 2 facts in-epoch + 1 prior, 7 typed correlates
**Facts:**
- select-by-id · reconcile · process · workaround: «classify v1.2's provenance arm re-anchored pre-existing chunk
  markers in history parentheses beside counts this pass moved; the markers were dropped from those parentheses
  (test-plan.md:22, :68, :104; session-lifecycle.md:11 restructured) to pass».
- visual-lint (gated) · reconcile · process · workaround: «classify's provenance arm keys a dated parenthesis on the
  8 tokens before it, so a count edited inside that window flagged the parenthesis's standing markers as new: dropped
  two chunk markers from arch :196 … moved a test-plan :22 insertion out of the window».
- Prior: session-recording (Epoch 3) · reconcile · product-logic · workaround: «re-shaped 3 edits to keep the anchors
  and dropped one superseded earlier-reading parenthesis».

Typed correlates: P2 ×4 and P1 ×3. The lookback's `provenance` theme hits 10 records here and 4 in Epoch 3.
**Level hypothesis:** the cause appears to live in the tool's anchoring rule. A count edit near a standing dated
parenthesis reads as new provenance. The fixes landed in the project: markers dropped from history parentheses,
insertions moved, and one Tier-3 learning that has recurred three times since it was curated. Dropping standing
markers changes what the masters record about history in order to satisfy the check.
**Proposal:**
- Have classify recognise an unchanged parenthesis whose anchor tokens moved as standing, for example by anchoring on
  the parenthesis text itself.
- Or have the replace/apply tool run the provenance check as a preview before writing, so the failure costs no
  retry.

### L2 — band-aid (+ chronic-degrade) — plan controls re-typed as `bash -c` strings — 3 facts in-epoch, 4 chunks
**Facts:**
- upsync · plan · environment · workaround: «the tally control typed as bash -c with triple single quotes broke on
  the Bash tool's eval; re-run from a scratchpad script under bash -o pipefail».
- shared-fixtures · plan · environment · workaround: «the CI-tally control typed inline as a nested bash -c string
  broke on quoting; run instead from a scratchpad script file».
- select-by-id · plan · process · workaround: «the per-OS tally control typed into the session broke on nested
  quoting; its run text was written to a scratchpad script».

Typed correlates: P6's two TOML cases (upsync and load-wait), which makes 4 chunks. The prior untyped case is
`2026-10-19T03:49:26Z-d`, an inline python heredoc nesting triple quotes. `tooling.host-shell` itself is recorded in 6
of 10 epochs and has never halted.
**Level hypothesis:** the cause appears to live in the pipeline's shape. A plan stores its run text as TOML
triple-quoted literals, and the per-OS and CI tally controls are fired by re-typing that text. The fix lands every
time as a per-chunk scratchpad script.
**Proposal:** the same `gate.py` single-entry or control runner as P6. It reads the entry's run text from `plan.md`
and runs it the way the gate does.

### L3 — band-aid — a pipeline tool's stdout suppressed against the read-bare rule — 2 facts in-epoch + 1 prior
**Facts:**
- select-by-id · validate · process · workaround: «one gate.py baseline call's output was piped into grep, against
  the host rule that tool output is read bare; later calls ran bare».
- large-screen · reconcile · process · workaround: «two tool calls (`cites.py apply`, `cascade.py replace`) had
  stdout sent to /dev/null and were read from their trail records instead».
- Prior: recorded-check-replay (Epoch 3) · reconcile: «cites.py apply's stdout was sent to /dev/null against the
  letter».

Lookback: the `stdout-suppressed` theme hits 0.2.0 Epoch 1 (4), Epoch 2 (1), Epoch 3 (1) and 0.1.0 Epoch 6 (2).
**Level hypothesis:** the rule has had two forms in the ledger. The large-screen fact quotes the skill's letter, «a
tool's stdout is never sent to a file and is read as printed». 0.1.0 Epoch 6's upstream-sync records quote an
operator directive, «every pipeline tool's stdout redirected to a run-dir file and read whole». Either form is broken
in the same few tools,
`cites.py apply`, `cascade.py replace` and `gate.py`, whose output is long. The facts record the breach but not why
it happened.
**Proposal:** two possible directions:
- Those tools print a bounded summary to stdout and leave the listing to their trail.
- The bash guard refuses `> /dev/null` on a pipeline tool, as it already refuses a pipe into head or tail.

### L4 — band-aid — many anchored edits applied by one scripted replace — 1 fact in-epoch + 1 prior
**Facts:**
- load-wait · research · process · workaround: «closed scope.md's 11 [inferred] tags with one python replace script
  (each anchor asserted unique) instead of 11 Edit-tool calls».
- Prior: one-virtual-clock (Epoch 3) · reconcile: «67 point amendments … applied by one scratch python script of
  exact-match replacements … instead of 67 anchored Edit calls».

The lookback's theme hits 7 epochs.
**Level hypothesis:** there is no batch form of an anchored edit, so the agent improvises one with uniqueness
asserts. The workaround looks safe, but it is outside any tool's trail.
**Proposal:** a batch anchored-replace in the pipeline tools (cascade.py replace already exists for masters), or an
explicit sanction for the asserted-unique script form.

### L5 — override — phase P4: founder forks as a dialog — 1 fact in-epoch + 4 prior
**Facts:**
- visual-lint · plan · process · overridden: «the operator answered the founder fork F1 as delegate with the most
  conservative option and directed a gated claim of v020-05».
- Prior, same rule: session-recording `2026-10-19T03:18:48Z-a` and one-virtual-clock `2026-10-18T07:41:31Z-a`
  (Epoch 3), engine-debts `2026-10-18T03:01:31Z-a` (Epoch 2), and driver-cli `2026-10-10T08:21:37Z-a` (0.1.0
  Epoch 5). In each the forks went to one file answered by a delegate in place of the P4 dialog.

**Level hypothesis:** the operator keeps making the same correction to the same rule, so P4's dialog form may not
fit how this project's forks actually reach the founder. The facts name a forks file, a delegate, and an answer
arriving by message or relay.
**Proposal:** give P4 a named forks-file-plus-delegate mode as a first-class route, rather than an override recorded
each time.

### L6 — override — wrap gates: a red light-gate entry stops the wrap — 1 fact in-epoch + 3 prior
**Facts:**
- shared-fixtures · gates · process · overridden: «light gate entry 12 red (the pre-CI commit made fixtures/mod.rs
  tracked…); the contract's course was to stop tests-failing, but the operator answered «Amend entry 12, re-gate
  (Recommended)»».
- Prior: file-splits `2026-10-16T17:56:52Z-a` (entry 13 after the pre-CI commit tracked new files; record-and-proceed),
  engine-debts `2026-10-18T21:40:26Z-a` (entry 21, source-list probe, `--skipped`), and project-readme
  `2026-10-06T11:52:55Z-a` (entry 17 diff guard).

**Level hypothesis:** in every case the entry overridden is a write-set or diff probe (X2), not a test. The rule is
right for tests but miscalibrated for probes that the pre-CI commit itself flips.
**Proposal:** fix the probe through X2's guard. Alternatively, the gates contract could name «a write-set probe red
only from the pre-CI commit's tracking» as a re-gate case.

**Deferred-forever:** no hit. The epoch has no `deferred` fact and no `deferral-open` signal. The one deferral, v020-05
held gated (`2026-10-20T01:37:33Z-b`, `contract.coverage-hold`), closed at visual-lint's gated wrap.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/plan — 2 cases (1 in-epoch + 1 recurring from Epoch 3) → proposed type `contract.unread-clock-stamp`
**Cluster:**
- load-wait `2026-10-20T01:56:03Z-b`: «wrote estimated clock stamps (Generated 01:56Z, control fired 01:58Z) into
  plan.md without reading the clock; the stamp-ahead hook refused the second, both re-written from date -u readings».
- recorded-check-replay `2026-10-19T08:09:01Z-b` (Epoch 3): «plan.md's Generated stamp was written as an estimated
  08:20Z without reading the clock; the stamp-ahead PostToolUse hook refused it».
- Same theme as a problem fact: select-by-id `2026-10-20T04:07:12Z-a#1` (removed-cause, 04:10Z against a clock of
  04:06Z). The lookback's `estimated-stamp` theme hits 8 epochs.

**Draft criteria line (phase plan.md playbook):** `contract.unread-clock-stamp` — a time written into a phase
artifact (Generated, a control's fired-at) was estimated rather than read from the clock; record the artifact, the
written and actual times, and what caught it (the stamp-ahead hook, or nothing).

The founder may also note that the recurrence points to a tool writing the stamp rather than to a type.

### U2 — wrap-session/report — 2 cases (1 in-epoch + 1 recurring from Epoch 3) → proposed type `retry.hand-tally-rework`
**Cluster:**
- visual-lint `2026-10-20T08:05:19Z-c`: «the site sweep's verbs12 pattern set ignore_case on an alternative holding
  the code token \bVERBS\b, so it matched every plain word verbs (36 rows); the first per-row disposition miscounted
  (21 + 15) and was re-derived twice to 26 + 10 against the printed rows».
- one-virtual-clock `2026-10-18T08:52:55Z-b` (Epoch 3): «the Expected-amendments per-master row counts are
  hand-tallied from the sweep's rows (the tool prints per-pattern totals, not per-master ones); three figures were
  wrong at first write».

Typed neighbours at reconcile are P2's disposition subset.
**Draft criteria line (wrap report.md playbook):** `retry.hand-tally-rework` — a count or per-row disposition derived
by hand from a tool's printed rows was wrong at first write and re-derived; record the rows, the first and final
figures, and whether the tool could have printed the tally itself.

## Below threshold — no action
**Typed groups (18):**
- wrap/reconcile UNTYPED 2. One halted (EDQUOT on Claude Code's temp dir, cause not identified); one is a leaf
  comparer's open background job.
- wrap/gates `tooling.light-gate-red` 1, halted. Shared-fixtures entry 12; in X2 and L6.
- wrap/reconcile `contract.false-positive-proposal` 2. upsync: Node-method registration rejected as over-reach;
  shared-fixtures: a boot-command spelling narrowed.
- wrap/reconcile `contract.cascade-miss` 2. visual-lint: session-lifecycle.md:11's unit count; large-screen:
  security-plan.md:84 and session-lifecycle.md:5. Both caught by the step-2 sweep.
- `contract.narrow-basis-claim` 2 (report). test-splits: a recurrence ordinal typed from no ledger count;
  visual-lint: «I1–I6 all cited» written from recall, I6 uncited.
- wrap/report `input.implement-outcome-unsettled` 2. select-by-id and visual-lint, each through the operator's plan
  edit (X2's smoke →implement-outcome→ report shape).
- wrap/report UNTYPED 1 → U2.
- implement/fix-loop UNTYPED 1. Plan step 12.7 was narrower than v020-09's acceptance.
- phase/plan UNTYPED 1 → U1.
- phase/validate UNTYPED 1. A must-fail control aimed outside its entry's pathspecs (in X2).
- phase/plan `retry.synthesis-rework` 1. large-screen's id-record lean, rewritten after re-reading execute.rs.
- `contract.jointly-contradictory-instructions` 1. visual-lint entry 7 against step 12 (in X2).
- `contract.skill-reference-drift` 1. The load-wait retraction record itself («misreading: … no letter-vs-tool drift
  existed»).
- `contract.structural-blind-spot` 1. visual-lint gated re-fire of exact counts frozen before two later chunks.
- `tooling.output-cap-overflow` 1. load-wait research: code-graph query of 147 rows (32.5 KB).
- implement/code `input.research-files-wrong` 1. visual-lint: stand_act_scroll.rs pins `VERBS.len() == 11`.
- implement/fix-loop `contract.spec-reality-gap` 1. select-by-id entry 12 omits `events/mod.rs` (in X2).
- wrap/gates `contract.coverage-hold` 1. v020-05 held gated, closed later.

**Untyped clusters (5 in-epoch singles; one prior-only):** a control aimed outside its entry's read · a leaf
sub-agent's open background job · EDQUOT · a plan step narrower than its matrix acceptance · (prior only) a python
heredoc nesting triple quotes, whose in-epoch analogues are typed (P6).

**Fact themes (10):**
- A step's contract holds no slot for the measurement it needs. select-by-id research wrote and removed a throwaway
  probe test; visual-lint smoke was run by hand. 2 in-epoch, no prior.
- A mandated read or no-read rule was replaced by a narrower equivalent. upsync: `cmp` in place of an evolve-system
  re-read; load-wait: one ledger line read by id. Two different rules.
- A bare `cd` moved the session cwd (shared-fixtures take-up).
- ugrep refused a bounded repetition (test-splits; in P6).
- A scratch-clone path put the socket past 94 bytes (visual-lint validate).
- A founder label sits in a gitignored incubator file that snap refuses (visual-lint take-up).
- Crate types are unnameable under the no-move manifest guard (visual-lint code; in X2).
- A stale code-graph DB was regenerated (select-by-id research).
- A leaf comparer's background job stopped by TaskStop (load-wait reconcile).
- An estimated stamp was refused by the stamp-ahead hook, removed-cause (select-by-id plan; folded into U1).
