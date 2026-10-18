# Evolve Diagnosis — escher-0.2.0 · Epoch 2 — Engine debts (epoch-to-date) · 2026-10-18T05:30:22Z

Epoch-to-date: both chunks are wrapped, and `2026-10-18-engine-debts` reads `gated` in the master record, waiting
only on the founder's word (`evidence/founder-word.md`). Every proposal below is obligation-free: accept, reject,
defer or modify it, and nothing on the mechanism's side follows. Evidence twins sit beside this file: `q-retractions.json`, `q-health.json`, `q-typed.json`, `q-untyped-raw.json`, `q-chains.json`,
`q-level-raw.json`, `q-level.json`, and the pass's script `analyze.py`.

## Mechanism health

- **Records:** 55 (28 step / 27 friction) over 2 chunks, plus 2 chunk-less `new-session/orientation` records. Version-qualified: the ledger's `Epoch 1 — Foundation` label spans 0.1.0 and 0.2.0, so lookback reads `version` and never the label alone.
- **Coverage vs expected:** `2026-10-17-upstream-sync-engine-debts` ran phase 5/5 · implement 3/3 · wrap 5/5 (`route-resolve` included). `2026-10-18-engine-debts` ran phase 5/5 · implement 3/3 · wrap 5/5. **No gaps.** `new-session` wrote 2 orientation records; the ledger alone cannot count session starts, so their coverage is not measurable here.
- **Unparseable:** 0 · **malformed-ts:** 0 (ledger-wide 0) · **id fill:** 55/55.
- **Untyped rate per step:** `phase/plan` 1/2 · `phase/validate` 1/4 · `wrap-session/reconcile` 1/10 · `wrap-session/gates` 1/2; overall 4/27 frictions.
- **Problem-fact fill:** 10/28 step records carry ≥ 1 fact; 14 facts in all.
- **Step outcomes:** 24 `ok` · 4 `halted-resolved` (10-17 report · 10-17 reconcile · 10-18 reconcile · 10-18 route-resolve).
- **Signal line:** `guard-refused 18 over 28 step records · exact-hit 2 over 2 curation records`. No signal string was malformed or misplaced.
- **Retractions (whole-ledger pre-pass):** retracted 0 (1 unresolvable) · clause-retracted 2 (kept, notes rendered — both outside this epoch) · no retraction targeted by a retraction. The unresolvable one, verbatim, for manual discount: `2026-10-07T04:21:25Z-a` — `{"id": null, "note": "in step record 2026-10-07T04:21:04Z-a (a step record, so no record or clause scope applies), produced git-state note: discount 'add -A staged 43 paths' — the measured count is 47 files changed"}`.
- **Epoch-less record (outside range):** 1 step record, `2026-10-16T06:45:44Z` `andromeda-new-session/orientation`, with `epoch: null`.
- **Calibration boundaries in range:** none apply. Every record dates from 2026-10-17 or later, after every field's deploy date (the latest is 2026-10-13), so each field and signal reads in its live era.

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 4 cases · weight 8

**Pattern:** One reconcile run (`2026-10-18-engine-debts`, the two-window wrap) wrote four pieces of wrong text, and each was caught by a check or a re-read before commit, at one retry each. The four came from four different causes:
1. A format the pass wrote for the first time.
2. A writer that parsed a tool's prose summary as data.
3. A hand-retyped fork id.
4. An estimated time.

**Evidence:** all 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-18-engine-debts | obs-plan's new detail group was written as an indented list item with no blank line after its close label; `classify.py check` failed it (units 2) and it was rewritten before the sidecar entry | retries 1 | `.andromeda/runs/2026-10-18T04-30-47-wrap/classify-2026-10-18-engine-debts.json` · `.andromeda/obs-plan.md:72` |
| 2026-10-18-engine-debts | the citation-dispositions writer counted the `cites.py` summary line beginning `out-of-range 1` as a row (112 rows for 111); the asserted count caught it and the parser was narrowed | retries 1 | — · `citation-dispositions.md` |
| 2026-10-18-engine-debts | the a11y-plan sidecar payload named the founder's F3 fork `C3`; a re-read before the append caught it and it was corrected to F3 | retries 1 | — · `a11y-plan-entry-2.md` |
| 2026-10-18-engine-debts | `relay-4.md` stamped the two dialog answers with estimated times (05:10Z, 05:16Z); `inputs.py snap` refused the later one as ahead of the clock and the stamp-ahead hook flagged it; rewritten as `by 05:12Z`, the clock read after them | retries 1 | inputs#I8 · `relay-4.md` |

