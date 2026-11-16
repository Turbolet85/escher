# Cascade dispositions — 2026-11-16-layout-explained (wrap 2026-11-16T03-41-54-wrap)

## Step 2 — the sweep
- Landings: architecture tx 1314 · security-plan tx 1315 · test-plan tx 1316 · obs-plan tx 1317.
- Pattern files: none. Each dry run said `patterns: none: {n} set(s) and retire(s) changed prose alone — no file`
  (architecture 1 · security-plan 1 · obs-plan 2) or `patterns: none — the call sets and retires no row` (test-plan);
  every other operation is an add. No pattern in any file → the sweep is skipped with that line; step 3's leaves read
  each changed row's delta.
- Main's own read of the old wording, all seven masters (`rec.py find --regex 'span per call it runs|No driver call
  puts screen content|MCP tools check (installs )?its own recorder'`, a master each): 0 hits in each — the three set
  rows' old phrasings stand nowhere else, the set rows themselves now carrying the new text.
- Lateral binds (test-plan §3 ↔ obs-plan §3 / §6, a11y schema): untouched — no operation changed a harness command or
  a log field.
- Curation homes and judgment bases: the old phrasings searched above are mechanism wording of a span and log rule;
  the pre-pass sweep's `curation` row (`rules/testing.md:70`, «36 readers» of `session_common`) is P3's.

## Leaves
- Mode: pass (`cascade.py leaves`, baseline b7a30649; epoch «Epoch 5 — Errors and explanation» open, markerless 3);
  12 groups · 22 leaves; `unmapped .claude/rules/host-linux.md (host)` — the host's leaf, no source states it, no
  step-3 edit.
- Findings 24 · applied 24 (by the tool, `--apply` of all 12 files) · by hand 0 · declined 0 — the tool's `total ·
  blocks 24 · applied 24`.
  - architecture-1: 2 applied (blitz-dom.md:15 — the reader and its re-exports).
  - architecture-2: 1 applied (dioxus-native-dom.md:14 — the re-exports beside `NodeId`).
  - architecture-3: 0 findings.
  - architecture+obs-plan: 2 applied (escher-telemetry.md:32).
  - architecture+security-plan+test-plan-1: 5 applied (escher-driver.md:6, :37, :49, :62 omissions; :57 stale —
    `BoundingRect` is now re-exported).
  - architecture+security-plan+test-plan+obs-plan: 3 applied (CLAUDE.md:21, :36, :122).
  - architecture+test-plan: 0 findings.
  - obs-plan: 4 applied (obs-summary.md:12, :49; observability.md:18, :24).
  - security-plan-1: 2 applied (security-summary.md:23).
  - security-plan-2: 2 applied (security.md:22).
  - test-plan-1: 2 applied (tests-summary.md:21, :44).
  - test-plan-2: 1 applied (testing.md:23).
- Each applied for the reader who would otherwise act against the source: a new public reader and session read the
  leaves did not name, a re-export one leaf denied, and two rows' changed exceptions.
