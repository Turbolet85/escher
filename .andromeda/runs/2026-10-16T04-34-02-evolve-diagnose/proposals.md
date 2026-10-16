# Evolve Diagnosis — escher-0.1.0 · Epoch 6 — Polish & ship · 2026-10-16T04:34:02Z

The founder chose the epoch: audit with mutants first, then this diagnosis. Every proposal below is
obligation-free. Accept, reject, defer or modify any of them; the mechanism keeps nothing pending.

## Mechanism health
- **Records:** 197 (112 step / 85 friction), across 8 chunks plus 8 `new-session/orientation` records (chunk `null`).
  All 8 chunks are `complete` in master-route.
- **Coverage vs expected:** every chunk has phase 5/5 · implement 3/3 · wrap 5/5, so no checkpoint failed to fire.
  There are 8 orientation records. The ledger alone cannot measure the number of session starts.
- **Unparseable:** 0 · **malformed-ts:** 0 · **id fill:** 197/197.
- **Untyped rate per step:** 20/85 (24%) overall.
  - Above the overall rate: report 6/11 · plan 2/3 · gates 1/2.
  - At or below it: validate 3/11 · take-up 1/4 · research 1/4 · reconcile 4/19 · code 1/6 · fix-loop 1/5.
  - None untyped: curation 0/16 · route-resolve 0/3 · orientation 0/1.
- **Problem-fact fill:** 26/112 step records carry facts, 29 facts in all: 18 workaround · 7 overridden · 3 removed-cause · 1 deferred.
- **Signal line:** guard-refused 34 over 112 step records · exact-hit 12 over 8 curation records.
  Every Epoch 6 record dates from 2026-10-13 or later, so no record is era.
- **Halts:** 6 step records read `halted-resolved`: 5 wrap `reconcile` (the designed founder-card halts) and 1 phase `plan` (a dialog timeout).
  Only 2 friction records carry halt impact.
- **Retractions** (whole ledger):
  - Retracted 0 in this epoch.
  - Clause-retracted 2 ledger-wide, both outside this epoch (E1 `2026-10-06T02:22:58Z-b`, E5 `2026-10-09T21:21:27Z-b`).
  - 1 unresolvable, reported verbatim for manual discount: `2026-10-07T04:21:25Z-a` (Epoch 4) · `unknown-id-or-scope` · `{"id": null, "note": "in step record 2026-10-07T04:21:04Z-a (a step record, so n…`.
  - Retraction of a retraction: none.
- **Calibration boundaries:** every listed boundary predates or coincides with the epoch's first record (2026-10-13T14:25Z).
  - The latest two are `implement-outcome`'s operator-pass reading and orientation's widened `ladder-uncovered-state`, both from 2026-10-13.
  - The whole epoch is therefore read in the current era.
- **Stage-3 join, observed:** friction anchors carry file paths, while producers' `produced[]` carry logical names (`report`, `plan`).
  Of 31 anchors, the mechanical join matched only 7 to a producer. The chains below were joined by hand on the logical artifact (`q-chains-hand.json`).

## Proposals (typed patterns)

Sorted by n × summed weight. `recall.*` and the universal types are grouped by type across steps.

### P1 — all steps · `recall.corpus-recurrence` — 17 cases · weight 26
**Pattern:** a rule that the corpus already holds came back in 7 of 8 chunks. Curation's Filter 1 deduplicated it each time.
The recorded learning did not stop it recurring. By step: curation 14 · reconcile 2 · gates 1.
The 17 cases fall into six sub-themes:
- the read-whole rule for pipeline-tool output: 8
- sweep-pattern hazards (bare numbers matching citation digits; plan wording instead of master wording): 4
- `gh run view --log` colour codes: 2
- `$TMPDIR` unset: 1
- a host temp path in committed prose: 1
- ugrep bounded repetition: 1

