# Cascade dispositions — 2026-10-06-accessibility-tree-identity

## The search
- Pattern set `cascade-patterns.toml` (8 patterns), run by `cascade.py sweep` after every body amendment of this pass; the listing is `cascade-sweep.txt` (baseline `076d74cb`, the pre-CI parent).
  - `written-nowhere` — the retired "the id is written nowhere" claim and its phrasings (`written to neither`, `written to no`).
  - `only-forward` — the retired "the Dioxus crates only forward the feature to blitz" claim (`only forward`, `forwards to blitz-dom`, `forwarding to the same-named`).
  - `build-a11y-tree` — every statement of the tree build's mechanism.
  - `text-child-name` — the retired "a control's only name source is its text children" mechanism (`labelled by them`, `text children`, `label their parent`, `accessible name`, case-insensitive).
  - `aria-label` — every site naming the attribute.
  - `count-444` · `stand-20` — the moved workspace and `run stand` counts.
  - `default-feature` — the "`accessibility` is a default feature" claim, whose scope the chunk measured (per crate, not the stand binary).
- Dropped: `update_tree` and a shell-inner pattern (`self.doc.inner()` / `from the inner BaseDocument`). Their known-positive controls never fired over the seven masters at `076d74cb`, so no master stated the shell's build mechanism in those tokens. That mechanism's sites were reached through `build-a11y-tree` and through the window.rs / blitz-shell accessibility.rs citations the line-map pass re-pointed.
- The line-map pass (`repoint.py`, `repoint-masters-dry.txt` = `repoint-masters-applied.txt`): 159 citations over the seven masters (arch 66 · a11y 33 · test 22 · design 13 · security 11 · obs 10 · layouts 4); registries 0. The same script found 0 citations in CLAUDE.md, `.claude/docs/**` or `.claude/rules/**` (dry run over all leaves). Spot-checked 18 target lines in the new files.

