# Cascade dispositions — 2026-11-22-epoch-2-corrective (wrap 2026-11-22T19-12-58-wrap)

## Step 2 — the sweep
- The landings: one masters TX, 3121 (architecture, `ops-main-architecture.md`: + decision/fork-additions-in-own-functions).
  Its dry run wrote no pattern file — `patterns: none — the call sets and retires no row`; the three proposer files'
  dry runs landed 0 operations each and wrote none.
- Main's patterns, `cascade-patterns.toml`: `ceiling` (`[Cc]ognitive[- ]complexity|ceiling of 15|rust-code-analysis`,
  the rule the new row qualifies) and `fork-in-upstream` (the new row's own verbs: `fork addition|escher's own, with
  one call|one call in upstream|upstream's (own )?(body|function|text)`). The first call refused `fork-in-upstream`:
  `control UNFIRED over the seven masters … at 40ec4749` — no master held the new claim's wording before this pass,
  its zero a statement about that pattern alone; re-fired with `--pattern ceiling` (`sweep-pass-1.1.txt`).
- Its listing: `total (1 pattern) · 7 rows over 5 files` · `per class · new 1/1 · standing 2/2 · leaf 2/2 · curation 2/2 · base 0/0`.
  - `.andromeda/architecture.md:79` standing `dec-ndcu6j63` decision/cognitive-complexity-ceiling — no change: true, the
    rule the new row qualifies, its wording unretired.
  - `.andromeda/architecture.md:86` new `dec-55obytqd` — the amended row itself.
  - `.andromeda/test-plan.md:573` standing `dec-akszmc27` decision/complexity-ceiling-host-only — no change: true, the
    reading's host-only placement, untouched by the new rule.
  - `.claude/rules/testing.md:72` curation — no change: a 2026-11-16 learning on the tool's output naming, nothing the
    new rule moves (preserve-verbatim).
  - `.claude/docs/session-learnings.md:12` curation — no change: a learning on reading upstream's rev with the tool,
    which the new rule leaves true (preserve-verbatim).
  - `.claude/rules/testing.md:48` leaf — step 3's set, its line a sweep line its group's comparer answers.
  - `.claude/docs/stack.md:39` leaf — step 3's set, its line a sweep line its group's comparer answers.
- Not looked for: the new rule's wording in another phrasing than the two patterns; the leaves' comparers re-compute
  each leaf against its sources whole.
- Lateral binds: test-plan §3 ↔ obs-plan §3 / §6 and a11y-plan ↔ the log schema — no row of either side changed. Stale
  statements of the amended claim across the seven masters: none — the row adds a rule beside
  decision/cognitive-complexity-ceiling and retires no wording (the `ceiling` rows above, each true).

## Leaves
`cascade.py leaves` (pass mode; epoch «Epoch 2 — Debts of 0.1.0–0.3.0» open, gated 1): `total · pass · 7 group(s) · 16
leaves · 387903 B · unmapped 1`; `unmapped .claude/rules/host-linux.md (host)` — the host leaf, a statement no source
states, no comparer's. One comparer a group, `--batch leaf`, 7 of 7 pass.
- added (`.claude/rules/testing.md`) · pass · swept `.claude/rules/testing.md:48 · ceiling · no source row matched` —
  answered by a finding (`@leaf_edit` omission, source dec-55obytqd) · findings 1 · applied 1 · by hand 0 · declined 0.
- architecture-1 (`commands.md`, `conventions.md`, `gotchas.md`) · pass · findings 0.
- architecture-2 (five `services/` leaves: blitz-dom, blitz-paint, blitz-shell, blitz-test-harness, blitz-traits) · pass · findings 0.
- architecture-3 (`services/dioxus-native-dom.md`) · pass · findings 0.
- architecture-4 (`services/escher-driver.md`) · pass · findings 0.
- architecture-5 (`services/escher-telemetry.md`, `services/seven_guis.md`) · pass · findings 0.
- architecture-6 (`stack.md`, `workflow.md`, `CLAUDE.md`) · pass · swept `.claude/docs/stack.md:39 · ceiling · no source
  row matched` — answered by a finding (`@leaf_edit` omission, source dec-55obytqd) · findings 1 · applied 1 · by hand 0 · declined 0.
- `cascade.py leaves --apply` (7 files): `total · blocks 2 · applied 2`; no `@leaf_stands` record in any file; no leaf
  line written by hand (the operator's directive point 6, inputs#I6).