**Evidence:** all 17 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-polish-and-ship | gate 50 counted `Running tests/…` over a `gh run view --log` capture, read 0 by construction — the Tier-3 colour-code entry of 2026-10-10 already records it | extra_reads 1 | chunks/…/evidence/ci.md |
| stand-contrast-harness (reconcile) | `inputs.py verify \| tail -2` against host-linux 2026-10-13 (bare or redirected); next call bare | retries 1 | runs/2026-10-13T18-00-36-wrap/inputs-….json |
| stand-contrast-harness | a path printed with a literal `$TMPDIR` read through the unset variable → `/andromeda-gate/…`; learning of 2026-10-07 | retries 1 | chunks/…/report.md |
| stand-contrast-harness | hygiene refused the implement outcome for a host temp path in prose; learning of 2026-10-06 | retries 1 | chunks/…/evidence/operator-15.txt |
| range-input-interaction (gates) | `route.py epoch` re-fired with output to /dev/null, against host-linux 2026-10-13; no decision rested on it | retries 0 | — |
| stand-keyboard-harness | `cascade.py sweep --pre-pass` output read through `command grep` at P1 despite host-linux 2026-10-13 | iterations 1 | runs/2026-10-13T23-23-45-wrap/curation.md |
| stand-a11y-assertions | bare-count sweep hazard (2026-10-12) recurred: `\b153\b` hit 25 rows, 23 citation digits | reformulations 1 | chunks/…/report.md |
| stand-a11y-assertions | obs-plan census wording hazard (2026-10-13) recurred: control did not fire, re-fired | reformulations 1 | chunks/…/report.md |
| stand-a11y-assertions | host-linux 2026-10-13 recurred at P2: leaves-apply output read through sed, then diffed whole | extra_reads 1 | runs/2026-10-14T19-50-50-wrap/cascade-dispositions.md |
| stand-requirement-sweep | the grep function's bounded-repetition refusal (2026-10-07) recurred: `.{0,60}` printed one hit then nothing | extra_reads 1 | chunks/…/report.md |
| stand-requirement-sweep | 'silence census' fired as a sweep pattern, control never fired (2026-10-13) | reformulations 1 | chunks/…/report.md |
| stand-requirement-sweep | host-linux 2026-10-13 recurred at P2: `cascade.py window \| tail`, leaves apply read through `grep -v` | extra_reads 1 | runs/2026-10-14T22-00-51-wrap/curation.md |
| quality-gates | host-linux 2026-10-13 recurred at implement, P1 and this window (`tail -25` over the leaf apply output) | extra_reads 1 | runs/2026-10-15T06-51-44-wrap/curation.md |
| quality-gates | bare-number count pattern matched citation digits (139, 716, 712 in 12 of 13 rows) | reformulations 1 | chunks/…/report.md |
| quality-gates | `gh run view --log` colour codes recurred at implement: a `Running` parser read 0 targets | reformulations 1 | chunks/…/report.md |
| cold-agent-test (reconcile) | a saved pipeline-tool capture read through grep twice this pass, against host-linux 2026-10-13 | extra_reads 0 | — |
| cold-agent-test | Filter 1 deduped the P1 `grep -v` over a saved sweep capture against host-linux 2026-10-13; logged recurrence-despite-learning | extra_reads 1 | runs/2026-10-15T09-44-57-wrap/curation.md |

**Proposal:** after a learning recurs despite being recorded, convert it from text into a check.
- **The read-whole rule (8 typed + 5 untyped, U2 + 6 problem facts, L1).** The bash guard (guard-refused 34) refuses a *pipe* from a pipeline tool. It does not see a second command that filters the saved capture file, which is the form every recurrence here took. Two shapes would remove that form:
  - each `andromeda-tools` script prints a bounded verdict and count summary and keeps its full output in the run dir, so nothing is left worth filtering;
  - or the guard also refuses `head`/`tail`/`grep`/`sed` over a known capture path.
- **Colour codes and `$TMPDIR` (3 cases).** The cause sits inside the tools: `gh` logs carry escape codes, and gate.py prints a literal `$TMPDIR`. A tool-side strip and an absolute path would remove both.
- **Curation trigger.** Curation could promote any entry seen as a `recall.corpus-recurrence` ≥ 2 times to a candidate guard or tool change, rather than a second text entry. The handoff already says "each remedy is a check, not a second entry"; this generalises that at pipeline level.

### P2 — phase/validate · `contract.mechanical-check` — 7 cases · weight 13 · rate 7/8
**Pattern:** P5's own reads found plan defects that P4 wrote and planlint did not list. Two of the seven are one mechanism: a baseline reading that holds an apostrophe, written as a TOML literal string.

