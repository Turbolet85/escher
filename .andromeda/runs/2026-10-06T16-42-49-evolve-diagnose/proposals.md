# Evolve Diagnosis — escher-0.1.0 · Epoch 2 — Element identity · 2026-10-06T16:42:49Z

Read-only over `.andromeda/friction-log.ndjson` (Epoch 1 as read-only lookback). Raw query outputs:
`q-retractions.json` · `q-health.json` · `q-typed.json` · `q-untyped.json` · `q-chains.json` · `q-facts.json` ·
`q-level.json` (all produced by `analyze.py` in this dir, except q-level, which holds the hand clustering). Every item
below is a direction for the founder. Accepting, rejecting, deferring or modifying any of them has no
mechanism-side consequence.

## Mechanism health

- **Records:** 128 (70 step / 58 friction), from 5 chunks plus 6 records with `chunk: null` (5 new-session
  orientations and 1 orientation friction).
- **Coverage per chunk vs expected** (phase 5 · implement 3 · wrap 5): every chunk is complete.

  | chunk | phase | implement | wrap-session |
  |---|---|---|---|
  | 2026-10-06-upstream-sync-element-identity | 5/5 | 3/3 | 5/5 |
  | 2026-10-06-stable-element-ids | 5/5 | 3/3 | 5/5 |
  | 2026-10-06-id-persistence | 5/5 | 3/3 | 5/5 |
  | 2026-10-06-project-readme | 5/5 | 3/3 | 5/5 |
  | 2026-10-06-accessibility-tree-identity | 5/5 | 3/3 | 5/5 |

  New-session: 5 orientations. No checkpoint gaps. A 0-pending wrap may carry at most a null-chunk `curation`
  record; none was written, and that is not a gap.
- **Parse quality:** 0 unparseable lines · 0 malformed `ts` · id fill 128/128.
- **Retractions:** 0 records retracted (whole ledger, pre-pass).
  - 1 clause-retraction exists in the ledger, written in Epoch 1. It targets Epoch 1 record
    `2026-10-06T02:22:58Z-b`, so nothing in this epoch's range is affected.
  - No unresolvable retraction · no retraction of a retraction.
- **Untyped rate:** 10/58 = 17%. Per step:
  - phase: take-up 1/3 · distill 2/3 · research 1/1 · plan 1/1 · validate 1/3
  - implement: fix-loop 1/4 · code 0/8 · smoke 0/1
  - wrap-session: report 3/9 · reconcile 0/10 · curation 0/8 · route-resolve 0/3 · gates 0/3
  - new-session: orientation 0/1
- **Problem-fact fill:** 16 of 70 step records carry facts (22 facts in all).
- **Calibration boundaries:** none fall inside this range. The whole epoch postdates the deviation scan, the
  required `id`/`retracts`, the Universal types and `contract.in-pass-correction` (2026-09-27), so no
  absence here is an era artifact.

## Proposals (typed patterns)

### P1 — wrap-session/{curation, reconcile} · `recall.corpus-recurrence` — 7 cases · weight 16
**Pattern:** a curated learning already stated a rule, and the same mistake recurred in 5 of 5 chunks (rate 1.2
per curation run). Four of the seven are the same Bash-guard learning, counted as its "third" and "fourth"
recurrence in-epoch. The handoff counts a fifth at the session-15 wrap; that wrap is outside the ledger and is
not counted here.
**Evidence:** all 7 cases (grouped by type across steps, per diagnosis-pass §Stage 1) —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-upstream-sync-element-identity | reconcile: the 2026-10-05 Tier-3 entry "a chunk that moves cited source lines stales the masters' citations" states the re-point rule; the headless-stand wrap still left 53 root Cargo.toml citations one line low | dialogue_rounds 1 | `08:46:30Z-d` · session-learnings.md:38 |
| 2026-10-06-stable-element-ids | a python-scripted .rs edit bypassed the PostToolUse rustfmt hook and reddened fmt, though session-learnings already says "run cargo fmt after a scripted edit" | iterations 1 | `09:46:51Z-b` · session-learnings.md |
| 2026-10-06-stable-element-ids | the Bash guard refused a file-target heredoc again (probe test file), past the curated entry and the handoff's carried recurrence | retries 1 | `09:46:51Z-c` · session-learnings.md |
| 2026-10-06-id-persistence | the curated Bash-guard entry recurred twice: a `cat >>` evidence heredoc and a leading cd into the scratchpad | retries 2 | `11:05:55Z-b` · session-learnings.md |
| 2026-10-06-id-persistence | the curated ugrep entry recurred: `grep -oE '.{0,60}crud.{0,80}'` failed "exceeds complexity limits" | retries 1 | `11:05:55Z-c` · session-learnings.md |
| 2026-10-06-project-readme | a leading cd into the skill references dir was refused, though session-learnings records the guard (third recurrence) | retries 1 | `11:48:46Z-b` · session-learnings.md |
| 2026-10-06-accessibility-tree-identity | the Tier-3 guard entry already states the rule; a cat heredoc to scope-record.md was still sent and refused whole (fourth recurrence) | retries 1 | `13:11:21Z-b` · session-learnings.md |

