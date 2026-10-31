# P5 review card — 2026-10-31-engine-debts, «Engine debts»

**Plan:** `escher-0.3.0/chunks/2026-10-31-engine-debts/plan.md` · 21 steps · 37 gates · starts from `2109f1c2e`.

## Goal
Six recorded debts end as the founder's word says: eight engine changes for every document, each ratified by his own word relayed verbatim (inputs#I8, inputs#I9), two driver or stand changes, the load page's rows, and text input and range pinned. CARRY 2's three checks land. No verb, node key, cause, listener, env var or dependency is added. The popup is not built.

| Fork | His word | What lands |
|---|---|---|
| F1 | «Починить (Рекомендую)» | `visible_region` reads clips through transforms |
| F2 | «Починить (Рекомендую)» | a non-invertible transform takes no hit |
| F3 | «Снимать фокус без blur (Рекомендую)» | focus cleared at the mutator's flush, no `blur` |
| F4 | «Откат на окно (Рекомендую)» | tree focus falls back to the window |
| F5 | «Писать на checkable-ролях (Рекомендую)» | native `toggled` on checkable roles |
| F6 | «Построить сейчас» | the `multiple` select |
| F7 | «Построить сейчас» | `optgroup` |
| F8 | «Давай нарисовать в отдельном чанке» | nothing here; see `## Beyond the plan` |
| Click | «Только строки multiple (Рекомендую)» | rows take pointer and driver clicks; a closed option stays refused `covered` |
| F9 | «Исключить закрытые option (Рекомендую)» | `overlaps` skips a closed single select's options |
| F10 | «Шлёт движок (Рекомендую)» | engine `change` on checkbox and radio; script's copy removed |
| F11 | «Закрепить ограничение (Рекомендую)» | text input and range pinned as firing no `change` |
| F12 | «Добавить строки (Рекомендую)» | the load page's rows; the `a11y` leg stays fourteen files |
| F13 | «Одна Tab-остановка (Рекомендую)» | Cells' grid one Tab stop with keys |

## One thing beyond his forks: the audit is red today
The baseline of `ci-leg.sh audit` reads **red on the untouched tree**: RUSTSEC-2026-0414 on `pulldown-cmark` 0.13.4, through `rdme`, published while the project stood still. CI's audit job would go red on this chunk's push. No exposed reader: `rdme` never sets `ENABLE_WIKILINKS`.

**Owner named: this chunk, step 21.** `cargo update -p pulldown-cmark --precise 0.13.5`, one package and four lock lines, no manifest or `deny.toml` line. Read over a scratch clone: `advisories ok, licenses ok`. This corrects the scope's "bumps no dependency" to "no coupled pin" (`.claude/rules/security.md` §Dependencies; playbook, External decay). If you'd rather keep it out, say so and I'll return to P4; the chunk's CI read then needs another owner.

## Leans inside F6 and F7, for your eye
His word settled the option; these settle what it reads. Each is stated in the plan's notes and is a one-line change now.
- **`select.value` on a `multiple` select** is its first chosen option, per HTML. A Dioxus handler reads the whole set through `evt.values()`, which the bridge fills.
- **A list shows every row.** `size` is not read, so `<select multiple size=3>` with fifty options is fifty rows tall unless its author sets a height. Reading `size` would be a further engine change.
- **A group's label is named in the tree, not painted**, since `content: attr()` is unbuilt. The popup's chunk owns it.
- **Chosen rows paint `#000000` on `#c8c8c8`** (12.56:1), a new engine default, the engine holding no tokens.
- **Keys:** arrows choose alone, Shift spans, Ctrl or Meta moves without choosing, Space toggles. Ctrl+A is not modelled.
- **The driver holds no Ctrl.** `press` has only `shift`, so a non-adjacent set cannot be built through the driver; adding an argument is a boundary question nobody asked. Its help says so.