**Lookback (read-only, for recurrence):**
- The type recurs in every 0.1.0 epoch: E1 7 · E2 4 · E3 7 · E4 9 · E5 10 · E6 3.
- It reads 0 in 0.2.0 Epoch 1.
- The class is chronic, but each case is cheap: every one was caught inside the pass.

**Proposal:** One direction per cause, for the founder's judgment:
1. **First labelled write:** the reconcile reference could carry a literal skeleton of a labelled detail group, with a blank line after its close label, so a pass writing one for the first time copies the shape instead of inferring it.
2. **Tool summary read as data:** `cites.py` could offer a machine form of its listing, for example `--json`, or prefix its summary lines distinctly, so no writer separates rows from summary by prose.
3. **Retyped fork id:** sidecar payloads could take fork labels from the snapped inputs ledger instead of retyping them.
4. **Estimated time:** the same cause appears at four steps in this epoch (Appendix, T3). A relay-writing helper that stamps from the clock could remove it at the source.

All four were caught as designed, so "no action — the checks are the cure" is an equally valid reading.

## Cross-step chains (starting heuristics)

### X1 — wrap-session/report →report→ wrap-session/reconcile — 2 chunks (cross-step pattern candidate)

**`2026-10-17-upstream-sync-engine-debts`:**
- **Producer:** report `2026-10-17T18:34:19Z-a`.
  - Outcome `halted-resolved`; signals `new-text-generated`, `new-text-rows:85`, `guard-refused:3`.
  - Its own frictions: `contract.premise-falsified`, `contract.token-proxy-check`.
- **Consumer:** reconcile `2026-10-18T01:57:54Z-a`.
  - Read `report` as **thin**: "three facts absent or wrong: util.rs:8 SUPER->META, the convert_events todo!() arms gone, the three closed ARIA reads credited to tests that do not exist".
  - Friction `2026-10-18T01:57:54Z-b` `input.report-insufficient` (extra_reads 3), evidence `.andromeda/runs/2026-10-17T15-18-19-wrap/fanout-results.md`.

**`2026-10-18-engine-debts`:**
- **Producer:** report `2026-10-18T04:38:25Z-a`.
  - Outcome `ok`; signals `new-text-generated`, `new-text-rows:123`, no quality signal.
- **Consumer:** reconcile `2026-10-18T05:13:34Z-a`.
  - Read `report` as `ok`, with the note "test-plan's per-OS tallies were the one fact it did not carry".
  - Friction `2026-10-18T05:13:34Z-g` `input.report-insufficient`: "the plan's expected amendment named test-plan §9's per-OS tallies, and the report carried only CI's 17/17 verdict… the entry was dispositioned with no fact to apply".

**Chain hypothesis:** In both chunks the report was produced formally `ok`, or halted and then resolved, yet lacked a fact that the plan or the code had named. In one case it credited coverage to tests that do not exist. In the other, a subject the plan's Expected amendments named was missing. Reconcile found the gap only by its own search, or not at all.

**Lookback:** `input.report-insufficient` recurs every epoch — 0.1.0: E1 4 · E2 1 · E3 4 · E4 6 · E5 5 · E6 3; 0.2.0: E1 1 · E2 2.

**Direction:** A report-side coverage control could list every subject the plan's Expected amendments name, and every test a Coverage line credits, each with a report row or a stated "none — why". That would turn a silent omission into a visible one before reconcile consumes the report.

### Single-chunk chain hypotheses (no pattern; listed for the record)

- **phase/research →research→ implement/code** (`2026-10-17-upstream-sync-engine-debts`):
  - **Producer** `2026-10-17T12:37:12Z-a`: `ok`, with signal `unresolved-questions` and its own `input.extract-signal-gap` friction.
  - **Consumer** `2026-10-17T14:39:30Z-a`: read research as **thin**, because a `use dioxus::prelude::*` glob shadows `keyboard_types`. Friction `2026-10-17T14:39:30Z-b` `input.conventions-gap`, iterations 3.
- **phase →working-route→ wrap-session/route-resolve** (`2026-10-18-engine-debts`):
  - **Consumer** `2026-10-18T05:16:28Z-a` read the working-route as **thin**: "no BLOCKED-ON though the ledger held v020-03 gated".
  - The artifact join reached take-up (`2026-10-18T02:40:27Z-a`), but the gated claim was validate's (`2026-10-18T03:16:13Z-a`, signal `claims-refined`).
  - This is the same event as level candidate L1.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 2 facts (1 halt-bearing)

