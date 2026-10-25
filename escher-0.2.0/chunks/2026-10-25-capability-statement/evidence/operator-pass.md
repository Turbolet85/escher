# Operator pass — 2026-10-25-capability-statement

Fired by hand, in plan order, on the operator's direction (2026-10-25): entry 12, the pre-CI commit, entry 13, entry 14,
then entry 15 on a green CI read; stopping at any red.

## Entry 12 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (fired as written, before the pre-CI commit)

Read 2026-10-25 at HEAD `e2eb01b44`: **red** — `expect` asks `contains hygiene: clean`.

```
gate v1.22 · e6ccbfe7
root . · case exact · planes rust · base HEAD (no --marker)
P1 .andromeda/runs/2026-10-25T02-58-45-phase/p5-controls.md:15 ×1 · home
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 485 (runs 478 · evidence 3 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The one hit is the phase run's P5 control table, row «4 secret search»: its must-fail column names the planted
fake home path the control put into a scratch copy of `docs/capabilities.md` (a `/home/` path under the fake user
name `someone`, not this host's user). No real host path. The pass stopped here, before the pre-CI commit: no
commit, no push, no CI read, the pointer's `commit:` line unchanged.

Respelled on the operator's word (2026-10-25, option 1): `.andromeda/runs/2026-10-25T02-58-45-phase/p5-controls.md:15`'s
must-fail cell now names the planted path in words («a home-directory path under the made-up user someone») instead of
spelling it, so hygiene's P1 home predicate no longer reads it; nothing else in that run dir was touched.

Re-fired as written after the respell: **green** —

```
hygiene: clean — read 486 (runs 478 · evidence 4 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

Subject exactly `chore(2026-10-25-capability-statement): operator pre-CI commit, for the run this chunk's verdict
reads`, every change in the tree (`git add -A`); its sha is the one entry 13 pushes and entry 14 reads.

Committed: `e7e71221d` (497 files), the tree clean after it.

## Entry 13 — the fast leg, the clean-tree guard and the push (`gate.py run --plan … --operator 13`)

```
operator entry 13 · history tripwire: git
 13 probe       green · exit 0 · 103.15s · 114147 B → 13.log · history moved: refs/remotes/origin/build/escher-0.2.0 e2eb01b4→e7e71221 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```

The history move is the intended one: the push of `e7e71221d` to `origin/build/escher-0.2.0`.

## Entry 14 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` (fired as written)

```
ci v1.3 · e46ce3dd
repo Turbolet85/escher (the push remote `origin`) · polled 40× over 1206 s
e7e71221dfdc verdict: green · checks 17/17 · wall 1191 s · runs CI#40184674449 completed/success
runs: CI#40184674449 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

## Entry 15 — the pointer's commit line (`gate.py run --plan … --operator 15`, after entry 14 read green)

HEAD `e7e71221dfdcd29c6df603d222104855f43c07f4`.

```
operator entry 15 · history tripwire: none of 7 forms matched — not a clearance
 15 probe       green · exit 0 · 0.01s · 2 B → 15.log · history: unmoved · sha=$(git rev-parse HEAD) && sed -i "s/^commit: .*/commit:… (219 chars)
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```

The pointer `../viola/viola-0.2.0-incubator/research/escher-capabilities.md` now reads
`commit: e7e71221dfdcd29c6df603d222104855f43c07f4`, copied into `evidence/viola-pointer.md`.

The pass stops here, after entry 15, on the operator's direction. Entry 16 reads `evidence/operator-confirm.md`, which
is the operator's own (step 6) and is not written by this pass.

## Entry 16 — the operator's confirmation (`gate.py run --plan … --operator 16`, after the operator wrote step 6)

`evidence/operator-confirm.md` written by the escher overseer on the founder's «Подтверждаю (Рекомендую)», naming
`e7e71221dfdcd29c6df603d222104855f43c07f4`, 2026-10-25, no corrections asked.

```
operator entry 16 · history tripwire: none of 7 forms matched — not a clearance
 16 probe       green · exit 0 · 0.0s · 2 B → 16.log · history: unmoved · grep -c -E "^confirmed: " escher-0.2.0/chunks/2026-10-25-c… (106 chars)
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```
