# Cascade dispositions — 2026-11-18-cold-agent-bar (wrap 2026-11-18T09-02-58-wrap)

Step 2 sweep: `cascade.py sweep --patterns-file cascade-patterns-obs-plan-1.toml` (the one file a dry run wrote; the six other landings changed prose alone and wrote none — their deltas are step 3's), part `sweep-pass-1.1.txt`.

- obs1-1 (`lint_calls|screenshot_calls|advance_calls`) — masters 9 rows (architecture 3 · obs-plan 6): 3 new (this wrap's own graph/actions/layout rows, «written after advance_calls») · 6 standing, a true claim sharing the token (the per-field rows of the three original counted verbs). No change.
  Leaves 3: `.claude/rules/verification-harness.md:26`, `.claude/docs/obs-summary.md:17`, `.claude/docs/tests-summary.md:16` list the three fields as the whole capability-unused set — stale, the leaf comparers' (step 3).
- obs1-2 (`tool_call`) — masters 57 rows (architecture 11 · security-plan 5 · test-plan 8 · obs-plan 33): 10 new (this wrap's) · 47 standing, a true claim sharing the token (the tool_call span's own contract and its readers). No change.
  Leaves 25: true claims about the tool_call span, except the three obs1-1 leaves above (same lines) — step 3.

## Leaves
`cascade.py leaves --all` (the epoch read `closes` at this point — P5 then adds two markerless entries), 15 groups · 25 leaves · unmapped 1 (`.claude/rules/host-linux.md`, the host's — no comparer's). One batch of 15 comparers, 15 of 15 pass. Apply: `total · blocks 33 · applied 21 · reshaped 1 · held 11`; 2 hand writes.

Applied (21): conventions.md:17 (the `mimalloc` exception) · blitz-traits.md:15 (stale defaults) · seven_guis.md:38 (stale: the click after a scroll now waits out the fade) · escher-telemetry.md:6 ×2 (six counted verbs; the span-name read) · escher-driver.md:32 (graph and actions to the provider) · gotchas.md:27 (inert focus vs disabled/hidden, the SC 2.4.3 gap) · CLAUDE.md:57 (`panic.payload` redacted) · CLAUDE.md:58 (`fmt` beside wasm_hello outside `--locked`) · obs-summary.md:17 ×2, :12 · observability.md:26 ×3, :18 · security-summary.md:19, :33 (the chained panic hook unscrubbed) · security.md:23 · verification-harness.md:30, :36 (`from_component_alone`, `boot_alone`).
By hand (2): verification-harness.md:26 and tests-summary.md:16 — the capability-unused set read the four 0.2.0 fields; now the eight (the step-2 sweep's obs1-1 leaf rows, no comparer flagged them); tests-summary.md:16's per-file test count «(38)» removed (a leaf names tests, never counts them). Drafts landed by `rec.py harness land`, tx 2125–2126.
Declined (12):
- reshaped escher-telemetry.md:18 — a backslash escape the line did not carry; the detail (carriage returns) stays in its master.
- detail additions a reader would not act against, the detail staying in its master: a11y-summary.md:26 · conventions.md:57 · conventions.md:7 · blitz-dom.md:12 · blitz-dom.md:15 · security-summary.md:13 · security.md:17.
- dioxus-native-dom.md:17 (stale) — the leaf is true by the code (`target_of` is called from `execute.rs` and `actions.rs`, `git grep`); the master row `ctr-xvgrmds5` («read by the driver's locate alone») is the narrower statement — routed at P5.
- escher-driver.md:61 and :29 — the comparer's «eighteen» is a count no cited source states; not written.
- tests-summary.md:29 — the leaf names the a11y leg's membership by the founder's chosen label; not false, not changed.
