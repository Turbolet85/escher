# P5 review — 2026-10-24-upstream-sync-polish-and-ship (card 1)

**Chunk:** «Upstream sync ahead of polish and ship» · escher-0.2.0 · Epoch 5's first entry · master record `pending`
**Plan:** `escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/plan.md` (462 lines, about 210 of prose) ·
research `research.md` · scope `scope.md` (closed at P3: 7 verified, 4 corrected, 1 closed)

**P3 resumed on overseer1's ruling.** Setup 3 (c) would have re-run P2 whole. On the take-up rule for this one case,
overseer1 ruled that P3 resume on P2's validated products in `.andromeda/runs/2026-10-24T09-31-30-phase/` (inputs#I1 is the
run mode). The forced sample stands, and the forced-sample word stands for the wrap. No hard cascade failure came up
in P3–P5.

## The take — the founder's own word
**The fork:** «Десять коммитов (Рекомендую)», the founder's own word, relayed verbatim by the overseer (inputs#I4). It is
the builder's option «Cherry-pick the ten (Recommended)». It ratifies the take, so nothing in it is PROVISIONAL (the playbook's
«An engine behaviour change for every document»).

**What is taken:** ten upstream commits by no-commit cherry-pick, in upstream order: `28f848d3b` · `9eeebb302` ·
`e4ca5b9b5` · `46d7cf993` · `fb297a38c` · `50bf2f235` · `f8723f495` · `21edc92d9` · `f102663b8` · `f9cbb2756`. Then
`cargo update -p tokio`, from 1.53.2 to 1.54.1, past RUSTSEC-2026-0386.

**What stays:** the pins are unchanged — Stylo 0.22.0, taffy 0.15 from the registry, Dioxus 0.7.3, no git dependency.

**The one conflict:** `construct.rs`'s `node::{…}` import line, resolved to upstream's list plus our `RangeInputData`.

**Measured on the whole take at P3:** fmt, clippy, the audit (`advisories ok, licenses ok`) and the workspace tests all
read green, at 206 · 1099 · 0 · 12. That is +1 result line and +1 passed, exactly upstream's `hit_test_dropped_nodes`. No stand number
moved.

**Left out, with the reason each was measured:**
- `06ecec886` (git Stylo 0.23-pre) and its two dependants. `3954ecb85` needs prefs that Stylo 0.22 lacks; `1c5265a8b`
  needs trait methods it lacks.
- `b8f4ba6fd` and `9babd796a` (git Taffy), plus CARRY 1's four Taffy-main commits.
- `286874cc1` (Dioxus 0.8.0-alpha.3). Escher's stable-id derivation fails to build against its vdom: 10 lib errors and
  15 test errors.

## What the plan does
1. Before readings, in `evidence/before.md`.
2. The ten picks and the one named resolution, in `evidence/take.md`.
3. The tokio lock update, in `evidence/lock.md`: tokio's version and checksum, 11 `windows-sys` edges and 1 `syn` edge re-pointed by cargo, the mimalloc edge, no package entering or leaving, and the mimalloc hand review re-read.
4. The 35 gates.
5. The CARRY 1–5 readings per row, plus `evidence/sink.md` and `evidence/counts.md`.
6. The operator pass:
   - hygiene;
   - the pre-CI commit with the ten `(cherry picked from commit …)` trailers;
   - the push;
   - the CI read (`--wait 2400`, since the lock change starts a cold cache);
   - the per-OS tallies against CI#39410865604.

## Top acceptance criteria (10, none claiming a capability)
- The take is exact, enforced by the exact-take, new-file, rename and `construct.rs` entries: six differing lines, the resolution alone.
- The pins stay: the manifest and `deny.toml` are byte-identical, the lock moves by +15 −14 with no name moved, and every tokio is ≥ 1.53.3.
- `ci-leg.sh audit` reads `advisories ok` and `licenses ok`.
- `fast` and `doc` pass; the workspace tally is predicted at 206 · 1099 · 0 · 12; `hit_test_dropped_nodes` is its own target; 145 files.
- The a11y leg reads 6 · 43, and `run stand` passes over 48 files with contrast, Tab tables, lint and bounds unmoved.
- CARRY 3–5 readings pass in both layout modes; blitz-dom lib tests pass.
- The sink re-check holds: the engine census reads only its one commented `dbg!`, 0 wall-clock reads, `host_log`/`mcp_log` pass under both builds, and the new target is silent.
- Fork CI is green on the pushed take, and its per-OS tallies are explained by name.

