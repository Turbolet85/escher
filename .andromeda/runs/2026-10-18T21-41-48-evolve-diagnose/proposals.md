# Evolve Diagnosis — escher-0.2.0 · Epoch 2 — Engine debts · 2026-10-18T21:41:48Z

Complete epoch: both chunks are `complete` in `.andromeda/master-route.md` and frozen in
`escher-0.2.0/working-route.md`. The whole document is recomputed from the ledger alone. The
epoch-to-date run of 2026-10-18T05:30:22Z was read only for the closing section, *What is new since the
05:30Z epoch-to-date run*, which the operator asked for at invocation. It did not suppress, merge or
re-rank anything above it.

## Mechanism health
- **Records:** 67 (36 step / 31 friction), all `escher-0.2.0`. The label `Epoch 2 — Engine debts` is unique in
  the ledger (0.1.0's Epoch 2 is «Element identity»). There are 0 unparseable lines and 0 malformed `ts`,
  epoch and ledger-wide. Ids are filled on 67/67.
- **Coverage vs expected:** no gaps.
  - `2026-10-17-upstream-sync-engine-debts`: phase 5/5 · implement 3/3 · wrap 5/5.
  - `2026-10-18-engine-debts`: phase 5/5 · implement 3 + 3 · wrap 5 + 5. The second set of each is
    the gated arm's resume (gated implement, gated-acceptance wrap), as designed.
  - `new-session/orientation`: 2 (chunk null).
- **Untyped rate:** 5/31 (16 %).
  - By step: plan 1/2 · validate 1/4 · reconcile 1/11 · gates 1/2 · smoke 1/2.
  - Every other step: 0.
- **Problem-fact fill:** 15/36 step records carry facts, 23 facts in all.
  - workaround/process 14 · overridden/process 7 · workaround/product-logic 1 · workaround/environment 1.
- **Signals:** guard-refused 18 over 36 step records · exact-hit 2 over 3 curation records.
  - 18/36 = 0.50 per step record, the highest per-record rate in the ledger. Since the signal went
    live: 0.1.0 E5 20/117 · E6 34/112 · 0.2.0 E1 11/47 · E2 18/36 · E3 (to date) 10/28.
  - The `cd` arm is the plurality (8 of the 18). The rest are pipes/redirects, a heredoc, the
    stamp-ahead hook and unnamed arms.
  - The third curation record (`2026-10-18T21:26:37Z-a`, the gated-acceptance wrap) carries no
    `exact-hit` word. It adds nothing and is not malformed.
- **Retractions** (ledger-wide pre-pass):
  - retracted 0, with 1 unresolvable outside this epoch: `2026-10-07T04:21:25Z-a`, an `id: null`
    target, kept for the founder's manual discount.
  - clause-retracted 3 (kept, notes rendered), 1 of them in this epoch:
    `2026-10-18T20:44:07Z-b` by `2026-10-18T20:44:29Z-a` (an evidence pointer).
  - No retraction is targeted by another retraction.
- **Calibration boundaries:** the whole epoch post-dates every boundary in `diagnosis-pass.md`
  (deviation scan, universal types, `in-pass-correction`, `operator-pass` entry, the 2026-10-13
  implement-outcome grading, new-text words, guard/exact-hit signals). None reads as era here.

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 4 cases · weight 8
**Pattern:** One reconcile run made four wrong first writes. That run was the engine-debts chunk's
first wrap, whose P2–P7 ran in a second window. A tool or a re-read caught each one before the
commit, at one retry each. The four have four distinct causes: a list-shape grammar, a summary line
counted as a row, a fork renamed in a payload, and estimated times.
**Evidence:** all 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-18-engine-debts | obs-plan's new detail group written as an indented list item with no blank line after its close label; `classify.py check` failed it (units 2), rewritten before the sidecar entry | retries 1 | `.andromeda/runs/2026-10-18T04-30-47-wrap/classify-2026-10-18-engine-debts.json` · `.andromeda/obs-plan.md:72` |
| 2026-10-18-engine-debts | the citation-dispositions writer counted the `cites.py` summary line `out-of-range 1` as a row (112 for 111); the asserted count caught it, the parser narrowed | retries 1 | — (`citation-dispositions.md`) |
| 2026-10-18-engine-debts | the a11y-plan sidecar payload named the founder's F3 fork `C3`; a re-read before the append caught it | retries 1 | — (`a11y-plan-entry-2.md`) |
| 2026-10-18-engine-debts | relay-4.md stamped two dialog answers with estimated times (05:10Z, 05:16Z); `inputs.py snap` refused the later as ahead of the clock, the stamp-ahead hook flagged it; rewritten `by 05:12Z` | retries 1 | inputs#I8 (`relay-4.md`) |

**Proposal:** Every case was caught in-pass, and its catcher is already there (`classify.py check`,
an asserted count, a re-read, `inputs.py`/hook). The fourth case belongs to the cross-epoch
stamp-ahead theme; see the appendix. The remaining generalization:
- The two hand-built-payload cases (the dispositions row count and the fork label) both came from
  re-typing a tool's output by hand.
- A direction: reconcile's dispositions and sidecar payloads are emitted by the tool from its own
  trail (row counts from the trail's JSON, fork labels from `inputs` ids), not transcribed. Then the
  re-read stops being the only guard.

### P2 — contract.structural-blind-spot (universal, cross-step) — 3 cases · weight 4
**Pattern:** The pipeline has three base finders: `gate.py scope`, `cites.py added` and
`cascade.py sweep`. Each resolves the chunk's base from this marker's oldest pre-CI commit across
the whole of HEAD's history. In a gated wrap with later chunks committed on top of the gated flip,
that base is the take-up's parent `34bcf71c`. The finders then read two other chunks' files as this
chunk's, or read every control unfired. By construction, no correct execution of the letter avoids it.
**Evidence:** all 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-18-engine-debts (implement/smoke) | gated take-up: `gate.py scope` took base 34bcf71c, so its set spans 275950dc0..1c9bc58b5 and lists 21 of the later chunks' files unrecorded; no scope-record line could be written truthfully, rows reported not resolved [clause retracted: evidence: `.andromeda/runs/2026-10-18T20-41-41-implement/scope-2026-10-18-engine-debts.json` — no such file; the scope call's record is in gate-2026-10-18-engine-debts.json] | extra_reads 1 | `.andromeda/runs/2026-10-18T20-41-41-implement/gate-2026-10-18-engine-debts.json` |
| 2026-10-18-engine-debts (wrap/report) | gated take-up with two later chunks on top: `gate.py scope` reads their 21 files unrecorded, `cites.py added --base {basis}` lists their 28 files, the plan's 34bcf71c-anchored source-list probe forecasts red (13); the letter has no rule, the operator ruled | extra_reads 3 | `.andromeda/runs/2026-10-18T20-46-02-wrap/gate-2026-10-18-engine-debts.json` · `cites-2026-10-18-engine-debts.json` · `plan.md:377` |
| 2026-10-18-engine-debts (wrap/reconcile) | `cascade.py sweep` resolves its baseline from the oldest pre-CI commit; in a gated wrap with later chunks on top that is the take-up's base, before the masters this pass amends, so every control reads unfired | retries 1 · extra_reads 1 | `.andromeda/runs/2026-10-18T20-46-02-wrap/cascade-2026-10-18-engine-debts.json` · `cascade-dispositions.md` |

**Proposal:** Curation absorbed this at project level: a Tier-3 entry «A gated wrap with later chunks
committed on top of its flip» (`2026-10-18T21:26:37Z-a`). The handoff also records the
`--baseline HEAD` friction. The pipeline-level direction has three parts:
- One shared base resolver for the three tools. It would take the gated basis commit (SKILL.md
  Setup 4's "gated basis bound") when the master record reads `gated`, and the oldest pre-CI
  parent otherwise.
- The resolver prints which rule it used.
- Plan-authored probes would be anchored on that resolver, not on a literal sha (the
  `34bcf71c`-anchored entry 21).

The same finder failed in the opposite direction one chunk earlier, from a merge-commit subject: see
`contract.token-proxy-check` in the appendix, absorbed as a 2026-10-18 Tier-1 learning. So the base
finder has failed both ways inside this one epoch.

## Cross-step chains (starting heuristics)

### X1 — wrap-session/report →report→ wrap-session/reconcile — 2 chunks (cross-step pattern candidate)
- **upstream-sync-engine-debts:**
  - Producer `2026-10-17T18:34:19Z-a`: `halted-resolved`, signals `new-text-generated`,
    `new-text-rows:85`, `guard-refused:3`.
  - Consumer `2026-10-18T01:57:54Z-a` read the report `thin`: "three facts absent or wrong:
    util.rs:8 SUPER->META, the convert_events todo!() arms gone, the three closed ARIA reads
    credited to tests that do not exist".
  - Friction `2026-10-18T01:57:54Z-b` `input.report-insufficient` (extra_reads 3).
- **engine-debts:**
  - Producer `2026-10-18T04:38:25Z-a`: formally `ok`, signals `new-text-generated`,
    `new-text-rows:123`.
  - Consumer `2026-10-18T05:13:34Z-a` read the report `ok`, noting "test-plan's per-OS tallies were
    the one fact it did not carry".
  - Friction `2026-10-18T05:13:34Z-g` `input.report-insufficient`: the plan's Expected amendment
    named the per-OS tallies, and the detector proposed nothing.
- **Hypothesis:** the report is written from session knowledge and a site sweep. It is never checked
  against the plan's Expected amendments, so a fact the plan says a master will need can be absent.
  Two upstream-sync-shaped chunks both lost a measured fact this way.
- **Lookback:** `input.report-insufficient` at reconcile appears in every epoch of the ledger, 26
  records.
- **Direction:** a report-side check that walks the plan's Expected amendments and confirms each one
  names a fact the report carries. The per-OS tallies (test-plan §9) are a fixed expected fact of every
  upstream sync.

### Single-chunk chain hypotheses (no pattern; listed for the record)
- **X2 — phase/distill →extracts→ phase/research →research→ implement/code** (upstream-sync-engine-debts).
  - Distill `2026-10-17T12:31:05Z-a` was `ok` (`scope-premise-challenged`). Its arch extract was wrong
    on how stand checks name keys (`2026-10-17T12:37:12Z-b` `input.extract-signal-gap`).
  - Research `ok` (`unresolved-questions`) wrote a key-migration convention that missed the
    `dioxus::prelude::*` glob shadowing `keyboard_types`.
  - Code read research `thin` and took 3 iterations (`2026-10-17T14:39:30Z-b`
    `input.conventions-gap`).
  - The transitive link is not scored.
- **X3 — phase (take-up / validate) →working-route→ wrap-session/route-resolve** (engine-debts).
  - Validate claimed v020-03 gated (`claims-refined`), but no step stamps a BLOCKED-ON on a frozen
    line taken up with no block.
  - Route-resolve read the working route `thin` and halted (`2026-10-18T05:16:28Z-b`
    `contract.no-sanctioned-channel`).
  - This feeds L1.
- **X4 — phase/plan →plan→ implement/code** (engine-debts).
  - Plan `2026-10-18T03:01:31Z-a` was `ok`. Its step 2 placed the inert focus-kept pin ambiguously
    ("its fourth test" under a 14-test paragraph).
  - The result was `2026-10-18T03:31:59Z-b` `input.plan-step-ambiguous` (weight 1).
- **Unjoined:** `2026-10-18T02:40:27Z-b` `input.working-entry-thin`. The entry says "nine" debts and
  lists none. Its producer is outside the chunk, so no same-chunk join exists.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 10 facts (6 workaround · 4 overridden), one chunk, nature process
**Facts** (all `2026-10-18-engine-debts`, `process`):

| record | step | solution | note |
|---|---|---|---|
| `2026-10-18T03:16:13Z-a`#1 | phase/validate | workaround | the gated arm, written for a BLOCKED-ON head, used for a chunk taken up with no block (claim --gated-file, a defer gated entry, step 13) |
| `2026-10-18T03:16:13Z-a`#2 | phase/validate | workaround | check 4 (9) asks a baseline of a NOT SEEN entry while gate.py runs no defer entry; baselined through --void in the scratch clone only |
| `2026-10-18T05:16:28Z-a`#0 | wrap/route-resolve | overridden | route-resolve edits only the markerless tail; on the operator's word an anchored Edit appended BLOCKED-ON to the frozen line so the gated flip can run |
| `2026-10-18T20:43:13Z-a`#0 | implement/code | workaround | code checkpoint appended after P2's single gated entry fired; the gated run's P1 was one file write |
| `2026-10-18T20:43:32Z-a`#0 | implement/fix-loop | overridden | the letter runs the block with the gated entry voided; the operator directed entry 26 alone |
| `2026-10-18T20:53:39Z-a`#0 | wrap/report | workaround | `cites.py added` took --base HEAD, not the gated basis fb842671 (that base listed 28 files / 843 lines of the later chunks) |
| `2026-10-18T20:53:39Z-a`#1 | wrap/report | overridden | `gate.py scope` read 21 later-chunk files unrecorded; the operator's ruling (inputs#I10) answered the stop |
| `2026-10-18T21:25:38Z-a`#0 | wrap/reconcile | workaround | `cascade.py sweep` default baseline 34bcf71c left every control unfired (exit 3); re-fired --baseline HEAD |
| `2026-10-18T21:40:26Z-a`#0 | wrap/gates | overridden | plan entry 21 (34bcf71c-anchored source-list probe) --skipped on the operator's word |
| `2026-10-18T21:40:26Z-a`#1 | wrap/gates | workaround | light gate fired before P3–P6, in parallel with the leaf apply, on a cold build |

Typed correlates:
- P2's three `contract.structural-blind-spot` records.
- `2026-10-18T05:16:28Z-b` `contract.no-sanctioned-channel`, a halt.
- `2026-10-18T05:28:03Z-b` `contract.coverage-hold`.
- The untyped `2026-10-18T03:16:13Z-c` (U1).

**Level hypothesis:** the cause appears to live in the pipeline's gated arm, which assumes two
things:
- (a) a BLOCKED-ON head at take-up;
- (b) nothing committed on top of the gated flip before the resume.

This chunk had neither. It was gated on the founder's discharge of a PROVISIONAL batch (see L2), and
two later chunks landed before his word came. Every step from validate to gates absorbed the gap at
project level, one step at a time: four operator rulings, six hand workarounds and a Tier-3 learning.

**Proposal:** a gated-arm revision in three parts:
- A sanctioned "gated on a word, no block" variant that writes its own BLOCKED-ON at validate, so the
  frozen line and the ledger cannot disagree.
- A gated-basis commit that every base-reading tool honours (P2).
- A rule for a gated resume with later chunks on top: what the light gate re-fires, how a
  sha-anchored probe is re-anchored, and whether the gated implement fires one entry or the voided
  block.

### L2 — band-aid + override — 3 facts in-epoch (1 workaround · 2 overridden), recurring from 0.1.0 Epochs 5 and 6
**Facts:**
- `2026-10-17T14:21:45Z-a`#0 · upstream-sync-engine-debts · phase/plan · process · workaround —
  "the AskUserQuestion round went unanswered for 58 min through the driver's dialog hook, which said
  to end the turn; the operator answered by message in the next turn, written verbatim to relay-2.md
  and snapped as inputs#I2".
- `2026-10-18T03:01:31Z-a`#0 · engine-debts · phase/plan · process · overridden — "the operator
  directed every founder fork into one run-dir file answered in the founder's stead as delegate, in
  place of a question dialog the overseer relays (inputs#I1); answers recorded PROVISIONAL".
- `2026-10-18T21:25:38Z-a`#1 · engine-debts · wrap/reconcile · process · overridden — "the
  Provisional discharge escalation resolved without a halt: the operator's resume directive (inputs#I11
  (b)) answered the card question … on the founder's own word inputs#I9".
- Related, not counted: `2026-10-18T03:16:13Z-a`#0, a technical fork at P5 asked by dialog.

Typed correlates:
- `2026-10-18T01:57:54Z-d` `ambiguity.escalation-rounds`: halted, the 58-minute hold expired, the
  wrap resumed in a second window.
- `2026-10-18T03:16:13Z-b` `contract.matrix-claim`: P5 replaced the founder's recorded answer with
  the delegate's PROVISIONAL one.
- `2026-10-18T05:28:03Z-b` `contract.coverage-hold`: v020-03 held below verified until his word
  came.

Lookback:
- `2026-10-10T00:34:22Z-a`#0 and `-c` (dialogs expired after 58 min).
- `2026-10-10T08:21:37Z-a`#0: overridden, the same "forks to one run-dir file" directive at
  `2026-10-10-driver-cli`.
- `2026-10-12T08:16:22Z-a`#0.
- `2026-10-13T06:23:35Z-b` (58 min, timed out).
- `2026-10-15T05:12:40Z-a`#0.
- After the epoch, context only: `2026-10-18T07:41:31Z-a`#0 at `2026-10-18-one-virtual-clock` takes
  the same directive shape.

**Level hypothesis:** the override signature is phase/plan's P4 rule. That rule asks the founder's
forks as a dialog the overseer relays inside a 58-minute hold, and the operator has overridden it
with one directive shape in three chunks over three epochs: forks written to one run-dir file and the
turn ended. The cause appears to live in the founder-question channel's timing assumption. The fixes
so far live in each chunk:
- relay files;
- delegate PROVISIONAL answers, which then need a discharge batch;
- a gated claim, which feeds L1.

**Proposal:** make the run-dir forks file (`founder-forks.md`) plus an ended turn the sanctioned P4
form whenever the founder is reached through a relay. Give its resume an inputs-snap rule, so the
operator's recurring directive becomes the letter. Then PROVISIONAL-and-discharge is needed only when
the founder delegates on purpose, not whenever the hold expires.

### L3 — chronic-degrade — `tooling.host-shell`, 6 records over 5 epochs, never halted
**Facts:**

| record | epoch | chunk / step | what |
|---|---|---|---|
| `2026-10-06T08:37:04Z-c` | 0.1.0 E2 | upstream-sync-element-identity / report | ugrep rejects `.{0,200}`; an empty stdout read as zero hits |
| `2026-10-06T21:59:01Z-b` | 0.1.0 E3 | snapshot-state-fidelity / validate | a bounded-repetition window errored under ugrep, printing no match |
| `2026-10-07T20:34:14Z-d` | 0.1.0 E4 | refusal-detection / report | a regex backslash pair collapsed in an inline heredoc; CI log colour codes as literal text |
| `2026-10-14T22:08:02Z-c` | 0.1.0 E6 | stand-requirement-sweep / report | `grep -o` with `.\{0,40\}` under the ugrep function printed nothing |
| `2026-10-15T08:17:08Z-b` | 0.1.0 E6 | cold-agent-test / research | `xargs command grep` — xargs looks for an executable `command`; the error discarded |
| `2026-10-17T14:21:45Z-c` | **0.2.0 E2** | upstream-sync-engine-debts / plan | a gate entry fired by hand as `bash -c` with TOML triple single quotes around single-quoted sed/awk broke the parse; re-run from a scratchpad script |

**Level hypothesis:** each specific was absorbed once it was seen:
- the ugrep rule went into the global CLAUDE.md and the 2026-10-07 session learning;
- the transport rules went into host-linux.md.

The class still returns in a new form each epoch, and this epoch's form (re-quoting a TOML-held gate
command) is not covered by any of them. The rules fix one specific at a time.

**Proposal:** a pipeline path that runs a plan entry's command without the host shell re-quoting it.
For example, a `gate.py` verb that prints one entry's command to a file, or runs it alone, outside the
wrap's `--entry` ban. A hand re-fire would then never go through a nested `bash -c`.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/validate · wrap-session/gates — 2 in-epoch cases, recurring from 0.2.0 Epoch 1 → proposed type: widen `contract.no-sanctioned-channel` to a Universal type
**Cluster:**
- `2026-10-18T02:10:02Z-b` (upstream-sync-engine-debts, gates): "the plan's preservation guard
  (entry 11) named deny.toml, which a later External-decay fix push had to move; the entry could only
  be read red by hand beside the tool (--entry is forbidden at the wrap) and skipped on the
  operator's directive".
- `2026-10-18T03:16:13Z-c` (engine-debts, validate): "validation.md check 4 (9) requires a baseline
  run of every NOT SEEN entry, but gate.py prints not run — defer for a gated entry and a not-run word
  may not be recorded as a baseline; resolved by firing the entry with --void over the scratch clone
  alone".
- Previous epoch: `2026-10-16T19:34:20Z-c` (fast-failing-mutants, validate): "plan-template's
  artifact key names 'the output removed first' … while this host's bash-guard refuses every
  recursive rm; the P4-authored rm -rf entries had to be re-spelt".
- Typed correlate: `contract.no-sanctioned-channel` exists today as route-resolve's local type. All
  8 records in the ledger sit there, and this epoch's `2026-10-18T05:16:28Z-b` is the halt.

**Draft criteria line:** `contract.no-sanctioned-channel` (universal). A step's letter (a check, a
template key, an arm) calls for an act that no deployed tool, flag or guard at that step lets it
perform. Record the act, the refusing tool or rule, and the substitute used (a hand read, a `--void`
scratch firing, a re-spelling). Where the conflict is between two written instructions rather than an
instruction and a tool, `contract.jointly-contradictory-instructions` wins.

