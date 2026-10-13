# Fan-out results — 2026-10-13-stand-contrast-harness

Batch: before `tree e767f48f08aecee37f972362685e5a770839f490 index dc19756181260625cbde39c056bdd5081213c8fb head dbc3049fb7d76e20e1b313671f084d337df66bbb` ·
after `tree 380ec1b059f953f5d6616e46d49a59f01aa01f55`, index and head unchanged · `git diff-tree` printed the seven `A` lines, nothing else.
Receipts match the files (3 · 1 · 0 · 0 · 10 · 4 · 4). No file needed a fix; no twin. Detector counts per prompt 2 · 3 · 1 · 1 · 3 · 3 · 2 = 15, the drift-base's 15.

architecture: 3 proposals · fanout-architecture.md — note: D-arch-decisions holds (`color` already in §Stack's Graphics row and in the graph through blitz-dom; no socket, port, env var or crate).
- 1 D-arch-resources §Standard Contracts → Test harness — apply (check 1: Accurate this-chunk addition, routine; the plan's expected entry names the contract, its exports and the `color` dependency; text re-derived)
- 2 D-arch-resources §Existing Scopes (blitz-test-harness row) — apply (check 1: routine; the plan's expected entry names the module count)
- 3 D-arch-resources §Existing Scopes (blitz-tests row) — apply (check 1: routine; the report's Counts names architecture.md:263, 17 → 18)

security-plan: 1 proposal · fanout-security-plan.md — note: D-security-input and D-security-auth hold; the proposal records a vetted, unbanned dependency.
- 1 D-security-deps §Dependency Security — apply (check 1: graded `escalate` by the detector's severity while it reports its invariant holding, and the plan's reviewed list names the change — the playbook rule of 2026-10-07, routine; no boundary widened: no package enters the lock)

design-system: 0 proposals · fanout-design-system.md — note: D-design-tokens holds; the plan's §Color Palette entry left to the expected-amendment channel.
- 1 (raised by main, check 5) §Color Palette, the seven_guis accent row (design-system.md:24) — apply, routine: the report substantiates it (the measured record) — white `#ffffff` text on the accent `#4a6cf7` measures 4.39 against 4.5 on the five accent buttons; every measured colour equals its declared value

layout-templates: 0 proposals · fanout-layout-templates.md — note: D-layout-surface holds; no surface or region added.

test-plan: 10 proposals · fanout-test-plan.md — note: D-tests-obs-harness holds; the `139 · 0 · 7` re-count stands at test-plan.md:116 (§3 agent-run Proof), not :330 as the report attributes; :330 holds the 179 · 893 row.
- 1 D-tests-coverage §1 → blitz-test-harness — apply (check 1: routine; plan entry §1)
- 2 D-tests-coverage §1 → tests/blitz-tests — apply (check 1: routine; plan entry, the new target and its two tests)
- 3 D-tests-coverage §2 Directory pattern, `mod common;` readers 17 → 18 — apply (check 1: routine; report Counts)
- 4 D-tests-coverage §3 stand-check helpers, the named readers 17 → 18 — apply (dependent of 3; report Counts names test-plan.md:100)
- 5 D-tests-coverage §2 Directory pattern, files/targets 118/117 → 119/118 — REJECT (the opening rule: a count the report does not carry, the detector's own derivation — its rationale says so); its fact re-raised by main as item 11
- 6 D-tests-coverage §3 agent-run Proof, `run stand` 141 · 0 · 7 over 37 — apply (check 1: routine; report Counts; the site read at test-plan.md:116)
- 7 D-tests-coverage §9 Local baseline, 180 · 900 · 0 · 12 — apply (check 1: routine; report Counts)
- 8 D-tests-coverage §4 → blitz-test-harness, the five `contrast.rs` tests — apply (check 1: routine; plan entry §4)
- 9 D-tests-framework §2 Harness-driven tests — apply (check 1: routine; plan entry, report Harness / gate surface)
- 10 D-tests-framework §3 Harness Core — apply (dependent of 9)
- 11 (raised by main, check 5 — the plan's §2 entry, "the new target") §2 Directory pattern: 119 `.rs` files, 118 test targets and the unbuilt `all.rs` — apply, routine: the report substantiates the new auto-discovered file (Files; Decisions & corrections, tests/blitz-tests/Cargo.toml untouched); the count measured by main, `ls tests/blitz-tests/tests/*.rs | wc -l` reading 119 now and `git ls-tree --name-only 198bf85c tests/blitz-tests/tests/` 118 at the base

obs-plan: 4 proposals · fanout-obs-plan.md — note: D-obs-pii holds; the report's `test-totals` sweep missed obs-plan.md:70's "17 of which read the shared module" (worded without the token) — the cascade sweep carries the wording.
- 1 D-obs-instrumentation §4, the observed-absent list — apply (check 1: routine; plan entry obs §4)
- 2 D-obs-instrumentation §3 Logging stack, the silent harness listing — apply (dependent of 1)
- 3 D-obs-stack §3 Logging stack, the silent census by name — apply (check 1: routine; plan entry obs §3)
- 4 D-obs-stack §3 Logging stack, "17 of which" → 18 — apply (dependent of 3; check 4: the site read at obs-plan.md:70, the report's Counts fact)

a11y-plan: 4 proposals · fanout-a11y-plan.md — note: D-a11y-obs-schema does not fire; the registry label `contrast-verification-harness-setup` kept as a key name (as `a11y-ci-gate-wire` keeps its own after discharge); §3's closing blockquote (a11y-plan.md:89) left as it reads.
- 1 D-a11y-surface §6 Color contrast, the stand's 18 rows — apply (check 1: routine; plan entry a11y §6; text re-derived, the route owner from inputs#I2)
- 2 D-a11y-surface §3 Contrast verification harness — apply (dependent of 1; plan entry a11y §3)
- 3 D-a11y-surface §3 → contrast-verification-harness-setup, discharged — apply (dependent of 1; plan entry a11y §3; the key file edited, `registry.py check` after)
- 4 D-a11y-surface §6 NOT YET MEASURED blockquote, narrowed — apply (dependent of 1; the marker kept for the part left unmeasured)

Validate summary: 22 proposals · 21 apply · 1 rejected for a source the report does not carry (test-plan 5) · 2 raised by main (design-system 1, test-plan 11) · 0 escalations.
- Check 2 (cross-contradiction): none — every pair edits a different line, or the same line in one direction (obs 2 · 3 · 4 at obs-plan.md:70).
- Check 3 (intent-consistency): the report holds to the entry and the plan's acceptance; the scope record is absent (scope: clean, recorded 0).
- Check 4 (absence needs evidence): obs 1's observed-absent rests on the two gate censuses the report carries (contrast.rs non-test part 0; manifest `tracing` 0); a11y 4 keeps the unmeasured part under its marker.
- Check 5 (expected amendments): all seven plan entries covered — six by detectors, design-system §Color Palette by main.
- Check 6 (disproved claims): the report's bullet reads none; `citation-dispositions.md` holds no `claim false` row.
- No boundary widening: the chunk adds no carrier, field, listener or input class.