## Touchpoints
- **New:** `layout/inline/{build,mod,layout}.rs`, `examples/multi_doc.rs`, `tests/blitz-tests/tests/hit_test_dropped_nodes.rs`, the chunk's `evidence/`.
- **Modified (upstream's text):** `layout/{construct,mod,paint_tree,replaced}.rs`, `resolve.rs`, `node/{node,element,mod}.rs`, `mutator.rs`, `stylo.rs`, `config.rs`, dioxus-native `lib.rs`, browser `document_loader.rs` · `toolbar.rs` · `Cargo.toml`, the WPT runner's `Cargo.toml` · `main.rs` · `attr_test.rs`, `tests/all.rs`, `flake.nix`.
- **Lock:** `Cargo.lock`.
- **Impact:** `hit_inner` and `compute_replaced_layout` keep their signatures, so no caller is edited.

## Test commands — 39 entries, baselined
The 33 new non-leg entries were baseline-run on the untouched tree (2026-10-24T11:28–11:34Z, `baseline` keys written):
- **Red where the take has yet to build its subject (each with a must-pass reading on a scratch clone carrying the take):**
  entries 1–5, the lock numstat, tokio unfixed, `multi_doc.rs` absent, the 144 files, `hit_test_dropped_nodes` absent, and the audit
  (`advisories FAILED` on RUSTSEC-2026-0386).
- **Green where the property already stands (each guard with a must-fail input):** the guard, no git dependency, no
  wall-clock read, the `keyboard_compat` exclusions, the 48 stand files.
- **The suites:** blitz-dom lib 106, the CARRY integration set 19 targets / 106, a11y 6 · 43, `run stand` 195 · 0 · 7,
  `run all` 616 · 0 · 12, seven_guis 12 lines / 36, the windowed build and smoke.

The controls are recorded in `.andromeda/runs/2026-10-24T09-31-30-phase/p5-controls.md`. One entry was re-spelt there: entry 4's `expect` became `['exit 1', 'last line 6']`, since under `pipefail`
`diff` exits 1 whenever the sides differ. It was then re-read both ways.

**Validation:** every mechanical check passes. planlint's one HIT (4 (13), the per-OS job-log read that `ci.py` never makes) is resolved by its
note and its P4 control on CI#39410865604. Validation-1 reads aligned.

## Concretization preview
None: a sync makes no 0.2.0 capability verifiable, so no cap is claimed. v020-10 and v020-11 belong to «Capability
statement» and «Cold-agent bar».

## Sampler — forced sample, P2's `detail.py compare`
- architecture — sampler: E carried 171 · the cascade lost 12 (conventions#10.1, conventions#3.1, conventions#7.1, design-philosophy#3.1, established-decisions#21.1, existing-scopes#1.26, occupied-resources#2.1, project-intent#2.1 …)
- security-plan — sampler: E carried 81 · the cascade lost 26 (api-security#2.1, api-security#2.6, api-security#2.9, authentication-authorization#2.2, authentication-authorization#2.6, authentication-authorization#2.7, data-protection#1.1, error-handling#1.1 …)
- design-system — sampler: E carried 47 · the cascade lost 27 (color-palette#1.3, color-palette#1.7, color-palette#2.4, color-palette#5.1, depth-strategy#2.1, depth-strategy#4.1, depth-strategy#4.2, iconography#1.1 …)
- test-plan — sampler: E carried 122 · the cascade lost 16 (integration-test-strategy#2.2, mocking-stubbing-discipline#1.1, mocking-stubbing-discipline#1.4, mocking-stubbing-discipline#1.5, mocking-stubbing-discipline#1.6, mocking-stubbing-discipline#1.7, mocking-stubbing-discipline#3.1, mocking-stubbing-discipline#3.2 …)
- obs-plan — sampler: E carried 51 · the cascade lost 21 (ci-integration#1.1, ci-integration#1.2, ci-integration#1.3, ci-integration#1.4, ci-integration#1.9, error-capture-reporting#1.1, error-capture-reporting#1.4, log-coverage#6.11 …)
- a11y-plan — sampler: E carried 91 · the cascade lost 11 (a11y-scope-summary#1.10, aria-patterns-roles#1.1, aria-patterns-roles#2.2, aria-patterns-roles#2.3, keyboard-navigation#1.6, keyboard-navigation#1.7, keyboard-navigation#1.9, keyboard-navigation#2.1 …)

The plan takes all 113 Lost items, each named «(sampler)» by its site in `plan.md` §Implementation notes and held by a gate, a criterion
or the wrap's citation re-point.

## For the operator, not a word of this card
- **rerere entries:** git rerere is on (`~/.config/git/config`). P3's trials left seven resolutions in `.git/rr-cache`: `d3e70713…`, `46a43bbc…`, `4f8055e9…`, `58a28736…`, `4439e693…`, `ce4a55b6…` and `d2055d86…`, all dated 2026-10-24 12:54–13:03 local. The host guard refused their recursive removal. The `construct.rs` one is the take's intended resolution and may apply itself at step 2.
- **Trial worktree:** `target/tmp/trial-wt` is still registered. Removing it (`git worktree remove --force target/tmp/trial-wt`) is the operator's.
- **The snapshot's miscount:** `inputs#I3` item 1 says "twelve" `windows-sys` edges; its own list and the take's numstat say eleven. The correction stands in `research.md` and `scope.md`.

## Beyond the plan
| # | Proposal | Owner / place |
|---|---|---|
| 1 | **The upstream delta this take leaves out**, which moves to escher's next version:<br>• `06ecec886` (git Stylo 0.23-pre) with `3954ecb85` and `1c5265a8b`, plus our `contain: paint` hit stop's getter under Stylo 0.23;<br>• `b8f4ba6fd` and `9babd796a`, with CARRY 1's four Taffy-main commits (git Taffy `802e497b…`);<br>• `286874cc1` (Dioxus 0.8.0-alpha.3), with the port of escher's stable-id derivation onto its vdom (10 + 15 errors measured), and CARRY 2's `keyboard_compat.rs`, retired upstream there.<br>Measured at this chunk's `research.md` §The trial takes. | A new `.andromeda/residuals.md` line, origin `2026-10-24-upstream-sync-polish-and-ship`, `(target: next)`, appended by the wrap's route-resolve. No later upstream-sync entry stands in escher-0.2.0's route. |
| 2 | **CARRY 3, 4, 5:** the merge surfaces of 2026-10-18-engine-debts, 2026-10-18-one-virtual-clock and 2026-10-20-select-by-id are re-read at the next sync, with nothing sent upstream. | A new `.andromeda/residuals.md` line, origin `2026-10-24-upstream-sync-polish-and-ship`, `(target: next)`, beside row 1 |

Apply? (yes / review / cancel)
