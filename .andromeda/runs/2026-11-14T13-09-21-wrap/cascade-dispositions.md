# Cascade dispositions — 2026-11-14-start-in-a-state

## Step 2 — the sweep
- The landings' dry runs wrote no pattern file: architecture (TX 920) `patterns: none: 1 set(s) and retire(s) changed prose alone — no file`; security-plan (TX 921) `patterns: none — the call sets and retires no row`; test-plan (TX 922) `patterns: none: 1 set(s) and retire(s) changed prose alone — no file`. No dry run was superseded.
- Main's own pattern, `cascade-patterns.toml`: `pathtests` — `(eight|8) cfg\(test\) modules|cfg\(test\) modules held in files of their own` (ignore case), the retired count of test-plan decision/inline-cfg-test-modules' `except` and its claim's phrasing, over all seven masters, their key files, the leaves, the curation homes and the bases. The kept-check set (architecture contract/driver-recording/kept-check) lengthened a true promise and retired no wording: no pattern.
- Search: `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.32), baseline 45c878c1; control fired at test-plan.md:108. `total (1 pattern) · 1 rows over 1 files` · `per class · new 0/0 · standing 1/1 · leaf 0/0 · curation 0/0 · base 0/0`; part `sweep-pass-1.1.txt`, read whole.
- `.andromeda/test-plan.md:108` pathtests standing dec-7nt3wnew edited → amended (the row this pass set; it now reads without the count — no duplicate of the retired count on the line).
- No leaf, curation or base row: the step adds no leaf to step 3's set.
- Binds: no obs-plan or a11y-plan row changed (TX 920–922 touch architecture, security-plan, test-plan only), so the test-plan §3 ↔ obs-plan §3/§6 and a11y ↔ log-schema binds are unmoved.

## Leaves
`cascade.py leaves` (pass mode, baseline 45c878c1): `total · pass · 10 group(s) · 20 leaves · 542305 B · unmapped 1`; `unmapped .claude/rules/host-linux.md (host)` — the host leaf, which no source states: no comparer, no step-3 edit. Applied by `leaves --apply`: `total · blocks 16 · applied 16`.
- architecture-1 · pass · 1 finding · applied 1 (conventions.md:30, the std `Result` convention) · by hand 0 · declined 0
- architecture-2 · pass · 0 findings
- architecture-3 · pass · 2 findings · applied 2 (seven_guis.md:19 `start_in` and its states, :26 the std `Result` convention where it bit) · declined 0
- architecture+security-plan+test-plan-1 · pass · 5 findings · applied 5 (escher-driver.md:6 the carrier absence, :15 ×2 `Session::reach` and the reach errors, :16 the recording, :62 `stand_start_in_state`) · declined 0
- architecture+security-plan+test-plan-2 · pass · 1 finding · applied 1 (CLAUDE.md:122, the start in a state in the generated architecture paragraph) · declined 0
- architecture+test-plan · pass · 3 findings · applied 3 (commands.md:61 ×2 the new target, blitz-test-harness.md:16 `start_in`) · declined 0
- security-plan-1 · pass · 0 findings
- security-plan-2 · pass · 2 findings · applied 2 (security.md:22 the closed name table and the reach errors' fixed texts) · declined 0
- test-plan-1 · pass · 1 finding · applied 1 (tests-summary.md:37 the by-hand equivalence) · declined 0
- test-plan-2 · pass · 1 finding · applied 1 (testing.md:14 stale «eight modules» → «some modules») · declined 0
- Not a step-3 edit: testing.md's reader counts («36 readers» of `session_common`, measured 38; the fixtures readers) — no master states them (report, Counts / qualifiers moved), so no comparer's; left to P3 curation.