**Evidence:** all 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| stand-contrast-harness | check 8: glyph-colour frame cited text.rs:25-48 (inline background); producer is text.rs:156-167 read at 616-638 | extra_reads 2 | — |
| stand-contrast-harness | check 4 (6): obs criterion named a manifest property no entry proved; probe entry added, both-way controlled | iterations 1 | — |
| stand-a11y-assertions | P3 accent sweep searched hex spellings only; one of three `rgba(74, 108, 247, …)` forms surfaced only at P5's census control | iterations 1, extra_reads 2 | runs/2026-10-14T06-38-55-phase/controls-p5.md |
| stand-requirement-sweep | check 4 (0): a baseline with an apostrophe written as a TOML literal with a doubled quote; re-written as a basic string | iterations 1 | runs/2026-10-14T20-46-15-phase/ |
| quality-gates | four REQUIRED-RESOLUTIONs, none listed by planlint: step 23 cited a step 25 that does not exist; evidence/hand controls named in no prose; a windowed run with no end; six NOT SEEN unmarked | iterations 1, extra_reads 2 | — |
| cold-agent-test | check 12: step 10 fired a reader whose input step 11 writes; check 4 (4): no prose naming evidence/live-verdict.json's producer | iterations 1 | — |
| cold-agent-test | baseline put `implement''s` inside a TOML literal; check 4 (0) dry-run read UNPARSED at fence line 13 | retries 1 | — |

**Proposal:** move three of the classes P5 found by hand into planlint or the baseline writer.
- **(a) Baseline quoting (2 cases).** The writer emits TOML basic strings, or parses its own output before it lands.
- **(b) Step references (2 cases).** A step number cited that does not exist, and a reader ordered before its producer. Both are mechanical.
- **(c) Unnamed artifacts (2 cases).** An evidence file or a hand control named in no prose.

The citation-frame and spelling-variant cases are reading work, which P5 is designed to do.

### P3 — wrap-session/reconcile · `contract.in-pass-correction` — 3 cases · weight 12 · rate 3/8
**Pattern:** texts that reconcile wrote were wrong on the first write and were corrected by re-reading them before the commit. The cases total nine corrections across three chunks. The failures were:
- counts
- heading names
- an unbalanced `)`
- a claim widened past its measurement
- a cited line off by five
- a placement claim

**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-polish-and-ship | five first writes wrong: answer-card counts 33/32 → 37/36; design-system §Depth Strategy → §Surface; arch entry put under Inherited Defaults → Established Decisions; a default.css change the body does not carry; … | retries 5 | runs/2026-10-13T15-50-37-wrap/ |
| range-input-interaction | arch :87 unbalanced ')'; arch :197 extended an unmeasured claim to apply_range_move with a dangling clause; answer-card cited plan.md:446-447 for I2/I3 at :451 | retries 3 | — |
| stand-requirement-sweep | arch sidecar entry 1 said C4, C5, C14 each placed a residuals line; C5 places none | retries 1 | runs/2026-10-14T22-00-51-wrap/architecture-entry-1.md |

**Proposal:** run a post-write lint before the commit over the amendments reconcile writes. Three checks would be mechanical:
- brackets balance
- a named § heading exists in its target
- a cited `file:line` holds the quoted text

That would catch the mechanical half of these nine corrections at write time, rather than leaving them to the author's re-read.

### P4 — implement/code · `input.plan-step-ambiguous` — 5 cases · weight 7 · rate 5/8
**Pattern:** a plan step that passed P5 asks for an outcome the code's actual state cannot give:
- no change flag
- a pin no shim can reach
- span fields the executor never records
- a test module that does not exist
- choices left open

Implement settled each case by judgment and surfaced it in the report. Four chunks are affected; the chain is X2.

**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| stand-a11y-assertions | step 6: Enter/Space on crud-update/-delete read as their click, but no change flag; reach types one character first | reformulations 1 | — |
| stand-requirement-sweep | step 7(1): a hit 'inside the box where scrolled-out content would lie' — such content lies outside the box; fixture designed to give the phrase a point | reformulations 1 | — |
| quality-gates | step 3's two shim pins for the coverage leg unreachable (no lcov, cwd-relative floor script) | extra_reads 1 | — |
| quality-gates | step 16 asks eight fields where execute.rs records six; step 17 names a unix test module client.rs lacks | extra_reads 2 | — |
| cold-agent-test | steps 5, 7 leave open which reasons accompany no-check/invalid check and whether stop ends 'the app started' | extra_reads 0 | — |

**Proposal:** add a plan-template or P5 check for steps that state a code fact. When a step names a field set, a module or a reachable pin, it would cite the line read for it, the way research's premise closure does for scope. When a step leaves an outcome open, it would name it as an open fork rather than leave it implicit.

### P5 — wrap-session/reconcile · `contract.cascade-miss` — 4 cases · weight 6 · rate 4/8
**Pattern:** master sites that copied an amended fact, and that no detector proposed. The step-2 sweep, main's count or main's owner sweep caught them.

