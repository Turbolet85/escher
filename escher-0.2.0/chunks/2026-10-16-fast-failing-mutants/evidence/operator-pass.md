# Operator pass — 2026-10-16-fast-failing-mutants

The operator's directive (this session, 2026-10-16): "Operator pass for 2026-10-16-fast-failing-mutants (G3 deployed,
go): fire entry 23 (gate.py hygiene) as written; then commit "chore(2026-10-16-fast-failing-mutants): operator pre-CI
commit, for the run this chunk verdict reads"; then entry 24 via gate.py run --operator 24 in the background; then
entry 25 (ci.py conclusion --wait) as written. Record each in evidence/operator-pass.md. Start no wrap; stop after
entry 25 and report the sha and verdict."

## Entry 23 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, fired as written

```
gate v1.19 · bebab50b
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 60 (runs 51 · evidence 5 · inputs 4) · trails 21 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

`hygiene: clean`, so the entry's `contains hygiene: clean` atom holds. The tool's stamp reads `bebab50b`, where the
/implement run's `gate.py run` lines read `457c1da1`: the tools were redeployed in between ("G3 deployed").

## The pre-CI commit

`git add -A`, the same shape as the previous pre-CI commit `54d4ea21`, then the directive's message:
`79cfb5d1fc99031b8b40ef8e1b06eb32fd173ae4 chore(2026-10-16-fast-failing-mutants): operator pre-CI commit, for the run
this chunk verdict reads`. After it, `git status --short` read 0 rows.

## Entry 24 — `gate.py run --plan escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/plan.md --operator 24`

The operator-pass form: no run dir, fired in the background.

```
operator entry 24 · history tripwire: git
 24 probe       green · exit 0 · 84.35s · 99543 B → 24.log · history moved: refs/remotes/origin/build/escher-0.2.0 494abd25→79cfb5d1 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

From its log: the fast leg passed, then the clean-tree guard, then the push
(`494abd258..79cfb5d1f  build/escher-0.2.0 -> build/escher-0.2.0` to `github.com/Turbolet85/escher`). The history move
is the push the entry exists for.

## Entry 25 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`, fired as written

```
ci v1.3 · b795b622
repo Turbolet85/escher (the push remote `origin`) · polled 12× over 342 s
79cfb5d1fc99 verdict: red · checks 17/17 · first-fail +338 s Dependency audit · runs CI#39013904578 in_progress/-
runs: CI#39013904578 push in_progress/-
failed 1: Dependency audit (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/escher/actions/jobs/117342884166/logs
running 12: oldest Documentation 35 s
run open: CI#39013904578 in_progress
```

**Verdict: red.** The entry's `contains verdict: green` atom fails. The red is the `Dependency audit` job (cargo-deny),
which ends `advisories FAILED, licenses ok`. Three advisories fire on `zip 2.4.2` (`Cargo.lock:912`), and each says
"Upgrade to >=9.0.0":
- RUSTSEC-2026-0367, SUID/SGID bits preserved during ZIP extraction;
- RUSTSEC-2026-0368, `ZipArchive::extract` symlinks with targets outside the destination;
- RUSTSEC-2026-0369, Unix permissions applied through symlinks.

The path is `zip v2.4.2 ← anyrender_serialize v0.8.0 ← browser v0.0.0` (the upstream reference browser app).

Read at the poll's end (`gh run view 39013904578`): Test [default features], Test CI scripts, Clippy and Rustfmt had
completed with success, and 12 jobs were still running.

This chunk did not introduce the red. `git diff --quiet 494abd25 HEAD -- Cargo.lock Cargo.toml deny.toml` reads exit 0,
so the dependency graph and the audit config are byte-identical to the base, whose own CI run read green (CI#38943566949 on `494abd25`, `success`, 2026-10-16T17:56Z). The
advisories are new against an unchanged lockfile. The fix the advisories name (zip ≥ 9.0.0) is a semver-major move
under an upstream crate (`anyrender_serialize`). It is not a `cargo update` inside this chunk's scope, so it is left
for the operator and the wrap.

## The founder's word, and the fix push

The founder's word, 07:39Z (2026-10-17), by dialog, as relayed by the operator: «Игнор по ID (Рекомендую)».

The operator's directive with it, verbatim: "Fix push: add RUSTSEC-2026-0367, -0368, -0369 to deny.toml [advisories]
ignore, per-ID, reason in the existing form: zip 2.4.2 via anyrender_serialize 0.8.0 (apps/browser capture feature
only), which reads entries by_name into memory and never extracts to disk; no fixed release reachable (0.8.0 latest,
zip ^2.1; fix in 9.0); re-checked by the audit leg on every push. Run ci-leg.sh audit locally (green), record the word
in operator-pass.md, commit "fix(deps): ignore the zip extraction advisories by ID", re-fire entry 24 then 25.
Residuals + incubator lines ride the wrap P1 line. Stop after 25."

- **`deny.toml`:** three per-ID entries appended to `[advisories] ignore`, after the two existing ones and in their
  form. Each reason reads: zip 2.4.2 reaches the graph only through anyrender_serialize 0.8.0 (apps/browser capture
  feature only), which reads entries by_name into memory and never extracts to disk; no fixed release is reachable
  (anyrender_serialize 0.8.0 is the latest, requiring zip ^2.1; the fix is in zip 9.0); re-checked by the audit leg on
  every push.
- **`bash .github/scripts/ci-leg.sh audit`, run locally:** exit 0, and `target/ci-logs/audit.log` ends
  `advisories ok, licenses ok`. Its one warning is the standing `license-not-encountered` on `NCSA`. No
  `advisory-not-detected` warning fired for the three new ignores, so the local advisory database holds all three IDs
  and the green is not a stale database's.
- **Left for the wrap, on the directive:** the residuals line and the incubator `.gitignore` line ride the wrap's P1
  line, not this fix push.
- **The fix commit:** `f072cd615c33928295a97a0b7cc4091cdd40c537 fix(deps): ignore the zip extraction advisories by
  ID`. It carries `deny.toml` and this file. After it, `git status --short` read 0 rows.

## Entry 24, re-fired — `gate.py run --plan escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/plan.md --operator 24`

```
operator entry 24 · history tripwire: git
 24 probe       green · exit 0 · 79.28s · 99543 B → 24.log · history moved: refs/remotes/origin/build/escher-0.2.0 79cfb5d1→f072cd61 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```

## Entry 25, re-fired — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`, as written

```
ci v1.3 · b795b622
repo Turbolet85/escher (the push remote `origin`) · polled 32× over 960 s
f072cd615c33 verdict: green · checks 17/17 · wall 888 s · runs CI#39014589486 completed/success
runs: CI#39014589486 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

**Verdict: green.** The entry's `contains verdict: green` atom holds, and all 17 checks passed, including the
Dependency audit, the Coverage report and Stand checks repeated.
