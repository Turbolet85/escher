# Cascade dispositions — 2026-10-27-readme-and-records

## Step 2 — the sweep

The search: one `cascade.py sweep` call (its trail `cascade-2026-10-27-readme-and-records.json`), baseline `316c6d25`
(the parent of the oldest pre-CI commit 64256172), over all seven masters, their category files, the registries, the
leaves and the curation homes, with the patterns of `apply-architecture.md` (`arch-01`), `apply-a11y-plan.md`
(`a11y-01`) and main's own `cascade-patterns.toml` (`main-01`…`main-05`; the writers of design-system, layout-templates
and security-plan named none):
- `arch-01` `0.2.0 planned in five stages` — the Front page's retired status wording
- `a11y-01` `accent.*\(design-system §Color Palette\)` — the a11y row's old pointer to the accent's home
- `main-01` design-system §Color Palette / §Typography / §Depth Strategy / §Spacing / §Border Radius cited as a home
- `main-02` `build/escher-0.1.0|default branch` — the default branch the front page renders
- `main-03` `0.2.0 (is )?planned|planned,? in five stages` — the README's old 0.2.0 state, other spellings
- `main-04` the moved rows' own names (`seven_guis accent` · `success` · `(untokenized)` · `cards pad|use` · `load page
  (stand)` · `Field — seven_guis`)
- `main-05` the moved typography row's verb (`registers the same bundled DejaVu Sans`)

Every control fired (four patterns read 0 rows with their control fired — a statement about the pattern).
`total (7 patterns) · 8 rows over 8 files` · `per class · new 1/1 · standing 1/1 · leaf 6/6 · curation 0/0 · base 0/0`.

| Row | Class | Disposition |
|---|---|---|
| `.andromeda/architecture-detail/project-intent.md:27` main-02 | new | amended — the front-page fact this pass wrote (`build/escher-0.3.0`) |
| `.andromeda/security-plan.md:23` main-02 | standing | no change — the WPT post-results workflow «checks out scripts from the default branch», a true claim sharing the token |
| `CLAUDE.md:65` main-01 | leaf | re-derived — step 3's set (the pointer table's design-tokens row; it points at sections, no stand value) |
| `.claude/rules/a11y.md:38` main-01 | leaf | re-derived — step 3's set (cites the accent's home as design-system §Color Palette) |
| `.claude/docs/commands.md:78` main-02 | leaf | re-derived — step 3's set (one build branch per version, not the default branch) |
| `.claude/docs/security-summary.md:25` main-02 | leaf | re-derived — step 3's set (the WPT checkout from the default branch) |
| `.claude/docs/workflow.md:6` main-02 | leaf | re-derived — step 3's set (one build branch per version) |
| `.claude/docs/design-summary.md:15` main-04 | leaf | re-derived — step 3's set (the moved `seven_guis success` row) |

Every amended line re-read: no intra-line duplicate of a retired wording stands (`arch-01`, `main-03` 0 rows).

## Leaves

Mode: WHOLE — this chunk closes Epoch 1 (`route.py epoch`: Epoch 1 · markerless 0 · pending 1), set call `cascade.py
leaves --all` with step 2's six `leaf` rows `--add`ed: `total · whole · 13 group(s) · 25 leaves · 553197 B · unmapped
1` — the unmapped `.claude/rules/host-linux.md` is the host leaf, no master source, no comparer. The write: `total ·
blocks 24 · applied 16 · held 8`. Every finding's source quote was read on its cited line (24 of 24 standing).

| Group | Findings | Applied | By hand | Declined (why) |
|---|---|---|---|---|
| a11y-plan | 1 | 1 (`a11y.md:38` the accent's home) | 0 | — |
| architecture-1 | 0 | 0 | 0 | — |
| architecture-2 | 3 | 2 (`stack.md:39` the complexity reader beside the mutation tier; `workflow.md:8` the default branch) | 0 | 1 — `stack.md:23` tokio's resolved 1.54.1: the requirement the leaf states holds, the resolution is the master's detail |
| architecture+layout-templates | 3 | 0 | 0 | 3 — `seven_guis.md:24` ×2 (the hover/active ratios; the moved card values) and `:19` (the font's boots): each true as the leaf states it, the values the masters' detail |
| architecture+obs-plan | 0 | 0 | 0 | — |
| architecture+security-plan+test-plan-1 | 0 | 0 | 0 | — |
| architecture+test-plan | 2 | 2 (`blitz-test-harness.md:21` the misattributed load-page clause; `:15` the class read off the last pass) | 0 | — |
| design-system+layout-templates | 4 | 3 (`design-summary.md:13`, `:15` the stand rows' new home; `:24` the stand's font retired from design-system) | 0 | 1 — `design-summary.md:28` the stdout ending's exit codes, the master's detail |
| masters | 1 | 1 (`CLAUDE.md:65` the pointer row names the stand values' new homes) | 0 | — |
| obs-plan | 5 | 4 (`obs-summary.md:14`, `:30`; `observability.md:19`, `:26`) | 0 | 1 — `obs-summary.md:32` the state directory's mode and socket, the master's detail |
| security-plan | 0 | 0 | 0 | — |
| test-plan-1 | 4 | 2 (`tests-summary.md:13` settle's classes; `:27` the vibey-script test counts) | 0 | 2 — `:37` the binary-driven E2E inventory and `:44` the complexity figures: true where the leaf stands, the detail test-plan §10's and `stack.md`'s and `testing.md`'s now |
| test-plan-2 | 1 | 1 (`testing.md:48` the complexity ceiling and the sibling-item rule) | 0 | — |
| **total** | **24** | **16** | **0** | **8** |
