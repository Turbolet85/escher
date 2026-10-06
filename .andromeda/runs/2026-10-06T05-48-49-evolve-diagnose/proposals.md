# Evolve Diagnosis — escher-0.1.0 · Epoch 1 — Foundation · 2026-10-06T05:48:49Z

Input: `.andromeda/friction-log.ndjson` (179 lines). Evidence twins in this run dir: `q-retractions.json` ·
`q-health.json` · `q-typed.json` · `q-chains.json` · `q-level.json`. Line numbers (`L{n}`) are ledger lines.
Every proposal below is obligation-free: accept · reject · defer · modify — nothing here is applied or remembered.

## Mechanism health
- **Records:** 179 (99 step / 80 friction), all in Epoch 1 — Foundation, ts 2026-10-05T18:43:33Z → 2026-10-06T04:52:12Z.
- **Coverage:** 7/7 chunks carry all 13 expected checkpoints (phase 5 · implement 3 · wrap 5) — 91/91, 0 gaps, 0
  duplicates. Chunk-null: 7 `new-session/orientation` (one per session start) + 1 `wrap-session/curation` (L3, a
  no-op wrap) — both legitimate.
- **Unparseable:** 0 · **malformed-ts:** 0 · **id fill:** 179/179.
- **Retractions:** 0 record-scope · 1 clause-scope (L124 → L118, note: "discount the clause 'focus excluded' …
  focusability parses the value as a bool (element.rs:629); :disabled matching stands") · 0 unresolvable · 0
  retraction-of-retraction.
- **Untyped:** 9/80 (11 %) — implement/fix-loop 3/9 · phase/validate 3/13 · phase/take-up 1/3 · phase/distill 1/7 ·
  wrap-session/report 1/2. Two of the nine had an existing type home: L156 (a cd-guard block → `tooling.hook-friction`)
  and L23 (a reference omitting a required flag → `contract.skill-reference-drift`).
- **Problem-fact fill:** 35/99 step records carry facts (42 facts: 35 workaround · 3 overridden · 2 deferred ·
  2 removed-cause).
- **Calibration boundaries:** the whole range post-dates every boundary in `diagnosis-pass.md` (id/retracts
  2026-08-18 · universal types · `contract.in-pass-correction` 2026-09-27) — no era discount applies.
- **Skill-field fold:** applied (bare skill names); no split groups resulted.

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 7 cases · weight 22
**Pattern:** every reconcile in the epoch (7/7) corrected its own first writes before commit; 4 of 7 are wrong
`file:line` citations into sources, the rest counts, attributions and formatting.
**Evidence:** ALL 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-05-fork-ci-reached | 5 first-write errors: arch ci-leg.sh:46-51→47-52, test_ci_workflows.py:13→12, :61-176→61-177 (wc -l); security sidecar written into the run dir under the wrong section; a Supersedes over a still-standing claim | retries 5 | — (L42) |
| 2026-10-05-ci-gate-legs | sweep regex `[^.]` could not cross 'ci.yml'; sidecar payload claimed a CARRY not yet in any body; trailing blank → double gap | retries 2 | runs/2026-10-05T23-50-17-wrap/cascade-dispositions.md (L67) |
| 2026-10-06-telemetry-bootstrap | 5 new citation ranges into escher-telemetry sources wrong on first write (lib.rs:90-133→90-134, panic.rs:4-21→4-24, format.rs:96-160→96-165, :17-85→17-84, lib.rs:32-55→32-54) | retries 3 | — (L97) |
| 2026-10-06-stand-test-contract | test-plan §4 split the 37 CI-script tests 23 + 14; the prior 23 covered every file — reworded | retries 1 | — (L145) |
| 2026-10-06-stand-test-contract | security-plan row cited agent-run.sh:160-161; grep put it at :161 alone | retries 1 | — (L146) |
| 2026-10-06-stand-test-contract | sidecar Why named the founder as the ruling's author; founder vs overseer delegate unknown — reworded | retries 1 | — (L147) |
| 2026-10-06-cold-agent-run-pipe | arch Names cited cold-agent.sh:14 for the MCP server name (it is :181); corrected count basis said 21 rows (it is 18) | retries 2 | runs/2026-10-06T04-34-08-wrap/cascade-dispositions.md (L172) |

**Proposal:** the dominant sub-class is a hand-typed citation range that a read-back then fixes. A direction: a
citation check in the reconcile toolset — for every `file:line[-line]` in a payload about to land, print the cited
lines (and the next/previous line at range edges) so the write is made against the reading, not before it. The same
subject appears below threshold elsewhere — stale citations after line shifts found only by the orchestrator (L39,
L98, `contract.structural-blind-spot` n=2), re-pointed by ad-hoc scratch scripts (problem facts L38:0, L95:0),
and re-derived without recall (L102) — so one tool that both re-points shifted citations and verifies new ones would
reach all of them. The founder may equally judge the in-pass read-back to be the mechanism working at acceptable cost.

### P2 — phase/validate · `contract.mechanical-check` — 10 cases · weight 21
**Pattern:** P5's mechanical checks raised a REQUIRED-RESOLUTION on the P4 plan in 6 of 7 chunks (rate 10/7); the
recurring classes are 4(9) vacuous / never-green probes (4), 4(4) evidence with no producing entry (3), and 4(2) a
boot path with no smoke (2) — L137 and L162 state that P4's self-check (dry-run + planlint) passed what P5 caught.
**Evidence:** ALL 10 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-05-as-built-baseline | check 4(4): criteria asserting evidence/baseline.md, an implement-authored record with no producing entry; resolved by a by-construction prose line | iterations 1 | — (L10) |
| 2026-10-05-fork-ci-reached | check 8 (mechanism reach): a lacks-font-skip atom on the full cargo test leg is vacuous (libtest captures a passing test's eprintln); moved to the --nocapture entry | iterations 1 | — (L30) |
| 2026-10-05-fork-ci-reached | check 4(9) baseline of the debuginfo probe read 746 MB for a 402 MB file (size -A Total summed); denominator switched to stat | retries 2 | runs/2026-10-05T20-51-19-phase/baseline-e8.sh (L31) |
| 2026-10-06-telemetry-bootstrap | check 4(2): P4 listed seven_guis main.rs (a boot path) with no smoke entry; resolved with a gated 10 s boot smoke + build entry | iterations 1 | — (L84) |
| 2026-10-06-telemetry-bootstrap | check 4(6): a Cargo.lock no-new-crate criterion with no entry proving it; probe added; plus a doubled-quote TOML literal written then fixed | iterations 1 | — (L85) |
| 2026-10-06-headless-stand | check 4(2): app.rs/lib.rs/timer.rs change what seven_guis_native launches; P4 wrote no smoke; added and baselined | iterations 1 | escher-0.1.0/chunks/2026-10-06-headless-stand/plan.md (L113) |
| 2026-10-06-headless-stand | check 4(9) known-positive control: `cargo tree -i blitz-dom \| grep woff` could never go green; replaced with `cargo tree -i wuff` | iterations 1 | runs/2026-10-06T01-54-25-phase/ (L114) |
| 2026-10-06-stand-test-contract | 4 REQUIRED-RESOLUTIONs the P4 self-check (0 hits) passed: Expected-amendments naming a rule leaf; 4(4) events.jsonl producer without artifact; 4(9) entry 12 green-vacuous on an absent script; 4(9) entry 16 unmarked new | iterations 1 | — (L137) |
| 2026-10-06-cold-agent-run-pipe | check 4(9): the P4 evidence-secrets census (grep -c over cat of absent files) read green on the untouched tree; P4's self-check does not baseline | iterations 1 | — (L162) |
| 2026-10-06-cold-agent-run-pipe | check 4(4): a criterion asserts committed evidence no gate entry produces; producer stated in Test Commands prose | iterations 1 | — (L163) |

**Proposal:** a shift-left direction — teach P4's authoring self-check (planlint or the dry-run) the three
recurring classes: (a) a criterion naming an evidence file with no producing entry; (b) a touched boot path
(`main.rs`, launch modules) with no smoke entry; (c) a new census/grep probe whose subject is absent at plan time
(reads green on a missing file — the `grep -c … || cat missing` shape in L137 and L162). Every case was caught
before implement at one iteration each, so the founder may also read this as P5 doing its designed job; the
proposal's value is the per-chunk round-trip it would remove.

### P3 — */`recall.corpus-recurrence` (cross-step) — 5 cases · weight 12
**Pattern:** a curated learning recurred anyway 5 times across curation and reconcile; 4 of 5 are the project's
Bash guard (leading `cd` / heredoc-to-file), stored at Tier 3 and in the handoff's deferred learnings — neither is
in context at the acting moment.
**Evidence:** ALL 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-06-telemetry-bootstrap | the 2026-10-06 Tier-3 entry on the Bash guards (leading cd refused) was already curated, yet two calls began with cd and were blocked; Tier 3 is not auto-loaded | retries 2 | — (L101) |
| 2026-10-06-telemetry-bootstrap | the 2026-10-05 Tier-3 entry on re-pointing citations after line shifts was already curated; this wrap derived the same procedure independently | extra_reads 0 | — (L102) |
| 2026-10-06-headless-stand | the Tier-3 learning on heredoc-to-file and leading cd recurred twice: a cat heredoc into a probe test file at implement, a leading cd at this wrap | retries 2 | — (L125) |
| 2026-10-06-stand-test-contract | session-learnings.md records the leading-cd refusal; implement and wrap issued two more leading-cd calls, both blocked | retries 2 | — (L149) |
| 2026-10-06-cold-agent-run-pipe | a cat heredoc with a file target was blocked, though the handoff's deferred learning states the guard refuses it | retries 1 | this session, the fanout-results append (L174) |

**Proposal:** the learning is correct and recorded; it does not reach the moment of action. Two directions for
the founder: (a) move the guard rule to an always-loaded home (CLAUDE.md Tier 1 / `.claude/rules/` always-loaded
file) — a project-level change; or (b) make the guard's refusal text carry the recipe (absolute paths, `( cd … )`
subshell, the Write tool for file payloads) so recall is unnecessary — the deeper fix, since it also covers sessions
that never read the learning. See L1 for the full workaround pile under this guard.

### P4 — wrap-session/reconcile · `input.report-insufficient` — 4 cases · weight 7
**Pattern:** in 3 of 7 chunks the wrap report carried a claim the reconcile detectors then contradicted — two
from grep output read without opening the hits, one an unmeasured mechanism carried from implement, one an internal
bullet inconsistency.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-05-fork-ci-reached | Dependencies bullet read 'none added' while Schema/config named the PyYAML install; the arch detector proposed from Schema/config; corrected at Validate | retries 1 | — (L40) |
| 2026-10-06-headless-stand | the report (same agent as implement) asserted an unmeasured mechanism — focusability presence-keyed — from implement's probe write-up; two detectors read the source and contradicted it | extra_reads 5 | runs/2026-10-06T02-44-36-wrap/fanout-results.md (L123) |
| 2026-10-06-cold-agent-run-pipe | report claimed no master states the CI-scripts 37 count; the grep returned test-plan.md:127 but rows were viewed through `cut -c1-220` and the hit (offset 1554) dismissed unread | retries 1 · extra_reads 2 | runs/2026-10-06T04-34-08-wrap/fanout-results.md (L169) |
| 2026-10-06-cold-agent-run-pipe | report counted an a11y-plan agent-run site from grep -c; the one hit was 'agent-runnable invariants', not the contract | retries 1 | runs/2026-10-06T04-34-08-wrap/fanout-results.md (L170) |

**Proposal:** a report-authoring self-check: a count/absence claim derived from grep cites the hits it read in
full (never a truncated or `-c` view); a mechanism claim cites the measurement that showed it; the Dependencies
and Schema/config bullets are cross-read. The detectors caught all four, so the cost is retries — see X1 for the
chain view (the producer records no signal in any of these chunks).

### P5 — implement/code · `input.plan-step-ambiguous` — 4 cases · weight 6
**Pattern:** in 4 of 7 chunks implement settled a plan gap by itself — a step order that forecloses a required
red reading, a dependency the manifest list omits (×2), and edge semantics left open.
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-05-ci-gate-legs | step 6 requires the extended unittest to run red against the untouched ci-leg.sh, but step 4 (earlier) edits ci-leg.sh; executed 6 before 4 | reformulations 1 | — (L57) |
| 2026-10-06-telemetry-bootstrap | step 7 lists dev-deps escher-telemetry + tracing only; the scrub test needs a log emitter the workspace lacks; added tracing-log | reformulations 1 | — (L88) |
| 2026-10-06-headless-stand | step 4 asks for a public ColorScheme::Light constant while step 5's manifest list gives seven_guis no crate exporting ColorScheme | extra_reads 2 | — (L116) |
| 2026-10-06-stand-test-contract | step 2 left three points open (run.start files for `all`, run stand with zero stand files, usage-vs-not-booted order); settled in impl and reported | dialogue_rounds 0 | — (L140) |

**Proposal:** two of the shapes are mechanically checkable at P4: (a) a step demanding a red reading "against the
untouched X" ordered after a step that edits X (also the problem facts L33:0 and L56:0, where implement reordered
steps for the same reason); (b) a step naming a symbol whose crate is not in that step's manifest list (also facts
L86:1, L115:0). Note: the consuming `implement/code` step graded `plan` as **ok** in all 4 chunks while recording an
`input.*` friction against it (X3) — the consumed-quality verdict and the friction disagree.

