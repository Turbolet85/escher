# Cascade dispositions — 2026-10-05-as-built-baseline

Amendments of the pass: O1 (architecture — the rustdoc "doc gate" claim, 5 sites) · O2 (architecture — Build environment
dev-host reading) · O3 (test-plan §9 — local baseline bullet). Sweep: `cascade.py sweep --patterns-file
cascade-patterns.toml` (cascade v1.1, baseline `eaca28ba`), run after every body edit. Every pattern's control fired on
the pre-pass masters.

## The search
- O1, names and mechanism: `rustdoc` (i) · `docs? gates?` (i) · `RUSTDOCFLAGS` · `warnings (are|as) errors` (i — the
  pre-pass phrasing at architecture.md:104; 0 rows post-pass, control fired) · `docs job|, docs,|Documentation job` (i —
  the job named in a list). Sections read whole: architecture §Stack and Technologies, §Conventions (Formatting and
  lints), §Infrastructure Patterns (Build system, CI/CD), §Inherited Defaults.
- O2: `1\.90` · `libfontconfig` · `Build environment` (i).
- O3 (extension, retires nothing): `CI Integration` (i) — finds leaves citing the section.
- NOT looked for: the CLAUDE.md done-gate rule's own wording ("Work is not done until") — read directly at CLAUDE.md:49.

## Rows
Masters:
- architecture.md:65 (flake190, buildenv, fontconfig) — amended (O2).
- architecture.md:67 (rustdoc ×2, docgate, rdflags, docsjob) — amended (O1).
- architecture.md:104 (rustdoc, docsjob) — amended (O1); intra-line re-read: "rustdoc warnings are errors" no longer stands.
- architecture.md:155 (rustdoc, docsjob) — amended (O1).
- architecture.md:174 (rustdoc, docgate, docsjob ×2, fontconfig) — amended (O1); `libfontconfig1-dev` is CI's apt package, a true claim, no change.
- architecture.md:217 (rustdoc, docsjob) — amended (O1).
- test-plan.md:64 (rustdoc) — no change: a rustdoc usage example for `DioxusDocument`, a true claim sharing the token.
- test-plan.md:277 (rustdoc, rdflags) — this pass's own text (O3).
- test-plan.md:262 · obs-plan.md:275 · a11y-plan.md:319 (ci-integ) — no change: §9 headings.
- security-plan.md:221 · a11y-plan.md:77 (ci-integ) — no change: other subjects (audit step absent; a11y checks absent).
- registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:3 · registries/contracts/a11y-plan/…:4
  (ci-integ) — no change: pointers to §9 for coverage tooling / a11y CI gate, both still true.

Leaves (re-derived from the amended masters):
- CLAUDE.md:49 (GENERATED:setup:warnings) — re-derived: the done-gate names the workspace rustdoc form and its baseline
  red owned by "CI gate legs".
- CLAUDE.md:81 (GENERATED:setup:workflow) — re-derived: Arch stand-in added beside `libfontconfig1-dev`.
- .claude/docs/commands.md:6 — re-derived: Arch dev-host line added after the CI apt line (kept, true).
- .claude/docs/commands.md:35 — re-derived: was "`RUSTDOCFLAGS="-D warnings" cargo doc` — the docs gate" (stale: documents
  no library crate); now the `--workspace --no-deps` form, its baseline red, and the bare form's reach.
- .claude/docs/stack.md:8 — re-derived: flake pin below MSRV + measured dev-host line.
- .claude/docs/stack.md:38-39 — re-derived: the rustdoc setting's reach; the `docs` job marked root-package-only.
- .claude/docs/tests-summary.md:36 — re-derived: gate row carries the workspace rustdoc form, its baseline red, owner;
  plus a `Coverage as built` baseline line (test-plan §9 is amended — O3).
- Provenance enumeration (`Extracted from|Source:|Derived from` over .claude/docs and .claude/rules): conventions.md
  (§Conventions — the amended clause is rustdoc; the leaf never carried a rustdoc line, recompute adds nothing) ·
  workflow.md (:27 lists fmt/clippy/targeted tests as the chunk gates — no rustdoc claim, no change) · gotchas.md (no
  amended section) · rules/testing.md (test-plan leaf; :39 "Whole workspace (CI leg)" true; the baseline figures live in
  tests-summary, the rule file stays lean — no change) · rules/{a11y,security,observability,verification-harness}.md
  (sources not amended).

Curation homes: 0 rows. Judgment bases (playbook, drift-base): 0 rows.