## Rows (cascade-sweep.txt, 58 hit rows)
- `architecture.md:134` written-nowhere · new — this pass's own text ("written to neither the DOM nor the vdom" beside the `author_id` carrier). True.
- `security-plan.md:107` written-nowhere · new — this pass's narrowed `id` row. True.
- `.claude/docs/services/dioxus-native-dom.md:15` written-nowhere · leaf — stale ("written nowhere") → **re-derived**.
- `architecture.md:189` only-forward · standing, edited — amended this pass (now names the dioxus-native-dom forward). True.
- `obs-plan.md:45` only-forward · standing — `log-phase-times` forwards to blitz-dom: a true claim sharing the token. No change.
- `a11y-plan.md:10` only-forward · standing — blitz-html's `accessibility` forwards to `blitz-dom/accessibility`: true. No change.
- `a11y-plan.md:14` only-forward · standing, edited ×2 — amended this pass (dioxus-native-dom's forwards to blitz-dom; dioxus-native's to blitz-dom, blitz-shell and dioxus-native-dom). True.
- `a11y-plan.md:177` only-forward · standing — the `autofocus` feature forwards to blitz-dom: true. No change.
- `.claude/rules/verification-harness.md:16`, `:25` only-forward · leaf — the `.ps1` scripts "only forward" to bash: a different subject. No change.
- `architecture.md:120` build-a11y-tree · new — this pass's Document-core text. True.
- `architecture.md:121` build-a11y-tree · standing, edited ×2 — amended this pass (the three name sources; the shell builds through the trait). True.
- `a11y-plan.md:8` build-a11y-tree · standing, edited (re-point only) — "`BaseDocument::build_accessibility_tree` builds an AccessKit tree from the DOM" is still true. No change beyond the re-point.
- `a11y-plan.md:64` build-a11y-tree · standing — tests call `build_accessibility_tree()` after `resolve`: still true (the new engine tests compare it with the trait method). No change.
- `.claude/rules/a11y.md:22`, `:38` build-a11y-tree · leaf — **re-derived** (rule file: name sources, `Document::accessibility_tree`, `author_id`, the Dioxus feature pin; the Tests bullet adds the Dioxus `author_id` read).
- `.claude/docs/a11y-summary.md:12` build-a11y-tree · leaf — **re-derived** (Tool line names the trait method and `author_id`).
- `.claude/docs/services/blitz-dom.md:15` build-a11y-tree · leaf — **re-derived** (trait method list; name sources).
- `architecture.md:121` text-child-name · new — this pass's text. True.
- `security-plan.md:108` text-child-name · new — the new name-source row. True.
- `test-plan.md:25` text-child-name · new — the stand-file list's new clause. True.
- `a11y-plan.md:266` text-child-name · standing, edited (re-point) — "Text nodes become `TextRun` nodes … the parent is labelled by them" is still true as ONE source; the two new sources are new bullets at :267-:268. No change beyond the re-point.
- `a11y-plan.md:269` text-child-name · new — this pass's text. True.
- `.claude/rules/a11y.md:25` text-child-name · leaf — **re-derived** (adds the two sources).
- `aria-label` rows: `architecture.md:121` · `security-plan.md:108` ×2 · `layout-templates.md:10` · `test-plan.md:25` · `a11y-plan.md:267` · `:269` — new, this pass's text, true. `a11y-plan.md:114` ×2 · `:119` · `:120` · `:121` · `:122` ×2 · `:291` — standing fixture descriptions (GitHub, servo, graphite, google fixtures carry `aria-label`s): true; they gain no claim. No change.
- `test-plan.md:314` count-444 · standing, edited — the history chain keeps "444 · 0 · 5 at id-persistence" as history; this pass appended the 454 re-count. True.
- `.claude/docs/tests-summary.md:28` count-444 · leaf — **re-derived** (454 · 0 · 5 at this chunk, 444 kept as history).
- `test-plan.md:111` stand-20 · standing, edited — history chain; this pass appended the 25 re-count. True.
- default-feature rows: `architecture.md:152` (seven_guis is a `[workspace.dependencies]` path entry with default features — of seven_guis itself, true) · `security-plan.md:218` (wptreport) · `test-plan.md:7` · `test-plan.md:311` (CI legs run default features) · `obs-plan.md:41` (`tracing` of blitz) · `a11y-plan.md:8` (blitz-dom's defaults include `accessibility`) · `a11y-plan.md:11` (blitz-shell) — standing, true claims sharing the token. No change. `a11y-plan.md:14` · standing, edited — amended this pass to scope the claim (per crate; the workspace pin means a crate gets it only by naming it; the stand binary names none).
- `CLAUDE.md:44` default-feature · leaf — "`accessibility` is a default feature" (the warnings block). Per crate it is true. The stand-binary consequence now lives in a11y-plan §1 and `.claude/rules/a11y.md`, and the working route carries the operator-directed note. **No change** to the warnings block (it states the invariant, not the stand's build).
- `.claude/docs/commands.md:24` · `.claude/docs/services/seven_guis.md:14` default-feature · leaf — "default features" of the CI test leg / of seven_guis itself: true. No change. (commands.md:27 and seven_guis.md:36 were re-derived for the new stand file anyway.)

## Leaves re-derived (step 3; the table's floor plus provenance)
- architecture (§Standard Contracts, §Cross-cutting, the Dioxus/blitz-dom/blitz-shell/seven_guis modules) → CLAUDE.md `GENERATED:setup:modules` (dioxus-native-dom bullet) and `:architecture` (stable ids paragraph) · `.claude/docs/commands.md` (stand test command) · `.claude/docs/conventions.md` (feature forwarding) · `.claude/docs/services/{dioxus-native-dom,blitz-dom,blitz-shell,seven_guis}.md`. `.claude/docs/gotchas.md`, `stack.md`, `workflow.md`, `services/{blitz-paint,blitz-traits}.md` read: no amended fact stated — no change.
- a11y-plan → `.claude/rules/a11y.md` · `.claude/docs/a11y-summary.md`.
- test-plan → `.claude/docs/tests-summary.md` (count, stand list) · `.claude/docs/services/seven_guis.md` (stand file list) · `.claude/rules/testing.md` and `.claude/rules/verification-harness.md` read: no amended fact (no stand-file list, no count) — no change. `services/blitz-test-harness.md`: no change.
- layout-templates / design-system → `.claude/docs/design-summary.md` (the attributes).
- security-plan → `.claude/docs/security-summary.md` and `.claude/rules/security.md` read: neither states the `id` row's "written nowhere" or any name-source fact — no change.
- obs-plan → re-points only, and no leaf carries a citation — no change to `obs-summary.md` / `rules/observability.md`.
- Curation homes (`USER:session-learnings`, Session Additions, `docs/session-learnings.md`) and judgment bases (`playbook.md`, `drift-base.md`): 0 rows in every pattern (`curation 0 · base 0`).
- Lateral binds: test-plan §3 ↔ obs-plan §3 untouched (no harness change); a11y ↔ obs schema untouched.
