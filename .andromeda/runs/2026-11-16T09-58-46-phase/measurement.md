# Taffy past #1372, measured before planned

Measured 2026-11-16, on this host, in the trial worktree `target/tmp/trial-taffy-1372`, detached at HEAD
`06c8ed8a25344134bfdff52da959cfc164be46b8`, with its own target dir. The operator's directive (`inputs#I1`): measure
the take (build, conflicts, legs, stand) before planning, and stop with the measurement if it does not land cleanly.
The founder's ratification of the reversed-scroll-origin change for this take: `inputs#I2` item 1.

The trial's staged diff against HEAD is `evidence/take.patch` (16 files, +632 −120).

## The picks

Each `git -c rerere.enabled=false cherry-pick -n {sha}`, in upstream order:

| # | sha | subject | conflicts | resolution |
|---|---|---|---|---|
| 1 | `0980cce5f` | Support reversed scroll origins using Taffy's `ScrollableOverflow` (#1294) | `document.rs` 1 hunk · `layout/writing_mode.rs` 1 · `node/node.rs` 2 (`Cargo.toml` and `Cargo.lock` auto-merged) | below |
| 2 | `ebacd68bd` | Hit test elements that have no computed styles yet (#1304) | none | — |
| 3 | `35cdfc859` | Bump Taffy to 5e035cad (taffy#1385, taffy#1386) (#1376) | none | — |
| 4 | `930448918` | Bump Taffy to pick up FloatInsets and zero-height float segments (#1377) | `Cargo.lock` | ours, re-resolved |
| 5 | `c51dd246c` | Bump Taffy to stop double-counting collapse-through margins (#1393) | `Cargo.lock` | ours, re-resolved |
| 6 | `752ed2e49` | Pin Taffy to main for taffy#1401 (#1451) | `Cargo.lock` | ours, re-resolved |
| 7 | `b4952e15f` | Bump Taffy to main (cache_get takes &self) (#1557) | `Cargo.lock` | ours, re-resolved |

### The three conflicts of `0980cce5f`

- `packages/blitz-dom/src/document.rs` — context only. Upstream's hunk wraps its writing-mode prefs in a block and adds
  `layout.css.writing-mode.sideways.enabled`, inside a pref block that escher's tree no longer holds there (escher's
  prefs stand in `init_stylo_prefs`, under Stylo 0.22's names; no `sideways` pref exists in `stylo-0.22.0`). Escher's
  side taken; nothing of the block enters.
- `packages/blitz-dom/src/layout/writing_mode.rs`, the second body of `physical_unrounded_geometry` — HEAD adds the
  box's physical location alone; upstream also subtracts the box's own `scroll_offset()` and passes `physical_layout`
  a fourth argument (is a scroll container). Taken: the fourth argument; kept: escher's ratified own-scroll rule (no
  subtraction of the box's own offset). `placer_geometry` (auto-merged) still subtracts each placer's offset.
- `packages/blitz-dom/src/node/node.rs` — (a) imports: both sides, then `CopyStyleGetters` dropped (below);
  (b) `hit_inner`'s `matches_self`: escher's `own_x`/`own_y` reading kept over upstream's scroll-shifted one. The
  content test auto-merged to upstream's form: the point against `scrollable_overflow_rect`'s four sides plus the
  client offset (border, and the scrollbar gutter on its RTL / vertical side).

## The lock

`cargo metadata --format-version 1` (no `--locked`): `Locking 1 package to highest compatible version` · `Adding
taffy v0.15.0 (https://github.com/DioxusLabs/taffy?rev=a3997d55bacd1fb6cd12fe31db0be15b20f91528#a3997d55)`. Against
HEAD, `Cargo.lock` moves one line: the `taffy` package's `source` from `…?rev=e90751fd…#e90751fd…` to
`…?rev=a3997d55…#a3997d55…`; version `0.15.0`, no checksum, dependency list unchanged. `Cargo.toml`: the taffy rev
line alone, the features unchanged.

## The legs, in order (`bash .github/scripts/ci-leg.sh {leg}` in the trial)

| run | leg | exit | time | reading |
|---|---|---|---|---|
| 1 | `fast` | 101 | 30 s | clippy: `blitz-dom` 2 errors — E0432 `stylo_taffy::CopyStyleGetters` unresolved (`node.rs:37`); E0308 `scrolling.rs:679`, escher's `scroll_into_view` destructures `scroll_state` as a pair, now a triple |
| — | edit | | | `CopyStyleGetters` import dropped; `let (current, min, max) = …; if min == Point::ZERO && max == Point::ZERO { continue; }` — `cargo check -p blitz-dom --all-targets`: clean |
| 2 | `fast` | 101 | 186 s | test: `select_popup` `a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes` panicked at `node.rs:1189` «layout_style() called on a node without computed styles»; backtrace `Node::layout_style` ← `LayoutPassState::physicalise_and_round_layout` (upstream's new overflow read), under the workspace's features (`writing-mode` on); passes under `-p blitz-tests` alone |
| — | edit | | | private `fn is_scroll_container(node: &Node) -> bool` in `writing_mode.rs`: no computed styles → `false`, else `layout_style().overflow()` read as upstream does; used at the three sites (`physicalise_and_round_layout`, the second bounds body, `placer_geometry`); `select_popup` 25/25 |
| 3 | `fast` | 1 | 1 s | rustfmt on the guard's `placer_geometry` call; `cargo fmt --all` |
| 4 | `fast` | 0 | 349 s | fmt, clippy, test, ci-scripts (`Ran 118 tests`, `OK`), embolden — green |
| | `msrv` | 0 | 50 s | `cargo +1.91 build --workspace --locked` |
| | `audit` | 0 | 2 s | `advisories ok, licenses ok` |
| | `doc` | 0 | 6 s | |
| | `build` | 0 | 8 s | |
| | `counter` | 0 | 27 s | |
| | `wasm` | 0 | 86 s | |
| | `a11y` | 0 | 17 s | |
| | `stand-repeat` | 0 | 848 s | `stand-repeat: 3 of 3 rounds green` |

Not run: `coverage` (llvm-cov, per-crate floors) and the WPT runner's suites (report-only on the fork).

## Tallies against HEAD (ANSI stripped, `test result:` lines summed)

HEAD: the main tree's `target/ci-logs/` (test 09:31Z, stand-repeat 09:44Z, on the code of `4c8ac67ab`, the last code
commit before HEAD).

| leg | HEAD | the take (trial) |
|---|---|---|
| `test` | 242 binaries · 1501 passed · 0 failed · 13 ignored | 242 · 1510 · 0 · 13 |
| `embolden` | 1 · 6 | 1 · 6 |
| `a11y` | 14 · 115 · 0 | 14 · 115 · 0 |
| `stand-repeat` | 258 · 1380 · 0 · 24 | 258 · 1380 · 0 · 24 |

The +9 in `test` are upstream's: `tests/blitz-tests/tests/inline_box_scrollable_overflow.rs` (+183 lines) and
blitz-dom's `mutator.rs` unit test `hit_testing_before_the_first_resolve` (`ebacd68bd`). `FAILED` lines in the trial's
`test.log` and `stand-repeat.log`: 0; the three `panicked` matches in `stand-repeat.log` are the passing test name
`a_panicking_paths_kept_check_holds_the_panicked_step_and_replays_green`, as in HEAD's log. The committed digests
(`stand_cross_os_readings`), the committed Counter checks (`stand_replay`, seven_guis `recording`) and every
stand-pinned reading pass unchanged.

## Verdict

The take lands: 7 of 7 picks, the three conflicts the founder was told of, each resolved keeping escher's ratified
reading; three seam edits (one import, escher's own into-view scroll on the triple, a no-panic guard on upstream's new
overflow read that escher's `select_popup` test reached); every leg measured green; no tally moved but upstream's
added tests; the stand's readings unchanged.

The trial worktree `target/tmp/trial-taffy-1372` stands as the record until the take lands in the main tree.