**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-polish-and-ship | directive's respelling list covered security-plan and arch only; step-2 sweep found four more (obs-plan :107, a11y-plan :46, :275, session-lifecycle :7) | extra_reads 1 | runs/2026-10-13T15-50-37-wrap/cascade-sweep.txt |
| stand-keyboard-harness | four stale sites no detector proposed (arch :133, :263; a11y-plan :280; session-lifecycle key file) | iterations 1 | runs/2026-10-13T23-23-45-wrap/cascade-dispositions.md |
| stand-requirement-sweep | five sites naming the completing route entry as owner or carrying its dated counts (arch :91, :136; test-plan :92, :324; security-plan :397) | extra_reads 3 | runs/2026-10-14T22-00-51-wrap/fanout-results.md |
| quality-gates | session-lifecycle key file restated `cli_commands 11` at :11; step-2 sweep's cli11 pattern caught it | iterations 1 | runs/2026-10-15T06-51-44-wrap/cascade-dispositions.md |

**Proposal:** two cases are the same class: sites that name the completing route entry as an owner. A route-resolve or reconcile sweep keyed on the completing entry's own title, run on every wrap, would reach that class. The session-lifecycle key file appears in 3 of 4 cases, which suggests the detectors' file set may not include `registries/contracts/`. That is worth one read of the detector scope. See X1 for the shared cause.

### P6 — all steps · `contract.narrow-basis-claim` — 3 cases · weight 4
**Pattern:** a number or an absence claimed on a narrower basis than the claim: an uncomputed ratio, an unsearched log dir, a pattern blind to other wordings.

**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| stand-a11y-assertions (plan) | a contrast ratio (4.79) written into a dialog option without being computed; operator corrected to 4.78 | dialogue_rounds 0, extra_reads 1 | runs/2026-10-14T06-38-55-phase/relay-3.md |
| stand-a11y-assertions (report) | evidence/ci.md said entry 13's output was not kept; its whole log stood in the gate tool's log dir | extra_reads 1 | chunks/…/report.md |
| stand-requirement-sweep (report) | the log-counts pattern found the host's dated counts and missed the mcp role's, worded '9 lines built alone…' at three sites | reformulations 1 | runs/2026-10-14T22-00-51-wrap/ |

**Proposal:** each case has a different shape, so no single change covers all three.
- **Absence claims.** A "not kept" claim about a gate entry could name gate.py's log dir as a place it was searched. The tool knows that dir: see P1, where it is printed as `$TMPDIR`.
- **Ratios.** A ratio offered in a dialog option could come from the stand's contrast reader, not be typed.

### P7 — wrap-session/route-resolve · `contract.carry-no-owner` — 3 cases · weight 3 · rate 3/8
**Pattern:** an owed check or reading arrived at route-resolve with no named owner. The wrap placed each one by judgment, with no halt.

**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-polish-and-ship | falsy-`inert` 'not measured' had no owner; placed on Stand keyboard harness by judgment | extra_reads 1 | runs/2026-10-13T15-50-37-wrap/fanout-results.md |
| range-input-interaction | owed checks (clear_attribute re-seed, type-change clearing) placed on Stand requirement sweep by the 2026-10-10 precedent; the first markerless entry did not fit | extra_reads 2 | — |
| stand-keyboard-harness | two unmeasured obs-plan log readings placed as one CARRY on Stand requirement sweep, move-if-wrong | extra_reads 1 | runs/2026-10-13T23-23-45-wrap/fanout-results.md |

**Proposal:** the project absorbed this fix. The session learning of 2026-10-10 ("every open finding … gets a route owner without a halt") and the operator memory "owed checks go on the earliest owner" both encode it, and all three placements followed them.
What remains is a pipeline-level generalisation, if wanted: write the owner-selection rule into the route-resolve reference, so the placement stops being re-derived per wrap. The rule is the earliest entry whose subject reads the owed thing, with move-if-wrong.

### P8 — wrap-session/reconcile · `input.report-insufficient` — 3 cases · weight 3 · rate 3/8
**Pattern:** the report's site sweep or its Spec-claims list missed a site worded differently from its pattern, or placed a site at the wrong line. A reconcile detector or main caught it.

**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| stand-contrast-harness | site sweep reached its pattern wording only: obs-plan :70 '17 of which' missed; run-stand re-count put at test-plan :330 (it stands at :116); no blitz-tests file count | extra_reads 3 | runs/2026-10-13T18-00-36-wrap/fanout-results.md |
| range-input-interaction | Spec claims disproved listed test-plan :26 as true @c4626 and missed the stale `stand_act_range 1` clause on the same line (@c17032) | extra_reads 1 | runs/2026-10-13T21-11-52-wrap/citation-dispositions.md |
| stand-keyboard-harness | the silence probe read stand_keyboard.rs alone while obs-plan §3 claims the shared modules, which the chunk extended | extra_reads 1 | runs/2026-10-13T23-23-45-wrap/fanout-results.md |

