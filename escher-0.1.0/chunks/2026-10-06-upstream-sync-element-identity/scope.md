# Scope — 2026-10-06-upstream-sync-element-identity

**Working entry (verbatim):** Upstream sync ahead of element identity — upstream/main merged, our changes kept additive; our tests and CI prove our logic survived (per intent §Principles)

**Intent anchor:** `escher-0.1.0/intent.md` §Principles, "Upstream stays in reach": an "Upstream sync" chunk at each epoch boundary merges `upstream/main`, and our tests and CI prove our logic survived. Small and often, with our changes mostly additive (founder, 2026-10-06).

## What it builds
- A merge of `upstream/main` (DioxusLabs/blitz) into `build/escher-0.1.0`, pinned at **`23354585`** ("Update Taffy for cyclic percentage grid minimum contributions (#1070)"), as measured at 07:52Z (inputs#I1, re-measured at take-up from this checkout's `upstream/main` ref: merge base `0f60502e`, upstream 11 commits ahead, ours 16 ahead, base..upstream 17 files +607/−247).
- The merge lands as a merge commit on the build branch, not a rebase. The branch is pushed and shared: it tracks `origin/build/escher-0.1.0`, and CI ran on the pushed shas `ab936a9e` and `d4113768`. Global rules forbid force-pushing, so a rebase that rewrites the 16 commits on our side is out.
- The pin is the measured sha. If `upstream/main` moves before /implement, the merge still targets `23354585`. Later commits go to the next sync, per "small and often". Changing the pin is the operator's call. At P3, `git ls-remote upstream refs/heads/main` still read `23354585`.
- After the merge, every escher logic proof still passes: the stand checks (`stand_boot`, `stand_counter`, `stand_crud`, `stand_flight_booker`, `stand_timer`), the telemetry tests (`telemetry_*`), `dioxus_falsy_disabled`, the agent-run and cold-agent contract tests, the incremental/non-incremental layout oracle, and the full local gate (`ci-leg.sh fast` + `ci-leg.sh doc`). Fork CI on the pushed merge must come back green.

## The upstream delta (11 commits, from `git log 0f60502e..upstream/main`)
- Layout and dependency pins: Taffy bumps (#1061 auto margins beside floats, #1070 cyclic percentage grid contributions), `align-content` for block, inline and table (#1059), self-alignment of absolutely positioned boxes (#977), Taffy's `normal` alignment keyword in `stylo_taffy` (#1063), Parley bumped to latest main (#1065).
- DOM API: inline fragment rect computation moved onto `Node` (#1062). `Node::inline_fragment_boxes` exposed, and `BaseDocument::inline_fragment_rects` now returns an iterator (#1060). The JS `document.children` binding in blitz-vibey-script (#1069).
- WPT: the pin is updated (#1068), and root titles are included in native checkLayout subtest names (#1067). Upstream's WPT workflows are upstream-only on the fork, guarded by `github.repository`.
- Files: Cargo.toml, Cargo.lock, blitz-dom (`cssom.rs`, `document.rs`, `layout/inline.rs`, `node/node.rs`, `node/text.rs`), blitz-paint `render.rs`, blitz-vibey-script (`dom/{document,element,node,stylesheet}.rs`, `runtime.rs`), stylo_taffy (`convert.rs`, `wrapper.rs`), `wpt/WPT_COMMIT`, and `wpt/runner/src/test_runners/attr_test.rs`. No `.github/` file changed.

## Boundaries and surfaces
- **Overlap**, meaning files both sides changed since the base: `Cargo.toml`, `Cargo.lock` and `packages/blitz-dom/src/node/node.rs`. At take-up an in-memory `git merge-tree --write-tree HEAD upstream/main` exited 0 with tree `aed58ca8`, which means no textual conflict. Our `node.rs` delta is two rustdoc link fixes from the real-rustdoc-gate work. `[premise-corrected: the delta adds one intra-doc link, `[item_alignment]` on `pub fn oof_item_alignment`, which resolves to a `pub fn` in the same `pub mod convert`. Our two `node.rs` doc fixes survive the merge (research.md)]` No new rustdoc warning source is visible statically. The `doc` leg (`RUSTDOCFLAGS=-D warnings`) on the merged tree remains the proof.
- **Coupled dependency pins** (CLAUDE.md Critical Warnings): upstream moves taffy `49152ca2` → `4142c9d8` and parley `718dcb72` → `e41dfea5`, and leaves skrifa at 0.44. The merge takes upstream's pins as one set. The parley bump keeps the skrifa/vello coupling within 0.44. Verified at P3: the merged lock holds one skrifa 0.44.0 beside parley 0.11.0 (`e41dfea5`) and vello 0.11.0, and `cargo metadata --locked` on the merged tree exits 0. A failure here at /implement is a halt, never a one-sided fix.
- **Additivity**: the merge introduces no NEW edit by us to upstream-owned code beyond what a conflict or a gate failure strictly requires. Any such fix is named, minimal, and recorded as non-additive. Our pre-existing fork changes to upstream crates (blitz-dom `traversal.rs`, `layout/replaced.rs`, `node/node.rs`; blitz-test-harness; dioxus-native-dom `mutation_writer.rs`; blitz-vibey-script `document.rs`) are carried as they are.
- `Cargo.lock` is not regenerated. The auto-merged lock resolves under `--locked` as it stands, and its crate name+version set is identical to HEAD's, with only 7 git `source` lines moving (P3). Builds stay `--locked`. The dependency audit leg (`ci-leg.sh audit`) runs because new git revs enter the graph (security rules §Dependencies).
- No new crate, port, env var or listener (arch §Occupied Resources unchanged).

## Out of scope
- Any element-identity work (Stable element ids and later entries).
- Bumping any dependency beyond upstream's own set. Sending our changes upstream. Upstream's WPT conformance numbers (the WPT runner is built and gated only as the workspace gates it).
- Adopting the new upstream APIs. `[premise-corrected: `children` (#1069) is a `pub(crate)` JS `document.children` binding in blitz-vibey-script, not a blitz-dom Rust API (`git show d3ced119`)]` The Rust-side candidate for Snapshot model bounds is `Node::inline_fragment_boxes` (and the iterator-returning `inline_fragment_rects`). This chunk only records it for later chunks.

## Folded freight and verdicts
- Freight on the taken-up entry: none (`route.py pins`: 0 blocks on line 26). The `PREREQ: close rust gate deferral (… doc …)` stays on Stable element ids (line 28), as the founder's adaptation directs.
- That PREREQ is the cold-agent pipe's `doc` leg recorded `not run — defer` for zero Rust delta, not a failure (verified: `chunks/2026-10-06-cold-agent-run-pipe/report.md:200`). This merge carries Rust delta, so its gate runs `doc` anyway. A green `doc` here is evidence the next chunk's PREREQ can cite. Discharging that PREREQ stays the wrap's/next chunk's act.
- CI verdict (Setup 5a, last wrap flip `d4113768` through HEAD):
  - `ab936a9e` green, checks 16/16, wall 464 s (CI#37423481705)
  - `d4113768` green, checks 16/16, wall 475 s (CI#37415725588)

  No red to disposition.
- Matrix: no 0.1.0 capability is made verifiable by a sync, and the chunk links none. Verified with `matrix.py show --unclaimed`: all 15 pool entries are id, snapshot, diff, screenshot, driver, CLI, MCP or cold-agent capabilities.