### P6 — */`tooling.output-cap-overflow` (cross-step) — 6 cases · weight 6
**Pattern:** a single `cat` of all seven distill extracts (30–34 KB) overflowed the tool-result cap in 4 of 7
distills, and a bundled `cat` of two skill references (31.5 KB) did so in 2 of 7 orientations — deterministic, never
halting.
**Evidence:** ALL 6 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| (orientation) | one cat of evolve-system.md + health-criteria.md (31.5 KB) overflowed; recovered via heading grep + offset Read | extra_reads 2 | — (L2) |
| 2026-10-05-as-built-baseline | one cat of all seven extracts (30.3 KB) overflowed; one Read of the persisted output | extra_reads 1 | — (L6) |
| 2026-10-05-fork-ci-reached | one cat of all 7 extracts (30.1 KB) exceeded the cap; one Read of the persisted file | extra_reads 1 | — (L25) |
| (orientation) | a bundled cat of two skill references (31.5 KB) exceeded the cap; persisted file read whole | extra_reads 1 | — (L75) |
| 2026-10-06-telemetry-bootstrap | a single cat of all seven extracts (30 KB) exceeded the cap; one Read of the persisted file | extra_reads 1 | — (L80) |
| 2026-10-06-cold-agent-run-pipe | one cat of all 7 extracts returned 33.8 KB; recovered by reading each extract with Read | extra_reads 7 | — (L157) |

