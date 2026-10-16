# escher 0.2.0 — requirements

*Source: `escher-0.2.0/intent.md` §Findings and §Principles.*
- Ids are version-scoped (`v020-NN`), continuing 0.1.0's `v010-NN` scheme.
- Every capability is verified headless, on the 7GUIs stand or on a minimal fixture beside it where the tasks lack
  the case (intent §Principles).
- Residuals are taken in from `.andromeda/residuals.md` (21 `open` lines): 13 absorbed into the capabilities below and
  8 re-carried (§Carried residuals). The 0.1.0 matrix holds no `deferred` entry, and 0.1.0's requirements carry no
  residuals section.

- v020-01 · Codebase structure — escher's over-ceiling event handlers are split into named functions along their range and activation branches; escher's inline test modules leave `document.rs` while upstream's body stays; `mcp.rs` and `schema.rs` are split along their seams; every check stays green (per intent §1, the version-end audit)
- v020-02 · Fast-failing mutants — each of the six mutants caught only by the timeout fails a test fast, and the mutation score holds at 0 survivors (per intent §1, the version-end audit)
- v020-03 · Engine debts dispositioned — each recorded engine debt reaches the founder as a fork, then is fixed and proven on the stand or a fixture with its upstream merge surface owned, pinned as a stated limit, or kept as a residual by his word (per intent §2; residuals: `inert`, `aria-checked`, `aria-activedescendant`, `Node::absolute_position`, `contain: paint`, the password caret, `disabled` focusability, the transformed client rect, `covered` on a zero-height box)
- v020-04 · Trace to regression check — any driver session, through the CLI or MCP, is recorded as a replayable check, ordinary test input that runs in CI, failing when the UI's behaviour changes and passing while it holds (per intent §3, F1)
- v020-05 · Visual lint — one command reports, for the current screen, each element that overlaps another, is clipped, has truncated text, lies off-screen, has zero size or fails text contrast, naming its stable id and measured reason (per intent §4, E1)
- v020-06 · One virtual clock — every time the engine and the app read in a headless run comes from one virtual clock only the driver moves; two shots of the same state read the same pixels (per intent §5, C1; residual: the overlay scrollbar fade's wall clock)
- v020-07 · Load wait — a command that meets a load in flight waits it out within a stated bound, proven on a fixture (per intent §6; residual: no driver command waits on a load)
- v020-08 · Select by id — a `select` is driven by id like any control and read in the snapshot, proven on a fixture (per intent §6; residual: no `select` interaction model)
- v020-09 · Large screen in parts — a screen over the snapshot text budget is read in parts an agent can walk, proven on a fixture (per intent §6, Cells over the 10,000-byte budget)
- v020-10 · Capabilities stated for a consumer — what escher can do, and its stated limits, is written where Viola's frontend planning reads it, before that planning starts (per intent §Principles "escher leads Viola" and §6)
- v020-11 · Cold-agent bar — 0.2.0 ends with a cold-agent run over its new capabilities, judged against a wrong-call bar the founder sets from 0.1.0's baseline of 0 wrong in 43 (per intent §7; residual: the cold-agent wrong-call bar; covers v020-04…v020-09 end to end)

## Carried residuals (NOT 0.2.0 capabilities — deliberately unnumbered, no matrix entry)
Each line below was dispositioned at this route's Phase A. The founder's chosen labels are relayed verbatim by the
overseer (2026-10-16; record: the escher overseer's `relays/route-0.2.0-founder-answers.md`).
- The six the intent's §Out of this version names were confirmed by «Подтвердить (Рекомендую)».
- The two it does not name went by «Перенести дальше (Рекомендую)».

Each line stays visible here and outside the matrix until a version asks for it.
- **Opt-in OTel export** (2026-10-06-telemetry-bootstrap). Named by intent §Out. Its take-up still needs the export
  transport and the credential path decided, and both registered first.
- **The actionable-key check as a driver command** (2026-10-12-self-description). Named by intent §Out. It is a new
  crossing for stable ids.
- **MCP revision `2026-07-28`** (2026-10-12-self-description). Named by intent §Out. The server keeps answering
  `2025-11-25` to any other revision.
- **Cells states its selected cell** (2026-10-14-stand-requirement-sweep, C8). Named by intent §Out. Cells' cells stay
  keyed and carry no selected reading.
- **Dropping the paste advisory ignore** (RUSTSEC-2024-0436; 2026-10-14-quality-gates). Named by intent §Out. It drops
  when the `rav1e` chain leaves paste.
- **memmap2's unsound advisory** (RUSTSEC-2026-0186; 2026-10-14-quality-gates). Named by intent §Out. It stays out of
  the audit by the `unsound` default.
- **The stand's platform accessibility adapter and its windowed AT witness** (2026-10-14-stand-a11y-assertions, F1).
  The intent does not name it. It is owed at an active assistive technology, which this version does not reach.
- **A snapshot reading for "reacts to a click"** (2026-10-14-stand-requirement-sweep, C3). The intent does not name
  it. The field rides with the UI graph of handlers (D2), which is out of 0.2.0.
