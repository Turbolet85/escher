# Cascade dispositions — 2026-11-07-ui-graph (wrap 2026-11-08T02-04-13)

## The search
- Patterns: the fourteen apply writers' `@pattern` records (`apply-*.md`, ids `a11y-*`, `arch1-*`, `arch2-*`, `arch3-*`,
  `lt-*`, `obs-*`, `sec1-*`, `sec2-*`, `tp1-*` … `tp4-*`) and main's own `cascade-patterns.toml` (`m-needles`,
  `m-cli-rows`, `m-cli-count`, `m-mcp-server`, `m-sc-readers`, `m-run-stand`, `m-iv-bridge`, for edits-main-1, -2 and -4);
  every pattern's control fired on the pre-pass masters.
- The listing: `cascade.py sweep` (cascade v1.29), baseline `be4180b8`, run after the last amendment
  (`sweep-pass-2.1.txt`, the one part, read whole). The first run (`sweep-pass-1.1.txt`, before edits-main-4) found
  the two stale rows edits-main-4 then amended.
- Copied from the tool: `total (89 patterns) · 157 rows over 37 files` · `per class · new 15/10 · standing 33/14 ·
  leaf 109/17 · curation 0/0 · base 0/0`.
- What was read: each master row's fact at its offset (`rec.py get --at-char`); not looked for — wording of the moved
  claims outside these patterns' phrasings (the detectors' own sweeps are the control for reach).

## Master rows — new 15
- amended — every `new` row is this pass's own text: test-scope-summary.md:180 fct-i5a6vv3t (×3 patterns, «twenty-one
  on the snapshot builder»), integration-test-strategy.md:64 fct-adkwyvky, pii-scrubbing-compliance.md:11 fct-hlttx6z4,
  e2e-test-strategy.md:8 fct-csryo2zo, unit-test-strategy.md:32 fct-klxgwgcy, session-lifecycle.md:11 fct-egnkfdyv
  (×2), e2e-test-strategy.md:12 fct-gtzxj2kr, obs-plan.md:76 fct-icom2cq7, test-plan.md:194 fct-6bs5asmb,
  security-plan.md:155 fct-frlob5k5, input-validation.md:375 fct-3atajnay, input-validation.md:378 fct-nzg4gdto —
  each re-read; no duplicate of a retired mechanism on the line.

## Master rows — standing 33
- amended (edited rows, the line carries this pass's change): established-decisions.md:200 fct-b7w6xfsd ·
  existing-scopes.md:122 fct-qkp2micb · occupied-resources.md:79 fct-jzxi3spj · test-scope-summary.md:14 fct-k5y3m6cu
  (one row, two offsets) · test-scope-summary.md:164 fct-nks3g3e7 · conventions.md:46 fct-knw5nmxh ·
  established-decisions.md:122 fct-nl4nv5ny — 7 rows.
- no change, a dated earlier step the line keeps as history: logging-monitoring.md:13 fct-4hqe6pzr (×4 patterns,
  `obs-03` · `sec2-09` · `tp2-04` · `tp4-06`, «the verb table holds eight instance-level verbs with the `lint` verb»,
  the step of 2026-10-12; the ninth stated in the pass's new group beside it) — 4 rows.
- no change, a dated reading: test-plan.md:192 fct-u5f4aal7 (`arch2-13`, «the twelve `mcp__escher__*` tools» of the
  2026-10-25 live run, named historical in the report) · test-plan.md:179 fct-lk6drchu (`tp1-03`, `tp4-03`, «237 · 0 · 7
  over 51 files», the reading of its chunk, named historical in the report) — 3 rows.
- no change, a true claim sharing the token: architecture.md:293 fct-7lmrueug, project-intent.md:11 fct-4ncs7geh,
  dependency-security.md:31 fct-7aitszps, quality-gates-coverage-targets.md:16 fct-fx4qekph — «twenty-one left out», the
  upstream sync's count (`arch3-04`, `arch3-06`, `tp2-15`, 12 rows); architecture.md:74 fct-fe5adstg,
  standard-contracts.md:46 fct-oq4wpu6j, security-plan.md:153 fct-qk7naa6l, input-validation.md:372 fct-omhuceqe, :381
  fct-et47i5mq, :384 fct-t6dnzfpt, :387 fct-wlz77kgt — the Dioxus mutations / bridge lines main's `m-iv-bridge` control
  reads, none stating the dispatch (7 rows) — 19 rows.
- stale, amended in this pass after the first listing: obs-plan.md:76 fct-icom2cq7 («thirty-two checks», now thirty-three
  with `stand_graph`) and test-plan.md:194 fct-6bs5asmb (`run stand` 237 · 0 · 7 over 51 files → 244 · 0 · 7 over 52,
  the report's stale row) — edits-main-4, tx 315; they read `new` in the final listing.
- label rows: none.

## Leaf rows — 109
- leaf — every leaf row joins cascade step 3's set: CLAUDE.md, `.claude/rules/{a11y,security,verification-harness,testing}.md`,
  `.claude/docs/{a11y,security,tests,obs}-summary.md`, `.claude/docs/{commands,conventions,gotchas,workflow}.md`,
  `.claude/docs/services/{escher-driver,dioxus-native-dom,seven_guis,blitz-dom}.md` (17 files, the tool's `leaf 109/17`).

## Curation and bases
- curation 0 · base 0 — no curation home and no judgment base quotes the moved wording.

## Leaves
- Mode: pass (the chunk does not close its epoch: Epoch 3 — The UI graph holds 2 markerless entries), baseline `be4180b8`;
  the set call with step 2's 17 leaves added (`--add`): 15 groups, 25 leaves; `unmapped .claude/rules/host-linux.md
  (host)` — the host's own leaf, a statement no source states, no comparer's.
- Re-pointed, no changed source: design-philosophy, color-palette, mocking-stubbing-discipline, cognitive-accessibility,
  visual-design-verification (citation digits alone).
- Per group — findings · applied · by hand · declined: a11y-plan-1 3·3·0·0 · a11y-plan-2 3·3·0·0 · architecture-1
  3·3·0·0 · architecture-2 11·11·0·0 · architecture+layout-templates 7·7·0·0 · architecture+obs-plan 2·2·0·0 ·
  architecture+security-plan+test-plan-1 32·32·0·0 · architecture+test-plan 7·7·0·0 · design-system+layout-templates
  3·3·0·0 · masters-1 18·18·0·0 · obs-plan 14·14·0·0 · security-plan-1 13·13·0·0 · security-plan-2 11·11·0·0 ·
  test-plan-1 20·20·0·0 · test-plan-2 6·6·0·0.
- Each file read whole and each finding's source checked at its cited fact; every omission applied is one a reader of
  the leaf would otherwise act against (a count, a verb list, a carried set).
- Copied from the tool: `total · blocks 153 · applied 153`; 18 leaves written, each read back.
