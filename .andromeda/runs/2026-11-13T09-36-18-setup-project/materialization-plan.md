# Materialization plan — escher · setup-project re-run (structural)

Run: 2026-11-13T09-36-18-setup-project · Development Style: agent-driven · primary language: rust (Cargo.toml) · host: linux · masters: rows

Structural change named at invocation: "the masters stand as rows (U65): a structural change in what setup derives from
them — re-render CLAUDE.md's overview · modules · warnings · pointer-table blocks from the rows: the verbs line as
claude-md-template.md spells it (get {id}); the module map from the workspace's packages and the name rows of class
crate, every one listed; the warnings as Phase 0 step 2 now reads them; the overview from overview/project-intent and
the versions from the manifest (Cargo.toml · rust-toolchain.toml), never from a row."

## Tier 1 — CLAUDE.md
- overview — paragraph = row `ovw-squxl57t` (overview/project-intent), standing; stack line's versions checked against
  `Cargo.toml` (0.3.0-beta.4 · edition 2024 · rust-version 1.91.0 · stylo 0.22.0 · taffy 0.15 · parley 0.12 ·
  anyrender 0.14 · vello 0.11 · winit =0.31.0-beta.3 · accesskit 0.25 · dioxus 0.7.3 · boa_engine 0.22 · tokio 1.42 ·
  reqwest 0.13) and `apps/browser/persistence/Cargo.toml` (rusqlite 0.32); no `rust-toolchain.toml` — every version
  agrees, the block stands byte for byte.
- modules — `cargo metadata --no-deps --offline` workspace_members (28, root `blitz-examples` included) ∪ the 17
  `name/crate/*` rows of doc:architecture: 28 entries, every one listed, libraries first. Added: `stylo_taffy`
  (ctr-z2b335z6) · `blitz-html` (ctr-yriabxml) · `blitz-net` (dec-qrst54im · ctr-yrd7nxmj) · `blitz-paint`
  (ctr-yqkw7r4g · ctr-ypako22f · ctr-yqjru6bf) · `blitz-shell` (dec-vu6xxheq · ctr-zjvdcre4) · `dioxus-native`
  (ctr-yb4itd3s · dec-qtsrmtgc) · `blitz-examples` (bare — no row names it). The 21 others stand.
- warnings — every plan's §Anti-Patterns (Universal) holds 0 rows → drawn from the guard sections and arch
  §Cross-cutting Patterns · §Conventions · §Design Philosophy (subagent read, rows re-checked by `rec.py get`):
  1. secrets only in Actions secrets/vars — dec-guzjzacp (+ security.md's never-commit line)
  2. one listener; new socket/port/IPC/credential path is an amendment first — dec-kap6zusi
  3. no panic path on untrusted input — dec-gj6pr3d7
  4. NodeId versioned; a stale id resolves to none, never a panic — dec-ogzi5gw6 (§Design Philosophy) ·
     dec-kls7eojr (security §Error Handling) [operator's review]
  5. no content in a log field — dec-gz6acavz
  6. `--locked` builds — dec-kuxsgya2
  7. no git dependency — dec-jm3wczan
  8. incremental oracle — dec-ufjhi4vc (§Design Philosophy) · dec-dxo77rw2 (test §2) [operator's review]
  9. DOM mutations through DocumentMutator — dec-xmlwfni7 (§Conventions) [operator's review]
  10. unwrap/expect allowed only locally, justified — dec-vlchwzsx
  Operator's review, not drawn: «get/contains_key, never indexed» — no row in those sections states it (the words
  stand only in dec-kls7eojr's evidence quotes); «dependency pins are coupled, never bump one side alone» — no row in
  those sections: dec-jlrj566g (security-plan §Dependency Security) and dec-vu7h54n4 (arch §Project Intent) state the
  pins stay, dec-odccjf3r (arch §Established Decisions) winit's exact pin alone; «coupled winit pin» stands only in
  .claude/rules/security.md, a harness leaf.
  Rejected (audit): typed results (dec-w6tgqg2v) · workspace dependencies (dec-nxphwkzn) — displaced by the cap of 10;
  unsafe annotated (dec-n4rcb6ju — verified `stale`); per-call tracing gating (dec-xpav2biu —
  engine crates only); deny.toml ignore-by-ID (dec-gixamm4o — in security.md, dependency-scoped); both layout modes ·
  one test target per file · stand bundled font (tests only); closed refusal causes · refusal holds no call input ·
  MCP no print or panic (escher-driver only); CI signing material ·
  publish secrets upstream-only (CI yaml only); driver reads only RUST_LOG (crate-scoped, in security.md); cold-agent
  operator login (pipe only, history); mutex poison recovery (narrow convention); coverage floors · stand-repeat
  budget (CI gates); recursion caps · font-path routing (in security.md); msrv policy (policy); tracing as telemetry
  (stack fact). The a11y and obs plans' rows are all crate- or path-scoped: none qualifies.
- pointer-table — verbs line byte-equal to the template; `rec.py contracts`: architecture n/a, test-plan 2 keys,
  obs-plan 1, a11y-plan 1 — every row's keyed clause agrees; 24 rows; stands byte for byte.
- workflow · architecture · imports — cascade-maintained, standing (the words name no change there).
- deeper-topics — recomputed from the `.claude/docs/` and `.claude/rules/` files that exist: unchanged.

## Tier 2 — .claude/rules/ (preserve)
security.md · host-linux.md · testing.md · observability.md · a11y.md · verification-harness.md — preserve all.

## Tier 3 — .claude/docs/ (preserve)
5 core · 5 summaries · services/ (9) · session-learnings.md — preserve all.

## Agent harness
scripts/agent-run.sh — preserve (not compared: the words name no 5-command change).

## Hooks · Gitignore · Code-graph pipeline · Code reviewer
Per §Upgrade: U02 · U05 · U06 · U07 · U56 · U03 ok — nothing to write. Reviewer: preserve.

## Upgrade
HEAD 22e7639824dcd58b3162e140bbcb0bc4f380dded · 5b path set: ` M .claude/session-handoff.md`

```
upgrade v1.25 · b0707b84
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
U14 · ok · hand · working-route markerless i… · 16 markerless entries · 0 introducers behind markup or after the em-dash
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
