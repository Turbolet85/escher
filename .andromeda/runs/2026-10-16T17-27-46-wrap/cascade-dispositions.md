# Cascade dispositions — 2026-10-16-file-splits

## Step 2 — the sweep

The search: `cascade-patterns.toml`, five patterns derived from every amendment of this pass, fired as `cascade.py
sweep` (trail `cascade-2026-10-16-file-splits.json`; baseline 4f393991, the pre-CI parent), each control fired on the
pre-pass masters:
- `one-module` — the retired «one out-of-line test module» claim in its phrasings: «one unit-test module», «bar one
  `cfg(test)` module», «one `cfg(test)` (child) module», «sits / held in a file of its own · out of line»;
- `count-72` — escher-driver's «72 unit tests» and «client.rs 2»;
- `file-count` — the moved tests' counts and claims under their old files: `mcp.rs` / `schema.rs` followed by a count,
  «pins», «that», «tests» or «unit»;
- `doc-count` — «18 in document.rs», a count by document.rs;
- `inline` — unit tests said to be inline.
Not looked for: a per-file count spelled otherwise (a number before the file name), and the dated history clauses
(«mcp.rs's eleventh», «mcp.rs's nine», test-plan.md:331), which are history by construction.

The listing: `total (5 patterns) · 15 rows over 7 files` · `per class · new 0/0 · standing 3/2 · leaf 12/5 · curation
0/0 · base 0/0`.

Master rows (3 standing, in 2 files):
- `.andromeda/architecture.md:254` one-module — the dioxus-native-dom row's «with one `cfg(test)` child module,
  `bridge_tests`» → no change: a true claim of that module (dioxus_document has one such child).
- `.andromeda/test-plan.md:18` inline — blitz-paint's inline unit tests in gradient.rs and css_box.rs → no change: true,
  a different crate.
- `.andromeda/test-plan.md:20` inline — escher-telemetry's 15 inline unit tests → no change: true, a different crate.
- `doc-count`: 0 rows, its control fired at test-plan.md:11 on the pre-pass text — the amended line no longer states it.
- `count-72` and `file-count`: 0 master rows — the amended key file and test-plan lines no longer state them.

Every amended line re-read for an intra-line duplicate of the retired claim: test-plan.md:21 keeps «mcp.rs's eleventh»
and «mcp.rs's nine» (dated history of the count, true as history) and its new «in mcp.rs until 2026-10-16-file-splits» /
«in schema.rs until …» (history); the key file keeps «72 until 2026-10-14-quality-gates» (history); none restates a
retired claim as current.

Leaf rows (12, in 5 files) → step 3's set (re-derived, below):
- `.claude/rules/testing.md:14` one-module — «one module sits in a file of its own» → stale (five);
- `.claude/docs/services/escher-driver.md:60` count-72 ×2 · file-count ×2 — «72 unit tests», «`client.rs` 2»,
  «`schema.rs` 6», «`mcp.rs` 11» → stale;
- `.claude/docs/tests-summary.md:24` file-count ×2 · inline — «73 inline unit tests», «`schema.rs` 6», «`mcp.rs` 11» →
  stale;
- `CLAUDE.md:85` inline — «`packages/escher-driver/src/mcp.rs` (unit tests inline)» → stale (now `mcp_tests.rs`);
- `CLAUDE.md:80` inline — «`…/{schema,command,refusal,describe}.rs` (unit tests inline)» → stale for schema (now
  `schema_tests.rs`);
- `CLAUDE.md:74`, `CLAUDE.md:75`, `.claude/docs/services/escher-telemetry.md:40`, `.claude/docs/tests-summary.md:22`,
  `:23` inline → no change: true of other crates and files.

## Leaves

The set: `cascade.py leaves` in the PASS mode (this chunk does not close Epoch 1: a markerless entry remains), the
step-2 leaf files added (`--add` testing.md · services/escher-driver.md · tests-summary.md · CLAUDE.md) — `total · pass
· 11 group(s) · 25 leaves · 363434 B · unmapped 1`. The deltas carry the citation sweep's re-points beside this pass's
nine amendments. `unmapped .claude/rules/host-linux.md (host)` → no comparer: the host's own leaf, a statement no
master states. The batch: 11 comparers in one snapshot pair (before `tree 85f31cca…`, after `tree c728b306…`, index and
head unchanged, `git diff-tree` exactly the eleven `A` lines). Apply: `total · blocks 11 · applied 11`.

- architecture-1 · pass · 1 finding · applied 1 — services/blitz-dom.md:35 (omission: `document`'s two `#[path]` test
  children named beside the in-file modules) · by hand 0 · declined 0.
- architecture-2 · pass · 4 findings · applied 4 — services/escher-driver.md:27 (omission: `mcp`'s three children and
  `mcp_tests.rs`), :34 (omission: `schema`'s `spec` and `schema_tests.rs`), :60 ×2 (stale: the counts' files) · by hand
  1 — services/escher-driver.md:60 «72 unit tests … `client.rs` 2» → «73 … `client.rs` 3», «72 until
  2026-10-14-quality-gates» added to its history: the operator's directive (inputs#I3), the count the test-plan key
  file now states (session-lifecycle.md:11), which this arch-only group does not read · declined 0.
- masters · pass · 2 findings · applied 2 — CLAUDE.md:80 (schema's tests in `schema_tests.rs`, its shape types in
  `schema/spec.rs`), :85 (`mcp.rs` with `mcp/{framing,listing,result}.rs`, tests in `mcp_tests.rs`) · by hand 0 ·
  declined 0.
- test-plan · pass · 4 findings · applied 4 — tests-summary.md:24 ×3 (stale: «inline», the two counts' files),
  rules/testing.md:14 (stale: one module → five) · by hand 0 · declined 0.
- a11y-plan · architecture+layout-templates · architecture+obs-plan · architecture+test-plan ·
  design-system+layout-templates · obs-plan · security-plan — pass · 0 findings each.
