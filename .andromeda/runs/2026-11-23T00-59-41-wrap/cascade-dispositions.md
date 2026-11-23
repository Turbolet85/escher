# Cascade dispositions — 2026-11-22-upstream-sync-ahead-of-the-canvas

## Step 2 — the sweep
- The landings: architecture TX 3219 (`ops-architecture.md`: 1 set, 2 adds) and TX 3220 (`ops-main-architecture.md`:
  1 add). Each dry run wrote no patterns file — `patterns: none: 1 set(s) and retire(s) changed prose alone — no file`
  (TX 3219, the `stylo-taffy-exports` promise lengthened, its old words kept) and `patterns: none — the call sets and
  retires no row` (TX 3220). No other master landed: security-plan, design-system and a11y-plan dry runs landed 0;
  layout-templates and obs-plan `@none`; test-plan two `@row_none`.
- Main's own patterns, `cascade-patterns.toml` (5): `main-stylo` (the amended contract's own terms:
  `to_taffy_style|convert::(float|clear)|TaffyStyloStyle`) · `main-takes` (the new rows' why: `ratified for its
  take|upstream behaviour change`) · `main-cmdtests` (`refusal schema|command_tests` — CARRY `cry-yojyfzsj`'s first
  line, the driver-schema pointer row; first drafted `command_tests|unit tests inline|…`, refused: control unfired
  over the seven masters at 31c8155f, re-pointed) · `main-keptwhole` (`kept whole` — the CARRY's second subject) ·
  `main-kbcompat` (`keyboard_compat` — the Dioxus hold's coverage exclusion, and the CARRY's second line). The first
  firing without `main-kbcompat` (`sweep-pass-1.1.txt`, 40 rows) is superseded by the second, the one `leaves` read.
- The listing read: `sweep-pass-2.1.txt`, read whole. `total (5 patterns) · 49 rows over 13 files` · `per class · new
  3/1 · standing 21/3 · leaf 25/10 · curation 0/0 · base 0/0`.
- Dispositions, per pattern and class:
  - `main-stylo` — standing 1: `ctr-z2b335z6` amended (TX 3219; `edited ×2`: its old export list kept as true, the
    converters' `ltr` flag added) · leaf 1: `CLAUDE.md:23` — the comparer's `@leaf_stands` (a one-line module summary
    the added argument does not falsify), accepted.
  - `main-cmdtests` — standing 2: `ctr-yimmfwol`, `ctr-yic3jmzu` no change (the one command and refusal schema, true) ·
    leaf 6: `CLAUDE.md:92` re-derived (finding `stale`, applied — the CARRY line); `CLAUDE.md:122`,
    `obs-summary.md:49`, `escher-driver.md:6`, `:51`, `:60` — `@leaf_stands`, accepted.
  - `main-keptwhole` — standing 7: `ctr-sfdr5vbz`, `ctr-s5aqwuyg` (architecture), `dec-nh5z5mxh` (test-plan),
    `ctr-7z65jeeg`, `ctr-5oi2v26e`, `ctr-nepx5i6r`, `ctr-6hw5uwhy` (obs-plan) no change (a kept check's answer kept
    whole; the oversize targets kept whole — all true) · leaf 12: each `@leaf_stands`, accepted.
  - `main-takes` — new 3: `dec-bb2kr7mh`, `dec-dc2mrix6`, `dec-xexhbk5x` (this pass's adds) · standing 7: the earlier
    takes' rows and `dec-sjvqc3hf` no change (true; the three adds follow its rule) · leaf 1: `blitz-dom.md:15` —
    `@leaf_stands`, accepted.
  - `main-kbcompat` — standing 4: `dec-rm2volpl` (architecture), `dec-bto66npw`, `dec-6jxz67n2`, `dec-apon7s4t`
    (test-plan) no change (the exclusion stands while Dioxus is held at 0.7.3, the founder's D1 A) · leaf 5:
    `testing.md:48` re-derived (finding `omission`, applied — the CARRY line); `commands.md:53`, `obs-summary.md:32`,
    `dioxus-native-dom.md:38`, `tests-summary.md:42` — `@leaf_stands`, accepted.
- Not looked for: the old wording of the security-plan, design-system and a11y-plan rejected adds (nothing landed
  there); a curation home or a judgment base — the sweep's `curation 0/0 · base 0/0`.
- The binds: no plan landed, so the test-plan §3 ↔ obs-plan §3 / §6 and a11y ↔ log-schema binds are untouched.

## Leaves
`cascade.py leaves --run-dir … --add CLAUDE.md --add .claude/rules/testing.md` (the pass mode; `epoch · «Epoch 3 — The
canvas painted» · markerless 4 · gated 0 · open`; `sweep leaves · 10 file(s) from sweep-pass-2.1.txt · 5 outside every
group`; `total · pass · 11 group(s) · 20 leaves · 634439 B · unmapped 1`). The two `--add`s carry CARRY `cry-yojyfzsj`
(the operator's directive, inputs#I5 point 4). `unmapped .claude/rules/host-linux.md (host)` — the host's own leaf, no
master source: no comparer, no step-3 edit.

| group | mode | sweep lines | answered | findings | applied | by hand | declined |
|---|---|---|---|---|---|---|---|
| architecture-1 | pass | 2 | 2 `@leaf_stands` | 0 | 0 | 0 | 0 |
| architecture-2 | pass | 1 | 1 `@leaf_stands` | 3 (omission, `blitz-dom.md:21` ×3 — E3/E4, E6, E2) | 3 | 0 | 0 |
| architecture-3 | pass | 1 | 1 `@leaf_stands` | 0 | 0 | 0 | 0 |
| architecture-4 | pass | 6 | 6 `@leaf_stands` | 0 | 0 | 0 | 0 |
| architecture-5 | pass | 0 | — | 0 | 0 | 0 | 0 |
| architecture-6 | pass | 4 | first run 4 `@leaf_stands` — `CLAUDE.md:92`'s read false (its because «no architecture row names the driver's test files»; `dec-rlfpyplu` does), RULED re-spawn `leaf-r1`; then 1 finding (`CLAUDE.md:92`, stale) + 3 `@leaf_stands` | 1 | 1 | 0 | 0 |
| added-1 | pass | 5 | 5 `@leaf_stands` | 0 | 0 | 0 | 0 |
| added-2 | pass | 2 | 2 `@leaf_stands` | 0 | 0 | 0 | 0 |
| added-3 | pass | 1 | 1 `@leaf_stands` | 0 | 0 | 0 | 0 |
| added-4 | pass | 2 | 2 `@leaf_stands` | 0 | 0 | 0 | 0 |
| added-5 | pass | 1 | first run 1 `@leaf_stands` — answered the `keyboard_compat` clause alone, the kept-whole omission against `dec-nh5z5mxh` unread, RULED re-spawn `leaf-r1`; then 1 finding (`testing.md:48`, omission) | 1 | 1 | 0 | 0 |

`cascade.py leaves --apply … --only` (5 ids): `total · blocks 5 · applied 5` — `.claude/rules/testing.md` 1,
`.claude/docs/services/blitz-dom.md` 3, `CLAUDE.md` 1, each read back. CARRY `cry-yojyfzsj`: both lines re-derived by a
comparer — `CLAUDE.md:92` (command's unit tests in `command_tests.rs`, `#[path]` child, `command.rs:335`; refusal's and
describe's inline, `refusal.rs:206`, `describe.rs:98`) and `testing.md:48` (escher's oversize integration-test targets
kept whole) — none fixed by hand; the CARRY is spent.
