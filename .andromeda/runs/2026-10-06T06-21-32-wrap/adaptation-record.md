# Adaptation record — the 2026-10-06T06-21-32 0-pending wrap (Epoch 1 boundary)

Master at Setup: 7 records · complete 7 · pending 0 · gated 0 → the no-op path (Setup step 6).

## 1. Operator route adaptation — "Upstream sync" at the head of Epochs 2–6

- **Authority:** word: "insert an "Upstream sync" entry at the HEAD of each of Epochs 2-6, ahead of its first entry
  (Epoch 2: ahead of "Stable element ids", which keeps its PREREQ)" — the founder's direction of 2026-10-06, relayed by
  the operator in this wrap's request. A recorded direction naming the entries and their slot satisfies the trajectory
  gate (route-resolve §Gradient).
- **Why (the direction's own reason):** Blitz upstream keeps adding functionality the project needs, so `upstream/main`
  is merged small and often while our changes stay additive, our tests and CI proving our logic survived.
- **Applied** — five markerless entries, each `splice.py append --after-line {header} --lines 2` (entry + `   ↓`),
  bottom-up, every write `term lf`:

  | Epoch header | New first entry (line) | Previous first entry, now second |
  |---|---|---|
  | Epoch 2 — Element identity | `Upstream sync ahead of element identity` (26) | Stable element ids (28) |
  | Epoch 3 — Observation model | `Upstream sync ahead of the observation model` (35) | Snapshot model (37) |
  | Epoch 4 — Driver core | `Upstream sync ahead of the driver core` (46) | Driver session (48) |
  | Epoch 5 — Agent surfaces | `Upstream sync ahead of agent surfaces` (61) | Driver CLI (63) |
  | Epoch 6 — Polish & ship | `Upstream sync ahead of polish and ship` (72) | Stand contrast harness (74) |

  Each entry's scope hint: `upstream/main merged, our changes kept additive; our tests and CI prove our logic survived
  (per intent §Principles)`.
- **Titles are distinct per epoch** so the five chunks do not share a title (phase mints a marker from the date plus a
  title slug, and the master refuses a duplicate marker). This is wording only; the direction's slots are unchanged.
- **Annotations:** none moved. The PREREQ `close rust gate deferral (… doc …)` stays on Stable element ids, now line 28,
  as the direction names. That overrides route-resolve §Operator-requested adaptation's default, which re-pins
  next-entry PREREQs onto a new first entry. The CARRYs on lines 39 · 58 · 82 · 84 stay with their entries.
- **Read back:** `route.py epoch` gives 6 headers · 35 entries (Epoch 2: 4 · 3: 5 · 4: 7 · 5: 5 · 6: 7, all markerless).
  `route.py cursor` gives next `working-route.md:26 · Upstream sync ahead of element identity`, half-promote 0 of 7.
  `route.py pins` gives 5 freight blocks, the same set as before the edit.
- **Intent edit rides:** `escher-0.1.0/intent.md` (operator-edited, uncommitted at Setup) adds the Principles bullet
  "Upstream stays in reach" and drops "Upstream sync is out of 0.1.0" from §Out of this version.

## 2. Boundary runs riding this commit

- `.andromeda/runs/2026-10-06T05-17-25-code-audit/` + `.andromeda/code-metrics.ndjson` (1 line): the Epoch 1 code
  audit. `gate.py hygiene` returns clean.
- `.andromeda/runs/2026-10-06T05-48-49-evolve-diagnose/`: its `__pycache__/` (one `.pyc`) was deleted, then
  `gate.py hygiene` returned clean.
- The prior wrap's post-commit tail (expected-transient): `friction-log.ndjson` +2 (its gates checkpoint),
  `runs/2026-10-06T04-34-08-wrap/evolve-….json` (that append's trail), `health-….json` (its check-1 read), and the
  session-end hook's line in `session-handoff.md`, which the full overwrite now replaces.

## 3. Duties this path always runs

- Gated premise re-check: 0 `gated` records, so there was nothing to re-verify.
- Curation: the conversation carried no correction, so none ran.
- PROVISIONAL items: untouched by the operator's word. They stay PROVISIONAL, and the founder will rule on them himself.
