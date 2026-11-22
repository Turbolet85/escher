# P5 review — revision 1 of 2026-11-22-epoch-2-corrective «Epoch 2 corrective»

**Plan:** `doc:2026-11-22-epoch-2-corrective/plan` rev 6 (rev 4 approved at the take-up). To read it in an editor, open
the rendered copy at `.andromeda/cache/docs/2026-11-22-epoch-2-corrective/plan.md`.
- No plan stands in a chunk's folder.
- Writing in that file changes no record until a `rec.py put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-22-epoch-2-corrective` shows
  a diff of the chunk's records.

**Why this revision:** your word inputs#I4. `/andromeda-implement` has not run on this plan, so no step stands done; it
enters at step 1.

## What changed — a lock-only update of `url`
**Measured on the host at `40ec4749e`:** `bash .github/scripts/ci-leg.sh audit` exits 1, `advisories FAILED, licenses
ok`. The one error is RUSTSEC-2026-0592, `url 2.5.8` at `Cargo.lock:799`. `ci.py conclusion` reads CI#45043508073 red
on Dependency audit alone (17/17 checks, first fail +732 s).

**The lock move, measured on a scratch clone:** `cargo update -p url --precise 2.5.9` moves `Cargo.lock` alone, 6 lines
in and 6 out. The audit leg then reads `advisories ok, licenses ok`. The move:

| package | before | after |
|---|---|---|
| `url` | 2.5.8 | 2.5.9 |
| `form_urlencoded` | 1.2.2 | 1.2.3 (url 2.5.9 requires it) |
| `syn` edge of `bindgen` 0.73.2 | 3.0.6 | 2.0.119 |
| `syn` edge of `cssparser-macros` 0.7.1 | 3.0.6 | 2.0.119 |

- **The two `syn` edges were not asked for.** Cargo makes them itself, the same with and without `--precise`. Both
  crates accept `syn >=2, <4`, and both `syn` packages stay in the lock. The plan names them rather than hand-editing
  the lock (Constraints).
- **No manifest moves.** `Cargo.toml:151` reads `url = "2.5.0"`, which admits 2.5.9.
- **MSRV:** url 2.5.9 states rust-version 1.63 and form_urlencoded 1.2.3 states 1.51, both under the workspace's
  1.91.0.

**One reading moves in the crate, not in escher's code.** In url 2.5.9, `Url::origin()` on a `blob:` URL whose inner
scheme is not `http`, `https` or `file` reads an opaque origin. Before, it recursed, which overflowed the stack on deep
nesting. escher reaches it only in a script document: an anchor's `origin` (`hyperlink.rs:124`) and `location.origin`
(`window.rs:74`). No check reads a `blob:` origin. This is a narrowing, since a crash a page could cause is gone. It
widens no boundary, so neither escalate pattern holds.

## Plan delta (rev 4 → 6)
- **Step 5, new:** `cargo update -p url --precise 2.5.9`, with the four moves above named. There is no other update, no
  manifest edit and no `deny.toml` ignore. Any other line moving means stop and report.
- **The old step 5 ("read the gates") becomes step 6.**
- **Touchpoints:** `Cargo.lock` is added under Files to modify.
- **Entry 4, the guard, widened:** it now also reads the root `Cargo.toml`, `Cargo.lock` and `deny.toml`. It lets
  `Cargo.lock` through by name; that is its allow. Its expected last line is `Cargo.lock` plus the five touchpoints.
- **Entry 5, new — the lock move:** it passes only the 12 diff lines of the table above.
- **Entry 6, new — the audit leg:** `bash .github/scripts/ci-leg.sh audit`, expecting exit 0 and
  `advisories ok, licenses ok`.
- **Renumbering:** the old entries 5-17 become 7-19, unchanged.
- **New acceptance line (security):** the audit leg green, no ignore added, the lock move exact, and the guard's allow
  in place.
- **Rows the plan now cites:** security-plan · decision/dependency-audit-cargo-deny and
  decision/upstream-sync-dependency-set (which names the «lock-only semver-compatible update» form); test-plan ·
  decision/dependency-audit-advisories-and-licenses. They were looked up small; no rows file held them.
- **Scope rev 5:**
  - item 4 is added;
  - two `[premise-corrected]` bullets: the url reading, and HEAD's CI that was in progress at the take-up and has
    since read red.

## Baselines — only the entries the change touches, on the untouched tree `40ec4749e`
| # | entry | baseline |
|---|---|---|
| 4 | guard (widened) | red — last line '' (the subject) |
| 5 | lock move | red — last line '' (the subject) |
| 6 | audit leg | red — exit 1, RUSTSEC-2026-0592 alone (the subject step 5 fixes) |

**Controls** were each fired as the entry on a shared scratch clone at `40ec4749e`:
- **Unmodified:** all three read as they do on the tree.
- **Must-pass** (lock moved, five touchpoints touched): all three read green.
- **Must-fail:**
  - the guard reads red with a root `Cargo.toml` edit, and red with a `deny.toml` edit;
  - the lock-move entry reads red with one extra lock line moved.

The record is `.andromeda/runs/2026-11-22T17-32-32-phase/p5-controls.md`.

**Other baselines stand.** Every other entry keeps its baseline: its `run` is byte-equal and the untouched tree is still
`40ec4749e`. The fast leg reads `seen green` (the previous chunk's wrap).

From step 5 on, every cargo entry builds on url 2.5.9:
- the four behaviour entries;
- the a11y, fast and coverage legs;
- CI.

That is where the move is proven to change nothing else.

## Unchanged from the approved card
Everything else is unchanged from `.andromeda/runs/2026-11-22T16-25-24-phase/p5-card-1.md`:
- the goal (`command.rs` under 800, three upstream bodies at 46 · 167 · at most 19);
- steps 1-4, the behaviour tables and the expected amendments;
- no claims (the ledger holds none for this chunk, and none is added).

## Checks
- **Fence:** parses, 20 entries.
- **`planlint`:** 0 hits. On 12 (d), the new «untouched» claim, on `Cargo.toml` and `deny.toml`, agrees with step 5.
- **`inputs verify`:** 4 entries, every one cited.
- **Research lists:** parsed, 6 items.
- **Validation-1:** the plan follows scope rev 5, item 4 included.

**The list sync was done by hand.** `gate.py scope --sync` refuses this revision: «no `*-implement` run … holds
outcome-… — a revision stands on a stopped implement run». Research's lists were synced to the delta by hand: +1
(`Cargo.lock`), −0. `## Revision 1` was appended and the lists re-parsed.

Research: lists synced to the revision's delta (+1 −0), rev 2; not re-derived since the take-up's rev 1.

**The row read was not re-run.** The take-up's arch and security tasks named the dependency surface, as untouched, and
you said «nothing else changes».

## Beyond the plan
none (standing — `.andromeda/runs/2026-11-22T16-25-24-phase/p5-card-1.md`: none)

Apply? (yes / review / cancel)