## Below threshold — no action
- **Stamp-ahead (cross-form, cross-epoch — for the founder's eye):** a time written from an estimate
  and refused by the stamp-ahead hook or `inputs.py`.
  - In-epoch it crosses three record forms: untyped `2026-10-17T14:21:45Z-b` (plan), fact
    `2026-10-18T02:00:13Z-a`#0 (route-resolve, a hand-typed ledger `ts`), and P1's
    `2026-10-18T05:13:34Z-e`. The plan step `2026-10-18T03:01:31Z-a` also notes it in a guard-refused
    signal.
  - No single stage reaches threshold: 1 untyped, 1 fact.
  - Lookback: 21 rows over 6 epochs. The cause was recorded `removed-cause` twice
    (`2026-10-06T09:06:35Z-a`, `2026-10-07T16:46:22Z-a`#1) and `prohibition` once
    (`2026-10-07T23:08:04Z-a`), and it has returned in every epoch since.
- `phase/validate · contract.mechanical-check` n=2 (upstream-sync-engine-debts):
  - `2026-10-17T14:33:31Z-b`: touchpoint paths folded and listed under the wrong section.
  - `2026-10-17T14:33:31Z-c`: no predicate reads an expected amendment's coverage of a ruling's subjects.
- `recall.corpus-recurrence` n=2 (type-alone):
  - reconcile `2026-10-18T01:57:54Z-c` and curation `2026-10-18T01:59:21Z-b`, both a bare `cd`
    against host-linux «Paths & the working directory»;
  - the fact `2026-10-18T20:53:39Z-a`#2 (cd-led compounds, no refusal);
  - see the signal line (cd arm 8 of 18). Lookback: 36 rows over 8 epochs.
- `wrap-session/reconcile · input.report-insufficient` n=2. Carried as X1.
- `wrap-session/reconcile · ambiguity.escalation-rounds` n=1, halt. In L2.
- `phase/validate · contract.matrix-claim` n=1. In L2.
- `wrap-session/route-resolve · contract.no-sanctioned-channel` n=1, halt. In L1 / U1.
- `wrap-session/gates · contract.coverage-hold` n=1. The deferral closed at `2026-10-18T21:40:26Z-a`
  (`verified:v020-03`), so there is no deferred-forever hit.
- `wrap-session/report · contract.token-proxy-check` n=1: `2026-10-17T18:34:19Z-c`. The scope base
  finder's fixed-string match missed the merge commit's subject, a false negative. It was absorbed as
  the 2026-10-18 Tier-1 learning, and is the opposite failure direction of P2.
- `wrap-session/report · contract.premise-falsified` n=1: `2026-10-17T18:34:19Z-b`. "audit ok with
  deny.toml unchanged" was falsified by the External-decay fix.
- `implement/code · input.conventions-gap` n=1 and `phase/research · input.extract-signal-gap` n=1.
  X2.
- `implement/code · input.plan-step-ambiguous` n=1. X4.
- `phase/take-up · input.working-entry-thin` n=1: the "nine" debts are unlisted. Unjoined.
- `wrap-session/curation · ambiguity.tier-routing` n=1: `2026-10-18T01:59:21Z-c`, an operator-memory
  process entry judged at the Tier-1 bar, 2 B over cap.
- `phase/plan · tooling.host-shell` n=1. Carried as L3.
- `wrap-session/reconcile · contract.false-positive-proposal` n=1: `2026-10-18T05:13:34Z-f`, two
  "observed absent" claims with no search.
- Untyped singletons:
  - `2026-10-18T01:57:54Z-e`: `cascade.py leaves --apply` refused a needed regex backslash as
    escaped; written by hand.
  - `2026-10-18T20:44:29Z-a`: an evidence pointer written from an assumed per-verb trail name.
    Lookback: the clause retraction of `2026-10-09T21:21:27Z-b` is the same shape, and the take-up
    `2026-10-18T02:40:27Z-a` notes a CI trail "named by a guessed filename". It is emerging; watch
    it next epoch.
- Singleton facts:
  - `2026-10-18T01:57:54Z-a`#0 (anchored Edit missed) and #1 (sweep stdout to /dev/null);
  - `2026-10-18T01:59:21Z-a`#0 (Tier-1 602 B);
  - `2026-10-18T02:10:02Z-a`#0 (ScheduleWakeup armed against the letter);
  - `2026-10-18T03:31:59Z-a`#0 (bullet scroll, product-logic);
  - `2026-10-18T03:31:59Z-a`#1 (a private `is_checkable` copy to stay inside the twelve-file source gate);
  - `2026-10-18T05:13:34Z-a`#0 (route.py blocked run between snapshots, overridden).

## What is new since the 05:30Z epoch-to-date run
The operator asked for this section. The run of 2026-10-18T05:30:22Z covered the ledger to
`2026-10-18T05:28:03Z-b`. Since then the epoch gained 12 records (8 step / 4 friction), all from the
engine-debts gated resume: implement 20:43–20:44Z and the gated-acceptance wrap 20:53–21:40Z.
- **New proposal — P2** (`contract.structural-blind-spot`, 3 cases). All three cases post-date the
  earlier run. It is the typed face of L1's base-finder half.
- **L1 grew from 2 facts to 10.** Eight new facts are the resume's, and the theme now has a second
  sub-cause: (b), later chunks committed on top of the gated flip.
- **L2 gained one fact:** `2026-10-18T21:25:38Z-a`#1, the Provisional discharge answered on the
  founder's own word. Its correlate `contract.coverage-hold` is now closed (`verified:v020-03`).
- **Unchanged in substance:** P1, X1 (the report→reconcile pattern; listed as X1 in both runs), L3,
  U1.
- **Appendix additions:** the untyped evidence-pointer singleton and its clause retraction.
- **Not read here:** which of the earlier findings the overseer's ES80–83 hold. That mapping is in
  the overseer's records, not this pass's inputs.