**Proposal:** see X1. P5, P8, P6's third case and P1's four sweep-wording cases share one instrument: a site sweep by literal token patterns.

## Cross-step chains (starting heuristics)

### X1 — wrap/report →report (site sweep · Spec-claims list)→ wrap/reconcile — 3 chunks
- **The ends:**
  - Producers are formally ok: `2026-10-13T18:05:30Z-a` `[new-text-generated, new-text-rows:72, guard-refused:1]` and `2026-10-13T21:16:02Z-a` `[new-text-generated, new-text-rows:104]`.
  - Consumers read the report `thin` (`2026-10-13T18:28:22Z-a`, `2026-10-13T21:41:41Z-a`, route-resolve `2026-10-13T21:44:23Z-a`) and record `input.report-insufficient` in three chunks (P8).
  - Downstream of the same instrument: `contract.cascade-miss` ×4 (P5), the narrow-basis sweep miss `2026-10-14T22:08:02Z-b`, and four sweep-wording recalls (P1).
- **Chain hypothesis:** the report's site sweep matches the fact's *spelling*. A master site that states the same fact in other words, as a count in prose, or as a citation digit, reads either as no hit or as a false hit. Reconcile's detectors and main's sweeps then re-derive it, at 1–3 extra reads each time. The report emits no quality signal about its sweep's reach, so the producer reads `ok`.
- **Direction:** two options, which can be combined.
  - The report states the sweep's reach as a signal (patterns run · controls fired · sites per pattern), so a thin sweep is visible at its producer.
  - The sweep keys on the changed fact's anchor (a citation `file:line` or a registry key) as well as its wording.

### X2 — phase/plan (validate ok) →plan→ implement/code — 4 chunks
- **The ends:**
  - Producers: stand-a11y-assertions `2026-10-14T06:59:30Z-a` `[unresolved-questions]`; stand-requirement-sweep `2026-10-14T21:13:31Z-a` (527 lines, past the ~400 template, planlint 0); quality-gates `2026-10-15T05:12:40Z-a`, then validate `2026-10-15T05:20:15Z-b` (4 resolutions planlint did not list); cold-agent-test `2026-10-15T09:03:52Z-a`, then validate `2026-10-15T09:12:15Z-b` (2 P4 defects).
  - Consumers: the five `input.plan-step-ambiguous` records (P4), plus `consumed plan thin` at `2026-10-14T21:32:12Z-a`.
- **Chain hypothesis:** in two of the four chunks, P5 had already found plan defects of the same family (P2). The code-facing ambiguities that remained are those no P5 check reads: whether a step's demanded outcome is reachable from the code as it stands.
- **Direction:** P4's proposal. The two halves together suggest a per-step "code fact cited" check at P5.

