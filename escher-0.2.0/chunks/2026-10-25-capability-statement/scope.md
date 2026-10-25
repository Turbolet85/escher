# Scope — 2026-10-25-capability-statement

**Working entry (verbatim, escher-0.2.0/working-route.md:53):** Capability statement — what escher can do, its stated
limits, a11y reach and what MCP client carries onward, where Viola's planning reads it (v020-10), with two `CARRY:`
blocks (folded below).

**Cap:** v020-10 «Capabilities stated for a consumer» — requirement: what escher can do, and its stated limits, is
written where Viola's frontend planning reads it, before that planning starts (intent §Principles «escher leads Viola»,
§6). Acceptance (as routed, `method: manual`): before Viola's frontend planning starts, the operator confirms one
statement of what escher can do and its stated limits, current with 0.2.0's capabilities, stands where that planning
reads it.

## What this chunk builds
- **One capability statement, current with 0.2.0** — a document, not code: what escher can do as of this tree, read
  from the shipped surfaces (the driver's twelve verbs on its carriers — socket `call`, command line, MCP tools, a kept
  check; stable element ids; the snapshot's id · role · name · state · bounds, its text form, its parts and its diff;
  the headless screenshot; the visual lint's six defects; session recording and replay; the load wait; the select
  model; the range input and keyboard activation; the one virtual clock).
- **Its stated limits** — every limit the tool and the masters already state, gathered in one place: the three
  measured-off readings (`off-screen` under a transformed scrolling box, `covered` on a zero-size box and on a box
  clipped on one axis), the boolean model's two limits, the select's unmodelled parts (`multiple`, `optgroup`, no
  popup), the shot's PNG bytes differing across OSes, HTML's focus fixup not built, RTL and vertical text read as LTR
  lines by `text_extent` — and the CARRY's unpaged diff and findings (below). [premise-corrected: the list is wider
  than named here — research.md §Files inspected and the P2 extracts add the host, client and recording as unix only
  (the in-process session and the MCP stdio role run elsewhere), the select's limits asserted by no check, the boolean
  model's limits pinned at unit and engine level only, no description carried by any driver carrier, the builder
  reading no `aria-activedescendant` and no native checkbox's or radio's checked state, no snapshot field for «reacts
  to a click», `:focus-visible` · `:valid` · `:invalid` · `:required` · `:target` always false, the telemetry sink's
  reach (upstream apps' `fmt::init()` and the WPT runner unscrubbed, the std panic hook's raw message), `file:` URLs
  read with no path restriction, no OTel export, and the 2026-10-15 live cold-agent run measured at eleven tools]
- **Its a11y reach** — what the WCAG target (SC 2.1.1 · 1.4.3 · 2.4.3) is proven on (the stand's Tab order, Enter/Space
  activation and contrast rows, `a11y` leg of fourteen files) and what is not measured (an assistive technology at a
  windowed run). [premise-corrected: the AT-witness line reads `re-carried:0.2.0` (.andromeda/residuals.md:14), not
  `target: next`, and requirements.md:42-43 states it is owed at an active assistive technology «which this version
  does not reach»; the leg's reading 14 · 84 · 0 · 0 on CI#40134371519 and SC 2.1.1 · 1.4.3 · 2.4.3 as the tier-0
  target verified against the a11y extract and its history]
- **What an MCP client carries onward** — the content escher hands its MCP client over stdio (ids, accessible names,
  control values with a password's and a file input's masked, snapshot text and diffs, a shot's PNG with a password
  painted masked, a lint's findings) and that whatever the client forwards from there (its model provider included) is
  beyond escher's boundary; the cold-agent pipe as the measured instance — verified against security-plan §Surfaces,
  arch §Established Decisions → Driver session and the cold-agent crossing (security-plan-detail/threat-model-summary.md:34-35);
  the carried set also holds a lint's findings, which that row does not name — a wrap amendment, not this chunk's edit
  (research.md §Open questions).
- **Where Viola's planning reads it** — settled at P4 by the founder's F1 «Док escher + ссылка в Viola (Рекомендую)» (inputs#I2): a tracked `docs/capabilities.md` in escher, linked from README.md, and an untracked pointer in Viola's incubator `research/` naming its path and commit. As read at take-up, no artifact settled it. Viola's 0.2.0 planning
  assembles `intent.md` from its gitignored incubator (`viola-0.2.0-incubator/`, inputs#I1), then routes it; no
  escher statement is named there. Candidates: escher's own `README.md` (tracked, the CARRY already edits it), a
  dedicated tracked escher document the README points to, or a file placed in Viola's incubator (another
  repository, untracked). A P4 fork — the location decides what «stands where that planning reads it» means for the
  acceptance.

## Folded freight
- **CARRY 1 — README's verb count.** README.md still says the MCP role gives «the eleven driver commands as tools»;
  since 2026-10-19-visual-lint's gated part the verb table holds twelve, `lint` the eighth instance verb on every
  carrier (escher-0.2.0/chunks/2026-10-19-visual-lint/report-gated.md, Symbols / APIs). Re-verified at take-up:
  README.md:48 reads «the eleven driver commands as tools»; CLAUDE.md's escher-driver row states `VERBS` holding twelve.
- **CARRY 2 — the unpaged diff and findings stated as a limit.** An acting verb's diff and a lint's findings are not
  paged — on a large screen either can pass 10,000 bytes, held only by the 1,048,576-byte answer bound, while
  `snapshot` reads a screen over the budget in parts (the operator's «yes» on the 2026-10-20-large-screen-in-parts P5
  card, row 1: escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/inputs/I3-relay-3.md.txt; measured at
  escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md, Counts «The stated limit» — re-verified at line 161
  and its route row at line 220). This chunk states the limit; it does not page them.

## CI read at take-up (Setup 5a)
- `10e8cc391` (the last wrap's flip): green · 17/17 checks · wall 1132 s · CI#40152171348.
- `e2eb01b44` (setup's U56 `.gitignore` upgrade): verdict not yet available — in progress, 4/4 checks so far,
  CI#40156254102. Its change is one `.gitignore` line; it is re-read at the wrap.

## Boundaries
- No code, engine, schema, verb, carrier, log field or span changes. The statement describes what stands; a defect
  it finds is a finding for the wrap, never a fix here — verified: the chunk changes no symbol (research.md §Graph
  impact); a CI-script pin of the statement's counts is test code outside every crate and stays a P4 fork (F2).
- README's version section stays the history it is; its 0.2.0 stage list may name the statement once it stands —
  verified against arch §Project Intent → Front page (README.md:52-73, one line per epoch).
- One write outside this repository: the Viola pointer, untracked, on the founder's F1 (inputs#I2); no other Viola file. The CI-script pin `.github/scripts/test_capability_statement.py` is test code outside every crate, on his F2 «Добавить пин (Рекомендую)» (inputs#I2).
- v020-11 (Cold-agent bar) is the next entry's, not this chunk's.