**Proposal:** the curation channel (a Tier-3 line in `.claude/docs/session-learnings.md`, read on demand) has not
reached the moment the command is composed for any of these four themes. The project absorbed each recurrence
by re-curating it or carrying it in the handoff, and the pattern held anyway. A pipeline-level option: once a
learning has recurred k times, route it out of the learnings corpus to a mechanical carrier. Candidate carriers
are a hook's refusal text naming the safe form, a line in the loop skills' shared tool conventions, or an
always-loaded rule. See L1 for the Bash-guard theme specifically.

### P2 — wrap-session/reconcile · `contract.in-pass-correction` — 4 cases · weight 15
**Pattern:** reconcile's first writes carried wrong citation spans, counts or section names in 4 of 5 wraps
(rate 0.8), each caught by a re-read before commit. Three of the four are `file:line` spans or counts.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-upstream-sync-element-identity | root-manifest sidecar entries for architecture and obs-plan named guessed Sections on first write; a per-citation section map corrected both before append | retries 1 | `08:46:30Z-c` · architecture-entry-2.md, obs-plan-entry-2.md |
| 2026-10-06-stable-element-ids | 7 citation spans and 2 counts wrong on first write (element_id.rs spans past fn ends, sidecar re-point counts 14/9 vs 8/7); caught by re-reading spans and counting cascade-citations rows | iterations 4 | `09:46:15Z-b` · runs/2026-10-06T09-35-44-wrap/cascade-citations.md |
| 2026-10-06-id-persistence | first Edit of services/seven_guis.md anchored on a leading backtick the line does not carry; re-anchored | retries 1 | `11:05:00Z-c` · — |
| 2026-10-06-accessibility-tree-identity | 6 first writes corrected: window.rs/accessibility_names.rs spans off by one, a layout-templates count split, a file list naming files it does not cite | retries 5 | `13:10:15Z-c` · runs/2026-10-06T12-55-34-wrap/repoint-masters-applied.txt |

