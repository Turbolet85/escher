# Fan-out results — 2026-10-18-upstream-sync-determinism-and-traces (resumed wrap)

Snapshot before: `tree f9121835f771a0613b497093b070630b8260b659 index 6627398bb3fcce96f25ec35274646c23cec8c887 head 275950dc0985f258c4fd26fc98faa80fb29df31d`
Snapshot after: `tree a4d3c95818aeb21c389fde6ad8e28903c6bf8875 index 6627398bb3fcce96f25ec35274646c23cec8c887 head 275950dc0985f258c4fd26fc98faa80fb29df31d`
`git diff-tree -r --name-status` between them: exactly seven `A` lines, one `fanout-{doc}.md` per agent; index and head unchanged. No file needed a fix: no twin. Receipts equal the files' counts.

architecture: 0 proposals · fanout-architecture.md
- note: neither detector's invariant broken; the `Upstream sync:` line and the CARRY 3 owner title move outside both detectors (report sweep).
- 1 (raised by main) §Project Intent, the `Upstream sync:` line → apply — check 5 (plan expected amendment 1; report Counts «The upstream-sync record»): upstream `main` read at `fb8d6c15e`, `fb8d6c15e` taken alone by no-commit cherry-pick in `275950dc0`, merge base stays `23b9b26a9`, the four Taffy-main commits wait; F1 B the delegate's answer, PROVISIONAL (inputs#I5, inputs#I7 (a)). Check 1: «Accurate this-chunk addition» routine; the PROVISIONAL mark is written, not discharged — «Provisional discharge» not reached (it joins the epoch batch, inputs#I7 (a)).
- 2 (raised by main) §Inherited Defaults / its upstream-sync paragraph at `architecture.md:90`, the CARRY 3 owner title «Upstream sync ahead of determinism and traces» → «Upstream sync ahead of visual lint and real-app readiness» → apply — check 5 (plan expected amendment 6) on the operator's recorded direction, inputs#I7 (c), naming the entry and the site; P5 moves the CARRY.

security-plan: 1 proposal · fanout-security-plan.md
- note: D-security-input and D-security-auth hold; severity warning, no banned dependency.
- 1 D-security-deps §Dependency Security → Pinning, the 2026-10-18 sync record beside the 2026-10-17 one → apply — check 1: «Accurate this-chunk addition» routine (no dependency moved; the plan's expected amendment 2 names the change); check 5 matched. The applied text re-derived from the report (Dependencies, Outcome (security), external claims), the PROVISIONAL F1 B mark written, not discharged.

design-system: 0 proposals · fanout-design-system.md
- note: D-design-tokens has nothing to judge; the cursor expected amendment is the orchestrator's.
- 1 (raised by main) §Iconography → Cursors and favicons (engine), `design-system.md:316`: over no box the cursor reads Default (shown windowed), where it read none (hidden) — upstream #1220 `fb8d6c15e` → apply — check 5 (plan expected amendment 3). Check 1: «An engine behaviour change for every document» — the fork answer is F1 B, a delegate's, so PROVISIONAL by that rule's note; no crossing widens (inputs#I5: the operator agrees no widening; the value reaches `ShellProvider::set_cursor` alone). `:317` no change (true: `None` still flows for `cursor: none`).

layout-templates: 0 proposals · fanout-layout-templates.md
- note: D-layout-surface holds; 0 `docrs-cite` rows.

test-plan: 0 proposals · fanout-test-plan.md
- note: the three detectors hold; the count moves are the orchestrator's from the report's Counts.
- 1 (raised by main) §1 blitz-dom, `test-plan.md:11`: 35 · 18 · 9 → 36 · 19 · 10 → apply — check 5 (plan expected amendment 4; pattern `dom-tests`).
- 2 (raised by main) §5 (the font-metric `hover_state_tests` line), `test-plan.md:293`: the two `hover_state_tests` → three, `2 passed` → `3 passed`, upstream's third asserting with the two-argument `qual_name!` fixture and no skip → apply — check 5 (pattern `hover-tests`).
- 3 (raised by main) §9 Per-OS test tallies, `test-plan.md:325`: re-read on CI#39104364530 on `275950dc0` — linux 192 · 991 · 0 · 12; macos and coverage 173 · 988 · 0 · 12; windows 173 · 964 · 0 · 9; a11y 6 · 43 · 0 · 0; stand repeat 144 · 582 · 0 · 21 — moving from CI#39056637407 directly, the engine-debts reading (CI#39100630123) named as skipped, never written → apply — check 5 on the operator's direction, inputs#I7 (b).
- 4 (raised by main) §9 Local baseline, `test-plan.md:331`: re-counted 192 result lines · 991 · 0 · 12, from 187 · 971, engine-debts' +5 lines · +19 passed named as that chunk's, this chunk's +1 passed (`hovering_below_content_reports_default_cursor`) → apply — check 5 on the operator's direction, inputs#I7 (b).

obs-plan: 0 proposals · fanout-obs-plan.md
- note: no hot path, logger or PII change; `document.rs` citations ≤ 2263.

a11y-plan: 0 proposals · fanout-a11y-plan.md
- note: no interactive element, no schema change.

Validate: check 2 (cross-contradiction) none · check 3 (intent) the report diverges from the working-route entry's «upstream/main merged» by taking one commit by cherry-pick — justified by F1 B (delegate, PROVISIONAL) and ruling 9's fallback (inputs#I5, inputs#I6); the master desc is rewritten at the flip · check 4 none (no absence claim) · check 5 every plan entry dispositioned above (entry 5, the citations, landed by `cites.py apply`, 35 re-pointed; entry 6, the CARRYs, P5) · check 6 «Spec claims disproved» none. Rejected for an unreported source: 0. Escalations: 0 — F1 B stays PROVISIONAL, no card (inputs#I7 (a)).