### X3 — route authoring →working-entry→ phase/take-up — 2 chunks
- **The ends:** stand-contrast-harness (`2026-10-13T17:03:08Z-a/-b`: no control set, threshold, source or boundary) and cold-agent-test (`2026-10-15T08:05:28Z-a/-b`: no form, delivery or runner for the agent's check).
- **The producer** wrote no step record in this epoch; the entries were authored at route time.
- **Hypothesis:** the entries were thin where their capability was new in kind: a measurement with a threshold, an agent-written artifact.
- **Direction:** none evident from two cases. Watch whether route authoring for 0.2.0 shows the same shape.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid (with the override context) — 6 facts + 5 overrides
**Facts:** in each fact, the output of a pipeline tool (`andromeda-tools`) was read through a filter first, against the read-whole rule.
| record | chunk · step | nature · solution | note |
|---|---|---|---|
| `2026-10-13T17:03:08Z-a` | stand-contrast-harness · take-up | process · workaround | pins output redirected, then counted with `grep -c` instead of read whole |
| `2026-10-13T22:48:08Z-a` | stand-keyboard-harness · validate | process · workaround | five baseline outputs first read through a grep filter |
| `2026-10-13T23:28:17Z-a` | stand-keyboard-harness · report | process · workaround | pre-pass output read through `command grep` twice |
| `2026-10-14T20:21:07Z-a` | stand-a11y-assertions · reconcile | process · workaround | leaf apply output first read through sed |
| `2026-10-15T06:59:44Z-a` | quality-gates · report | process · workaround | pre-pass output first read through `head -5` |
| `2026-10-15T09:49:28Z-a` | cold-agent-test · report | process · removed-cause | `grep -v ' leaf'` dropped the per-pattern count lines |

Context: in the overrides `2026-10-13T14:30:13Z-a` … `14:59:57Z-a`, the operator directed at every phase step of upstream-sync that output be redirected to a file and read whole. The record says *"the skill's own form is a bare call read inline"*. Typed correlates: 8 recalls (P1) and 5 untyped (U2).
A keyword lookback matched 14 prior-epoch facts, which were not individually read.

**Level hypothesis:** the obstacle is how the pipeline tools' output is shaped. The output is long, and only part of it matters to the caller. The fix landed as a project rule (host-linux.md Session Additions 2026-10-13), as an operator directive, and as a guard on pipes. The recurrences come after the rule and route around the guard, by filtering the saved file.

**Proposal:** fix this in the tools, not the project rule: each tool prints a bounded, whole-readable summary (verdict · counts · run-dir path of the full capture). The skill references' "bare call read inline" form and the project rule would then agree. See P1.

### L2 — band-aid · chronic-degrade (`tooling.output-cap-overflow`) — 3 facts
**Facts:**
| record | chunk · step | nature · solution | note |
|---|---|---|---|
| `2026-10-14T06:13:21Z-a` | stand-keyboard-harness · reconcile (halted-resolved) | process · workaround | spec-body lines run 3–46 KB; 47 body edits through a scratchpad exact-once replace script |
| `2026-10-14T20:21:07Z-a` | stand-a11y-assertions · reconcile | process · workaround | master body edits by a scratchpad exact-substring script; clauses on single lines up to 37 KB |
| `2026-10-14T20:48:17Z-a` | stand-requirement-sweep · take-up | process · workaround | the 14 KB working line split at its freight introducers by python, not Read |

Correlates:
- `tooling.output-cap-overflow` `2026-10-14T23:05:31Z-b`: a grep over test-plan's multi-KB §1/§9 lines returned 82.4 KB.
- Lookback: `tooling.long-line-edit` ×3 in Epoch 5, plus 2 scratchpad-replace facts.
- `output-cap-overflow` across epochs is E1 6 · E2 2 · E3 1 · E5 1 · E6 1, never halting.

**Level hypothesis:** the master and route grammar puts one clause or one entry on one physical line, and the lines grow without bound. Each chunk writes its own exact-once replace script to edit them.

**Proposal:** two options.
- Ship one pipeline exact-once apply tool for master and route bodies, asserting each old text is unique, with a dry-run diff. The leaves apply may already be close to this shape.
- Or put a line-length bound or a continuation form into the master grammar.

### L3 — band-aid · chronic-degrade (`tooling.host-shell`) — 2 facts in-epoch, recurring from Epochs 2–3
**Facts:**
| record | chunk · step | nature · solution | note |
|---|---|---|---|
| `2026-10-14T22:08:02Z-a` | stand-requirement-sweep · report | environment · workaround | a bare grep with a bounded repetition ran through the Bash tool's ugrep function and printed nothing; re-read with `command grep` |
| `2026-10-15T08:17:08Z-a` | cold-agent-test · research | environment · workaround | `xargs command grep -l` cannot exec the builtin `command`; empty result, stderr discarded; re-ran with /usr/bin/grep |

The lookback finds 4 facts in Epochs 2–3 (`2026-10-06T08:37:04Z-a`, `10:57:29Z-a`, `20:10:37Z-a`, `21:59:01Z-a`). `tooling.host-shell` reads E2 1 · E3 1 · E4 1 · E6 2, never halting. Both E6 records are this theme.

**Level hypothesis:** the cause is in the harness: Claude Code's `grep` shell function. Each instance returns an empty result where there are matches, and the record reads it as an absence. The fixes landed in project and user text: a session learning, the global CLAUDE.md, and its `command grep` remedy. That remedy then created the second failure (`xargs command`).

**Proposal:** two directions.
- In the pipeline: the bash guard flags a bare `grep` carrying `{n,m}` and an `xargs command …`.
- In the harness: send the empty-on-refusal behaviour as Claude Code feedback.

### L4 — band-aid — 2 facts in-epoch, recurring from Epochs 4–5
**Facts:**
| record | chunk · step | nature · solution | note |
|---|---|---|---|
| `2026-10-14T06:46:59Z-a` | stand-a11y-assertions · distill | process · workaround | during the batch wait the orchestrator wrote a script to the scratchpad; the path check is blind outside the tree by construction |
| `2026-10-15T10:18:37Z-a` | cold-agent-test · reconcile (halted-resolved) | process · workaround | `route.py pins`/`cursor`, though read-only, appended a run-dir trail inside the snapshot pair; the path check printed an extra line, attributed by timestamp |

The lookback finds `2026-10-07T03:02:42Z-a` and `11:54:44Z-a` (E4) and `2026-10-10T00:55:21Z-a` (E5). Correlate: untyped `2026-10-15T10:18:37Z-b`.

**Level hypothesis:** the snapshot-pair contract says the orchestrator writes nothing between the snapshots. However, the waits are where the orchestrator works, and the pipeline's own read-only tools write trails. Each time, the conflict is absorbed by attributing it by hand.

**Proposal:** two options.
- The path check names and excludes the orchestrator's own tool trails (`route-*.json` in the run dir).
- Or the trail-writing tools take a flag that suppresses the trail inside a batch window.

**Other signatures:**
- **Deferred-forever:** 0 hits. The one deferred fact (`2026-10-14T19:58:01Z-a`, hygiene entry 12) was closed by gates `2026-10-14T20:44:28Z-a`. The `covered-deferral` dialogue went to residuals as designed.
- **Override:** 0 hits across chunks. See the appendix.
- **Removed-cause:** no recurring theme. The 3 facts have 3 causes, and one of them joins L1.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/plan + phase/validate — 5 cases → proposed type `contract.estimated-stamp`
**Cluster:**
- `2026-10-13T17:17:03Z-b` plan Generated stamp estimated, 3 min ahead.
- `2026-10-13T20:20:07Z-b` snap `--origin` guessed, 5 min ahead.
- `2026-10-13T22:48:08Z-b` snap `--origin` estimated, 3 min ahead.
- `2026-10-15T05:12:40Z-b` plan Generated stamp 7 min ahead.
- `2026-10-15T09:12:15Z-d` four stamps estimated, one landing ahead of the clock in the I5 manifest.

Each one was refused by the stamp-ahead hook or `inputs.py`, or landed wrong.
- **Lookback:** 5 more untyped cases (E2 report ×3, E2 validate, E4 plan).
- **Typed elsewhere:** E3 `ambiguity.filter-borderline`, where this learning scored exactly 0.6 at Filter 4, plus one `narrow-basis-claim` and one `in-pass-correction`.
- **Recurrence:** the theme shows up in 4 of 6 epochs (E2 · E3 · E4 · E6). E5's one stamp-hook record is a hook false positive, not an estimate.

**Draft criteria line:** `contract.estimated-stamp` — an authored time (a Generated stamp, a baseline, an `--origin`, a report Date) was written from an estimate instead of read from the clock. Record which tool or hook caught it, or that it landed. *(Universal-type candidate, since it occurs at plan, validate and report.)*
A tool-side alternative would remove the cause: `inputs.py snap` defaults `--origin` to now, and a stamp helper writes Generated.

### U2 — wrap/report · wrap/gates · phase/take-up — 5 cases → extend `recall.corpus-recurrence` to these playbooks
**Cluster:** the cases are `2026-10-13T15:55:05Z-c` (I2 broken twice: `gate.py show | head`, `gate.py hygiene | tail`), `2026-10-13T23:28:17Z-b`, `2026-10-14T06:36:45Z-b`, `2026-10-14T20:48:17Z-b` and `2026-10-15T09:49:28Z-b`. Each is a recurrence of the read-whole rule, recorded at a step where the writer left it untyped. With these, the theme stands at 13 friction records this epoch.

**Draft criteria line:** at report, gates and take-up, a standing corpus rule (rules, session learnings, curated tiers) that recurred in this step's own work is `recall.corpus-recurrence`, with the rule cited by file and date. Alternatively, move `recall.corpus-recurrence` into §Universal types.

### U3 — wrap/report · wrap/reconcile — 2 cases, recurring from Epoch 5 → proposed type `tooling.tool-argument-refusal`
**Cluster:**
- This epoch: `2026-10-13T18:05:30Z-b` and `2026-10-14T20:21:07Z-b`. In both, a `cascade.py sweep` pattern id was over the 16-character bound, and the tool exited 2 on first fire.
- Epoch 5: `2026-10-12T09:00:00Z-b` and `2026-10-12T20:41:49Z-c`. Repeated `--pattern` flags ran only the last pattern, with no refusal. A recall `2026-10-13T06:24:40Z-b` followed even after the rule was curated.
- These are two mechanisms on one tool's argument interface.

**Draft criteria line:** `tooling.tool-argument-refusal` — a pipeline tool refused, or silently narrowed, its arguments on first fire, for a bound or form its §Tool states. Record the bound and the re-fire cost.
A tool-side alternative: `action='extend'` for `--pattern`, and a refusal that lists every over-long id at once.

## Below threshold — no action

**Typed groups (n, weight):**
- phase/research `contract.instrument-validity` 2 · 5
- `tooling.host-shell` (all steps) 2 · 4 — the theme is L3
- wrap/reconcile `recall.corpus-recurrence` 2 · 3 and wrap/gates `recall.corpus-recurrence` 1 · 1 — both are in P1
- wrap/report `contract.narrow-basis-claim` 2 · 3 and phase/plan `contract.narrow-basis-claim` 1 · 1 — both are in P6
- `contract.premise-falsified` (all steps) 2 · 2: report 1, fix-loop 1
- phase/take-up `input.working-entry-thin` 2 · 2 — X3
- wrap/reconcile `ambiguity.escalation-rounds` 1 · 6 (halted)
- wrap/reconcile `ambiguity.playbook-no-match` 1 · 6 (halted)
- wrap/reconcile `contract.false-positive-proposal` 1 · 3
- phase/validate `ambiguity.review-cycles` 1 · 3
- implement/fix-loop `tooling.environmental` 1 · 2
- wrap/report `tooling.host-shell` 1 · 2
- phase/research `tooling.host-shell` 1 · 2
- implement/fix-loop `contract.test-expectation` 1 · 2
- new-session/orientation `input.handoff-git-mismatch` 1 · 1 — the post-wrap code-audit and evolve-diagnose run dirs were not in the handoff. This run dir will do the same.
- wrap/curation `recall.curated-rule-not-applied` 1 · 1
- implement/fix-loop `contract.premise-falsified` 1 · 1
- wrap/curation `ambiguity.filter-borderline` 1 · 1
- wrap/report `contract.token-proxy-check` 1 · 1
- phase/take-up `tooling.output-cap-overflow` 1 · 1 — in L2
- implement/fix-loop `tooling.result-not-run-stable` 1 · 1

The halt-bearing pair (escalation-rounds and playbook-no-match) are each n=1, so F-2's halt clause does not lift them.

**Untyped clusters (emerging, watch next epoch):**
- gate.py prints its log dir as a literal `$TMPDIR` while TMPDIR is unset: `2026-10-13T17:36:44Z-b`. Its kin are 2 recalls, one of them in E5.
- a long or background command outlived its window: `2026-10-13T18:28:22Z-d` (120 s timeout) and `2026-10-14T19:58:01Z-c` (fired before a context clear). These are two mechanisms.
- an orchestrator trail inside a snapshot pair: `2026-10-15T10:18:37Z-b`, which is in L4.
- the code graph reads a cfg-gated caller as no caller on the Linux-built index: `2026-10-13T22:22:28Z-b`.
- a relative redirect wrote a stray file into `.andromeda/runs/`: `2026-10-13T21:41:41Z-d`.
- an Edit anchor was recalled rather than read: `2026-10-14T07:11:49Z-c`, plus E5 `2026-10-09T20:07:28Z-b`.
- a sweep pattern was worded from the plan instead of the master: `2026-10-13T21:16:02Z-b`. Its kin are 2 recalls (P1).

**Problem-fact themes under threshold:**
- a founder dialog expired at 58 min and its forks went by relay: `2026-10-15T05:12:40Z-a`. In E5 there are 2 untyped records and 1 override.
- `state-dir-too-long` from the scratchpad: `2026-10-13T20:53:23Z-a`, plus E5. A host-linux rule already encodes it.
- a measurement with no listed producer was read by a throwaway probe or worked around: `2026-10-13T20:53:23Z-a`#2 and `2026-10-14T21:32:12Z-a` (product-logic: the client rect reads a transformed box at its layout position).
- a manual smoke on the boot path: `2026-10-13T23:06:20Z-a`.
- `cargo tree -i` matched nothing: `2026-10-14T23:19:09Z-a`.
- removed-cause: a bare `cd` moved the cwd and the guard was silent (`2026-10-13T17:08:40Z-a`); ci.md said a log was not kept (`2026-10-14T19:58:01Z-a`).
- a proposal was rejected for a count the report did not carry, then re-raised with a measured basis: `2026-10-13T18:28:22Z-a`.

**Overrides under the cross-chunk signature:**
- The read-whole directive was overridden 5 times in one chunk; this is in L1.
- The founder's verbatim Cyrillic label was overridden 2 times in one chunk, at stand-contrast-harness reconcile and route-resolve. The session learning of 2026-10-13 encodes it. The lookback has kin in E1, E4 and E5 (keyword match).