**Facts:**
- `2026-10-18-engine-debts` · phase/validate `2026-10-18T03:16:13Z-a`[1] · process · workaround — "the gated arm, written for a BLOCKED-ON head, was used for a chunk taken up with no block: v020-03 claimed --gated-file on the founder's discharge of the PROVISIONAL batch, with a defer gated entry and step 13".
- `2026-10-18-engine-debts` · wrap-session/route-resolve `2026-10-18T05:16:28Z-a`[0] (outcome `halted-resolved`) · process · overridden — "route-resolve edits only the markerless tail; on the operator's word one anchored Edit appended a BLOCKED-ON block to the frozen [2026-10-18-engine-debts] line so the gated arm's flip can run".
- **Typed correlate:** `2026-10-18T05:16:28Z-b` `contract.no-sanctioned-channel` (halted 1, dialogue_rounds 1, extra_reads 2) — "phase claimed v020-03 gated in the ledger (claim --gated-file) but stamped no BLOCKED-ON on the frozen line; route.py flip --to gated refuses without one and the wrap has no sanctioned frozen-line write…", evidence inputs#I8.
- **Also:** the route-resolve consumer's `working-route` verdict was **thin** (X1, single-chunk list).

**Level hypothesis:** The cause appears to lie in the pipeline's gated arm.
- `claim --gated-file` at phase P5 accepts a gated claim without the frozen-line `BLOCKED-ON`.
- `route.py flip --to gated` at the wrap requires that block.
- No step between them may write it.

The fix so far lives in the project: a one-off frozen-line write on the operator's word, recorded in the handoff as "a sanctioned frozen-line write".

**Proposal:** Pick one sanctioned owner for the block, for example:
- `claim --gated-file` stamps `BLOCKED-ON`, or refuses without one, at phase P5; or
- `route.py flip --to gated` accepts the ledger's gated claim in place of the frozen-line block; or
- route-resolve's reference names a gated-arm frozen-line stamp as a permitted write.

### L2 — band-aid — 2 facts (+1 halt-bearing correlate)

**Facts:**
- `2026-10-17-upstream-sync-engine-debts` · phase/plan `2026-10-17T14:21:45Z-a`[0] · process · workaround — "the AskUserQuestion round went unanswered for 58 min through the driver's dialog hook, which said to end the turn; the operator answered by message in the next turn, written verbatim to relay-2.md and snapped as inputs#I2".
- `2026-10-18-engine-debts` · phase/plan `2026-10-18T03:01:31Z-a`[0] · process · overridden — "the operator directed every founder fork into one run-dir file answered in the founder's stead as delegate, in place of a question dialog the overseer relays (inputs#I1); answers recorded PROVISIONAL (inputs#I2)".

**Typed correlates of the same theme:**
- `2026-10-18T01:57:54Z-d` `ambiguity.escalation-rounds` (dialogue_rounds 2, **halted 1**): "the answer card's dialog went unanswered within the driver's 58-minute hold; the wrap stopped on the dialog fallback and resumed on the founder's late relayed answer in a second window". Evidence `.andromeda/runs/2026-10-17T15-18-19-wrap/relay-3.md`.
- `2026-10-18T03:16:13Z-b` `contract.matrix-claim` (dialogue_rounds 2, iterations 1): the delegate's PROVISIONAL answer replaced the founder's recorded one in v020-03.
- `2026-10-18T05:28:03Z-b` `contract.coverage-hold` (deferred 1): v020-03 held below verified.
- Signal `dialog-fallback-used` ×1.

**Counting note:** The halt sits on a correlate in a third step record, not on either fact's own record. Discounting it leaves n=2 facts, below the halt clause.

**Level hypothesis:** The founder-answer channel assumes he answers inside the driver's 58-minute dialog hold. When he does not, each chunk absorbs the gap its own way: a relay file, a second wrap window, delegate answers marked PROVISIONAL, and finally a gated claim. Both chunks of this epoch hit it, and the epoch's last chunk now stands gated on it. L1 is downstream of this theme.

**Proposal:** A sanctioned asynchronous founder-word arm: one defined shape for a pending founder answer and one defined way a chunk proceeds against it. That would mean one PROVISIONAL marker, one gated claim and one discharge file, owned by the pipeline instead of improvised per chunk from operator directives.

