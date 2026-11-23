# P5 review — 2026-11-23-canvas-placement-and-text · revision 1 (RUSTSEC-2026-0594)

The plan's render, to open in an editor: `.andromeda/cache/docs/2026-11-23-canvas-placement-and-text/plan.md`.
No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-canvas-placement-and-text` is a
diff of the chunk's records. The plan is at rev 5; the plan the implement executed was rev 2. The `res-evkq636g` line is now an implementation
note: check 2 lets *Expected amendments* name only the seven masters.

## Goal (added paragraph)
The plan takes the patch for RUSTSEC-2026-0594 before the operator pass. The advisory is on `cookie` 0.18.2, «Private
cookie nonces can repeat in processes that fork», and 0.18.3 is the patched release. `cookie` reaches the lock through
`cookie_store` 0.22.1 ← `reqwest` 0.13.5 ← `blitz-net`; `http-cache-reqwest`, `reqwest-middleware` and `rdme` also pull
in `reqwest`, and `reqwest` depends on `cookie` directly too. The move is lock-only: `cookie`'s version and checksum
lines alone. The `syn` edges of `bindgen` and `cssparser-macros` stay at 3.0.6 by edit. That is the second instance of
residual `res-evkq636g`; no new residual is opened, no manifest line changes and no `deny.toml` ignore is added
(inputs#I8 · I9 · I10). The wrap records the reviewed pin.

## The one added step
- Steps 1–9 are done as the implement recorded them (`2026-11-23T10-01-43-implement`). `/andromeda-implement`
  re-enters at **step 10**.
- **Step 10:** edit `Cargo.lock`'s `cookie` entry with the Edit tool, changing `version` 0.18.2 → 0.18.3 and the
  checksum `1a373e36…f2f87` → `d55f2d6c…615f8a`, those two lines and nothing else. No `cargo update`. Then fire gates
  9, 14 and 12, in that order.

## The readings (scratch clone of `d43bc0e7f` carrying the chunk's tree, `$TMPDIR/scratchpad/cookieclone`)
**The lock diff of the move, line by line** (`evidence/cookie-lock-hand-edit.diff`):
```
@@ -2042 +2042 @@ name = "cookie"
-version = "0.18.2"
+version = "0.18.3"
@@ -2044 +2044 @@ source = "registry+https://github.com/rust-lang/crates.io-index"
-checksum = "1a373e3602691c3cdea496d2f0ee5935151e6168fe87739483c463db1b2f2f87"
+checksum = "d55f2d6c30eeec7d5c6f98bd4395d4abb1895629eab97ee6522d340623615f8a"
```
No other package moves: no edge, version, new package or removed package.

For comparison, `cargo update -p cookie --precise 0.18.3` gave 4+ 4−. Besides the two lines above, it moved
`bindgen` 0.73.2 (line 856) and `cssparser-macros` 0.7.1 (line 2258) from `"syn 3.0.6"` to `"syn 2.0.119"`
(`evidence/cookie-lock-move.diff`). It is not taken.

**`cargo build --workspace --locked`** on the edited lock (`ci-leg.sh build`, the verified skia archive):
- exit 0 and no `warning:` line; `Finished dev profile … in 48.17s`;
- `cookie v0.18.3`, `cookie_store`, `bindgen v0.73.2`, `blitz-net` and every workspace crate compiled;
- the lock still read 2+ 2− afterwards (`evidence/cookie-lock-build.log`).

**`bash .github/scripts/ci-leg.sh audit`** on the edited lock: exit 0, last line `advisories ok, licenses ok`. On the tree
as it stands it reads exit 1, `advisories FAILED, licenses ok` (RUSTSEC-2026-0594).

`cargo tree --locked --workspace --all-features -i cookie` exits 0 and leaves the lock unchanged.

## The gates for the step
| entry | change | baseline on the tree | must-pass on the clone with the edit |
|---|---|---|---|
| 9 the lock probe | expect `last line 1+0` → `3+2` (the `parley` line + the two `cookie` lines) | red · `1+0` | green · `3+2` |
| 12 the audit leg | `new = true`, expect `exit 0` + `last line advisories ok, licenses ok` | red · RUSTSEC-2026-0594 | green |
| 14 `ci-leg.sh build` (new) | `cargo build --workspace --locked`, expect `exit 0` + `lacks warning:` | green · 11.72 s | green |
Unmodified, the clone read what the tree reads for 9 and 12 (`runs/2026-11-23T11-01-36-phase/p5-controls.md`). One
side is not controlled: no input on hand makes the workspace build warn, so `lacks warning:` has no fail-side reading.

**What the implement re-fires:**
- **Re-fired: 9, 14 and 12.** 9 counts lock lines, so it moves. 14 compiles every workspace crate against `cookie`
  0.18.3, its one reach. 12 reads the advisory.
- **Not re-fired: 1–8, 10, 11 and 13** (unit, stand, fast, doc, stand-repeat). The move changes nothing they read.
  No test reads a cookie: `git grep -i cookie` finds the store switched on at `packages/blitz-net/src/lib.rs:90-91`
  behind the `cookies` feature, plus comments.
- **Step 9's audit firing** is the red the implement already recorded (the stop). Step 10 re-fires it.

## Checks
- **Mechanical:** the fence parses (17 entries); planlint reports 0 hits; inputs read 10 entries, 0 drifted.
- **Lists and revision sync:** `scope lists: parsed 10 items`; `scope sync: PASS · delta +0 −0`. Research is synced to
  the revision's delta (+0 −0), now rev 3. It was not re-derived since the take-up's rev 2.
- **Unchanged claims:** planlint check 12 (d) lists seven «unchanged» claims, and I read each as consistent.
- **Escalate patterns:** none applies; a patch release crosses no boundary.
- **Validation-1:** aligned. The scope gained the closed cookie bullet (rev 6), and the plan follows it.
- **Capabilities:** none claimed or changed. The chunk holds no claim; `v040-09` stays pooled, as at the take-up.

## Beyond the plan
none (standing — `.andromeda/runs/2026-11-23T08-25-01-phase/p5-card-1.md`: `none`)

Apply? (yes / review / cancel)
