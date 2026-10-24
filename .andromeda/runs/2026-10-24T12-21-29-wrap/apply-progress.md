# P2 Apply — progress at the context stop (2026-10-24, 13:33Z)

Validate is closed: every disposition is in `fanout-results.md` (this run dir). The halt question is answered (inputs#I10:
category-file citations owned forward — re-point only gone-path citations and the citations inside text this wrap amends;
the 402-citation digit drift is F334, owned by a `.andromeda/residuals.md` line, target next, the count named in it).
The cascade's tally (`detail.py tally … --no-fit 0` per extracted doc) runs after Apply. Sidecar entries wait for the
step-2 sweep (`cascade-patterns.toml` → `cascade.py sweep`). Helpers (session scratchpad, re-creatable): `lines.py`
(print lines with lengths), `remap.py FILE LINE…` (old→new line of each citation into a file the take changed, by
`git diff -U0 a356f9086 HEAD`, with a text check). `inline.rs` → `inline/layout.rs`: every cited line keeps its number
(measured: the base file and `inline/layout.rs` differ only at 23-24 and 919-925).

Rule kept on every new line: no provenance (no chunk-marker stamp, no date, no who-ruled) in new text — the founder's
word (inputs#I4) and the dates go to the sidecar entries' **Why:**; new detail lands as a new list item or group, a
value replaced in place stays in place; citations inside amended lines re-pointed with `remap.py`.

## Done
- architecture.md (body): :21 flake drift dropped + mimalloc clause; :44 [Style threading] rewritten (Sequential field
  doc, PARALLEL_TRAVERSAL_LOCK, `Parallel` launches, config.rs:11-17); :143 hit skips dropped nodes; :187 `multi_doc`;
  :211 lock + mimalloc; :216 MULTI_DOC_*; :276 `inline/layout.rs:236` · `:877-880`; :286 Upstream sync `f9cbb2756`,
  thirteen taken, Stylo 0.23-pre · six Taffy-main · Dioxus 0.8 waiting.
- architecture-detail/design-philosophy.md :53 (out_of_flow, classes; element.rs cites re-pointed 94-97 · 130-134 ·
  168-170 · 303-340 · 526-539).
- architecture-detail/stack-and-technologies.md :39 tokio resolved 1.54.1; :60 mimalloc default (Cargo.toml:14 added);
  :66 wpt runner mimalloc + Cargo.toml 36-43 · 46-53; :69 rust-bin 1.91.0, drift sentence → «matches».
- architecture-detail/established-decisions.md :32 browser defaults + mimalloc; :67 range merge surface at `f9cbb2756`;
  :69 keyboard merge surface at `f9cbb2756`; :75 · :76 · :78 owners → residuals line (target next); :57 and :85
  `inline/layout.rs` paths.

## Done at the resume (2026-10-24, second window)
- Remaining items 1–10 applied (every body and category edit), `detail.py check` 0 defects ×6, `classify.py check
  --mode standing` PASS on six bodies and every edited category file; the six tally rows read (trail
  `detail-{marker}.json`). The hand sweep added one fold: «until upstream retires it» → «until escher takes upstream's
  retirement of it» (test-plan.md:408, quality-gates :14, standard-contracts :255, existing-scopes :76).
- Step 2: `cascade-patterns.toml` (21), sweep (11 fired; 10 controlled by hand over the category files),
  `cascade-dispositions.md` written; 12 sidecar entries appended and read back (`*-entry-*.append.md`).
- Helpers now in this window's scratchpad: `remap.py`, `linecites.py`, `detailsweep.py`.
- Next: item 11's `cascade.py leaves` (pass mode — epoch 5 has 4 markerless entries), the comparers (≤ 16), decisions,
  `leaves --apply`, `## Leaves` in `cascade-dispositions.md`; then item 12.

## Remaining (in this order, as written at the first stop)
1. established-decisions.md: NEW group after the dec-layout group (after line 87 `<!-- /detail -->`):
   `<!-- detail: values · dec-layout -->` · «- A replaced element's natural height follows its usable `aspect-ratio`
   in `compute_replaced_layout` (`is_usable_ratio`: finite and > 0), clamped at 0, an upstream engine change for every
   document (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211).» ·
   `<!-- /detail -->`; NEW group after the second dec-dom-semantics group (after line 103): «- A hit test skips a node
   dropped since the last resolve, and the paint tree's walk does the same — an upstream engine change for every
   document (§Standard Contracts → Scrolling, selection, tree).»
2. standard-contracts.md: :45 hit-walk clause (cascade-g3 1 text, no stamp; node.rs 1414-1417 · 1572-1574 · 1587-1589 ·
   1621-1623 · 1646-1650; paint_tree.rs 164-172; hit_test_dropped_nodes.rs 32-82); two bullets after :75 in the
   contract-nodes-and-widgets group ((sampler) standard-contracts#9.2 texts, no stamp); :229 CLIs `multi_doc`.
3. occupied-resources.md: two records after :49 ((sampler) #6.1 lock and mimalloc texts, no stamp); :55 MULTI_DOC_*.
4. project-intent.md :9 (cascade-g4 1, no stamp); existing-scopes.md :46 (inline directory), :103
   (`hit_test_dropped_nodes`), :123 (`multi_doc`); cross-cutting-patterns.md :15 (multi_doc timing); conventions.md :15
   ((sampler) mimalloc exception — read the entry first), :27 ×2 and :46 `inline/layout.rs` paths.
5. security-plan.md: :53, :234, :294, :363 (`inline/layout.rs:732`), :369, :370. security-plan-detail:
   input-validation.md :13, :245, :248 path; dependency-security.md new record after :20 (the sync) and after :10
   (tokio); error-handling.md new group after :26 (the lock, no panic path); logging-monitoring.md :15.
6. design-system-detail/depth-strategy.md :26 (short clause → architecture Standard Contracts). layout-templates.md
   §Surface: cli `multi_doc` beside `paint_bench`.
7. test-plan.md :19, :176 (re-count link: run stand 195 · 0 · 7 over 48 files, run all 617 · 0 · 12); test-plan-detail:
   test-scope-summary.md :19, :22 (two items), :29, :97; test-strategy.md :48 (145 files / 144 targets);
   ci-integration.md :40, :59; quality-gates-coverage-targets.md :14.
8. obs-plan.md §1 examples scope, §3 silence census (the body's `absence · logs` block); obs-plan-detail:
   metric-coverage.md :16 row, log-coverage.md :33 cite, pii-scrubbing-compliance.md new item after :11.
9. a11y-plan-detail: visual-design-verification.md :75 (short clause), a11y-scope-summary.md :18 re-read.
10. `detail.py check --project . --master {doc}` 0 defects per extracted doc; `classify.py check --mode standing` per
    edited body section; `detail.py tally … --no-fit 0` per extracted doc (report the six rows to the operator).
11. Cascade: `cascade-patterns.toml` (every amendment's old wording and verbs: «defaults to `Parallel`», «documented to
    panic», «1.90.0», «three of its commits», «17de049b1», «four Taffy-main», «optional `mimalloc`», «owned by a CARRY on
    «Upstream sync…»», «only tree-structure», «layout/inline.rs», «143 `.rs`», tallies 205 · 1098 / 186 · 1095 / 186 ·
    1051, «616»), `cascade.py sweep`, `cascade-dispositions.md`, then the sidecar entries (`sidecar.py check
    --entry-file`, `splice.py append`), then `cascade.py leaves` + one comparer per group (≤ 16 running), decisions,
    `leaves --apply`.
12. Then P2's evolve checkpoint (`references/evolve/reconcile.md`), P3–P7. P5 appends three `.andromeda/residuals.md`
    lines: P5 card rows 1 and 2 (inputs#I5) and F334's (inputs#I10, the 402 count).