### L3 — chronic-degrade — `tooling.host-shell` across 5 epochs, never halted

**Facts:**
- **In epoch:** `2026-10-17T14:21:45Z-c` (`2026-10-17-upstream-sync-engine-debts` · phase/plan · retries 1) — "firing a gate entry's run by hand as bash -c with TOML-style triple single quotes nested around single-quoted sed/awk programs broke the session shell's parse (syntax error near '('); re-run from a scratchpad script file".
- **Lookback (read-only):**
  - `2026-10-06T08:37:04Z-c` (0.1.0 E2, ugrep bounded repetition, retries 2)
  - `2026-10-06T21:59:01Z-b` (0.1.0 E3, ugrep bounded repetition, retries 1)
  - `2026-10-07T20:34:14Z-d` (0.1.0 E4, heredoc backslash collapse + colour codes, retries 2)
  - `2026-10-14T22:08:02Z-c` (0.1.0 E6, ugrep bounded repetition, retries 1)
  - `2026-10-15T08:17:08Z-b` (0.1.0 E6, `xargs command grep`, retries 1)

**Level hypothesis:** The ugrep case was absorbed into the global CLAUDE.md and the session learnings, and it has not recurred in 0.2.0. This epoch's specific is different: shell re-quoting of a gate entry's TOML command. The rules and learnings fix one specific at a time, while the class keeps returning in a new form.

**Proposal:** Remove the re-quoting step itself. A tool-side way to fire one gate entry's command straight from its record, outside the wrap where `--entry` is forbidden, would mean no agent ever re-wraps TOML strings into `bash -c`. The founder can judge whether such a mode already exists.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/validate · wrap-session/gates — 2 in-epoch cases, recurring from the prior epoch → proposed type: widen `contract.no-sanctioned-channel` to a Universal type

**Cluster:** In each case, a step's reference requires an act that the path being run gives no permitted surface for.

| id | chunk | step | what |
|---|---|---|---|
| `2026-10-18T03:16:13Z-c` | 2026-10-18-engine-debts | phase/validate | validation.md check 4 (9) requires a baseline run of every NOT SEEN entry, but gate.py prints not run — defer for a gated entry and a not-run word may not be recorded as a baseline; resolved by firing the entry with --void over the scratch clone alone |
| `2026-10-18T02:10:02Z-b` | 2026-10-17-upstream-sync-engine-debts | wrap-session/gates | the plan's preservation guard (entry 11) named deny.toml, which a later External-decay fix push had to move; the entry could only be read red by hand beside the tool (--entry is forbidden at the wrap) and skipped on the operator's directive |
| `2026-10-16T19:34:20Z-c` (prior epoch) | 2026-10-16-fast-failing-mutants | phase/validate | plan-template's artifact key names 'the output removed first' as a producer's freshness form, while this host's bash-guard refuses every recursive rm; the P4-authored rm -rf entries had to be re-spelt to a per-run unique dir at P5 |

The typed `contract.no-sanctioned-channel` at route-resolve (`2026-10-18T05:16:28Z-b`, L1) is the same shape. Today that type is local to `route-resolve.md`; no other step's playbook lists it, so the identical event left untyped at validate and gates. Read as one class, it has 3 in-epoch occurrences over 3 steps, plus 1 prior.

**Neighbour:** The universal `contract.jointly-contradictory-instructions` requires two instructions that are unsatisfiable in ANY order. Here each case had a workaround path (`--void`, a by-hand read, a unique dir), so that gloss does not cover it.

