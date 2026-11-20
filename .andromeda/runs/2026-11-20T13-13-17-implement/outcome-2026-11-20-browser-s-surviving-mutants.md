# Outcome — 2026-11-20-browser-s-surviving-mutants

**Outcome:** green · run `2026-11-20T13-13-17-implement` · 0 fix iterations (the first block run read green)

## Files changed
- `apps/browser/src/capture.rs` — the three declaration lines `#[cfg(test)]` · `#[path = "capture_tests.rs"]` · `mod tests;` appended after `try_get_save_path`, behind one blank line (added 123-126, `cites.py added`); lines 1-122 unchanged.
- `apps/browser/src/capture_tests.rs` — new, 57 lines: `make_doc` and the two tests `render_scene_returns_the_viewport_size` (800 × 600 at 1.0 → `(800, 600)`) and `render_scene_returns_the_physical_size_at_a_hidpi_scale` (1280 × 720 at 2.0 → `(1280, 720)`). Each asserts its fixture first, then calls `render_scene` into `anyrender::NullScenePainter`.

## Gates (`gate.py run`, trail `.andromeda/runs/2026-11-20T13-13-17-implement/gate-2026-11-20-browser-s-surviving-mutants.json`)
```
  1 unit        green · exit 0 · 29.48s · 11167 B → 1.log · cargo test -p browser --locked -- capture::tests
  2 unit        green · exit 0 · 586.62s · 164717 B → 2.log · bash .github/scripts/ci-leg.sh fast
  3 probe       green · exit 0 · 0.03s · 2 B → 3.log · test -f apps/browser/src/capture_tests.rs && git diff --nu… (168 chars)
  4 probe       green · exit 0 · 0.0s · 2 B → 4.log · git diff -U0 --src-prefix=a/ --dst-prefix=b/ 80c31730e59b9… (355 chars)
  5 probe       green · exit 0 · 0.94s · 381 B → 5.log · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ga… (262 chars)
  6-10 probe    not run — leg operator (the letter drives it)
entries 10 · green 5 · red 0 · recorded 0 · timeout 0 · not-run 5
```

Entry 5's guard pass line (`gate.py show … --n 5`), whole:
```
guard: allowed 2 — apps/browser/src/capture.rs, apps/browser/src/capture_tests.rs · companions excluded 0 · changed 4 · excluded 2
guard: clean
```
The two excluded files are the pipeline's own uncommitted artifacts, outside the guard's subject by construction. They were already modified in the tree at session start by the preceding wrap and phase: `.claude/session-handoff.md` (the session-end writer) and `escher-0.4.0/working-route.md` (the route). This run edited neither.

## Hand controls (`gate.py control`, `evidence/controls.md`)
```
control c00 · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 37.56s · 1.38s → 1.without-c00.log
control c01 · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 37.56s · 1.33s → 1.without-c01.log
control c10 · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 37.56s · 1.42s → 1.without-c10.log
control c11 · entry 1 · holds — in place green, removed red · exit 0 ✗ (exit 101) · 37.56s · 1.33s → 1.without-c11.log
control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed
```

## Smoke
Skipped: no boot-path or UI-surface change. The change is test-only, and the plan lists no smoke or self-verify entry.

## Matrix
No ref written. The chunk claims no capability (P5 `matrix.py audit`: `claimed by … 0`). v040-08 stays `planned, unclaimed`, carried to «Dependency and CI debts» (p5-card-1/2, inputs#I2).

## Deviations
None. `gate.py scope`: `scope: clean — changed 2 · listed 2 · recorded 0`. The scope record holds no line.

## Surfaced / stuck
Nothing.

## Operator pass (next)
Entries 6 (hygiene), 7 (push), 8 (CI read) belong to the operator pass. Entries 9 and 10 (the cargo-mutants re-run) are fired only on the founder's word (inputs#I1). Without it they are recorded `not run — no word`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc / browser test binaries (entries 1, 2 and the control firings) | this run | terminated — `gate.py census --comm cargo --comm rustc --comm browser --args andromeda-control`: `census: none running` |
| control clone `~/.cache/andromeda-control/7a99a39e` | this run (`gate.py control`) | removed (`clone removed`) |

## Decisions & corrections
- Guard refusal: a `cd` into the skill's references directory was refused by the Bash guard (`cd … moves the session cwd`). It was re-issued once as Read calls by absolute path.
