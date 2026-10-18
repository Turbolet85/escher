## 2026-10-18-upstream-sync-determinism-and-traces — §1, §8, §9: upstream's cursor test counted, the tallies re-read on CI#39104364530
**Section:** §1 (blitz-dom) · §8 (the font-metric `hover_state_tests` line) · §9 (Per-OS test tallies · Local baseline)
**Change:**
- §1 blitz-dom: was 35 `#[test]` functions, 18 `document::`, 9 in document.rs; now 36, 19, 10 — upstream's `hovering_below_content_reports_default_cursor`.
- §8: the two `hover_state_tests` gain a third, upstream's, on the same two-argument `qual_name!` fixture, no precondition and no skip; `3 passed`.
- §9 Per-OS: re-read on CI#39104364530 (`275950dc0`, green 17/17) — linux 192 · 991 · 0 · 12; macos and coverage 173 · 988 · 0 · 12; windows 173 · 964 · 0 · 9; a11y 6 · 43 · 0 · 0; stand repeat 144 · 582 · 0 · 21; was CI#39056637407's 187 · 971 / 168 · 968 / 168 · 944 / 6 · 43 / 141 · 576. The skipped reading, CI#39100630123 on `fb842671` (2026-10-18-engine-debts' final HEAD), is named in the body: engine-debts' +5 lines · +19 passed (stand repeat +3 · +6), this chunk's +1 passed on the four jobs that run blitz-dom's lib.
- §9 Local baseline: re-counted 192 result lines, 991 · 0 · 12, from 187 · 971 — engine-debts' +5 · +19 (its 192 · 990 never written) and this chunk's +1 passed, no new result line.
**Why:** the operator's direction (the operator, 2026-10-18): the tallies move from the masters' recorded reading straight to CI#39104364530, the skipped engine-debts reading named. Rule for later syncs: a wrap that does not re-read the tallies leaves the next to attribute the gap.
**Ref:** .andromeda/runs/2026-10-18T06-47-20-wrap/
