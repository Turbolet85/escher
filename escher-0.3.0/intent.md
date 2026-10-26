# escher 0.3.0 — intent

escher is a fork of Blitz (DioxusLabs/blitz) turned into an AGENT-FIRST UI framework: a UI an agent can see, drive
and verify headless. 0.2.0 closed verified 11/11 at `5e15b78d6`. It made the codebase ready to grow, paid the engine's
recorded debts, and added recorded-check replay, visual lint, one virtual clock and real-app readiness. Its cold-agent
bar run drove six apps in 66 calls with 0 wrong, for $0.82, under the founder's 5 % bar. 0.3.0 builds the deep agent
layer: what escher knows about the UI beyond what it draws.

Assembled 2026-10-26 by the escher overseer from the founder's words, given in the escher overseer session and quoted
verbatim below. Sources: `deferred-from-0.2.0.md` and `research/draft-deep-agent-layer-2026-10-20.md` beside this
file; `../escher-0.2.0-incubator/capability-list.md`; `.andromeda/residuals.md`; the Epoch 5 code audit
`.andromeda/runs/2026-10-25T21-43-01-code-audit/proposals.md`. The founder's answers are recorded in the escher
overseer's `relays/route-0.3.0-founder-scope-2026-10-26.md`.

## Principles for this version
- **The codebase, the README and upstream first.** The founder, 2026-10-26: «первыми чанками надо будет сделать
  улучшения кодовой базы на основе аудита, обновить ридми под новую версию и синхронизироваться с апстримом». The first
  three entries are, in this order, the improvement on the Epoch 5 audit, the README, and the upstream sync. The theme
  comes after them.
- **The theme is the deep agent layer.** The founder, 2026-10-20, «Глубокий агентский слой»: the priority goes to what
  no alternative has. On 2026-10-26 he took all of it into 0.3.0: «D2 граф UI + B3 + E3 (Рекомендую)», «E2
  автоисследование + C4 фикстуры», «D1 ошибки вместе с действием», «D3 + F2 + C2/C3».
- **D2 first, then its consumers.** The founder, 2026-10-26, «Сначала D2, потом потребители (Рекомендую)», which is his
  standing rule «infrastructure ahead of the queue» (2026-10-05). The UI graph is built before B3, E3, E2 and D1 read
  it, and its first step is a probe of how much the Dioxus runtime exposes without a fork-level change.
