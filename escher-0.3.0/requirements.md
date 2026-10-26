# escher 0.3.0 — requirements

*Source: `escher-0.3.0/intent.md` §Findings, §Principles and §Also carried.*
- Ids are version-scoped (`v030-NN`), continuing 0.2.0's `v020-NN` scheme.
- Every capability is verified headless, on the 7GUIs stand or on a minimal fixture beside it where the tasks lack
  the case (intent §Principles).
- Residuals taken in: 14 `open` lines in `.andromeda/residuals.md` and the 8 lines 0.2.0's requirements carried. Of
  these, 12 are absorbed into the capabilities below, 9 are re-carried (§Carried residuals) and 1 is dropped
  (line 38: discharged at 2026-10-25-cold-agent-bar, measured with no limit left; the operator's word, 2026-10-26).
  The 0.2.0 matrix holds no `deferred` entry.

- v030-01 · Codebase health — escher's three functions over the cognitive-complexity ceiling (the lint's findings walk and two table-asserting tests) fall under it as a table walk or named helpers; test functions stay in the count, the engine function stays as it is; every check stays green (per intent §1, the Epoch 5 audit; the founder's «Код escher, тесты таблицами (Рекомендую)»)
- v030-02 · The version on the GitHub page — the README shows 0.2.0 done and verified, with what it can do and its cold-agent result, and 0.3.0 planned with its stages; the capabilities statement describes the 2026-10-25 run (per intent §2; residual: line 41)
- v030-03 · Upstream sync — upstream's new work is taken in by the take the founder chooses, the fork's engine-change merge surfaces keep their owners, and every check stays green (per intent §3; residuals: lines 35, 36)
- v030-04 · Engine debts dispositioned — each recorded debt reaches the founder as a fork, then is fixed and proven on the stand or a fixture, pinned as a stated limit, or kept as a residual by his word (per intent §4; residuals: lines 31 transformed scrolling box, 32 `inert` focus fixup, 33 native checkedness, 34 select model limits, 39 select-option overlap, 40 served pages' keyboard and contrast rows)
- v030-05 · The UI graph — one read gives, for the current screen, node → component → handlers → the state they write → the nodes that read it; a probe of what the Dioxus runtime exposes without a fork-level change comes first and reaches the founder before the graph is built; the "reacts to a click" reading rides with it (per intent §5, D2; residual: line 23)
- v030-06 · What can I do here — one command lists each actionable node on the current screen with its verbs, and why it can or cannot be acted on now (disabled, covered, off-screen), read from the UI graph; the actionable-key check joins it if it fits (per intent §6, B3; residual: line 12)
- v030-07 · Handler coverage — one command lists the handlers no recorded check exercised, read from the UI graph (per intent §6, E3)
- v030-08 · Start in a state — a session can start in a given state, a fixture, building on 0.2.0's shared fixtures (per intent §7, C4)
- v030-09 · Auto-exploration — one command drives every available action breadth-first within a step budget, catches panics and refusals, and records each path as a replayable check (per intent §7, E2)
- v030-10 · Errors with the action — an action's answer carries the panic, error or handler log it caused, tied to the node id and, through the UI graph, to the handler; what of a log may enter an answer reaches the founder as a boundary fork, bounded by his log rulings of 2026-10-07/09 (per intent §8, D1)
- v030-11 · Layout explained — an agent can ask why an element is where it is and at its size, read from the layout and style chain (per intent §9, D3)
- v030-12 · A component in isolation — one component renders alone with given props, across the UI graph's component boundary (per intent §9, F2)
- v030-13 · Seeded randomness — a run's randomness comes from a seed, so two runs of one seed read the same state (per intent §9, C2)
- v030-14 · Bundled fonts — a run's fonts are bundled, so two hosts read the same state the same way (per intent §9, C3)
- v030-15 · Cold-agent bar — 0.3.0 ends with a cold-agent run over its new capabilities, judged against a bar the founder sets from 0.2.0's baseline of six apps, 66 calls, 0 wrong (per intent §10; covers v030-05…v030-14 end to end)
- v030-16 · Records kept true — the two verified ledger refs name the tests' current names, the design-system stand rows (the accent, the success row, the cards item, the stand's DejaVu Sans item) move out of the product surface into architecture and layout-templates, and the incubator is ignored by `.gitignore` in place of a local exclude (per intent §Also carried; residual: line 42)

## Carried residuals (NOT 0.3.0 capabilities — deliberately unnumbered, no matrix entry)
Each line below was dispositioned at this route's Phase A. Every one is named by intent §Out of this version («The
residuals not named above stay residuals until asked»), except the citation backlog, which the intent does not name
and which is re-carried by default. Each stays visible here and outside the matrix until a version asks for it.
- **Opt-in OTel export** (2026-10-06-telemetry-bootstrap, line 8). Its take-up still needs the export transport and
  the credential path decided, and both registered first.
- **MCP revision `2026-07-28`** (2026-10-12-self-description, line 13). The server keeps answering `2025-11-25` to any
  other revision.
- **The stand's platform accessibility adapter and its windowed AT witness** (2026-10-14-stand-a11y-assertions, line
  14). It is owed at an active assistive technology, which this version does not reach.
- **Cells states its selected cell** (2026-10-14-stand-requirement-sweep, line 24). Cells' cells stay keyed and carry
  no selected reading.
- **Dropping the paste advisory ignore** (RUSTSEC-2024-0436; 2026-10-14-quality-gates, line 26). It drops when the
  `rav1e` chain leaves paste.
- **memmap2's unsound advisory** (RUSTSEC-2026-0186; 2026-10-14-quality-gates, line 27). It stays out of the audit by
  the `unsound` default.
- **Dropping the zip advisory ignores** (RUSTSEC-2026-0367 · 0368 · 0369; 2026-10-16-fast-failing-mutants, line 29).
  They drop when `anyrender_serialize` reaches zip 9.0 or leaves zip.
- **Dropping the zbus-family advisory ignores** (RUSTSEC-2026-0370 · 0371 · 0372; 2026-10-17-upstream-sync-engine-debts,
  line 30). They drop when accesskit_unix moves to zbus 6.
- **The category-file citation backlog's hand reads** (F334; 2026-10-24-upstream-sync-polish-and-ship, line 37). Three
  hand-read sites, five held rows and two unresolved rows remain. This is pipeline bookkeeping with no capability
  behind it.