**Proposal:** the extract set's size (≈30 KB) sits just over the cap in every chunk, so the overflow is
structural, not careless. Directions: the distill letter reads extracts per file (or a tool prints a sub-cap digest
of them); the orientation letter already says references load per file (problem fact L74:0 names the deviation) —
that rule could be enforced by the reference layout (one file per load) rather than relied on. See L5 for the level view.

### P7 — */`contract.narrow-basis-claim` (cross-step) — 4 cases · weight 4
**Pattern:** a count/absence claim made from a narrower source than the claim, caught by re-derivation at four
different steps (research, plan, report, reconcile).
**Evidence:** ALL 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-05-fork-ci-reached | the overseer relay's uncached-job list named 5 jobs; re-derivation over ci.yml found 7 and that main-only save-if leaves even the cached job cold on build/** | extra_reads 0 | escher-0.1.0/chunks/2026-10-05-fork-ci-reached/research.md (L27) |
| 2026-10-05-ci-gate-legs | the audit-tool question stated 1 vuln + 3 warnings from cargo-audit's lockfile scan; cargo-deny's graph fired on 1; the operator answered on the wider framing | extra_reads 3 | research.md §Measurements (L53) |
| 2026-10-06-telemetry-bootstrap | expected-amendment site grep over the 7 masters read 0 hits for two labels that live in .andromeda/registries — re-derived over the registries | extra_reads 1 | — (L94) |
| 2026-10-06-headless-stand | implement's fix-loop friction claimed focus exclusion for disabled=false from a probe that never measured focus | extra_reads 5 | runs/2026-10-06T02-44-36-wrap/fanout-results.md (L124) |

