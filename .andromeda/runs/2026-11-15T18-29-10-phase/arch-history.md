@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (fork CI through one leg script) set up `.github/scripts/ci-leg.sh` with every cargo leg `--locked` (but `examples/wasm_hello`, which has no `Cargo.lock`), the `msrv` leg as `cargo +1.91 build`, and `fast` as the local pre-push gate. These are the legs the git-Taffy move must keep green under `--locked`, and the reason `examples/wasm_hello`'s taffy resolution has to be read on its own.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — added the `audit` leg (cargo-deny over the root `deny.toml`) and a real `doc` gate (`cargo doc --workspace --no-deps`, warnings denied). Its cache sub-entry records a trap: a `Cargo.lock` change re-mints every rust-cache key and evicts until the old keys age out, so the budget runs over. The Taffy lock move will cold-start every CI cache.

@history_item n=3 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — mapped `align-items`/`justify-items`/`align-self`/`justify-self: normal` to Taffy's first-class `NORMAL` keyword, resolved by the aligned box's layout mode, and recorded the sync merge base. It is the origin of `decision/normal-self-alignment-taffy` and `decision/upstream-sync-merge-base`. Both depend on Taffy's API, which the git pin may move.

@history_item n=4 marker=2026-10-06-project-readme
**2026-10-06-project-readme** — recorded `stylo_taffy`'s `MIT OR Apache-2.0 OR MPL-2.0` licence as a licensing exception. The `audit` leg's licences check now has to read a git-sourced Taffy's licence and its dependency graph as well.

@history_item n=5 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (the second upstream sync, Parley's pin form) left taffy "the one git dependency pinned by `rev`" while parley moved to the registry. It noted "the Taffy rev is spelled in no master, so its bump amends nothing", and kept the parley/skrifa/vello coupling rule. This is the precedent for taffy as a git rev in the manifest before 2026-10-13 moved it to the registry.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — (Taffy's tree traits on `LayoutPassState`) moved Taffy's nine traversal and layout traits from `BaseDocument` to the crate-private `LayoutPassState`, and grew `stylo_taffy`'s export list (`to_taffy_style_in`, `WritingMode`, `WritingModeExt`). It is the origin of `contract/layout/taffy-traits` and `contract/layout/stylo-taffy-exports`, the surfaces a Taffy trait-signature change between 0.15.1 and the `a3997d55` pin would break.

@history_item n=7 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (taffy from the registry) moved taffy from a git rev to registry "0.15" (resolved 0.15.0, eight features unchanged), leaving "no git dependency in the manifest". It is the origin of `decision/no-git-dependencies` and of `decision/taffy-layout`'s "from the crates.io registry". It also flagged a trap: the CLAUDE.md coupled-pins warning no longer names a taffy git rev. This chunk reverses the move, so the warning and both rows have to be re-read.

@history_item n=8 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — widened `audit` to `cargo deny --locked --workspace --all-features check advisories licenses`, and added upstream's blitz-dom feature-subset clippy line (run only when the workspace line is green) and the `stand-repeat` leg (three rounds, zero failures). These are three gates the measurement must read on the git Taffy (`decision/dependency-and-licence-audit`, `decision/blitz-dom-feature-subset-clippy`, `decision/stand-repeat-zero-failure-budget`).

@history_item n=9 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — (the sync at `23b9b26a9`) set the merge base that still stands. It first deferred the four Taffy-main commits past it to the next sync (`6032d0b9d` inside them reverted by upstream's `85340347f`), so Taffy stayed registry "0.15". This is the start of the Taffy chain this chunk finally takes.

@history_item n=10 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — took upstream by no-commit cherry-pick with the merge base left in place, making partial takes the pattern (`decision/upstream-taken-by-cherry-pick`). It named the four Taffy-main commits `1a11761bc` · `4a5ab9cc9` · `510b592ab` · `1e5479ae6` as waiting "since they put a git Taffy in the root manifest". `1a11761bc`'s `inline.rs` line is part of this chunk's scope.

@history_item n=11 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — a second partial cherry-pick take of upstream engine behaviour for every document. It was PROVISIONAL under the playbook's "an engine behaviour change for every document" until the founder's own word. This is the precedent for the fork this chunk must name: whether the Taffy chain's layout changes are ratified by «Снять для Taffy» or each need a ratification of their own.

@history_item n=12 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (stack and project intent at `f9cbb2756`) moved the flake's rust-bin to "1.91.0" to match `rust-version`, and resolved tokio past an advisory by a lock-only move. It recorded the six Taffy-main commits as waiting for escher's next version, owned by a residual. This is where `decision/deferred-upstream-dependency-bumps` comes from; the MSRV 1.91 bound must still hold for the git Taffy.

@history_item n=13 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — (recorded style facts; the `inline` paths) added `Node::out_of_flow`, read by `LayoutPassState::is_out_of_flow`, and the replaced element's natural height in `compute_replaced_layout`. It turned `layout/inline.rs` into the directory `layout/inline/` (`inline/layout.rs`, `inline/build.rs`). Trap: upstream's `1a11761bc` edits an `inline.rs` that is `packages/blitz-dom/src/layout/inline/layout.rs` in escher, so the `place_floated_box` line lands at a different path. It also records that `decision/node-holds-structure` was shaped here.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (six upstream behaviour changes ratified for this take) is the origin of `decision/upstream-behaviour-changes-ratified-per-take`: upstream changes for every document are ratified per take under the playbook pattern and are not counted among escher's direct engine changes. Its `::before`/`::after` flow-classification entry is the origin of `decision/pseudo-elements-vote-in-flow-classification`, a layout reading a Taffy move could shift.

@history_item n=15 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the upstream take read at `ebacd68bd`) left sixteen commits out, each with its measured reason, the Taffy-main commits among them, with "no git dependency" and the lock and manifests byte-identical. It shows how left-out Taffy commits are recorded, and that the take's evidence lives in the chunk's `evidence/lock.md`.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the legs that do not run locked) corrected the `--locked` claim: every cargo leg runs locked except `fmt` and `examples/wasm_hello` (`decision/cargo-legs-locked`). Its pulldown-cmark entry is the precedent for a semver-compatible move in `Cargo.lock` alone with no coupled pin. Its eight-engine-changes entry records the `hit_inner` conflict with upstream and the merge-surface owner, which the Taffy chain's blitz edits may touch.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (upstream's both-modes exception ended) made `pseudo_element_flow_classification` run in both layout modes. The `incremental_oracle` and both-modes invariant (`decision/one-incremental-pipeline`, `decision/both-layout-modes`) must hold after the Taffy move.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (taffy is no re-export of blitz-dom) restated `decision/taffy-layout`: taffy is part of blitz-dom's public API through its signatures (`taffy_node_id`, `dom_node_id`), with no `pub use`. A git Taffy changes the type identity in those public signatures.

