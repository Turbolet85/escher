# Cascade dispositions — 2026-11-23-canvas-in-a-headless-shot (wrap 2026-11-23T07-18-30-wrap)

## The search
- Step 2's sweep: `cascade.py sweep --patterns-file cascade-patterns.toml` (main's own four patterns — `m-canvas`,
  `m-cw`, `m-stand`, `m-sesscommon`; trail `cascade-2026-11-23-canvas-in-a-headless-shot.json`, listing
  `sweep-pass-1.1.txt`, read whole). No `rows-land` dry run wrote a pattern file: each of the three landings
  (tx 3271 architecture, 3272 security-plan, 3273 test-plan) printed `patterns: none — the call sets and retires no
  row` — five adds, no set or retire, so no old wording was retired anywhere. The three rejected files' dry runs
  (design-system, obs-plan, a11y-plan) would land 0 and wrote none either.
- Phrasings looked for: a canvas, a shot's or screenshot's pixels («as pixels in a shot / screenshot»), blitz-paint's
  `custom-widget`, the `stand_*` glob, `run stand`, the `stand_` prefix, `stand_canvas_shot`, `session_common`. Not
  looked for: a count of the stand checks written in words with no glob token beside it.
- `total (4 patterns) · 96 rows over 25 files` · `per class · new 13/3 · standing 24/7 · leaf 57/17 · curation 2/2 ·
  base 0/0`.

## Masters — new 13 (amended)
The pass's own text, the five rows it landed: architecture dec-vd5ewvhr (decision/canvas-crosses-as-pixels),
dec-rlsleth2 (decision/harness-paints-custom-widgets); security-plan dec-vsucegk2
(decision/canvas-content-crosses-as-shot-pixels), dec-sy7cum7k (decision/harness-blitz-paint-custom-widget);
test-plan dec-nqizbumm (decision/app-widget-shows-in-shots) — 13 rows across the four patterns, each amended.

## Masters — standing 24 (no change)
- `m-canvas` standing 14 — another sense of `canvas`: the replaced-element list (architecture dec-shqlwmib,
  dec-yvakvbdt), wasm_hello's canvas (dec-rzvlmuor, dec-sfcwutjd), the layout answer's `canvas` key (architecture
  ctr-px3dwwji, layout-templates ctr-nikwwkdh), Circle Drawer's canvas handlers (architecture ctr-rq3jq44s, test-plan
  ctr-clnjioqg, dec-b3hlr6zg), the `circle-canvas` id (layout-templates ctr-hbpncsrs), the wasm canvas
  (design-system ovw-ijnxaifd); or a true claim of other pixels in a shot: the select list (security-plan
  dec-goyp2c5r), the masked password (dec-hhuvkfkm), the open list's log absence (obs-plan dec-565r3cp3). None states
  how a canvas's content crosses; the new rows stand beside them.
- `m-cw` standing 4 — custom content as an `object` (architecture dec-yazarqox, ctr-xamiigmw), widget events forwarded
  (dec-vyyisbox), the attribute replay behind blitz-dom's feature (security-plan dec-ifvrmapd): true, untouched by a
  feature turned on in blitz-paint.
- `m-stand` standing 5 — the `stand_*` glob or prefix as a class (architecture ctr-nwzuxlhy, test-plan ctr-exq4f34j,
  ctr-eiuupfq4, dec-aovxp22d, a11y-plan dec-3qlfzc22): no count stated, `stand_canvas_shot` joins by the glob.
- `m-sesscommon` standing 1 — architecture dec-ve2bmybh names the module, no count.

## Curation — 2 (preserved, P3)
- `.claude/docs/session-learnings.md:88` (`m-stand`) — states no count; no change.
- `.claude/rules/testing.md:70` (`m-sesscommon`) — a dated Session Additions entry quoting «36 readers» of
  `session_common` as read on its date; preserve-verbatim, not edited by the cascade, routed to P3 (no extension
  owed: the entry is a dated reading, not a standing count).