**Draft criteria line (Universal types, `evolve-system.md`):** `contract.no-sanctioned-channel` — a reference requires an act (a write, a run, a baseline, a read) that the path being run gives no permitted surface for — a tool word that refuses it, a flag forbidden at this step, a guard — say which act, which refusal, and what was done instead. (Moves from route-resolve-local to universal; route-resolve's gloss stays as its local reading.)

## Below threshold — no action

**Stage 1 groups** (per step; a universal or `recall.*` type is also counted once by type alone):
- `phase/validate/contract.mechanical-check` n=2, w5: plan touchpoints folded paths `2026-10-17T14:33:31Z-b`; an expected amendment's coverage of a ruling's subjects `2026-10-17T14:33:31Z-c` (10-17). No halt.
- `*/recall.corpus-recurrence` n=2, w4: a bare `cd` moved the cwd, at reconcile `2026-10-18T01:57:54Z-c` and curation `2026-10-18T01:59:21Z-b` (10-17, one event seen twice). It recurs from the prior epoch's untyped `2026-10-16T14:13:59Z-b`, and the handoff logs it as recurrence-despite-learning. The guard now refuses the act; type-alone lookback: 0.1.0 E6 17 · 0.2.0 E1 3.
- `wrap-session/reconcile/input.report-insufficient` n=2 — surfaced as chain X1.
- `wrap-session/reconcile/ambiguity.escalation-rounds` n=1 (halted) — a correlate in L2.
- `phase/validate/contract.matrix-claim` n=1 — a correlate in L2.
- `wrap-session/route-resolve/contract.no-sanctioned-channel` n=1 (halted) — L1 / U1.
- `contract.token-proxy-check` n=1 (wrap-session/report, 10-17): the scope base finder's fixed `chore({marker}): operator pre-CI commit` string missed a merge subject, a false negative. Already curated as the 2026-10-18 Tier-1 learning.
- `contract.premise-falsified` n=1 (wrap-session/report, 10-17): "deny.toml unchanged" was falsified by the External-decay fix. Same event as the untyped gates record in U1.
- `implement/code/input.conventions-gap` n=1 (10-17, iterations 3): the `keyboard_types` glob shadow — single-chunk chain.
- `tooling.host-shell` n=1 (phase/plan, 10-17) — L3.
- `wrap-session/curation/ambiguity.tier-routing` n=1: Tier-1 extension 2 B over cap.
- `implement/code/input.plan-step-ambiguous` n=1: "its fourth test" under the wrong paragraph.
- `phase/research/input.extract-signal-gap` n=1: arch extract on key re-exports.
- `phase/take-up/input.working-entry-thin` n=1: "nine deferrals" unlisted; the ninth carried `target: 0.2.0`.
- `wrap-session/gates/contract.coverage-hold` n=1 (deferred) — routed: BLOCKED-ON plus plan step 13. A correlate in L2.
- `wrap-session/reconcile/contract.false-positive-proposal` n=1: two "observed absent" claims with no search, returned under check 4.

**Stage 2 (untyped) clusters below F-4:**
- An estimated time written ahead of the clock (`2026-10-17T14:21:45Z-b`, phase/plan) — n=1 untyped. See T3.
- `cascade.py leaves --apply` refused 2 of 35 findings as escaped for a legitimate regex backslash (`2026-10-18T01:57:54Z-e`, reconcile) — n=1, no prior member.

**Stage 4 Pass A themes below threshold:**
- **T3 — an estimated time stamped ahead of the clock:**
  - One problem fact: `2026-10-18T02:00:13Z-a`[0] route-resolve, "the step record's first append carried a hand-typed ts ahead of the clock".
  - Four correlates: phase/plan untyped `2026-10-17T14:21:45Z-b`; P1's `relay-4.md` case; the plan's `guard-refused` note on a guessed Generated time `2026-10-18T03:01:31Z-a`; reconcile's `guard-refused` note on an estimated dialog time `2026-10-18T05:13:34Z-a`.
  - Five occurrences over four steps and both chunks, each held by the stamp-ahead hook at one re-issue.
  - Watch it next epoch: as a fact theme it reads n=1.
- **T4 — a pipeline tool's stdout sent to `/dev/null` and read back from the trail:** `2026-10-18T01:57:54Z-a`[1] (reconcile) plus prior `2026-10-16T17:49:24Z-a`[1] (reconcile). n=1 in-epoch, recurring.
- **Single-fact themes:**
  - an anchored Edit missed (`2026-10-18T01:57:54Z-a`[0])
  - a ScheduleWakeup armed against the letter (`2026-10-18T02:10:02Z-a`[0])
  - a P5 technical fork asked in P4's dialog form (`2026-10-18T03:16:13Z-a`[0])
  - a private copy of `is_checkable` kept inside the twelve-file source gate (`2026-10-18T03:31:59Z-a`[1])
  - the password caret's scroll frame (product-logic, `2026-10-18T03:31:59Z-a`[0])
  - the leaf-batch path check reading the orchestrator's own trail append (overridden, `2026-10-18T05:13:34Z-a`[0])

**Pass B signatures that did not fire:**
- deferred-forever: no deferral signal and no `deferred` fact; the one deferred impact is routed.
- override: three `overridden` facts, each on a different rule.
- removed-cause: none recorded.
