# Operator pass — entries 16, 17, 18 (2026-10-16)

Fired in this order on the operator's directive. The order: hygiene, then the pre-CI commit, then fast, clean-tree and
push, then the CI read.

## Entry 16 — `gate.py hygiene`, fired as written

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, from the repository root, before the
commit:

```
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 65 (runs 57 · evidence 4 · inputs 4) · trails 12 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Exit 0. `contains hygiene: clean` holds: green.

## The pre-CI commit

`c71f7ed14d5ba8c0e9ffa3c66cb91747688861e2` — `chore(2026-10-16-handler-splits-and-readme): operator pre-CI commit, for
the run this chunk verdict reads`. The whole tree was staged (77 files), on `build/escher-0.2.0` over
`d236e9186e4ff6ca2610ddfbc9f54f590e8dd01b`, and the tree was clean after it.

## Entry 17 — fast, clean-tree guard, push

`gate.py run --plan {chunk_dir}/plan.md --operator 17`, the operator pass's form, with no run dir. Its line and the
call's summary:

```
operator entry 17 · history tripwire: git
 17 probe       green · exit 0 · 79.83s · 99286 B → 17.log · history moved: refs/remotes/origin/build/escher-0.2.0 d236e918→c71f7ed1 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 18 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 17
```

The history move is the push the entry exists for. After it, `origin/build/escher-0.2.0` reads `c71f7ed1`, equal to
HEAD.

## Entry 18 — `ci.py conclusion`, fired as written

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`:

```
repo Turbolet85/escher (the push remote `origin`) · polled 30× over 901 s
c71f7ed14d5b verdict: green · checks 17/17 · wall 897 s · runs CI#38917429198 completed/success
runs: CI#38917429198 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

Exit 0. `contains verdict: green` holds: green. The run is CI#38917429198 on `c71f7ed1`, 17/17 checks, and `Stand
checks repeated` (the stand-repeat leg) is among them.
