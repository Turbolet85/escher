# Founder forks — 2026-11-20-readme-and-records (README and records, v040-03)

Written at phase P4 on the operator's directive (inputs#I1): the founder is away, so his forks are written here, not
asked by dialog, and the plan takes the option that changes least until he answers. His answer is appended below
each fork, snapped and cited where it is applied.

## F1 — The form in which the README states 0.3.0 closed

Today `README.md` line 87 reads `### 0.3.0 — built, its cold-agent run passed` (the overseer's interim heading), beside
`### 0.1.0 — done and verified` and `### 0.2.0 — done and verified`. The 0.3.0 paragraph already states the six
stages, «All 17 of its capabilities are verified» and the cold-agent run (seven apps, 76 calls, 0 wrong, under the
5 % bar). Unlike 0.1.0's and 0.2.0's, it states no version-end audit reading. That audit is recorded (tx 2303, at
`bc7661240`). Its mutation testing over the epoch's diff:
- escher-driver: 88 caught of 89 (1 unviable; the unit exited incomplete, 89 of 96 run).
- blitz-paint: 42 of 43 (1 unviable).
- the browser's capture module: 0 of 4 caught.
- the demo stand: 6 caught, 7 unviable.
- blitz-dom: not measured at this boundary.

1. **`done and verified`, with the audit's reading added (Recommended).** The heading becomes
   `### 0.3.0 — done and verified`. The paragraph gains one sentence on the version-end audit's mutation testing, as
   0.1.0's and 0.2.0's carry theirs, the browser's four surviving mutants named as a debt 0.4.0 takes («Browser's
   surviving mutants», Epoch 2). Why: 0.3.0 closed 17 of 17 verified and its cold-agent run passed, which is the bar
   the two earlier versions were written «done and verified» on; the page then reads three versions in one form.
   Trade-off: the sentence states a weaker audit than 0.2.0's (four mutants survive), which the page then says openly.
2. **`done and verified`, paragraph unchanged.** Only the heading moves. Why: the closest match to the earlier
   headings with the least new text. Trade-off: 0.3.0 alone of the three states no audit reading.
3. **Keep `built, its cold-agent run passed`.** Nothing changes. This is the least change, and the plan carries it
   until you answer. Trade-off: the page keeps the interim heading, and v040-03's «0.3.0 closed in the form the
   founder chooses» stays open.
4. **Another wording**, given in your own words.

**Answer:** _(open)_

## F2 — The citation backlog (residual res-xe7pyput, absorbed into v040-03)

The backlog was 402 citations in the masters' `*-detail/` category files whose line digits a take had moved. After
the 2026-10-24 repair, what remained was three hand-read sites (`a11y-scope-summary.md:19` → `apps/browser/Cargo.toml:14`,
`established-decisions.md:85` → `paint_tree.rs:420-462`, `session-learnings.md:223` → `host.rs:350`), five held rows
and two unresolved rows.

Read at this take-up:
- Since the masters became rows (2026-11-12), no row of the seven masters holds a `file:line` citation. This was
  checked by a regex over each master, with positive controls hitting. So none of those sites, the held rows
  included, has a counterpart in a master.
- The two unresolved rows and the `host.rs:350` site are illustrative text inside a session learning
  (`.claude/docs/session-learnings.md`, now line 363), which explains how to anchor a citation replace. They are not
  citations of the tree.

1. **Read through — close the residual (Recommended).** The residual's row records the read and closes. No file
   changes. Why: no citation remains to re-point, so «read through» is what the tree shows. Trade-off: none known.
   The learning's examples stay as teaching text.
2. **Keep it as a residual**, with this reading as its reason, owned forward to a named entry. Why: only if you want
   the backlog re-read again after the canvas work. Trade-off: it carries a row with nothing left in it.
3. **Leave it untouched until a later epoch.** This is the least change, and the plan carries it until you answer.
   The row stays `absorbed` into v040-03 as it reads, and v040-03 stays open.

**Answer:** _(open)_

## What waits on these answers

v040-03 is not claimed by this chunk. Its acceptance needs both answers, and Epoch 6's «Capability statement» entry
carries its README «0.4.0 done» leg. This chunk writes README's 0.4.0 planned section and the stack document. Each
answer is applied by this chunk if it arrives before `/andromeda-implement` runs. Otherwise it rides a CARRY onto
«Capability statement» (Epoch 6), which the wrap writes.
