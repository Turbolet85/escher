# Cascade dispositions — 2026-11-16-errors-with-the-action (wrap 2026-11-16T08-47-42-wrap)

## Step 2 — the sweep
Skipped: every `rows-land` dry run of this pass printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file; step 3's leaves read each changed row's delta` (architecture 6 and 1, security-plan 1 and 2, test-plan 2, obs-plan 5), so no pattern file stands and the sweep would refuse «no pattern in any file». Step 3's leaves read each changed row's delta.

What was looked for instead, and seen: P1's pre-pass sweep (`sweep-pre-pass-1.1.txt`, trail `cascade-2026-11-16-errors-with-the-action-pre-pass.json`) over the seven masters for the retired phrasing «installs no subscriber» (`no-sink`, 8 master rows) and the EnvFilter's reach (`envfilter`, 4 master rows). Every `no-sink` master row was set in this pass: architecture `decision/driver-library-side-effect-free` · `decision/driver-spans-only` · `decision/escher-driver-spans-ungated`, security-plan `decision/driver-installs-no-subscriber` · `decision/driver-no-subscriber-no-env` (their `except`), obs-plan `decision/driver-library-installs-no-subscriber` · `decision/headless-stand-no-sink` · `decision/driver-emits-no-event`. `envfilter`: obs-plan `contract/telemetry-sink/rust-log` and `contract/escher-sink-env/rust-log` set; architecture `contract/telemetry/init` and `decision/escher-telemetry-sink` left (fit read the lengthening `not_master`; each still true — init installs one stderr sink; the sink has an EnvFilter layer). Curation homes holding the phrase (`.claude/docs/session-learnings.md:267-268`, about a census's wording) are P3's: no change — they quote a past master wording as a sweep hazard. Lateral binds: no harness command and no log-schema field changed (test-plan §3 ↔ obs-plan §3, §6 untouched).

## Leaves
Pass mode (epoch «Epoch 5 — Errors and explanation» open, markerless 2); 12 groups, 22 leaves; `cascade.py leaves --apply`: `total · blocks 40 · applied 39 · anchor moved 1`.
- architecture-1 · pass · 1 finding · applied 1 · by hand 0 · declined 0
- architecture-2 · pass · 3 · applied 3
- architecture-3 · pass · 1 · applied 1
- architecture+obs-plan · pass · 5 · applied 5
- architecture+security-plan+test-plan-1 · pass · 8 · applied 8
- architecture+security-plan+test-plan+obs-plan · pass · 3 · applied 3 (CLAUDE.md `GENERATED` blocks)
- architecture+test-plan · pass · 1 · applied 1
- obs-plan · pass · 8 · applied 8
- security-plan-1 · pass · 4 · applied 4
- security-plan-2 · pass · 2 · applied 2
- test-plan-1 · pass · 3 · applied 2 · declined 1 — `.claude/docs/tests-summary.md:18#1` (omission of `caused` on the acting verbs' return) read `anchor moved`; declined: no reader acts against the source for want of it, and the acting verbs' `caused` stands in `.claude/rules/verification-harness.md:32` and the service leaf
- test-plan-2 · pass · 1 · applied 1
- unmapped `.claude/rules/host-linux.md` (host) — the host's own leaf, no master source: no comparer's