**Related in-epoch records (outside this group's n):** citation maintenance also surfaces as:
- `08:37:04Z-e` structural-blind-spot: 55 Cargo.toml citations read one line low, and no detector could see it.
- `08:46:30Z-a#0` override: the operator chose fix-now for 53 stale citations.
- `08:46:30Z-b` and `11:05:00Z-b` false-positive-proposal: detectors cited spans the report does not carry.
- `09:46:15Z-c` playbook-no-match: citation re-points had no detector.
- `13:10:15Z-b` report-insufficient: shift-point citations were missing from the report.
- Two `drift-base: thin` verdicts (`08:46:30Z-a`, `13:10:15Z-a`): "no detector owns citation re-points"; 159
  re-points were orchestrator-raised.

Epoch 1 facts `2026-10-05T22:17:51Z-a#0` and `2026-10-06T01:48:08Z-a#0` re-pointed 112 and 80 citations by
scratch scripts ("no pipeline tool re-points citations after a chunk's line shifts").
**Proposal:** make citation spans a measurement rather than a memory. Two directions:
1. A pipeline tool that validates every `file:start-end` a reconcile pass writes against the file at HEAD (span
   inside the named symbol, end on the symbol's last line), and re-points citations from the chunk's git line
   map. That replaces the per-wrap scratch re-point scripts.
2. A citation form less brittle than line spans (symbol-anchored).

### P3 — implement/code · `contract.premise-falsified` — 3 cases · weight 8
**Pattern:** at implement, verification falsified plan premises about the third-party framework or build graph
(dioxus-core internals, cargo feature resolution). This happened in 2 of 5 chunks (rate 0.6 per code run).
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-stable-element-ids | plan step 1a grammar premises: base_scope named "root" (it is RootScopeWrapper over SuspenseBoundary/ErrorBoundary/root); VComponent.name a bare name (it is the full type path) | iterations 1, extra_reads 4 | `09:21:01Z-b` · plan.md, element_id.rs |
| 2026-10-06-stable-element-ids | unit case 5 premise "a re-render removal leaves the old NodeId no longer resolving" is false: dioxus-native-dom only detaches the node until its ElementId is reassigned | iterations 2 | `09:21:01Z-d` · element_id.rs |
| 2026-10-06-accessibility-tree-identity | plan step 3 gated the override behind dioxus-native-dom's accessibility feature, which nothing in the blitz-tests/seven_guis graph enables; every manifest that could was on the diff-guard list | dialogue_rounds 1 | `12:28:40Z-b` · chunks/2026-10-06-accessibility-tree-identity/scope-record.md |

**Proposal:** see X1. The three chunks with plan-premise trouble at implement are exactly the three where
research emitted `unresolved-questions`. A direction is a validate check or plan rule: every research
unresolved question becomes either a resolved premise, a plan fork, or an explicit implement-first probe step
(e.g. `cargo tree -e features`, a read of the locked dependency's source). That way an open question cannot
pass validate as a plain premise.

### P4 — implement/code · `input.plan-step-ambiguous` — 3 cases · weight 5
**Pattern:** plan steps left a test scenario's preconditions or a check's exceptions unstated, and implement
filled them (2 chunks, rate 0.6).
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-stable-element-ids | grammar not unique by construction: two same-name component instances share a path; restarting segments at every template root collides a nested root with a top-level one (and contradicts the plan's own pinned CRUD literal) | iterations 1 | `09:21:01Z-c` · plan.md |
| 2026-10-06-id-persistence | step 4c asserts every post-remount NodeId is fresh; the document skeleton (/html:0, head, body, #main) lives outside the VirtualDom and survives | iterations 0 | `10:11:37Z-b` · — (surfaced in the P4 report) |
| 2026-10-06-id-persistence | step 3 left CRUD Create's boot unspecified; Create after Delete appended text to the selected person's filled fields; reordered create → select → delete | iterations 1 | `10:11:37Z-c` · — |

**Proposal:** this sits close to P3 and to X1's chain: the same plan artifact, consumed thin at code. A
direction: the plan template's test steps state each scenario's starting state (fresh boot or prior steps) and
any elements a whole-set invariant excludes. Validate's mechanical checks could flag a whole-set assertion
("every NodeId", "all ids") that names no exceptions.

### P5 — wrap-session/route-resolve · `contract.carry-no-owner` — 2 cases (both halted) · weight 12
**Pattern:** an operator directive placed a note on the route without naming an existing owning entry, and each
case cost a halt plus one dialogue round (2 of 5 route-resolve runs).
**Evidence:** all 2 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-project-readme | the P5 directive named "the Polish & ship entry that owns release metadata", but no Epoch 6 entry names release metadata; armed halt with four dispositions, operator chose pin to Quality gates | dialogue_rounds 1, halted 1 | `11:49:50Z-b` · working-route.md |
| 2026-10-06-accessibility-tree-identity | the operator directed the seven_guis-ships-without-AccessKit note onto the route without naming an entry; one armed placement question, answered Stand a11y assertions | dialogue_rounds 1, halted 1 | `13:12:15Z-b` · — |

**Proposal:** when a directive's target is unnamed or unmatched, route-resolve could rank candidate owning
entries itself (by keyword over the remaining epochs) and offer them in the halt's single question. This keeps
the round from opening with an open-ended placement ask. The halt itself appears to be working as designed (one
round each); the cost is that it recurs.

### P6 — {wrap-session/report ×2, wrap-session/gates ×1} · `contract.token-proxy-check` — 3 cases · weight 6
**Pattern:** a count or sweep tested for a token where the intended property was semantic. There were 2 false
positives and 1 false negative, and each time the truth came from reading the hits or from a structured re-count.
**Evidence:** all 3 cases (grouped by type across steps) —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-upstream-sync-element-identity | report: the plan's expected-amendment count matched basenames (document.rs, node.rs, …). False positive: unchanged same-name files counted; truth from a map keyed on the 17 full paths | extra_reads 1 | `08:37:04Z-d` · plan.md:304 |
| 2026-10-06-project-readme | report: the site sweep used `grep -i 'MPL'`, which matched "exaMPLe" (architecture 81 hits vs 1). False positive; recurred in a second pipeline | retries 2 | `11:44:41Z-b` · architecture.md |
| 2026-10-06-accessibility-tree-identity | gates: the evolve-nudge record count read 0 for the Epoch 2 label through grep (ugrep) while a JSON read gave 126 / 57. False negative on an em-dash fixed string | retries 1 | `13:15:16Z-b` · friction-log.ndjson |

**Proposal:** two of the three sit on pipeline-side counts:
- The gates nudge count could come from a tool that parses the ledger (with diagnosis-pass §Tolerant parsing's
  epoch folding) rather than an agent-composed grep.
- The plan's expected-amendment site counts could be keyed on full changed paths by the tool that derives them.

## Cross-step chains (starting heuristics)

### X1 — phase/plan →plan→ implement/code — 3 chunks (also →fix-loop in 2, →wrap-session/report in 4)
**Consumer ends:** plan consumed `thin`/`wrong` at implement in 3 chunks:
- stable-element-ids `09:21:01Z-a`
- id-persistence `10:11:37Z-a`
- accessibility-tree-identity `12:28:40Z-a`

It was also consumed thin/wrong at fix-loop (`10:17:15Z-a`, `12:39:20Z-a`) and at smoke (`12:40:26Z-a`). The
wrap report then rated the plan thin in 4 chunks (`08:37:04Z-a`, `09:37:59Z-a`, `10:57:29Z-a`, `12:58:15Z-a`).
Typed frictions at the consumer: P3's 3, P4's 3, `spec-reality-gap` ×2 (`10:17:15Z-b`, `-d`).

**Producer ends:** every `phase/plan` record is `outcome: ok` with no quality-warning signal. Validate consumed
the plan as `ok` in all 5 chunks.

**Chain hypothesis:** research emitted the signal `unresolved-questions` in exactly the three chunks where
implement later found plan premises false (`09:01:30Z-a`, `10:00:34Z-a`, `12:11:01Z-a`). It emitted it in
neither chunk where implement found none (upstream-sync, project-readme). Each time, plan rated research `ok` and
validate rated the plan `ok`. The open questions seem to leave research as a signal and arrive at implement as
premises, and no step in between turns them into a probe.

**Direction:** P3's validate check (each unresolved question → a resolved premise, a fork, or an implement-first
probe).

### X2 — implement/{fix-loop, smoke} →implement-outcome→ wrap-session/report — 3 chunks
**Consumer end:** the report rated `implement-outcome` thin in 3 chunks:
- upstream-sync `08:37:04Z-a`: the API list missed upstream #1063.
- id-persistence `10:57:29Z-a`: superseded by the operator pass and the entry-3 amendment.
- accessibility-tree-identity `12:58:15Z-a`: superseded by an operator directive amending gate 6, plus the
  operator pass. Typed correlate: `12:58:15Z-b` `input.implement-outcome-unsettled`.

**Producer ends:**
- upstream-sync: fix-loop `08:15:16Z-a` ok · `green`.
- id-persistence: fix-loop `10:17:15Z-a` halted-resolved · `surfaced` (3 frictions).
- accessibility-tree-identity: fix-loop `12:39:20Z-a` and smoke `12:40:26Z-a`, both ok-degraded · `surfaced`.

**Chain hypothesis:** two of the three share one shape. Implement ends `surfaced`, the operator then amends the
plan and runs the pass, and the outcome artifact has no form that records the supersession, so the report
rebuilds its basis each time. The upstream-sync case is a different cause (a merge carrying upstream changes no
session wrote; see `recall.change-reconstruction` in the appendix).

**Direction (for the 2-chunk shape):** a settled-outcome record, re-emitted after an operator directive or pass,
that the report consumes instead of the superseded P4 outcome.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — the project Bash guard (leading cd / heredoc-to-file / chained rm -rf) — 4 facts (11 in Epoch 1)
**Facts (Epoch 2):**
- upstream-sync · research · environment · workaround: "permission guard refused a chained rm -rf + cd + git
  archive export; re-run as plain mkdir + git archive|tar" (`08:02:00Z-a#0`)
- stable-element-ids · distill · process · workaround: "refused a leading cd into the run dir … re-ran with a path
  variable (recurrence of the carried deferred learning)" (`08:58:03Z-a#0`)
- id-persistence · distill · environment · workaround: "a non-leading cd … moved the session cwd; the Bash guard
  then refused a leading cd back; continued with absolute paths" (`09:56:54Z-a#0`)
- accessibility-tree-identity · code · process · workaround: "PreToolUse guard refused a cat heredoc writing
  scope-record.md (the whole call, manifest edits included, did not run)" (`12:28:40Z-a#1`)

**Lookback (Epoch 1, read-only):** 11 facts of the same theme, listed in q-level.json.

**Typed correlates in-epoch:**
- untyped `08:02:00Z-b`, `08:58:03Z-b`, `09:56:54Z-c`
- `tooling.hook-friction` `09:21:01Z-e`, `12:28:40Z-c`
- `recall.corpus-recurrence` `09:46:51Z-c`, `11:05:55Z-b`, `11:48:46Z-b`, `13:11:21Z-b`

That is 13 records across 5 of 5 chunks. As a chronic-degrade signal, `tooling.hook-friction` also recurs
across both epochs (2 + 2) and has never halted.

**Level hypothesis:** the obstacle is a standing interaction between the environment's PreToolUse guard and the
shell forms the agent reaches for, while skills direct it into run dirs and reference dirs. The fixes land one
call at a time in the project, and the project-level learning has not changed the rate: 15 facts over two
epochs, every chunk affected.

**Proposal:** an environment- or pipeline-level fix in place of a further project learning. Options:
- the loop skills' shared tool conventions name the guard-safe forms (absolute paths / `--root`, the Write tool
  for file payloads, no cwd-moving `cd`) at the points that send reads into run dirs;
- the guard's refusal message prints the safe rewrite;
- the founder re-weighs which guarded forms (e.g. a subshell `( cd … )`) need blocking.

### L2 — band-aid — the run-bare rule (tool output piped through head / tail) — 4 facts (2 in Epoch 1)
**Facts (Epoch 2):**
- stable-element-ids · validate · process · workaround: "the P4 unclaimed-pool listing was read through head -5,
  against the tools' run-bare rule; re-listed whole at P5 step 4" (`09:10:26Z-a#2`)
- id-persistence · take-up · process · workaround: "route.py pins piped through head, against the bare-call rule;
  output was 3 lines under the cap, nothing cut" (`09:52:27Z-a#0`)
- id-persistence · distill · process · workaround: "sidecar.py cites piped through tail -n +2 … the verdict lines
  were all kept" (`09:56:54Z-a#1`)
- accessibility-tree-identity · validate · process · workaround: "matrix.py show --unclaimed was piped through
  head -5 … the full listing had been read at P4" (`12:22:11Z-a#0`)

**Lookback:** `2026-10-05T20:43:00Z-a#0`, `2026-10-06T01:59:52Z-a#0`. No typed friction correlate exists: these
are smooth workarounds the deviation scan alone caught.

**Level hypothesis:** the rule lives in the pipeline, and the deviations are habitual and recorded as harmless in
every case. The cause may be the rule's calibration (bounded reads of short listings) or the tools' lack of a
bounded mode, rather than anything project-side.

**Proposal:** the founder judges the rule against this record. Options:
- give the tools a bounded flag that prints an explicit truncation marker, so the rule and the habit agree;
- or narrow the rule to the outputs where a cut can hide a verdict.

### L3 — band-aid — TOML string quoting in the plan's Test Commands fence — 2 facts (1 + 1 recurring from Epoch 1)
**Facts:**
- stable-element-ids · validate · process · workaround: "baseline of a TOML triple-quoted run was first fired
  through bash -c with the TOML quotes (syntax error); re-run from a scratchpad script holding the entry's exact
  run" (`09:10:26Z-a#0`)
- Epoch 1, telemetry-bootstrap · plan · process · workaround: "fixed an invalid TOML literal (doubled quote in a
  single-quoted note) … with a python heredoc replace" (`2026-10-06T00:28:48Z-a#1`)

**Untyped correlates:**
- Epoch 2 `09:06:35Z-b`: "an unescaped apostrophe in a literal note … caught by the dry-run" (a secondary
  clause).
- Epoch 1 `2026-10-05T22:49:41Z-b` and `2026-10-06T03:22:08Z-c`: a baseline string with an apostrophe in a TOML
  literal, which made the fence unparseable.

That is 5 occurrences across 2 epochs.

**Level hypothesis:** the fence's grammar (TOML strings carrying shell commands and free prose) is hand-authored
and hand-fired, and each chunk repairs its own instance.

**Proposal:** a fence writer and runner in the pipeline tools:
- emit entries through a TOML serializer (basic strings);
- fire baselines from the parsed fence, never from copied TOML text.

### L4 — band-aid — no sanctioned home for validate's known-verdict control files — 2 facts (1 + 1 recurring from Epoch 1)
**Facts:**
- stable-element-ids · validate · process · workaround: "a hook refused control .rs copies in the run dir
  (gate.py hygiene P3); controls written to the session scratchpad instead" (`09:10:26Z-a#1`)
- Epoch 1, as-built-baseline · validate · environment · workaround: "a control-file Bash call (shell redirects +
  rm -r into the run dir) was denied by permission; controls re-done with Write-tool files in the session
  scratchpad" (`2026-10-05T19:47:53Z-a#0`)

**Level hypothesis:** validate asks for controls, and the run dir refuses them, by different mechanisms in each
epoch (hygiene hook vs permission). The agent lands on the scratchpad each time.

**Caveat:** the mechanisms differ, so this is the thinnest of the level candidates.

**Proposal:** the validate reference names where controls live (the scratchpad, with the verdicts recorded in
the plan baseline), so hygiene and the letter agree.

### L5 — chronic-degrade — `tooling.output-cap-overflow` — 2 in Epoch 2, 6 in Epoch 1, never halted
**Facts:**
- orientation (null chunk): "evolve-system.md and health-criteria.md read in one bundled cat (31.5 KB) overflowed
  the tool-result cap" (`08:51:43Z-b`)
- id-persistence · distill: "one cat of the 7 extracts (45.2 KB) exceeded the tool-result cap" (`09:56:54Z-b`)

**Lookback:** 6 Epoch 1 records plus fact `2026-10-06T00:13:35Z-a#0`: "loaded evolve-system.md and
health-criteria.md in one cat call although the letter says skill references load per-file".

**Level hypothesis:** the reference and extract sizes sit near or above the cap, and the bundled read recurs in
every epoch at orientation and distill. Each overflow costs only one re-read, so the halt policy never surfaces
it.

**Proposal:** two options:
- size distill's extract output and the orientation references below the cap (or give them an index line);
- state the cap in bytes in the letter that already says "per-file".

### L6 — override — plan-authored guard entries read red for a reason outside the chunk's intent — 3 facts
**Facts:**
- id-persistence · fix-loop · process · overridden: "entry 3 reds on 4 leading spaces after the row loop
  un-nested; operator chose keep fix + surface entry 3, plan.md entry to become whitespace-insensitive"
  (`10:17:15Z-a#0`)
- project-readme · gates · process · overridden: "light-gate entry 17 (diff guard) red only from the wrap's own P3
  curation write to .claude/docs/session-learnings.md; operator chose accept-record-commit over red-means-no-commit"
  (`11:52:55Z-a#0`)
- accessibility-tree-identity · code · product-logic · overridden: "the operator chose to add
  features=["accessibility"] to tests/blitz-tests/Cargo.toml, a diff-guarded path (widening recorded)"
  (`12:28:40Z-a#0`)

**Correlates:**
- `10:17:15Z-c` contract.instrument-validity ("its P5 controls never tried a re-nest")
- `11:52:55Z-b` tooling.light-gate-red (halt): "gate.py scope's ruled exclusion set covers .claude/docs/** and
  read clean"
- `12:39:20Z-a` consumed plan thin: "entry 6 … reads red by construction"

**Level hypothesis:** the operator corrected the same family of call three times. The plan restates its own
exclusion set, whitespace sensitivity and guarded path list per chunk, and those restatements drift from what
the pipeline already rules (gate.py scope's exclusion set) and from sanctioned widenings.

**Caveat:** the three rules differ in letter, and grouping them is this diagnosis's judgment.

**Proposal:** three directions:
- plan diff guards inherit gate.py scope's ruled exclusion set instead of restating it;
- presentation guards compare whitespace-insensitively by default, with a re-nest control at validate;
- a sanctioned widening updates the guard list it crosses.

**No hits:**
- **Deferred-forever:** Epoch 2 has no `solution: deferred` fact and no `deferral-open` signal.
- **Removed-cause theme at threshold:** none. The single stamp fact routes to U1.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — wrap-session/report ×3, phase/validate ×1 — 4 cases → proposed type `contract.unmeasured-stamp` (universal)
**Cluster:**
- report `08:37:04Z-f`: "the report's Date field was written as an estimated time 8 min ahead of the clock; the
  stamp-ahead PostToolUse hook refused it"
- validate `11:26:17Z-c`: "two time stamps were estimated instead of read from the clock … the stamp-ahead hook
  refused the first, the second was found and corrected from the gate trail's ts"
- report `11:44:41Z-c`: "report Date written as an estimated 11:50Z without reading the clock; … refused it (+5 min)"
- report `12:58:15Z-c`: "the report's Date was written as an estimated 13:05Z; … refused it (+7 min ahead of the
  clock)"

**Same class as a fact:** `09:06:35Z-a#0` (plan · removed-cause: "stamp-ahead hook refused the plan's Generated
stamp written without reading the clock"). The class spans 3 steps in 2 skills, so a universal type fits better
than one playbook.

**Draft criteria line:** "`contract.unmeasured-stamp` — a time-valued field (Date, Generated, a re-read or
baseline stamp) was written from an estimate instead of a clock read in the same step; record the field, the
estimate's offset from the clock, and what caught it (the stamp-ahead hook, a later re-read, or nothing)."

### U2 — phase/{research, distill ×2} — 3 cases → extend the phase playbook with `tooling.hook-friction`
**Cluster:**
- research `08:02:00Z-b`: "Bash permission guard refused a chained export command containing rm -rf and a
  leading cd … recovered with guard-safe forms"
- distill `08:58:03Z-b`: "leading cd into the run dir refused by the project's Bash guard during extract probing"
- distill `09:56:54Z-c`: "a cd mid-command … moved the session cwd; the next call's leading cd back to the project
  root was refused"

**Recurrence:** Epoch 1 distill `2026-10-06T04:02:06Z-b` belongs to the same cluster.

**Draft criteria line (phase playbook):** "`tooling.hook-friction` — a PreToolUse/permission guard refused a
command and the step re-ran it in a guard-safe form; record the refused form and the safe form."

**Note:** the type already exists at implement (`09:21:01Z-e`, `12:28:40Z-c`). The records here were filed
untyped at phase, so it appears absent from phase's list; that is inferred from the ledger, not read from the
playbook. Promotion to a universal type is the alternative. The cause-side direction is L1.

### U3 — implement/fix-loop — 2 cases (1 + 1 recurring from Epoch 1) → proposed type `tooling.format-hook-gap`
**Cluster:**
- `09:24:50Z-b`: "the fast leg's fmt check went red on element_id.rs: edits applied through a python script in
  Bash bypass the PostToolUse rustfmt hook that Write/Edit trigger"
- Epoch 1 `2026-10-06T00:40:59Z-c`: "the PostToolUse rustfmt hook left lib.rs unformatted: it was written before
  its mod format file existed, so the hook's rustfmt could not resolve the module"

**Correlate:** `recall.corpus-recurrence` `09:46:51Z-b`.

**Caveat:** the two cases have different mechanisms (hook not fired vs hook fired but unresolved).

**Draft criteria line (implement playbook):** "`tooling.format-hook-gap` — the fmt gate went red on a file the
format-on-write hook was relied on to format (edit made outside Write/Edit, or the hook could not resolve the
file); record the edit path and why the hook missed it."

### U4 — phase/{plan, validate} — 3 cases across 2 epochs (1 in Epoch 2) → proposed type `contract.fence-quoting`
**Cluster:**
- Epoch 2 plan `09:06:35Z-b`: secondary clause, "an unescaped apostrophe in a literal note … caught by the
  dry-run".
- Epoch 1 validate `2026-10-05T22:49:41Z-b`: "the scripted baseline insert wrote a baseline string holding a bare
  apostrophe into a TOML literal (fence unparseable)".
- Epoch 1 validate `2026-10-06T03:22:08Z-c`: "baseline 1's text held an apostrophe … so the fence went UNPARSED".

**Draft criteria line (phase playbook):** "`contract.fence-quoting` — a Test Commands fence entry failed to parse
or fire because of TOML string quoting (literal string holding an apostrophe, triple-quoted run passed to a
shell); record the entry, the string form and the catcher."

**Caveat:** the in-epoch member is a secondary clause of a record whose main subject is a stale citation. The
cause-side direction is L3.

## Below threshold — no action

Typed groups:
- implement/fix-loop · `contract.spec-reality-gap` — 2 (id-persistence `10:17:15Z-b`, `-d`). Plan premises about
  if-in-for row identity and the remount skeleton; related to X1.
- phase/validate · `contract.mechanical-check` — 2 (`08:09:16Z-b`, `11:26:17Z-b`). Required-resolutions closed in
  place.
- wrap-session/reconcile · `contract.false-positive-proposal` — 2 (`08:46:30Z-b`, `11:05:00Z-b`). Detectors cited
  source spans the report does not carry; see P2's related records.
- implement/code · `tooling.hook-friction` — 2 (`09:21:01Z-e`, `12:28:40Z-c`). Counted in L1.
- wrap-session/curation · `ambiguity.filter-borderline` — 2 (`08:47:42Z-b`, `13:11:21Z-c`). Candidates scoring
  exactly 0.6.
- `contract.structural-blind-spot` — 2 by type: report `08:37:04Z-e` (Cargo.toml citations), smoke `12:40:26Z-b`
  (the AT-SPI-inactive host never builds the platform tree).
- `tooling.output-cap-overflow` — 2 by type. Surfaced as L5 (chronic-degrade).
- wrap-session/gates · `tooling.light-gate-red` — 1, halted (`11:52:55Z-b`). Counted in L6.
- wrap-session/reconcile · `ambiguity.escalation-rounds` — 1, halted (`13:10:15Z-d`). author_id crossing to the
  platform adapter; ratified.
- wrap-session/report · `tooling.host-shell` — 1 (`08:37:04Z-c`). ugrep bounded repetition, a false zero; see the
  ugrep theme below.
- implement/fix-loop · `contract.instrument-validity` — 1 (`10:17:15Z-c`). Counted in L6.
- wrap-session/route-resolve · `ambiguity.trajectory-halt` — 1 (`11:07:07Z-b`). The adaptation request arrived as a
  bare pasted block.
- wrap-session/gates · `contract.coverage-hold` — 1 (`11:09:47Z-b`). v010-02 reached coverage with ref withheld.
- phase/take-up · `contract.skill-reference-drift` — 1 (`07:53:29Z-b`). promotion.md §External inputs omits
  `--origin` for `--message-file`, and `inputs.py` refuses without it. **The same defect was recorded untyped in
  Epoch 1 (`2026-10-05T20:52:30Z-b`).** It is a pipeline letter defect recurring across epochs, under the typed
  threshold only because the first occurrence was untyped.
- wrap-session/report · `recall.change-reconstruction` — 1 (`08:37:04Z-b`). The merge chunk's ~600 upstream
  lines were reconstructed by git archaeology.
- phase/take-up · `input.working-entry-thin` — 1 (`08:53:27Z-b`). "author key else component path" had no engine
  definition.
- wrap-session/reconcile · `ambiguity.playbook-no-match` — 1 (`09:46:15Z-c`). 6 expected amendments had no
  detector; see P2.
- wrap-session/report · `input.implement-outcome-unsettled` — 1 (`12:58:15Z-b`). Counted in X2.
- wrap-session/reconcile · `input.report-insufficient` — 1 (`13:10:15Z-b`). Shift-point citations were missing
  from the report; see P2.
- wrap-session/reconcile · `recall.corpus-recurrence` — 1 (`08:46:30Z-d`). Folded into P1 by type.

Untyped singletons:
- phase/plan `09:06:35Z-b` (main clause): a hygiene atom citation copied from the prior plan as
  gate-contract.md:271 had moved to :279.
- phase/take-up `09:52:27Z-b`: working-route.md read at a guessed `.andromeda/` path; it lives in
  `escher-0.1.0/`.

Problem-fact themes under threshold (Pass A):
- **ugrep as host grep** — 2 facts (`08:37:04Z-a#0`, `10:57:29Z-a#0`), none in Epoch 1. The typed correlates
  `08:37:04Z-c`, `11:05:55Z-c` and `13:15:16Z-b` make it 3 distinct incidents in 3 chunks. It is the
  nearest-to-threshold theme; the threshold counts facts only.
- **cascade.py refuses a sweep pattern whose control never fired** — 2 facts (`08:46:30Z-a#1`, `11:05:00Z-a#0`),
  plus the signal `control-refused` at `13:10:15Z-a`: 3 chunks if the signal counts, which the fact basis does
  not.
- **Smoke used a feature-enabled cargo build instead of the plan's `just seven_guis`** — 1 (`12:40:26Z-a#0`).
- **Plan premises vs dioxus-core reality, worked around in code** — 2 facts (`09:21:01Z-a#0`, `#1`) plus 2 in
  Epoch 1. Its nature is product-logic, which is outside the band-aid signature; it is routed to P3 and X1.

Override singleton:
- `08:46:30Z-a#0`: the playbook's "Not this chunk's drift" routing was overridden to fix-now for 53 stale
  citations (one occurrence on that rule; see P2).

Single-chunk chain shapes:
- wrap-session/report →report→ wrap-session/reconcile (accessibility-tree-identity)
- phase/research →research→ implement/fix-loop (id-persistence)
- phase/validate →matrix→ wrap-session/gates (id-persistence)
- phase/plan →plan→ wrap-session/gates (project-readme; see L6)
- phase/plan →plan→ implement/smoke (accessibility-tree-identity; see X1)
