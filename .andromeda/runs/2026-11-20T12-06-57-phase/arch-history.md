@history_item n=1 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — (the mutation tier's host instruments) recorded the host-run mutation tier: cargo-mutants 27.1.0 with `--test-tool nextest`, cargo-nextest under the single `mutants` profile of `.config/nextest.toml` (default profile untouched), each unit's mutant test command read green on the unmutated tree first, never in CI. This is the origin of decision/mutation-tier-host-only and decision/nextest-mutants-profile. Any re-run of the four `capture.rs` mutants has to use this tier as recorded: on the dev host, on the founder's word, with the browser unit's test command read green first.

@history_item n=2 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** — (Stack and Technologies) added decision/mutation-tier-excludes-demo-shells: the founder ruled two test-less units out of the mutation tier's scope, and the code audit reads extra exclusions only from an architecture decision row. This is the direct precedent for the "kept by the founder's word" branch. If the founder keeps the four browser mutants rather than catching them, the keep needs an architecture decision row of the same shape, or the next boundary audit will read them as missed again.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — killed seven surviving mutants by placing tests inside `dioxus_document`, "the only place one of them can be observed", and set up the first out-of-line `#[cfg(test)] #[path]` child module. Precedent for this chunk's catch path: `render_scene`'s returned size is read by no caller, so it can be observed only by a unit test inside `capture.rs`'s own `#[cfg(test)]` module that calls it directly.

@history_item n=4 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — widened the unit-test placement convention from one out-of-line module to five `#[cfg(test)]` children declared through `#[path]` at their parent. This is the history of decision/unit-test-placement and decision/unit-tests-at-file-end, which a new test module in `apps/browser/src/capture.rs` must follow: at the end of the file, or in a `#[path]` child file.

@history_item n=5 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — moved three more test modules into `#[path]` children (eight in all), keeping test paths unchanged. Further history of the out-of-line form a `capture.rs` test module may take.

@history_item n=6 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (Conventions) changed the except of decision/unit-tests-at-file-end from a fixed count ("Eight unit-test modules…") to the uncounted form "Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent". Because the row no longer holds a count, a new browser test child adds no count amendment.

@history_item n=7 marker=2026-11-15-browser-corrective
**2026-11-15-browser-corrective** — (Conventions) emptied the except of decision/justified-expect-unwrap: the browser favicon test module's `clippy::expect_used` allow now sits below its justification, and no bare browser allow remains. Trap: a new `#[cfg(test)]` module in `capture.rs` that uses `expect`/`unwrap` must carry a justification beside its allow, or it reintroduces the retired exception.

@history_item n=8 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — (Conventions) narrowed the justified-expect-unwrap exception to the browser's favicon test module, since the browser app had gained a second test module whose allow carries a justification. Precedent for how an escher-added browser test module is written and reviewed under the clippy allow rule.

@history_item n=9 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** — (Stack and Technologies; Conventions) moved the AnyRender set to 0.15. decision/anyrender-rendering-abstraction now names vello-gpu in place of vello-hybrid, and the except of decision/in-repo-default-features-off now keeps `seven_guis`'s default `vello-gpu` feature. This is the anyrender bump (`4f61cffac`, the one upstream change to `capture.rs` since the audit sha, adding `AntiAliasing::Enabled` in `scene.fill`). A test building an `impl PaintScene` for `render_scene` must use the 0.15 API and the backends this row now lists.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the harness's screenshot) is the precedent for painting in a test. `Harness::screenshot` resolves, then paints the viewport at its scale and device size through blitz-paint into `VelloCpuImageRenderer`, using `blitz-paint` (default features off), `anyrender` and `anyrender_vello_cpu`. It shows how a test builds a `PaintScene` and a scaled viewport, which `render_scene` reads through `doc.viewport().scale_f64()` (contract/provider-traits/viewport-scale, contract/paint/paint-scene). P3's question is whether the browser crate's own `#[cfg(test)]` can do the same under its default features.

@history_item n=11 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (Document core) added `clock` to `DocumentConfig`'s fields: `None` in `Default`, falling back to a fresh `WallClock`. This is part of contract/document-core/documentconfig and decision/test-document-setup. A direct `render_scene` test that builds its `BaseDocument` from a `DocumentConfig` will read wall time unless it sets a clock.

@history_item n=12 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — (the default sheet from the config) changed `DocumentConfig.ua_stylesheets` to a `Vec<UserAgentStylesheet>` whose hand-written `Default` is the default sheet; a config that sets the field is used as given. Bears on the `DocumentConfig` a browser unit test builds (contract/document-core/documentconfig, decision/document-config, decision/test-document-setup).

@history_item n=13 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — two entries bear here:
- (`Parallel` style traversals serialised; mimalloc) the browser's default feature set gains `mimalloc`, and the browser's document loader builds its documents with `style_threading` `Parallel`, serialised by `PARALLEL_TRAVERSAL_LOCK`.
- (stack) mimalloc is the browser's global allocator by default.

Together these define the default-feature build (decision/per-crate-default-features) that the browser crate's test run and its `cargo mutants -p browser` run compile.

@history_item n=14 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the browser's tab lenses) recorded that the browser is the upstream-only app, so a change there moves no escher surface. This is the root of the scope's fork: removing `render_scene`'s unused return value edits upstream code in `apps/browser` and adds to the upstream divergence, while a test-only module or a founder keep does not.

@history_item n=15 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — (Project Intent) retired decision/upstream-taken-by-cherry-pick in favour of decision/upstream-sync-by-two-parent-merge: upstream main is now taken by a two-parent merge. This makes any escher edit to the upstream-owned `apps/browser/src/capture.rs` body (for example returning `()`) a standing conflict surface at every later merge. Weigh this in the test vs. remove vs. keep fork.

@history_item n=16 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (fork CI through one leg script) created the `ci-leg.sh` legs, `test` among them (contract/ci-leg-legs/test). The `test` leg is the run that will carry a new browser unit test in CI, while mutation runs stay off CI.

@history_item n=17 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host cognitive-complexity reading) recorded a second dev-host-only, never-in-CI audit reading beside the mutation tier, kept inside the founder's M1 scope ruling and claimed as no project-wide convention. This is the earlier codebase-health leg's precedent: audit findings closed on the dev host, the founder's scope ruling bounding what is touched.