**Proposal:** one sub-shape repeats outside this type: the report's expected-amendment site search covers the
seven masters but not `.andromeda/registries` (L94 here; the L64 report note says the same — "two bootstrap keys
live in registries/contracts, not the masters"). A direction: the report letter's site-search scope names the
registries. The other three are basis-naming discipline (a claim states the source it was read from); low weight,
and all four were caught.

## Cross-step chains (starting heuristics)

### X1 — wrap-session/report →report→ wrap-session/reconcile — 3 chunks
fork-ci-reached (producer L36 · consumer L38 `thin` · frictions L40, L41, L42), headless-stand (L121 · L122
`wrong` · L123, L124), cold-agent-run-pipe (L167 · L168 `thin` · L169, L170). The producer is formally `ok` with
**empty** `signals` in all three — the chain is invisible from the producer's side. Hypothesis: the report step has
no self-check that would turn its own grep-derived or carried claims into a signal; the reconcile detectors are
where they surface. Direction: as P4; additionally a report-step signal (e.g. `claims-from-grep-unread`) would make
the producer end visible to the next diagnosis.

### X2 — phase/validate →plan→ implement/fix-loop — 2 chunks
as-built-baseline (producer L9, signal `baseline-exposed-vacuous-entry` · consumer L12 `thin` · L13 credential probe
matching its own verbatim quote in baseline.md, L14), telemetry-bootstrap (L83, `baseline-controls-run` · L89
`wrong`, outcome ok-degraded · L90 gate.py maps exit 124 to its own timeout, so an `exit 124` atom is unreadable
green). Near-shape: ci-gate-legs phase/plan →plan→ fix-loop (L52 → L59 `thin` · L60, L61, L62). Hypothesis: P5
baselines run against the untouched tree, so atoms whose failure needs implement's own outputs (a record quoting the
probe; a timeout-exit collision with the runner) pass validation. Direction: a planlint/dry-run rule for the two
known collisions (an atom expecting exit 124/137; a probe pattern that the plan also orders written into evidence).