## Cells, beyond the bare fork
- Arrows bring the current cell into view, or a keyboard user could not see a cell past the viewport. That adds one `onmounted` per cell (676), holding handles in a map.
- A pointer-opened editor reads exactly as today, so `stand_cells_served`'s `commit` is forecast unmoved.
- `cell-input`'s blur and click are tightened, because a keyboard-opened editor makes an existing defect reachable: clicking another cell wrote that cell's text into the old one.
- `cells-grid` becoming keyed may re-anchor the header cells' ids. `PARTS` 5 and `FINDINGS_AT_BOOT` 490 are forecast unmoved, to be measured.

## Top acceptance criteria
- A target in a transformed world's inner scroller reads inside `visible_region`, is hit, takes a driver `click`, and lints clean, at translate and `translate + scale(2)`; degenerate transforms read `None` with no panic.
- A node made inert: all three focus readings agree on none, Enter clicks nothing, a typed character lands nowhere.
- A `multiple` select's rows have boxes, take keys and clicks, one `input` then one `change` per change; form entries read the set; `optgroup`'s options list, a disabled group's skipped.
- A closed option stays refused `covered`, also under a group. `packages/blitz-paint/` shows no diff.
- `stand_lint` reads `[0, 0, 0, 0]`; Cells works by keyboard alone; the load page's rows are green.
- `describe`'s count row stays `[12, 13, 12, 3, 8, 4, 15]`; `host_log` and `mcp_log` read 0 needles under both builds; CI reads `verdict: green`.

## Touchpoints
5 new test files · 38 modified: blitz-dom's `scrolling`, `document`, `node`, `node/select`, `mutator`, `accessibility`, `events/{keyboard,pointer}`, `form`, `assets/default.css`; blitz-vibey-script's `runtime/events`; dioxus-native-dom's `snapshot`, `dioxus_document`; escher-driver's `execute`, `lint`, `refusal`, `schema`, `lib`; blitz-shell's `event`; `seven_guis`' `cells`; `docs/capabilities.md`; `Cargo.lock`; `ci-leg.sh` and its pin; 12 standing checks and `tests/common/mod.rs`. Graph impact: research.md.

## Test commands
37 gates: 8 inline guards · `cargo test` per crate · the five new targets · 41 standing targets · `ci-leg.sh fast`, `doc`, `audit`, `a11y` · `agent-run.sh` stand and all · `seven_guis` · windowed smoke · 4 operator entries (hygiene, push, CI read, per-OS tally). Baselines and controls: `.andromeda/runs/2026-10-31T03-24-40-phase/p5-controls.md`. The controls caught one wrong gate of mine, re-spelt there.

## Concretization preview — `v030-04` «Engine debts dispositioned»
**Now:** "Each debt the intent names reaches the founder as a fork and ends as his word says: fixed and proven on the stand or a fixture, pinned as a stated limit, or kept as a residual."

