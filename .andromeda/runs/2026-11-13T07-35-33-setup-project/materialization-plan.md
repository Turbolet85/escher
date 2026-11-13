# Materialization plan — escher · structural re-run · 2026-11-13T07-35-33

**Form:** structural. The operator's words at invocation: «the masters now stand as rows (U65, landing 2026-11-13T07-21-39): a structural change in the masters — re-render CLAUDE.md's overview · modules · warnings · pointer-table blocks from the rows: the verbs line as claude-md-template.md now spells it (get {id}), the pointer table's «one address per key»; the overview from overview/project-intent and the versions from the manifest (Cargo.toml · rust-toolchain.toml), never from a row; the modules from the name rows of class crate and the crates §Inherited Defaults' rows name; the warnings from §Anti-Patterns (Universal) rows — «none» where 0.»

**Development Style:** agent-driven (`decision/agent-driven-development`). **Primary language:** rust (root `Cargo.toml`).

**Phase 0's reads:** scoped to the four blocks the operator named, each from its source:
- every master's tree;
- architecture's `overview` rows and `name` rows;
- §Inherited Defaults, rendered;
- §Cross-cutting Patterns, rendered;
- every plan's §Anti-Patterns (Universal) area (0 rows in each, from `rec.py tree`);
- `rec.py contracts` for each master;
- `Cargo.toml` and the absent `rust-toolchain.toml`.

Every other block and leaf is preserved and re-derived from nothing.

## Tier 1 — CLAUDE.md

**overview — re-rendered.**
- The paragraph is from `overview/project-intent`.
- The stack line takes its versions from `Cargo.toml` alone:
  - workspace 0.3.0-beta.4, edition 2024, rust-version 1.91.0;
  - stylo 0.22.0, taffy 0.15, parley 0.12, anyrender 0.14.0, vello 0.11, winit =0.31.0-beta.3, accesskit 0.25, dioxus 0.7.3, boa_engine 0.22, tokio 1.42, reqwest 0.13;
  - rusqlite 0.32 (in `apps/browser/persistence/Cargo.toml`).
- No `rust-toolchain.toml` stands, and the line says so.
- The key directories (5) are read from the tree.

**modules — re-rendered.** The map is the union of the two sources the operator named, 21 entries, library crates first:
- the `name` rows of class `crate`: accesskit_xplat, blitz, blitz-dom, blitz-traits, blitz-vibey-script, browser, browser-persistence, counter, debug_timer, escher-driver, escher-telemetry, rdme, seven_guis, todomvc, transparent, wasm_hello, wgpu_texture;
- the crates §Inherited Defaults' rows name: blitz-test-harness, dioxus-native-dom (`decision/coverage-floors`), escher-telemetry, escher-driver, blitz-vibey-script, rdme (`decision/internal-crates-unpublished`), blitz-dom (`decision/fmt-and-clippy-gates`), wpt and blitz-tests (`decision/unpublished-crates` names wpt; blitz-tests by name).

Each purpose is one line, from the crate's manifest `description` and its rows. Six workspace crates neither source names leave the map: blitz-html, blitz-net, blitz-paint, blitz-shell, stylo_taffy and dioxus-native. The pointer table and `.claude/docs/services/` still name their files.

**warnings — re-rendered.**
- Every plan's §Anti-Patterns (Universal) area holds 0 rows: security-plan, design-system, test-plan §11, obs-plan §11 and a11y-plan §11. Each gives «none», and no candidate is drawn for a plan from another section.
- The operator's word confines the block to those rows, so architecture §Cross-cutting Patterns (59 decisions) is read and not drawn. It is listed in the audit below.
- The block reads one «none» bullet, naming the rule files that load on every turn.

Rejected, for the audit:
- **Not drawn — the operator's word:**
  - architecture §Cross-cutting Patterns' candidates: `decision/typed-results`, `decision/targeted-restyle-on-mutation`, `decision/dirty-descendant-invariants`, `decision/interaction-state-cleared-on-removal`, `decision/telemetry-target-prefix-drop`, `decision/escher-telemetry-sink`, `decision/resolve-before-js-reads`, `decision/platform-gating`.
  - §Inherited Defaults' `decision/engine-tracing-gated-per-call-site`.
- **Dropped:** the ten bullets of the old block. Each is now carried by a rule file (security.md: secrets, the one listener, the sink; a11y.md: the a11y target) or by rows of the masters (NodeId, DocumentMutator, incremental oracle, dependency pins, the CI gate). None is a §Anti-Patterns (Universal) row.