### X3 — phase/plan|validate →plan→ implement/code — 4 chunks
ci-gate-legs (L52 → L57), headless-stand (L111 → L116), telemetry-bootstrap (L83 → L88), stand-test-contract
(L136 → L140). All four consumers graded `plan` **ok** in `consumed[]` while recording `input.plan-step-ambiguous`.
Hypothesis: the consumed-quality verdict is set before the gap is met (or not revised after) — a capture-side
inconsistency. Direction: see P5; for the mechanism, the founder may consider whether an `input.*` friction should
imply a non-ok verdict on the named artifact.

### Weak / unjoined (observations)
- phase/plan →plan→ wrap-session/report: 2 chunks (fork-ci L28 → L36; ci-gate-legs L52 → L64), plus telemetry via
  validate (L83 → L93) and gates (L83 → L104) — consumers grade `plan` thin with little downstream friction (L37,
  L94). No hypothesis offered.
- `working-entry` thin at take-up in 4/7 chunks (L4, L77, L130, L153), with `input.working-entry-thin` at L131 and
  L154: the producer (route authoring) is outside the chunk, so the heuristic's in-chunk join cannot reach it.
  Recorded for the founder's eye; n=2 typed (appendix).

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 11 facts (project Bash guard: leading `cd` / heredoc-to-file)
**Facts:** L1:0 orientation · env · workaround (leading cd into skill references → absolute paths) · L5:0
as-built/distill · env (cd into run dir) · L16:0 as-built/report · env (cd into .andromeda → subshell) · L24:0
fork-ci/distill · process (cd into run dir) · L49:0 ci-gate-legs/distill · env (an earlier bare cd moved the
session cwd; next leading cd blocked → subshell) · L64:0 ci-gate-legs/report · env (→ scratch python) · L78:0
telemetry/distill · env (bare cd moved cwd; → subshell) · L81:0 telemetry/research · env (cwd still in run dir) ·
L117:1 headless/fix-loop · process (cat heredoc into a probe file → Write tool) · L155:0 cold-agent/distill ·
process (cd → absolute paths) · L168:0 cold-agent/reconcile · env (cat-heredoc append → Write tool).
Typed correlates: `tooling.hook-friction` L58, L87 · `recall.corpus-recurrence` L101, L125, L149, L174 (P3) ·
untyped L156. All 7 chunks plus orientation.
**Level hypothesis:** the obstacle is a project-installed PreToolUse guard; each session absorbs it one call at a
time, and the curated learning (P3) has not stopped it. Three facts (L49, L78, L81) show an interaction: a bare
`cd` that *was* allowed moves the persistent cwd, after which the next leading `cd` is refused.
**Proposal:** a direction at the guard rather than the sessions — its refusal message names the accepted forms;
or it accepts the `( cd … && … )` subshell form the sessions already fall back to; or the skills' letters state
"absolute paths, no leading cd, Write tool for file payloads" once, where every step reads it.