- **One engine-debts entry.** The founder, 2026-10-26, «Один пункт «долги движка» (Рекомендую)». It comes after the
  first three entries, and each engine change comes to him as its own fork (the playbook's engine-change rule).
- **Kept from 0.2.0.** escher leads Viola. The stand is the proof, with a minimal fixture beside it where the tasks lack
  a case. Upstream is merged at each epoch boundary while our changes stay mostly additive. The API explains itself.
  The project's name is «Escher», capital E, in prose; crate, binary and path names stay lowercase.

## Findings (OBSERVED → EXPECT)

1. **Codebase health.**
   OBSERVED (Epoch 5 audit, 2026-10-26, M1): functions over the cognitive-complexity ceiling of 15 went 84 → 87 → 88
   over three epochs, with small crossings just over the ceiling. Three escher-written functions crossed it:
   `escher-driver/src/lint.rs::findings` (16), the listing test in `escher-driver/src/mcp_tests.rs` (18), and
   `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` in `tests/blitz-tests/tests/stand_requirement_sweep.rs`
   (18). The two tests assert a table row by row inline. `stand_requirement_sweep.rs` grew 553 → 1,007 lines, the one new
   file over 800. One engine function, `blitz-dom/src/document.rs::clear_interaction_state_for_removed_node`, went
   11 → 17 by a fork change. Mutation at the version end: blitz-dom 53 of 53 caught; seven_guis unscored (12 of 15
   unviable); wpt 0 of 5 caught, all five in upstream's `wpt/runner/src/test_runners/attr_test.rs`.
   EXPECT: `lint.rs::findings` and the two test functions are factored into a table walk or named helpers and fall
   under the ceiling. The founder, 2026-10-26, «Код escher, тесты таблицами (Рекомендую)»: test functions stay in the
   count, and the engine function is left as it is. Every check stays green.

2. **The README shows 0.2.0 done and 0.3.0 planned.**
   OBSERVED: `README.md` describes 0.1.0 as done and 0.2.0 as planned (its "0.2.0 — planned, in five stages").
   `docs/capabilities.md:288-289` still describes the 2026-10-15 live run (`.andromeda/residuals.md:41`, the founder's
   «Строка в residuals (Рекомендую)», owned by 0.3.0's first P1).
   EXPECT: the GitHub page shows 0.2.0 done and verified, with what it can do and its cold-agent result, and 0.3.0
   planned, with its stages. The capabilities statement describes the 2026-10-25 run (six apps, 66 calls, 0 wrong).

3. **Upstream sync.**
   OBSERVED (measured 2026-10-26T19:10Z after a fetch): `upstream/main` is at `6fee0db25`, with 37 commits not in HEAD
   by `git log 5e15b78d6..upstream/main`. Some may already be in as cherry-picks: Epoch 5's sync landed ten by
   cherry-pick under the founder's «Десять коммитов (Рекомендую)», 2026-10-24. The pins are Stylo 0.22.0, taffy 0.15 and
   Dioxus 0.7.3. The Dioxus 0.8 alpha was passed over then.
   EXPECT: upstream's new work is taken in by the take the founder chooses at the sync's phase. The merge surfaces of
   the fork's engine changes keep their owners (`.andromeda/residuals.md`), and every check stays green.

4. **The engine's recorded debts.**
   OBSERVED (`deferred-from-0.2.0.md` §4): a target inside a transformed scrolling box reads outside its visible region,
   so a driver call on it can be refused `off-screen`. A focused node that becomes `inert` keeps focus. A native
   checkbox or radio never states its checked state to the platform adapter. A `multiple` select and `optgroup` are not
   modelled, no popup is painted, and an option is not clicked. The driver lint reads a select's options as overlapping
   in every document (`.andromeda/residuals.md:39`). The served pages (Cells, the load page) hold no Tab-order,
   keyboard-activation or contrast row (`.andromeda/residuals.md:40`).
   EXPECT: each debt is taken to the founder as a fork, then fixed, pinned as a stated limit, or kept as a residual by
   his word. Every fix is proven on the stand or a fixture.

5. **The UI graph (D2).**
   OBSERVED: an agent reads what is drawn (the snapshot, stable ids, refusal causes), but not which component rendered
   a node, which handlers it carries, which state they write, or which nodes read that state.
   EXPECT: one read gives, for the current screen, node → component → handlers → the state they write → the nodes that
   read it. Its first step is a probe of how much the Dioxus runtime exposes without a fork-level change. What the
   probe shows comes to the founder before the graph is built. The "reacts to a click" snapshot reading
   (`deferred-from-0.2.0.md` L23) rides with it.

6. **"What can I do here" (B3) and handler coverage (E3).**
   OBSERVED: an agent learns an action is refused only by trying it. Nothing says which handlers no check reached.
   EXPECT: one command lists each actionable node on the current screen with its verbs, and why it can or cannot be
   acted on now (disabled, covered, off-screen). One command lists the handlers no recorded check exercised. Both read
   the D2 graph. The actionable-key check as a command (L12) folds into B3 if it fits.

7. **Auto-exploration (E2) and starting in a state (C4).**
   OBSERVED: a check starts from boot and clicks its way to the state it tests. Every path an agent explores is driven
   by hand.
   EXPECT: a session can start in a given state (a fixture), building on 0.2.0's shared fixtures. One command drives
   every available action breadth-first within a step budget. It catches panics and refusals and records each path as a
   replayable check (0.2.0's recorded-check replay).

8. **Errors returned with the action (D1).**
   OBSERVED: a panic, an error log or a handler's log reaches the agent only through the log sink, not tied to the
   action that caused it.
   EXPECT: an action's answer carries the panic, error or handler log it caused, tied to the node id and, through D2,
   to the handler. What of a log may enter an answer comes to the founder as a boundary fork at D1's phase, bounded by
   his log rulings of 2026-10-07/09.

9. **Layout explained (D3), a component in isolation (F2), seeded randomness (C2), bundled fonts (C3).**
   OBSERVED: lint names an element's finding but not its cause. A component is seen only inside its app. Randomness and
   fonts depend on the host.
   EXPECT: an agent can ask why an element is where it is and at its size, read from the Taffy and Stylo layout chain.
   One component renders alone with given props, across D2's component boundary. A run's randomness comes from a seed,
   and its fonts are bundled, so two hosts read the same state the same way. These are the trust layer under E2's
   replays.

10. **The cold-agent bar.**
    OBSERVED: 0.2.0's run is the baseline: six apps, 66 calls, 0 wrong, $0.82, under the founder's «Не больше 5%
    (Рекомендую)».
    EXPECT: 0.3.0 ends with a cold-agent run over its new capabilities, judged against a bar the founder sets from that
    baseline.

## Also carried
- **The design-system stand rows.** The accent row, the success row, the cards item and the stand's DejaVu Sans item
  move out of design-system's product surface into architecture and layout-templates (the founder's «В 0.3.0, через
  инкубатор (Рекомендую)», `deferred-from-0.2.0.md`, end). There is no residuals line yet, so the route places it.
- **Two stale ledger refs** (`.andromeda/residuals.md:42`): v010-17 and v020-05 cite renamed tests. This is a factual
  correction with no word needed.
- **A `.gitignore` line for `escher-0.3.0-incubator/`**, replacing its local `.git/info/exclude`.

## Out of this version
The residuals not named above stay residuals until asked: opt-in OTel export (L8), MCP revision `2026-07-28` (L13),
the platform accessibility adapter and its windowed AT witness (L14), Cells' selected-cell state (L24), and the
dependency advisories with their drop conditions (paste, memmap2, zip, zbus). Upstream PRs to DioxusLabs/blitz wait on
#1083's reaction (the founder, 2026-10-07).
