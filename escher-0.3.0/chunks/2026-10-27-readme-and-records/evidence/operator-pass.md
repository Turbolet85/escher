# Operator pass — 2026-10-27-readme-and-records

Fired on the operator's word in session, 2026-10-27, in the plan's order. Entry 14 (the gated front-page read) stays
deferred until the founder answers F3; it was not voided.

## Entry 15 — hygiene (fired as written)

First firing, refused:

```text
P1 .andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh:52 ×1 · home
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 523 (runs 515 · evidence 2 · inputs 6) · trails 16 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The hit was phase P5's planted control literal, a home-directory path of a made-up user `someone` (entry 7's must-fail
control). On the operator's dialog answer «Placeholder (Recommended)» — «Operator: same as the chunk-4 precedent
(p5-controls planted [that literal]). Respell the literal so hygiene stops matching, keep the control's meaning, name
the change in the commit body, and re-run hygiene.» — its user name was rewritten to the placeholder `<user>` (entry
7's `/home/` probe still reads the control red). Re-fired:

```text
hygiene: clean — read 523 (runs 515 · evidence 2 · inputs 6) · trails 16 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`642561729e7cee5d69c93e2a8533a9873770cb1f` — `chore(2026-10-27-readme-and-records): operator pre-CI commit, for the run
this chunk's verdict reads`; explicit paths, no force; the rewrite named in its body.

## Entry 16 — fast, the clean-tree guard, the push (`gate.py run --operator 16`)

```text
 16 probe       green · exit 0 · 111.06s · 115385 B → 16.log · history moved: refs/remotes/origin/build/escher-0.3.0 316c6d25→64256172 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
```

## Entry 17 — CI conclusion (fired as written)

```text
642561729e7c verdict: green · checks 17/17 · wall 1145 s · runs CI#40456250076 completed/success
```