### L2 — band-aid — 6 facts (shell/python writes where the letter names Write/Edit)
**Facts:** L5:1 as-built/distill · process (six history files via one printf redirect) · L48:0 ci-gate-legs/take-up
· process (one-line master-record.txt via printf) · L77:0 telemetry/take-up · process (same file, same way) ·
L82:1 telemetry/plan · process (TOML literal + census regex fixed by a python heredoc replace on plan.md) ·
L103:0 telemetry/route-resolve · process (CARRY appended by a python line-indexed rewrite with an exact-prior-text
assert) · L122:0 headless/reconcile · process (report/evidence corrections by python heredoc replaces).
**Level hypothesis:** the same payload shapes recur — a one-line file written inside a promote call, a
line-indexed route append, a multi-site replace — and each time the agent judges the Write/Edit form costlier and
routes around it. The convention may be miscalibrated for those shapes, or the tools that own those writes
(promote, route-resolve) could write them themselves. Possible interplay with L1: the guard refuses `cat` heredocs
to files but not python heredocs that write files.
**Proposal:** the founder chooses between (a) the owning tools write these payloads (promote writes
master-record.txt; a route-append verb), (b) the letter sanctions the shell form for named shapes, (c) keep the
rule and the deviation record.

### L3 — band-aid — 3 facts (committed evidence carrying host paths, masked by hand)
**Facts:** L128:0 headless/gates · process · removed-cause (hygiene refused evidence/operator-16-ci.txt for a
home-directory path; operator-14 carried the same, committed earlier) · L144:0 stand-test/reconcile · process ·
workaround (a raw fan-out twin's absolute basis path made repo-relative so hygiene would not refuse it) · L165:0
cold-agent/fix-loop · process · workaround (live transcript copy masks the session tmp dir and home prefix).
Composition: 2 workaround + 1 removed-cause, 3 chunks.
**Level hypothesis:** the evidence producers (operator-pass run lines, raw fan-out twins, transcript copies) emit
absolute host paths and `gate.py hygiene` refuses them at the end; each chunk masks by hand, and one already-
committed file slipped past (L128).
**Proposal:** mask at capture — a shared helper the evidence writers call (home → `~/`, session tmp →
`/tmp/<session-dir>`) so hygiene becomes a backstop rather than the place the fix happens.

### L4 — override — 2 facts on one rule (a boundary widening needs the founder's own word) · halt impact
**Facts:** L65:0 ci-gate-legs/reconcile · process · overridden (E1 widening applied on a delegate overseer
ratification, recorded PROVISIONAL — "the contract ratifies a widening only on the founder's own word, who was
unavailable"; step outcome halted-resolved, friction L68 halted 1) · L82:0 telemetry/plan · process · overridden
(OTel export: the overseer delegate chose Defer over the recommended option, naming the boundary-widening
escalation to the founder).
Correlates on the same rule: L32 (untyped — P4 recorded fork answers as delegate-provisional although the note
named the founder's words; corrected at review) · L119 (scope-pressure soft-exit answered by the delegate) ·
L147 (ruling's author founder-or-delegate unknown) · L117:0 (engine fix as a widening on the delegate's word) ·
plan signals `delegate-answer-provisional` (L28) and "answered by the overseer under the founder's delegation,
PROVISIONAL" (L159).
**Level hypothesis:** the rule assumes the founder is reachable at the widening; the project runs under a standing
delegation, so each widening is applied PROVISIONAL and the founder-word debt accumulates — the operator corrects
the same call in each chunk. The rule, not the project, may be what is miscalibrated.
**Proposal:** a founder decision on the rule itself — e.g. recognise delegated ratification with a batched founder
review at epoch boundaries, or keep the founder-only rule and accept the PROVISIONAL queue. Not a project change.

### L5 — chronic-degrade — 6 frictions + 3 facts (tool-result volume)
**Facts:** `tooling.output-cap-overflow` L2, L6, L25, L75, L80, L157 (P6) — persistent within the epoch, never
halting. Band-aid facet (theme T7, 3 process workarounds): L18:0 as-built/curation (health.py output grepped for
one row instead of run bare) · L108:0 headless/distill (sidecar.py summary piped through `head -3` though the letter
says every tool call runs bare) · L74:0 orientation (two references loaded in one cat although the letter says
per-file; it overflowed — L75).
**Level hypothesis:** the obstacle is output volume vs the cap; the letters' reading rules (run bare, load per
file) are being traded off against it in both directions — bundled reads overflow, filtered reads break "run bare".
**Proposal:** as P6 — size the reads in the pipeline (per-file extract reads, tools that print a bounded verdict
block by default) so neither trade-off is needed.

**Signatures with no hit:** deferred-forever — 0: the three deferrals are all routed (L56:1 cargo-deny reach →
CARRY on Quality gates, working-route.md:72; L122:1 stale doc comment → PREREQ on Stand test contract, discharged
per L150; L179 doc leg → PREREQ on Stable element ids, working-route.md:26). Recurring removed-cause theme — none.

## Playbook-extension candidates (untyped patterns, F-4)
None — no untyped cluster reaches n ≥ 3, and no prior epoch exists for the recurrence route. Emerging clusters are
in the appendix.

## Below threshold — no action
**Typed groups (n < 3, no halt/soft-exit pair):**
- implement/code `tooling.hook-friction` n=2 (L58, L87) — the L1 guard; counted there.
- */`contract.token-proxy-check` n=2 (L13 fix-loop, L173 reconcile) — a probe matching its own quoted pattern; a sweep pattern matching inside a marker.
- */`contract.structural-blind-spot` n=2 (L39, L98) — masters' citations into edited files go stale; no detector reads line numbers (see P1).
- wrap-session/reconcile `contract.false-positive-proposal` n=2 (L66, L96) + `contract.proposal-format` n=1 (L171) — detectors citing line numbers the report does not carry ("re-derivation tell"); 3 combined across two types, a typing split worth the founder's eye.
- phase/distill `contract.extract-format` n=2 (L79, L109) — arch-history bold span carries marker + title; same agent, same shape, no check fails.
- phase/take-up `input.working-entry-thin` n=2 (L131, L154) — see the unjoined chain anchor.
- phase/plan `retry.synthesis-rework` n=2 (L135, L160).
- wrap-session/curation `ambiguity.filter-borderline` n=2 (L70, L176) — both candidates scored exactly 0.6 at the filter edge.
- `contract.instrument-validity` n=1 each at research (L51), fix-loop (L90), gates (L105) — L90 and L105 are the same gate.py exit-124 cause in one chunk (with fact L89:0 and the chunk's two ok-degraded outcomes, L89 and L104).
- n=1: `ambiguity.escalation-rounds` L68 (halted 1, weight 6 — part of L4) · `contract.spec-reality-gap` L118 [clause retracted: discount the clause 'focus excluded' in '(:disabled matches, focus excluded)' — focusability parses the value as a bool (element.rs:629); :disabled matching stands] · `ambiguity.scope-pressure` L119 · `contract.vacuous-check-found` L61 · `retry.fix-iterations` L60 · `ambiguity.tier-routing` L44 · `tooling.health-false-red` L76 · `tooling.gate-deferral` L179 · `tooling.result-not-run-stable` L73 · `ambiguity.playbook-no-match` L99 · `contract.cascade-miss` L41.

**Untyped clusters (emerging — watch next epoch):**
- Scripted TOML literal-string writes break on an apostrophe in plan baselines: L55 (validate), L138 (validate) — n=2; typed-adjacent L85 (clause) and fact L82:1 show the same shape.
- Hooks: L91 (PostToolUse rustfmt could not resolve a module written before its mod file) · L156 (cd guard — fits `tooling.hook-friction`).
- Singletons: L14 (backgrounded gate.py block-buffers off a tty; a Monitor saw 0 events) · L23 (inputs.py `--origin` required but unnamed in promotion.md — fits `contract.skill-reference-drift`) · L32 (authority mis-recorded; see L4) · L37 (operator-pass evidence mirrored the plan's forecast, not the listing) · L62 (plan prose numbers entries 1–18, the fence holds 19).

**Note themes under threshold (Pass A):**
- T4 citation re-pointing by ad-hoc script — n=2 (L38:0, L95:0); strong typed correlates (L39, L98, L102) — see P1.
- T5 plan steps reordered for a red-before-green reading — n=2 (L33:0, L56:0); correlate L57 (P5).
- T6 gate.py runner behaviour routed around — n=2 (L12:1 stdout buffering; L89:0 exit 124 = own timeout); correlates L14, L90, L105.
- T8 plan omission filled at implement — n=2 (L86:1, L115:0), nature product-logic; correlates L88, L116 (P5).
- Singletons: L9:0 (Bash redirects + rm -r denied by permission → Write-tool files) · L12:0 (verbatim gate text matched its own credential pattern) · L50:0 (cargo tree host-only resolution → Cargo.lock walk) · L83:0 (approval word's standing instruction recorded in the run dir) · L86:0 (tracing macro field-name form) · L117:0 removed-cause (falsy `disabled` engine fix).
- Override singleton: L158:0 (a second live probe denied at the permission prompt; neutral-cwd equality left unmeasured).
