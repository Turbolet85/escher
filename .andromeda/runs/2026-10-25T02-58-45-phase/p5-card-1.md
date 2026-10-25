# P5 review — 2026-10-25-capability-statement «Capability statement»

**Plan:** `escher-0.2.0/chunks/2026-10-25-capability-statement/plan.md` (458 lines, about 340 of prose; 6 steps; 12
acceptance criteria; 16 gate entries) · **Research:** `escher-0.2.0/chunks/2026-10-25-capability-statement/research.md`
· **Forks:** F1 A and F2 A, the founder's words relayed verbatim (`inputs#I2`)

## Goal
Write one statement of what Escher 0.2.0 can do, its stated limits, its a11y reach and what an MCP client carries
onward. It stands where Viola's 0.2.0 frontend planning will read it: a tracked `docs/capabilities.md`, linked from
README.md, plus an untracked pointer in `viola-0.2.0-incubator/research/escher-capabilities.md` naming its path and
the pushed commit. README's MCP line moves from «eleven» to twelve (CARRY 1). The unpaged acting-verb diff and lint
findings are stated as a limit (CARRY 2). A CI-script pin keeps the statement's verb names and counts equal to the
schema's tables.

## Top acceptance criteria
1. `docs/capabilities.md` holds five sections in order. Its verbs, causes, session errors, node keys and exit statuses
   are read from the tables (12 · 8 · 15 · 13 · 4), and `test_capability_statement.py` reads `Ran 4 tests`, `OK`.
2. README.md says «the twelve driver commands as tools» and links the statement; only that one line changes.
3. The MCP section names the trust boundary (the spawning client holding stdio, no authentication) and the carried set.
   That set includes lint findings and diffs, and no descriptions. The client's onward use is the client's, and the
   cold-agent run is its measured instance, at eleven tools and not re-fired since `lint`.
4. The a11y reach is SC 2.1.1 · 1.4.3 · 2.4.3 only, with the `a11y` leg at 14 · 84 · 0 · 0 (CI#40134371519). The
   assistive-technology reading is unmeasured; its residuals line is `re-carried:0.2.0` and this version doesn't
   reach it.
5. Each stated limit says how it stands: pinned by a named check, asserted by no check, or unmeasured, with its owner.
6. `ci-leg.sh fast` and `doc` exit 0. The pushed sha's CI reads `verdict: green`. Nothing outside the three files
   changes (`guard: clean`).

## Touchpoints
- New: `docs/capabilities.md` · `.github/scripts/test_capability_statement.py`
- Modify: `README.md` (line 48)
- Outside this repository: `../viola/viola-0.2.0-incubator/research/escher-capabilities.md`, untracked (Viola's
  `.gitignore:6`). Its `commit:` line is written by the operator pass after the push.

## Test Commands (16)
- Pin and probes 1–8: each new entry was baseline-run on the untouched tree. The guard, secret-search and README
  entries also ran must-pass and must-fail controls in a scratch clone (`p5-controls.md` in this run dir).
- Gates 9–11: the `fast` leg, its tally, the `doc` leg (each seen green at the last wrap).
- Operator entries 12–16: hygiene, then the pre-push gate and the push, then `ci.py conclusion --sha HEAD --wait 1800`,
  then the pointer's commit line, then the confirmation read (`evidence/operator-confirm.md`).

## Concretization preview — v020-10 «Capabilities stated for a consumer» (`manual`)
- **Now:** Before Viola's frontend planning starts, the operator confirms one statement of what escher can do and its
  stated limits, current with 0.2.0's capabilities, stands where that planning reads it.
- **Proposed:** `docs/capabilities.md` stands at the pushed commit on `build/escher-0.2.0`. It has five sections, its
  counts read from the schema's tables, and its pin `test_capability_statement.py` is green. README.md links it.
  Viola's untracked `viola-0.2.0-incubator/research/escher-capabilities.md` names `docs/capabilities.md` and that
  commit. Before Viola's `viola-0.2.0/` exists (absent today), the operator confirms the statement is current with
  0.2.0 where that planning reads it, recorded in `chunks/2026-10-25-capability-statement/evidence/operator-confirm.md`.
- It is not weaker: the operator's confirmation stays the decisive act, and the place, the pin and the dated record are
  added to it.
- v020-11 (the cold-agent bar) is not claimed. Its working-route entry «Cold-agent bar» owns it.

## The cascade's sampler (forced sample, the operator's word)
- architecture — `sampler: E carried 128 · the cascade lost 13 (cross-cutting-patterns#1.1, cross-cutting-patterns#2.1, design-philosophy#2.1, established-decisions#5.1, inherited-defaults#1.1, inherited-defaults#2.1, inherited-defaults#3.1, occupied-resources#3.1 …)`
- security-plan — `sampler: E carried 77 · the cascade lost 2 (error-handling#2.1, input-validation#1.35)`
- design-system — `sampler: E carried 17 · the cascade lost 2 (motion#4.3, surface#1.2)`
- test-plan — `sampler: E carried 92 · the cascade lost 4 (ci-integration#5.1, quality-gates-coverage-targets#2.1, quality-gates-coverage-targets#3.1, test-strategy#6.1)`
- obs-plan — `sampler: E carried 24 · the cascade lost 2 (ci-integration#1.6, log-coverage#2.1)`
- a11y-plan — `sampler: E carried 56 · the cascade lost 8 (a11y-strategy#3.3, aria-patterns-roles#2.1, aria-patterns-roles#2.5, keyboard-navigation#1.9, keyboard-navigation#2.4, keyboard-navigation#4.6, keyboard-navigation#4.8, visual-design-verification#1.6)`
- All 31 lost items are carried into the plan, marked `(sampler)`. Most land in the statement's content lists; the
  coverage-floor and pre-push-gate items land in the gates (plan.md, Implementation notes).
- One of them, arch `design-philosophy#2.1`, is the entity whose drop the operator accepted at P2 (F343). It is still
  carried, as the no-op-providers limit.

## Flags
- B coverage: 0 extracts read `No domain coverage`, so there is no warning.
- One premise is corrected: the AT-witness owner is `re-carried:0.2.0` (`.andromeda/residuals.md:14`), which overrides
  three extracts' «target: next».
- Six expected amendments go to the wrap. They cover security-plan's MCP-stdio carried list and its «cause set
  (nine)», the masters' «target: next», arch's Front page and Directory structure, and test-plan §4's new CI script.

## Beyond the plan
none