## Base — 0

## Leaf — 57 rows over 17 files
Step 3's: `cascade.py leaves` reads them from this listing.

## Leaves
Set: `cascade.py leaves` (pass mode; epoch «Epoch 3 — The canvas painted» open, markerless 3) — `total · pass · 15 group(s) · 24 leaves · 750666 B · unmapped 1`; `sweep leaves · 17 file(s) from sweep-pass-1.1.txt`; every `swept` row read `no source row matched` (the pass retired and set no row, so no source line was taken out). Unmapped `.claude/rules/host-linux.md` (host) — no source states it, no comparer's.
Batch `leaf` 15/15 pass; `leaves --apply --dry-run` refused group architecture+security-plan+test-plan-3's receipt (`read: CLAUDE.md` quoted «- 2026-11-18: A docs leaf states no per…», the last line opens «<!-- USER:session-learnings end -->») → one RULED re-spawn (`leaf-r1`, 1/1 pass); its earlier file at `.raw-leaf-architecture+security-plan+test-plan-3.md`.
Apply: `total · blocks 14 · applied 13 · held 1` — no leaf line written by hand.
- added-1 (a11y-summary.md) — pass · 0 findings · 1 stands (line 14).
- added-2 (design-summary.md) — pass · 0 findings · 3 stands (27, 28, 29).
- added-3 (obs-summary.md) — pass · 1 finding applied (43, the canvas crossing) · 2 stands (12, 49).
- added-4 (rules/a11y.md) — pass · 0 findings · 2 stands (35, 42).
- architecture-1 — pass · 1 applied (blitz-paint.md:32, the harness's feature) · 3 stands (conventions.md 48, 6; blitz-dom.md 20).
- architecture-2 — pass · 0 findings · no swept line.
- architecture-3 — pass · 1 finding declined (services/seven_guis.md:38: the check lives in blitz-tests and the stand only boots it; the crossing stands in services/escher-driver.md, the driver's leaf) · 1 stands (stack.md 39).
- architecture+security-plan+test-plan-1 (gotchas.md) — pass · 0 findings · no swept line.
- architecture+security-plan+test-plan-2 (services/escher-driver.md) — pass · 3 applied (32, 11, 61) · 4 stands (16, 20, 62, 63).
- architecture+security-plan+test-plan-3 (CLAUDE.md) — pass, re-spawned once · 1 applied (122) · 4 stands (98, 110, 93, 95).
- architecture+test-plan — pass · 2 applied (services/blitz-test-harness.md:11, commands.md:61) · 5 stands (commands.md 52, 14, 62, 63; blitz-test-harness.md 15).
- security-plan-1 (security-summary.md) — pass · 1 applied (32) · 1 stands (13).
- security-plan-2 (rules/security.md) — pass · 2 applied (21, 28) · 2 stands (17 ×2).
- test-plan-1 (tests-summary.md) — pass · 1 applied (26) · 8 stands (13, 46, 16, 18, 21, 29, 30, 43).
- test-plan-2 — pass · 1 applied (rules/verification-harness.md:37) · 6 stands (testing.md 15, 41, 48; verification-harness.md 15, 17, 21).
Every `@leaf_stands` read and held: none of the moved counts is stated as a closed count those lines hold (`stand_canvas_shot` joins by the `stand_*` glob and prefix; it declares `mod session_common;` and no `mod fixtures;`, and the `session_common` lines are per-check narratives that are not lists of every declarer).
Named and carried, no comparer's for this chunk: `.claude/rules/verification-harness.md:21` reads «`run stand` runs the 24 `stand_*` readers of the shared module» while `git grep -l '^mod fixtures;' -- 'tests/blitz-tests/tests/stand_*.rs'` counts 31 — a count that predates this chunk (`stand_canvas_shot` declares no `mod fixtures;`), and per the 2026-11-18 learning a leaf states no such count.
