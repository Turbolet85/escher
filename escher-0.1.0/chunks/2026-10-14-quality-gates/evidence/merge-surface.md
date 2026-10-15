# Merge surface — CARRYs 11 and 12

Read at implement, 2026-10-15. Base: upstream `main` at `434e0661`, the base of the last upstream sync
(2026-10-13-upstream-sync-polish-and-ship). Fork side: `eb5ba90c` (HEAD at this chunk's take-up). All files
under `packages/blitz-dom/src/`. No upstream pull request is opened (outward-facing, no word for it).

## Upstream's movement since the base

`git fetch upstream main`, then `git ls-remote upstream refs/heads/main` → `e72650a6e679…`, equal to
`upstream/main` (`e72650a6`): three commits past `434e0661` —

- `8d35e562c` Honor baseline-source when aligning atomic inlines (#1198)
- `6032d0b9d` Bump Taffy to main (6c2106e3) (#1212)
- `e72650a6e` vibey-script: Element.attributes, getAttributeNames, clientTop/clientLeft, offsetParent and
  window.event (#1211)

`git diff --stat 434e0661 e72650a6` touches `Cargo.lock`, `Cargo.toml`, `packages/blitz-dom/src/layout/inline.rs`,
`packages/blitz-vibey-script/src/dom/element.rs` and `packages/blitz-vibey-script/src/runtime.rs` — none of
either set below. `git diff --stat 434e0661 e72650a6 --` over the union of both sets reads empty.

## CARRY 11 — the range interaction model (seven upstream files and one new file)

`git diff --stat 434e0661 eb5ba90c --` over the set:

| file | insertions · deletions (fork vs base, `--numstat`) | upstream since base |
|---|---|---|
| `events/keyboard.rs` | 117 · 1 | none |
| `events/pointer.rs` | 58 · 3 | none |
| `form.rs` | 63 · 0 | none |
| `layout/construct.rs` | 23 · 2 | none |
| `mutator.rs` | 64 · 0 | none |
| `node/element.rs` | 71 · 2 | none |
| `node/mod.rs` | 3 · 0 | none |
| `node/range.rs` | 522 · 0 (new file) | absent upstream |
| **total** | 8 files, 921 insertions, 8 deletions | |

## CARRY 12 — keyboard activation, a click's focus, `disabled` by presence (four upstream files)

`git diff --stat 434e0661 eb5ba90c --` over the set:

| file | insertions · deletions (fork vs base, `--numstat`) | upstream since base |
|---|---|---|
| `events/keyboard.rs` | 117 · 1 | none |
| `events/mod.rs` | 3 · 3 | none |
| `events/pointer.rs` | 58 · 3 | none |
| `node/element.rs` | 71 · 2 | none |
| **total** | 4 files, 249 insertions, 9 deletions | |

`events/keyboard.rs`, `events/pointer.rs` and `node/element.rs` carry both changes: the figures above are the
whole fork delta of each file against the base, so the two sets' rows for those three files are the same
lines, not a sum.

## Conflict risk

Against upstream's current `main` the risk is nil: no file of either set moved upstream since `434e0661`, so
both changes apply as they stand. The one upstream change near either set is `layout/inline.rs` (#1198),
outside both. The next sync also meets taffy taken from git at `6c2106e3` (#1212) — a fact for that sync's
take-up (the coupled taffy pin), not for these two changes.