**Proposed:** each debt reached him as a fork in `founder-forks.md` (inputs#I8) and ends so, in both layout modes:
- **Debt 1, fixed:** `visible_region_transformed`, `stand_act_transformed`, `transformed_client_rect`.
- **Debt 2, fixed:** `inert_subtree`.
- **Debt 3, fixed:** `accessibility_states`, its proof the tree through `&dyn Document`.
- **Debt 4, fixed:** `multiple` and `optgroup` by `select_interaction`, `stand_act_select` and `form.rs`'s tests; checkbox and radio `change` by `checkable_change_event`. **Pinned as stated limits:** text input and range by `text_range_no_change_event`; a closed option's `covered` by `stand_act_select`, until the popup's chunk his word places.
- **Debt 5, fixed:** `stand_lint`.
- **Debt 6, fixed:** `stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`.

Never weakened: the popup half of debt 4 ends "pinned until its chunk" because that is his word, not because it is dropped. The requirement `v030-04` is proven by this chunk alone; no other markerless entry lists it. It is not an affordance cap read through a proxy: every key and click goes through the real dispatch.

## Validation
- **Mechanical checks:** pass. Sections present, every modify path exists, fence parses, 37 entries; `gate.py baseline` 16 seen green · 17 new, none unbaselined. `planlint` 1 hit, the per-OS job-log read, its control fired at this P5 and matching `.andromeda/test-plan-detail/ci-integration.md:50`. 181 lines outside the fence.
- **Check 12 (d):** one claim contradicted a step ("`set_attribute`'s arms untouched" beside step 9) and was re-worded.
- **Check 10:** `I2` reads drifted: the overseer ticked one checklist row in the relay; no fact moved. No `UNPARSED`.
- **Validation-1:** aligned. Two scope amendments, both intent-incomplete: his answers (`scope.md` §The founder's word) and the lock bump.
- **Tree:** no code changed; `git status --short -- packages tests examples docs .github Cargo.lock Cargo.toml` prints nothing.

## Sampler
- `sampler: architecture — E carried 175 · the cascade lost 13 · listed 9`
- `sampler: security-plan — E carried 75 · the cascade lost 5 · listed 2`
- `sampler: design-system — E carried 28 · the cascade lost 3 · listed 5`
- `sampler: test-plan — E carried 113 · the cascade lost 6 · listed 8`
- `sampler: obs-plan — E carried 46 · the cascade lost 4 · listed 4`
- `sampler: a11y-plan — E carried 82 · the cascade lost 7 · listed 1`

Lost ids: the `sampler:` lines in the trail `.andromeda/runs/2026-10-31T03-24-40-phase/detail-2026-10-31-engine-debts.json`. Each lost item stands in the plan marked `(sampler)`: at its step, or in `## Implementation notes` under the sampler's other carried items.

## Beyond the plan
| # | Proposal | Where it lands | By whom |
|---|---|---|---|
| 1 | **New working-route entry:** «Select popup — a closed select opens a painted list of its options, chosen by pointer and key; the founder's fork (`{id}`)», markerless, not started | `escher-0.3.0/working-route.md`, directly after the frozen `[2026-10-31-engine-debts]` line: the last of Epoch 2, before `### Epoch 3 — The UI graph` | this chunk's wrap at its P5, as an operator-requested adaptation: phase edits no entry but its own stamp |
| 2 | **That entry's freight:** `CARRY:` F9 is restated when options read real boxes open; `covered`'s closed-option reading ends there; a group's label and `size` are its to paint or read; the focus and key contract, reduced motion and engine colours come to him as its P4 forks; `CARRY:` playbook Boundary widening, its wrap halting once as his answer card | on that entry | the same wrap |
| 3 | **New requirement and ledger entry** for the popup: `- {id} · Select popup — …` in `requirements.md`, `matrix.py add` minting the id, never typed (corrected at the word, inputs#I11: `v030-16` is «Records kept true»'s), `planned`, unclaimed | `escher-0.3.0/requirements.md`, `verification-matrix.json` | the same wrap |
| 4 | **New residual line, target next:** `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver | `.andromeda/residuals.md` | the same wrap |
| 5 | **Residual lines 31, 32, 33, 39, 40 absorbed** into `v030-04`; **line 34** absorbed but for its popup half, which rows 1 to 3 own | `.andromeda/residuals.md` | the same wrap |

**How the popup's entry lands, in short:** your word on this card is snapped with this card's path, and this chunk's wrap applies rows 1 to 3 from this card. Nothing starts it: «escher's pause stands, and only he lifts it» (inputs#I9). Your `yes` here also starts nothing: `/andromeda-implement` stays yours to fire.

**At the word (inputs#I11, inputs#I12, added after the card was shown):** row 2's `size` stands on his «В чанк с попапом (Рекомендую)»; row 4 stands on his «Ограничение и residual (Рекомендую)»; row 3's id corrected as it reads above. The lock bump stays in this chunk on the operator's word.