**pointer-table — re-rendered.**
- The verbs line is the template's, byte for byte (`get {id}`, the id the command prints).
- A keyed plan's row reads `(keyed: \`rec.py contracts {plan}\`)`, with `, one address per key` on test-plan's row.
- architecture answers `n/a — it carries no keyed-contract section`, so its row names no `rec.py contracts`. Its directory tree is the tree's own, `git ls-files`, as the template spells it on rows.
- Every subsection label a text master held is dropped, because a row has none: «→ Primary screens», «→ Headless stand», «(Invalidation and state integrity)», «(Driver session)», «(Driver session · CLIs)», «(CLI arguments)», «(MCP server)».
- Three paths no longer in the tree become store addresses:
  - `.andromeda/registries/contracts/test-plan/session-lifecycle.md` → `doc:test-plan/contracts/session-lifecycle`;
  - `.andromeda/drift-base.md` · `.andromeda/playbook.md` → the `detector` · `rule` rows, `rec.py find --root . --kind detector|rule`.
- Every other row stands.

**Blocks that stand byte for byte:** workflow · architecture · imports (U01 `ok`) · deeper-topics (recomputed from the files that exist: the same 9 services and 6 rule files, so no change).

## Tier 2 — .claude/rules/
Six files are present, each `preserve`: security.md · host-linux.md · testing.md · observability.md · a11y.md · verification-harness.md. U04 reads `noted`: 9 template lines are missing above `## Session Additions` in host-linux.md. That is a card line, and `regenerate` is the operator's word.

## Tier 3 — .claude/docs/
Every file is present and `preserve`: the 5 core leaves, the 5 summaries, 9 services and session-learnings.md.

## Agent harness
`scripts/agent-run.sh` and `.claude/rules/verification-harness.md` are present and preserved. The host is linux, so no `.ps1` is owed.

## Hooks
`.claude/settings.json`: U02 reads `ok` (write current · bash current · PostToolUse on the stdin prologue). The matrix's render for rust (the rustfmt formatter) is the entry standing, so nothing is written.

## Code-graph pipeline
U03 reads `ok`, 0 lines behind on each of the three files. Each file is present.

## Code reviewer
`.claude/agents/code-reviewer.md` is present and preserved.

## Gitignore · gitattributes · formatter config
U07, U56 and U06 read `ok`, and U05 finds `rustfmt.toml` present. Nothing is written.

## Upgrade
HEAD at Setup 5b: `a1cf8eb3ae83f5c549190b945ebf4b663d6284e3`. The path set at 5b is empty (a clean tree).

```
upgrade v1.25 · aa6ce739
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag (rows)
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts … · scripts/agent-run.sh carries `ensure_fresh_artifacts`
U11 · n/a · hand · code-metrics.ndjson … · past U61 the audit's records are `metric` rows: no ledger file, no terminator
U12 · n/a · hand · .andromeda/residuals.md header · past U61 the residuals are `residual` rows: no file, no header
U13 · ok · hand · .andromeda/{doc}-amendme… · masters in the store — every sidecar entry on the form held at write (U60)
U14 · ok · hand · working-route markerless i… · 14 markerless entries · 0 introducers behind markup or after the em-dash
U35 · ok · hand · masters' lo… · masters in the store — the logs and the keyed contracts out of the body, a key regrown refused held at write (U60)
U36 · ok · noted · .claude/docs/{security,design,tes… · .claude/docs/security-summary.md carries `cascade re-derives it`
U48 · ok · hand · CLAUDE.md p… · masters in the store — CLAUDE.md's pointer rows naming `rec.py contracts {plan}` held at write (U60)
U56 · ok · setup · .gitignore base ignor… · a run's prompts/ ignored by the root .gitignore (.andromeda/runs/*/prompts/)
U57 · ok · hand · the friction family in the store — `.andromeda/db/{moved, log/*.ndjson, reco… · friction: in the store
U58 · ok · hand · the spine in the store — `version` · `epoch` · `chunk` · `carry` · `capability`… · spine: in the store
U59 · ok · hand · a chunk's and a version's documents and files in the store — `doc` · `section` ·… · docs: in the store
U60 · ok · hand · the masters in the store — the seven `.andromeda/{doc}.md` bodies, their cate… · masters: in the store
U61 · ok · hand · the bases in the store — `.andromeda/residuals.md`, `playbook.md`, `drift-base.… · bases: in the store
U62 · ok · hand · the run dirs sealed — every `.andromeda/runs/{run}/` but the sealer's own as `.a… · runs: in the store
U63 · ok · hand · the harness files as the store's render — CLAUDE.md, `.claude/rules/**`, `.cl… · harness: in the store
U65 · ok · hand · the masters as rows — every `section` and `fact` of the seven ma… · masters: 7 master(s) stand as rows
upgrade: for setup 0 · blocks phase 0 · awaiting a door 0 · noted 1 (U04) · INDETERMINATE 0 · 26 detectors of 60 registry entries
```

## Consistency
- Every file written is CLAUDE.md alone (one harness land).
- The four named blocks are re-rendered, and every other block stands.
- No master is written, so this run lands no master TX.
