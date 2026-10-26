# Evolve Diagnosis — escher-0.2.0 · Epoch 5 — Polish & ship · 2026-10-25T21:30:11Z

Run dir `.andromeda/runs/2026-10-25T21-30-11-evolve-diagnose/` · evidence twins `q-retractions.json` · `q-health.json` ·
`q-typed.json` · `q-chains.json` · `q-level.json` · working script `analyze.py` (dumps `dump-untyped.json`,
`dump-problems.json`). Chunks: `2026-10-24-upstream-sync-polish-and-ship` (UPS) · `2026-10-24-stand-requirement-sweep`
(SRS) · `2026-10-24-a11y-re-read` (A11Y) · `2026-10-25-capability-statement` (CAP) · `2026-10-25-cold-agent-bar` (CAB).

## Mechanism health
- **Records:** 146 in the epoch (70 step / 76 friction), all `version: escher-0.2.0`; 0 retracted and 0
  clause-retracted in range. The whole-ledger pre-pass found 1,680 records with 0 unparseable and 0 malformed `ts`.
  It reads retracted 1 (`2026-10-20T01:37:33Z-c`, Epoch 4, outside this range; 0 unresolvable inside it) ·
  clause-retracted 4 (all before this epoch, kept) · retraction targeted by retraction: none. One retraction is never
  honored and is reported verbatim: `2026-10-07T04:21:25Z-a` → `{"id": null, "note": "in step record
  2026-10-07T04:21:04Z-a (a step record, so n…"` (0.1.0, for the founder's manual discount).
- **Coverage:** every chunk has every checkpoint: phase 5/5 · implement 3/3 · wrap 5/5 (report, reconcile, curation,
  route-resolve, gates). There were 5 new-session orientations. 0 gaps.
- **Outcomes not `ok`:** `halted-resolved` at phase/distill 3/5 (UPS, CAP, CAB), wrap/reconcile 4/5 (UPS, A11Y, CAP,
  CAB) and wrap/route-resolve 1/5 (CAB). No `ok-degraded`.
- **Untyped rate per step (untyped/frictions):** distill 4/23 · research 4/7 · plan 1/3 · validate 2/3 · code 1/4 ·
  fix-loop 1/2 · report 1/3 · reconcile 3/25 · curation 1/3 · orientation 0/1 · take-up 0/1 · route-resolve 0/1
  (18 untyped of 76).
- **Problem-fact fill:** 19/70 step records carry facts (27 facts: workaround 13 · overridden 5 · deferred 5 ·
  unresolved 2 · removed-cause 2). **Id fill:** 146/146.
- **Signal line:** guard-refused 43 over 70 step records · exact-hit 5 over 5 curation records. The step notes
  name what was refused: mostly the bash-guard `cd` arm, then a pipeline tool piped into a filter, plus a stdout
  redirect, a bounded-repetition grep and a heredoc to a file.
- **Calibration boundaries in range:** every deploy date in diagnosis-pass §Calibration boundaries (latest
  2026-10-19) predates this epoch (2026-10-24 to 10-25), so the whole range reads in the current era of every field.
- **Observation:** the distill playbook (`andromeda-phase/references/evolve/distill.md`) describes the step as seven
  distillers plus history agents with checks 1–6 and H1–H3. The deployed step also runs the forced-sample cascade:
  category readers, task writers, Haiku classifiers and strong readers, under the named checks QUOTES_VERBATIM ·
  CARRIED_ARE_CITED · COVERED_DROP · OWN_FILE · CLEAN_EXTRACT · EVERY_ENTITY · UNPLANNED_DROP. Re-spawns of those
  readers were recorded as typed `retry.distiller-respawn` four times and untyped three times (see P2 and U1).

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 6 cases · weight 16
**Pattern:** in each of the 5 chunks, reconcile wrote text that its own re-read or check then corrected before
landing (rate 6/5). Read-only lookback: the type appears in all 10 epochs of the ledger, at 3 to 10 per epoch.
**Evidence:** all 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UPS | 3 first writes corrected before commit: test-scope-summary.md:22 continuation lacked its ';'; cross-cutting-patterns.md:15 miscounted Instant::now() reads; cascade-dispositions.md Leaves bullet muddled | iterations 3 | `.andromeda/runs/2026-10-24T12-21-29-wrap/cascade-dispositions.md` |
| SRS | design-system-entry.md's Why first named an upstream layout move as the cite's cause (no report basis); cascade-dispositions.md first counted 14 edited standing rows, 17 on recount | retries 2 | — (`design-system-entry.md`, `cascade-dispositions.md`) |
| A11Y | first apply wrote provenance into 23 master sites across 9 sections (new chunk markers, a date, «relayed verbatim by»); `classify.py check --mode standing` failed each, and the tokens were stripped to the sidecar | retries 1 | `.andromeda/runs/2026-10-24T21-49-26-wrap/classify-2026-10-24-a11y-re-read.json` |
| CAP | standard-contracts.md:261 first written with the pin contract on the same line, past 2000 chars; split onto a continuation line | iterations 1 | — |
| CAP | cascade-patterns.toml first written with a doubled-quote TOML literal (sweep exit 2) and an atowner class `[^;.]` that the `.andromeda` dot defeats (exit 3); rewritten | retries 2 | `.andromeda/runs/2026-10-25T11-21-44-wrap/cascade-2026-10-25-capability-statement.json` |
| CAB | secret-management.md:14 fold duplicated the line's later second-run sentence; caught by the security word-diff read | iterations 1 | `.andromeda/runs/2026-10-25T17-25-04-wrap/replace-5.md` |

**Proposal:** each correction was caught by the pass itself, so the mechanism works; the cost is repeated
iterations. Three classes have a mechanical shape a tool could catch on the draft, before apply:
- **Provenance tokens in a body.** The A11Y case: 23 sites. The same event is the curation record
  `2026-10-25T01:35:44Z-b`, `recall.corpus-recurrence`, despite the 2026-10-19 learning.
- **A master line over 2000 chars.** The CAP case.
- **A `cascade-patterns.toml` literal or character-class fault.** The CAP case.

A direction: a pre-apply draft lint in classify/cascade that the writer runs on its own text, so these reach no
master or sweep. The judgment slips (an unbased Why, a recount, a duplicated clause) keep their re-read.

### P2 — phase/distill · `retry.distiller-respawn` — 4 cases · weight 17
**Pattern:** forced-sample cascade readers failed a named check on their first try in 2 chunks. QUOTES_VERBATIM
leads. Counting the three untyped records of the same class (U1) as well, the class spans all 5 chunks with 48
re-spawns (13 typed + 35 untyped), 28 of them in UPS alone.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SRS | cascade readers re-spawned 4×: category reader a11y keyboard-navigation on QUOTES_VERBATIM ('focusing' for 'focuses'), then OWN_FILE; strong arch g6 on QUOTES_VERBATIM + CARRIED_ARE_CITED; strong tests g4 on COVERED_DROP; each passed on its last try | retries 4 | `.andromeda/runs/2026-10-24T15-06-04-phase/.raw-a11y-cat-keyboard-navigation.md`, `-2.md`, `.raw-arch-cascade-g6.md`, `.raw-tests-cascade-picked-g4.md` |
| CAB | seven cascade agents failed QUOTES_VERBATIM on first try (catread security error-handling, tests e2e-test-strategy; strong arch g2/g3/g6/g7, a11y g3); each re-spawn passed | retries 7 | `.andromeda/runs/2026-10-25T12-59-18-phase/fanout-2026-10-25-cold-agent-bar.json` |
| CAB | strong reader design-cascade-g3 dropped depth-strategy#2.2 as a row no plan holds (UNPLANNED_DROP); re-spawn passed | retries 1 | `.andromeda/runs/2026-10-25T12-59-18-phase/.raw-design-cascade-picked-g3.md` |
| CAB | Haiku classifier a11y-cls-a11y-scope-summary.b1 wrote a stray `</content>` line after its answers (EVERY_ENTITY); re-spawn passed | retries 1 | `.andromeda/runs/2026-10-25T12-59-18-phase/.raw-a11y-cls-a11y-scope-summary.b1.md` |

**Proposal:** the dominant cause is the one overseer1 recorded as F332 (record `2026-10-24T10:42:33Z-c`): quotes drop a
provenance parenthetical or clause from the middle of a run. Two directions would remove it rather than retry it:
- The reader cites line ranges and a tool pastes the verbatim text.
- The prompt and the check share an explicit elision form.

The stray-tag and drop classes are small one-offs.

### P3 — wrap-session/reconcile · `retry.detector-respawn` — 4 cases · weight 16
**Pattern:** reconcile's fan-out re-spawned detectors, writers or classifiers in 4 of 5 chunks (9 re-spawns, 1
halt). OWN_FILE is the only check that recurs (SRS, CAP), and the halt was the harness's concurrency cap (see P8).
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UPS | one classifier spawn refused at the harness's 20-subagent limit; resumed at ≤16 running on the operator's word (inputs#I9) | retries 1 · halted 1 | `.andromeda/runs/2026-10-24T12-21-29-wrap/fanout-results.md` |
| SRS | architecture task writer [NEW_FACTS], architecture--conventions [OWN_FILE], test-plan--integration-test-strategy [YAML_FORM], two category detectors, one security-plan classifier that never returned | retries 6 | `.andromeda/runs/2026-10-24T17-32-19-wrap/fanout-results.md` |
| A11Y | test-plan cascade task writer named a reached file under `## Untouched` [UNTOUCHED_REACHED]; re-spawned once, passed | retries 1 | `.andromeda/runs/2026-10-24T21-49-26-wrap/batch-notes.md` |
| CAP | two security-plan category detectors failed [OWN_FILE] (no origin line of their file nor a body heading); re-spawned once, both passed | retries 1 | `.andromeda/runs/2026-10-25T11-21-44-wrap/fanout-2026-10-25-capability-statement.json` |

**Proposal:** OWN_FILE fails genuinely at reconcile (here) and is checked falsely by the orchestrator at distill
(P4). One direction is to make the own-file anchor something the agent writes structurally: a required
`origin: {file}:{line}` field checked by the fanout tool, where today it is inferred from prose. The cap-driven
halt belongs to P8.

### P4 — universal · `contract.token-proxy-check` — 5 cases · weight 12
**Pattern:** in 4 chunks, a structural check written for the run tested for a token where the property is
grammatical. Three of the five are one probe re-made in three chunks: OWN_FILE matching `(path:line)` inside
verbatim `  > ` quote lines. Read-only lookback: the type appears in 6 of 10 epochs.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UPS | my OWN_FILE probe matched (path:line) cites inside verbatim `>` quote lines and read 43 false positives; re-run excluding quote lines read 0 (false positive) | extra_reads 1 | — |
| UPS | my transcript audit for -detail/ reads first matched the readers' own Write content (19 hits), then two Bash regex strings; no -detail/ file was opened (false positive) | extra_reads 2 | — |
| SRS | the orchestrator's structural checker read tokens twice: [OWN_FILE] matched cites inside quoted `  > ` text (7 false hits), [CLEAN_EXTRACT] matched the `## Carried` heading and prose 'carried' (20 false hits) | reformulations 2 · extra_reads 2 | session scratchpad scripts `stage1_struct.py`, `strong_struct.py` |
| A11Y | the orchestrator's [OWN_FILE] probe matched any (path:line) and flagged 3 category readers on plan citations inside `  > ` quotes (false positive) | extra_reads 1 | — (`arch-cat-design-philosophy.md`, `arch-cat-project-intent.md`, `security-cat-dependency-security.md`) |
| CAP | COVERED_DROP fired on arch-cascade-g1.r1 for design-philosophy#2.1, whose not-in-scope reason the operator ruled sound: the check read the remark's wording as a held-by-another drop (false positive) | halted 1 · dialogue_rounds 1 | `.andromeda/runs/2026-10-25T02-58-45-phase/batch-notes.md` |

**Proposal:** the orchestrator rewrites its own OWN_FILE, CLEAN_EXTRACT and -detail/-audit probes in each session
(the scratchpad scripts), and the same false positive comes back. A direction is to ship those checks in the
fanout tool as grammar-aware commands: skip quote lines, and tell a heading from prose. The `-detail/` transcript
audit would read tool-input paths only. For COVERED_DROP, a closed reason class written by the reader
(held-by-another · not-in-scope) would replace inference from wording.

### P5 — universal · `contract.grammar-irregularity` — 6 cases · weight 6
**Pattern:** in one distill run, `sidecar.py summary` printed UNPARSED on six sidecars. Each was a different site,
so each site is n=1 and by the site rule "the data". All six are the same heading written by one writer: the
U55 upgrade's door heading, `## U55 — the door: …`, which is not in `{marker} — {title}` form.
**Evidence:** all 6 cases (UPS · distill · no impact · each `.andromeda/runs/2026-10-24T09-31-30-phase/sidecar-2026-10-24-upstream-sync-polish-and-ship.json`) —
| chunk | what | impact | evidence |
|---|---|---|---|
| UPS | U55 door heading not in `{marker} — {title}` form | — | `.andromeda/architecture-amendments.md:1114` |
| UPS | same | — | `.andromeda/security-plan-amendments.md:743` |
| UPS | same | — | `.andromeda/design-system-amendments.md:191` |
| UPS | same | — | `.andromeda/test-plan-amendments.md:875` |
| UPS | same | — | `.andromeda/obs-plan-amendments.md:461` |
| UPS | same | — | `.andromeda/a11y-plan-amendments.md:431` |

**Proposal:** this is one writer's form across six files, not six data one-offs. The plane to settle is whether
the upgrade door heading belongs in the sidecar grammar or should be written by setup-project's upgrade in
`{marker} — {title}` form. The ledger holds no later UNPARSED record for these sites in the epoch; whether the
summary still prints it is not measurable from the ledger.

### P6 — wrap-session/reconcile · `contract.false-positive-proposal` — 5 cases · weight 6
**Pattern:** Validate rejected detector proposals in 3 chunks (rate 5/5). The largest single class is re-pointing
citation digits that the P2 sweep had already written: 20 of SRS's 31 rejections. Read-only lookback: the type
appears in 9 of 10 epochs.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| SRS | Validate rejected 31 of 94: 20 re-pointing citation digits the P2 sweep already wrote, 7 duplicates, 4 no-change claims the report rules no-change; 0 for a coordinate | — | `.andromeda/runs/2026-10-24T17-32-19-wrap/fanout-results.md` |
| A11Y | test-plan strong reader g2 proposed F4 («the one file») — a duplicate of standing test-scope-summary :97/:100 whose exclusivity a dioxus-native-dom unit test contradicts; rejected at check 7 | — | `.andromeda/runs/2026-10-24T21-49-26-wrap/fanout-results.md` |
| A11Y | test-plan strong readers g1 and g2 each placed F5 at :19 and :97; g1's rejected as the duplicate placement | — | same |
| A11Y | the a11y-plan leaf comparer's replace-with for a11y-summary.md:35 named seven of eight a11y-leg files (inert_subtree left out); written by hand | — | `.andromeda/runs/2026-10-24T21-49-26-wrap/cascade-dispositions.md` |
| CAP | obs-plan's cascade proposed an observed-absent row whose own searched: clause said no silence census was fired; rejected at check 4 | iterations 1 | `.andromeda/runs/2026-10-25T11-21-44-wrap/fanout-results.md` |

**Proposal:** the A11Y untyped record `2026-10-25T01:33:33Z-g` shows the same root: three strong readers read
already re-pointed digits as old, and one note would have moved a range twice. Detectors appear to read cites
without knowing what the sweep has already re-pointed. A direction is to hand detectors the post-sweep tree, or the
sweep's list of applied re-points, so they do not re-propose them. Cross-reader duplicates could be merged by the
tool before Validate.

### P7 — universal · `contract.structural-blind-spot` — 4 cases · weight 6
**Pattern:** a documented mechanism could not reach something by construction. Three cases are one cause in UPS:
cascade.py's sweep, pre-pass and leaves read master bodies only, after U55 moved most master text into
`{doc}-detail/` category files. The fourth is a comparer writing outside the tree, which the path check cannot see.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UPS | `cascade.py sweep --pre-pass` reads the seven master bodies and registries but not the U55 category files; the owner-title control read unfired while established-decisions.md:75, :76, :78 name the chunk title as CARRY owner | extra_reads 2 · retries 1 | `.andromeda/runs/2026-10-24T12-21-29-wrap/cascade-2026-10-24-upstream-sync-polish-and-ship-pre-pass.json` |
| UPS | cascade.py sweep reads bodies and registries only: 10 of 21 step-2 patterns stood in category files alone and were refused unfired; the hand sweep found the stale «until upstream retires it» clause | extra_reads 2 · retries 1 | `.andromeda/runs/2026-10-24T12-21-29-wrap/cascade-2026-10-24-upstream-sync-polish-and-ship.json` |
| UPS | `cascade.py leaves` writes each leaf-delta diff from the body alone, so a category-only change (existing-scopes.md:76) never reached its comparer; services/dioxus-native-dom.md:38 fixed by hand | extra_reads 1 | `.andromeda/runs/2026-10-24T12-21-29-wrap/cascade-dispositions.md` |
| CAB | a leaf comparer overwrote the orchestrator's scratchpad script wdiff.py (outside the project), which the batch's path check cannot see; nothing in the tree touched | — | `.andromeda/runs/2026-10-25T17-25-04-wrap/prompts/leaf.manifest.json` |

**Proposal:** the category-file blind spot was absorbed mid-epoch: cascade v1.16 and cites v1.5 read the category
files (the project CLAUDE.md learning of 2026-10-24). What remains is the generalization: a document-layout
upgrade like U55 could list the tools that read the old layout and gate on them. For the fourth case, one
direction is a comparer write scope that is enforced and not only checked inside the tree. Another is to keep
orchestrator scripts in a path the batch's agents are not told about.

### P8 — universal · `contract.skill-reference-drift` — 2 cases · weight 10 (halt)
**Pattern:** `andromeda-phase/references/fan-out.md` disagreed with deployed reality in two places. Each cost the
operator a ruling.
**Evidence:** all 2 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| UPS | fan-out.md caps a launch message at 20 agents but not concurrent running agents; a forced-sample stage 1 needs 64 and the harness refused the 17th spawn of the second message (CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS=20); recorded by overseer1 as F331 | retries 1 · halted 1 · dialogue_rounds 1 | `.andromeda/runs/2026-10-24T09-31-30-phase` |
| A11Y | fan-out.md halts on a second failure of the same check, validation-cascade-strong.md re-spawns a reader ONCE; a reader whose retry failed a new check read as both a retry and a halt; the operator ruled per check | dialogue_rounds 1 | — (`fan-out.md`, `phase-1/validation-cascade-strong.md`) |

**Proposal:** the running cap recurs outside this group. Reconcile hit the same harness refusal
(`2026-10-24T13:59:24Z-f`, halted, P3). A pacing slip ran 17 against the operator's 16-running cap
(`2026-10-24T19:09:54Z-c`). The forced sample's 164 subagent notifications drove the window to the 85% context
alarm and stopped the take-up after P3 (`2026-10-24T16:10:58Z-b`, halted). The one problem fact is
`2026-10-24T10:42:33Z-a`, an environment workaround of paced waves.

One direction: fan-out.md states the running cap the operator ruled (≤16 running) and a context budget for a
forced sample, and states the halt rule per check as ruled. That rule's override is L3.

## Cross-step chains (starting heuristics)

### X1 — wrap-session/report →report→ wrap-session/reconcile — 3 chunks
- **UPS:** report `2026-10-24T12:28:09Z-a` was `ok` with signals `reconstructed`, `new-text-rows:50`. Its note
  says the site sweep's eight unfired patterns were re-located by git grep, and the producer carries the friction
  `contract.structural-blind-spot`. Reconcile `2026-10-24T13:59:24Z-a` graded the report `thin`. Friction
  `2026-10-24T13:59:24Z-d` (`input.report-insufficient`): a stale «until upstream retires it» dispositioned
  no-change.
- **SRS:** report `2026-10-24T17:39:11Z-a` was `ok` with signal `owner-control-unfired` and its own
  `contract.narrow-basis-claim` (`2026-10-24T17:39:11Z-b`, hand-typed hit counts). Reconcile
  `2026-10-24T19:09:54Z-a` graded it `thin`: the report did not say whether stand_act_spans declares `mod fixtures;`.
- **CAP:** report `2026-10-25T11:26:31Z-a` was `ok`, with the note «site sweep exit 3 on two unfired controls
  (title, docsdir), re-fired on the six others». Reconcile `2026-10-25T12:22:01Z-a` graded it `thin`. Friction
  `2026-10-25T12:22:01Z-c` (`input.report-insufficient`): atowner rows and disproved claims missed four sites.
- **Adjacent, at reconcile:** `contract.cascade-miss` twice (CAP `2026-10-25T12:22:01Z-d`, CAB
  `2026-10-25T20:17:45Z-b`). The step-2 sweep, re-fired, found stale master sites that no detector had proposed:
  8 and 7.

**Chain hypothesis:** in each chunk the report was formally `ok` while its site sweep left a control unfired or
exited 3. Reconcile then paid by re-reading, hand-sweeping or folding sites in-pass. **Direction:** a report whose
site sweep reads a control unfired could carry that as a signal reconcile reads as "sweep incomplete". Or the
report could re-fire the unfired patterns before grading itself `ok`, as UPS did by hand.

### X2 — phase/plan|validate →plan→ implement/code — 2 chunks (producer step differs)
- **SRS:** validate `2026-10-24T16:42:33Z-a` was `ok` (`baselines-17-green`, `review-fold-rerun-full-set`). Code
  `2026-10-24T16:49:41Z-b` (`input.plan-step-ambiguous`): step 3's assertion compares an 80 px box with
  "the twin's", but the only named twin is 300 px.
- **CAB:** plan `2026-10-25T14:11:57Z-a` was `ok` (4 founder forks). Code `2026-10-25T15:04:38Z-c`
  (`input.plan-step-ambiguous`): step 4 puts the select's options in the tab tables, which the union rule forbids.
  Code `2026-10-25T15:04:38Z-b` (`input.research-files-wrong`): stand_lint, red with the select, was missing from
  Files to modify.

**Chain hypothesis:** both are a plan step's assertion inconsistent with the fixture or rule the step itself names.
The green baselines and review did not test the assertion against that named fixture or rule. **Direction:** weak
at k=2. A validate predicate that checks each new assertion's named comparand would be the shape, if the founder
reads it as a pattern.

*Near-shape, below the heuristic:* research's Files to modify was thin in SRS (consumed by plan, `input.research-thin`
`2026-10-24T16:26:03Z-b`) and in CAB (consumed by code and fix-loop: stand_lint and stand_accessibility_ids
missing). The producer and artifact are the same but the consumer steps differ, so this is not counted.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 5 facts
**Theme:** disposable measurement spaces and their cleanup.
**Facts:**
- UPS · phase/research · process · workaround · `2026-10-24T11:10:12Z-a`[1] — research is read-only on the main
  tree; build feasibility measured in a throwaway worktree under target/tmp.