@history_item n=19 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a failed stylesheet through `NetHandler::error`) records a standing trap: a trait default method can hide a take's fix, because a do-nothing default compiles silently through a wrapper. A later take must therefore sweep the implementers of any trait it extends, which applies to `LayoutPassState`'s Taffy trait impls if the git Taffy adds defaulted methods.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the twelve commits ratified for their take; the upstream line at `e42ee1f25`) set the per-take ratification wording behind `decision/upstream-take-engine-changes`. It kept "Taffy the registry 0.15, no git dependency". Its `AGENTS.md` entry is the origin of `decision/upstream-agents-md-excluded`.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the layout entries) recorded layout behaviour that sits on Taffy and that the move must leave unchanged or measure:
- a table box's position, padding or border change gives `ALL_DAMAGE` through `table_needs_rebuild` (`decision/damage-bits-drive-relayout`, `decision/table-layout-by-taffy-grid`);
- the resolved `width`/`height` read `unrounded_layout()` (`contract/resolved-style/width-height`);
- the inline box's paint skip adds the second `Node` fact (`decision/node-holds-structure`);
- the key-press panic fix clears `layout_parent` in `deallocate_anonymous_block`.

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (the `embolden` step of `fast`; the orphan comment gone) made `fast` run fmt → clippy → test → ci-scripts → embolden, the local pre-push gate the measurement runs. It also removed the "Re-export taffy" comment from `packages/blitz-dom/src/lib.rs`, moving `decision/taffy-layout`'s citation to `lib.rs:115-125`.

@history_item n=23 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the advisory fix) moved pulldown-cmark to 0.13.6 in `Cargo.lock` alone and added an ignore with a reason to `deny.toml` after the audit went red. This is the precedent for how an audit red from a lock move is handled (`decision/dependency-and-licence-audit`: each per-ID ignore carries a reason). Its MountedData sub-entry moved the left-out upstream commits' owner, the Taffy chain among them, to a `residual` row in the store.

@history_item n=24 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — set `decision/upstream-take-engine-changes` to the twenty-nine commits at `563d3db73` and added `decision/default-font-fallback`. It gave `decision/both-layout-modes` its exception: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode. Any upstream test the Taffy chain brings falls under that exception.

@history_item n=25 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — added `decision/no-prng-crate`, whose reason is that "no package enters the lockfile". It shows the lock-discipline bar: this chunk's lock may move only as the Taffy move forces.

@history_item n=26 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — added `decision/stand-readings-same-on-every-ci-os` (six served tasks reading the same digests on linux, windows and macOS, both layout modes, guard `stand_cross_os_readings`) and restated `decision/stand-typography-dejavu-sans`. Stand layout readings pinned byte for byte are what a Taffy layout change would turn red.

@history_item n=27 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (Project Intent) rewrote `decision/deferred-upstream-dependency-bumps`: the Taffy-main content comes through registry Taffy 0.15.x except #1303, "which waits with the later git Taffy chain for its own chunk". This chunk is that one; the row must be restated once the git pin lands.

@history_item n=28 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (Established Decisions) gave `decision/no-git-dependencies` its exception, approved by the operator's "Apply all three": "The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs". This is the master rule this chunk acts under and must not exceed: a second git source or a `[patch]` falls outside it, and the rule half "no git dependency is left in the manifest; taffy is a registry version" becomes stale once the move lands.