- UPS · phase/research · environment · unresolved · `2026-10-24T11:10:12Z-a`[2] — the global git config enables
  rerere, so the worktree trials wrote 7 resolutions into the shared `.git/rr-cache`. The host guard refuses a
  recursive rm, so they stand.
- CAB · phase/research · process · workaround · `2026-10-25T14:03:00Z-a`[0] — no tool measures a non-lean task as
  a driver session, so a throwaway blitz-tests target was written, run and removed.
- CAB · implement/code · process · workaround · `2026-10-25T15:04:38Z-a`[2] — recursive rm denied; probe dirs
  `target/tmp/recprobe-1` and `rbg-1` left in place, fresh unique dirs used.
- CAB · phase/distill · environment · removed-cause · `2026-10-25T13:52:50Z-a`[0] — /tmp tmpfs inodes 100% used
  (about 1M files in another project's finished Claude scratchpads). Every Bash call exited 1 with ENOSPC, the run
  halted, and overseer1 cleaned on the founder's word.

Correlates: `2026-10-24T11:10:12Z-b` (untyped, the rr-cache side effect) · `2026-10-25T10:01:52Z-b` (untyped, a
scratch clone lacking the uncommitted chunk folder read red twice) · `2026-10-25T13:52:50Z-b`
(`tooling.host-shell`, halted). Read-only lookback, prior epochs: 0.1.0 Epoch 2 `2026-10-06T08:02:00Z-a`/`-b` and
0.2.0 Epoch 1 `2026-10-16T19:34:20Z-a`/`-c` (a recursive rm refused, re-spelt to unique dirs).
**Level hypothesis:** the cause appears to live in the pipeline and environment, and the fixes so far live in the
project. The host's blanket recursive-rm deny is a sound safety default, but the pipeline gives no sanctioned way
to make, or to remove, a throwaway measurement space. Each chunk mints fresh unique dirs (the host-linux.md rule:
«fresh unique dirs, no `rm` in a launch path»), and residue accumulates: in target/tmp, in the shared rr-cache, in
finished sessions' /tmp scratchpads. In CAB the environment failed and a distill halted.
**Proposal:** a pipeline-level helper, either:
- a scoped removal tool in andromeda-tools that deletes only under a run-owned scratch root (target/tmp/{run},
  the session scratchpad) and refuses anything else; or
- a sanctioned disposable-measurement facility (a worktree or probe target with `rerere` off and teardown built
  in), plus a reaper for finished sessions' scratchpads.

### L2 — band-aid — 2 in-epoch facts and records, recurring from 2 prior epochs
**Theme:** a founder dialog outlives the driver's 58-minute dialog hold.
**Facts:**
- CAP · phase/plan · process · workaround · `2026-10-25T09:53:39Z-a`[0] — the P4 AskUserQuestion round went
  unanswered for 58 min and the driver hook ordered a stop. The forks were re-posed as text and answered by an
  operator relay (inputs#I2).
- CAB · implement/fix-loop · `2026-10-25T16:56:06Z-c` (untyped, dialogue_rounds 1) — the dialog asking the
  operator's word on the live run timed out after 58 min. The run stopped with an interim outcome and resumed on the
  relayed word.

Read-only lookback: 0.1.0 Epoch 5 `2026-10-10T00:34:22Z-a`/`-c`, `2026-10-10T08:21:37Z-a`,
`2026-10-12T08:16:22Z-a`, `2026-10-13T06:23:35Z-b`; 0.2.0 Epoch 2 `2026-10-17T14:21:45Z-a`,
`2026-10-18T01:57:54Z-d`.
**Level hypothesis:** the founder is reached through the operator in another session, and that relay's turnaround
regularly exceeds the dialog channel's 58-minute hold. Each time, the project absorbs it with a stop, a re-pose as
text, a relay file snapped as an input and a resume.
**Proposal:** a designed relay path in the pipeline, not a timeout fallback. For example, founder forks could be
written whole to a run-dir card from the start, as the operator directed on 2026-10-10. Or the dialog-hold expiry
could be a named state that ends the turn cleanly and resumes from the relay file.

### L3 — override — 2 chunks (halt-bearing)
**Rule:** a cascade reader halts on a second failure (fan-out.md) versus a reader being re-spawned once
(validation-cascade-strong.md).
**Facts:**
- A11Y · phase/distill · `2026-10-24T20:43:53Z-c` (`contract.skill-reference-drift`, dialogue_rounds 1) — a reader
  whose retry failed a new check read as both a retry and a halt; the operator ruled per check, on overseer1's
  standing ruling.
- CAP · phase/distill · process · overridden · `2026-10-25T03:40:32Z-a`[0] — arch-cascade-g1 failed COVERED_DROP
  twice, on different entities. P2 stopped, and the operator accepted the re-spawn output as it stands (F343).
- The same CAP event as a friction: `2026-10-25T03:40:32Z-c` (`contract.token-proxy-check`, halted 1,
  dialogue_rounds 1). The check's first firing was itself a false positive.

**Level hypothesis:** the operator corrects the same halt call in two chunks. The rule halts per agent while
failures are per check, and one of the checks has a known false-positive mode (P4). The rule, not the project,
may be miscalibrated.
**Proposal:** state the halt rule per check, as ruled, and pair it with P4's closed reason class for COVERED_DROP.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/distill — 3 cases → proposed: widen `retry.distiller-respawn` (or a sibling `retry.cascade-reader-respawn`)
**Cluster:**
- UPS `2026-10-24T10:42:33Z-c` (retries 28) — [QUOTES_VERBATIM] failed on 14 of 51 category readers and 14 of 19
  strong readers: quotes dropped a provenance parenthetical or clause; each re-spawned once and passed; F332.
- A11Y `2026-10-24T20:43:53Z-b` (retries 4) — 3 of 18 strong readers failed first-try: arch g5 [QUOTES_VERBATIM] 9
  + [CARRIED_ARE_CITED] 4, security g2 [QUOTES_VERBATIM] 1, tests g3 [CARRIED_ARE_CITED] 4; arch g5's retry then
  failed [COVERED_DROP].
- CAP `2026-10-25T03:40:32Z-b` (retries 3) — three strong readers failed COVERED_DROP on the first try.
- Adjacent: SRS `2026-10-24T16:05:12Z-d` (extra_reads 2) — two strong-reader retries stepped outside the prompt's
  form without failing a named check (a second Write; reads outside the read list).

The four typed P2 cases record the same class as `retry.distiller-respawn`, so the split is the playbook's text,
not the events. **Draft criteria line** (distill.md, beside the retry ledger): *"Re-spawns of the forced-sample
cascade's category readers, task writers, classifiers and strong readers, on a named cascade check
(QUOTES_VERBATIM · CARRIED_ARE_CITED · COVERED_DROP · OWN_FILE · CLEAN_EXTRACT · EVERY_ENTITY · UNPLANNED_DROP),
are `retry.distiller-respawn`; name the reader class and the check."*

### U2 — cross-step (plan · validate · report) — 3 cases → proposed: widen `contract.narrow-basis-claim` (or a universal `contract.unmeasured-figure`)
**Cluster:**
- UPS · phase/plan · `2026-10-24T11:25:43Z-c` (retries 2) — a lock-edge count written at P3 as twelve where the
  measured list names eleven, and a full sha typed from memory in a validation probe.
- SRS · phase/validate · `2026-10-24T16:42:33Z-c` — relay-2.md stamped the operator's word «about 16:43Z» by
  estimate before the clock was read (16:42:07Z), and it was snapped with that stamp.
- A11Y · wrap/report · `2026-10-24T21:54:29Z-b` (retries 1) — per-master standing counts hand-tallied from the sweep
  listing, three wrong, caught by re-summing to the tool's totals.

Typed neighbours of the same act: `contract.narrow-basis-claim` SRS `2026-10-24T17:39:11Z-b` (hand-typed hit
counts) and CAB `2026-10-25T14:03:00Z-c` (a hand-tallied reference count, 36/486 vs 49/466). Also
`contract.in-pass-correction` SRS `2026-10-24T19:09:54Z-e` (14 vs 17 on recount). Read-only lookback: one
hand-tally record in 0.2.0 Epoch 3. **Draft criteria line** (evolve-system.md §Universal types, narrow-basis-claim's
gloss): *"…including a figure — a count, a sha, a timestamp — typed by hand, from memory or by estimate where a
tool's count, the repository or the clock was available, and corrected once measured."*

## Below threshold — no action
**Typed groups**
- `contract.narrow-basis-claim` n=2 (SRS, CAB) — see U2.
- `recall.corpus-recurrence` n=2 (A11Y provenance tokens; CAP home path written verbatim) — the first is P1's A11Y
  event.
- reconcile `contract.cascade-miss` n=2 (CAP, CAB) — cited in X1.
- code `input.plan-step-ambiguous` n=2 (SRS, CAB) — X2.
- `tooling.host-shell` n=1, halt (CAB, /tmp inode ENOSPC) — L1. Lookback: the type is in 7 of 10 epochs, and its one
  in-epoch record halted, so no chronic-degrade.
- route-resolve `contract.no-sanctioned-channel` n=1, halt (CAB: the P5 card's directed source edit reached P5 with
  no permitted surface; with untyped `2026-10-25T20:17:45Z-e` and deferred fact `2026-10-25T20:25:45Z-a`). Read-only
  lookback: the type is in 5 epochs (9 records), so watch.
- reconcile `input.report-insufficient` n=2 — X1.
- validate `contract.mechanical-check` n=1 (sampler items cited under shortened ids).
- code `input.research-files-wrong` n=1 — X2.
- plan `retry.synthesis-rework` n=1.
- fix-loop `contract.test-expectation` n=1.
- orientation `input.handoff-git-mismatch` n=1 (handoff Position stale against HEAD 4b4804ba5).
- plan `input.research-thin` n=1 — X2 near-shape.
- distill `contract.extract-format` n=1.
- take-up `input.working-entry-thin` n=1 (the «where Viola's planning reads it» destination named nowhere).
- `contract.premise-falsified` n=1 (scope's AT-witness target).
- research `input.extract-signal-gap` n=1 (flight.sh).

**Untyped groups at the count but without a common cluster**
- reconcile untyped n=3: pacing 17 vs 16 (P8) · strong readers reading re-pointed digits as old (P6) · a directed
  edit missing its slot (no-sanctioned-channel, above).
- research untyped n=4, halt: rr-cache side effect (L1) · bare `cd` persisting the session cwd · context alarm 85%
  (P8) · tests extract claiming an order the pin does not assert.

**Untyped singletons**
- curation `2026-10-24T14:01:05Z-b` (guard-held duplicates first logged as recurrence).
- validate `2026-10-25T10:01:52Z-b` (scratch clone, L1 correlate).
- code `2026-10-25T15:04:38Z-d` (a stray `cat > /dev/null` blocking on stdin).
- fix-loop `2026-10-25T16:56:06Z-c` (L2).

**Problem-fact themes under threshold**
- Cascade sweep body-only, n=2, one chunk: absorbed by cascade v1.16 (P7).
- Subagent concurrency, n=1 fact (P8).
- Standing tool-use rule bypassed then corrected, n=3, but three different conventions with no shared obstacle:
  ledger no-read `2026-10-24T13:59:24Z-a`[1] · bare-read `2026-10-24T20:43:53Z-a`[0] · Edit tool
  `2026-10-24T13:59:24Z-a`[0].
- Session cwd moved by a `cd` that passed the guard, n=1 fact + `2026-10-24T11:10:12Z-c`.
- Singletons: `gh api` job log → `gh run view`; research's new file replaced at P4; tree.db stale and regenerated
  (removed-cause); a ScheduleWakeup fallback left scheduled (unresolved).
- Single overrides: Setup 3(c) resume at P3 · operator-added `-detail/` stop · AUTO-COMPACT handoff append vs the
  path check · F342 task-notification wait.

**Deferred-forever:** no hit. All 5 deferred facts name their destination: check 7 run in the resumed window ·
residuals.md:39 · the wrap report · residuals.md:41 (0.3.0's first chunk) · residuals.md:42.

**Chains with k=1:** research→plan (SRS) · distill →extracts→ research (CAB) · plan→validate (SRS) ·
research→code / fix-loop (CAB).
