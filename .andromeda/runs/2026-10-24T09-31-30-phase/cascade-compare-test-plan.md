# Cascade compare — test-plan · phase · 2026-10-24-upstream-sync-polish-and-ship
E items 160 · held 33 · lost at classify 8 · lost at strong 8 · cascade only 7 · body 38 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### integration-test-strategy#2.2 · lost at strong · .andromeda/test-plan-detail/integration-test-strategy.md:18
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) Fork CI green with every job settled includes the windows leg, the one leg that compiles and runs the non-unix session arms, and its tally is part of the per-OS re-read.
  > the non-unix arms, which return `Unsupported`, are compiled and run by the fork's windows CI leg only

### mocking-stubbing-discipline#1.1 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:10
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:10) The hand-written `ShellProvider` redraw-counting fakes live in `mutator.rs` and `document.rs`, a predicted conflict and a file upstream's delta reaches. Every resolution must keep both fakes and their tests passing, and the row's line cites must be re-pointed wherever the take moves them.
  > | Shell | hand-written `ShellProvider` fakes count redraw requests | (packages/blitz-dom/src/mutator.rs:1715-1724; packages/blitz-dom/src/document.rs:3160-3168) |

### mocking-stubbing-discipline#1.4 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:13
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:13) A stand-in inside an upstream test file stays upstream's own, file-private one. The take must leave upstream's test stand-ins, including any its new targets bring, in upstream's files and not fold them into escher's shared fixtures. Its cites are re-pointed if the take moves `render_blocking_stylesheet.rs`.
  > | Network | `ManualNetProvider` holds requests and handlers so the test delivers responses when it chooses — upstream's file-private one in `render_blocking_stylesheet` | (tests/blitz-tests/tests/render_blocking_stylesheet.rs:16-30; tests/blitz-tests/tests/render_blocking_stylesheet.rs:62-66) |

### mocking-stubbing-discipline#1.5 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:14
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:14) `HeldNet` must stay escher's one held network stand-in, with no `is_noop` override, through the merged engine. `stand_settle` must still read `NotSettled { busy: Loads }` through it, with no real fetch.
  > | Network | `HeldNet`, escher's one held network stand-in since 2026-10-19-shared-fixtures, in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider` and `stand_boot`'s `RecordingNetProvider`: `fetch` keeps each request's url and handler, `held()` and `urls()` (request order) read them, and `answer(body)` answers the last held request through `NetHandler::bytes` and returns its url; it does not override `is_noop`, so the harness counts its loads — `stand_settle` reads `NotSettled { busy: Loads }` through it, answering a head stylesheet and then its `@import` on `.test` URLs with no real fetch | (tests/blitz-tests/tests/fixtures/mod.rs:64-107) |

### mocking-stubbing-discipline#1.6 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:15
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:15) `stand_load_wait`'s `LateNet` arm and its `HeldNet` never-answered arm must both still pass on the take's tree. This is the load wait's reading on the merged engine.
  > | Network | `LateNet`, `stand_load_wait`'s in-file self-answering stand-in since 2026-10-20-load-wait: `fetch` answers each request from its own thread after a 200 ms sleep, so a load is in flight when the step settles and is answered inside `LOAD_WAIT`; the never-answered arm uses the shared `HeldNet`, and its threads end with the test process | (tests/blitz-tests/tests/stand_load_wait.rs:52-60) |

### mocking-stubbing-discipline#1.7 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:16
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:16) The settle unit tests' `Scripted` `Document` and `Held` `NetProvider` implement engine traits that the take's `blitz-dom` and Dioxus moves may reshape. They must still compile and pass, adapted only as far as a trait change requires.
  > | Document, network (settle unit tests) | a `Scripted` `Document` whose `poll` answers from a script and counts its calls, and a `Held` `NetProvider` that keeps each handler until the test takes it | (packages/blitz-test-harness/src/settle.rs:207-242; packages/blitz-test-harness/src/settle.rs:317-374) |

### mocking-stubbing-discipline#3.1 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:43
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:43) The take moves the manifests and `flake.nix` and adds `mimalloc`. It must bring in no mocking library, and the absence must be re-read over those files.
  > - Mocking libraries in CI and build configuration · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

### mocking-stubbing-discipline#3.2 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:44
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:44) `mimalloc` lands in `apps/browser`. The apps' absence of a mocking library must stand after the take.
  > - Mocking libraries in the apps · searched: `mockall|mock` (case-insensitive) over the 86 files of slice s04

### quality-gates-coverage-targets#4.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:27
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:27) The take replaces upstream's side of `mutation_writer.rs`, one of the three files whose scoped mutation reading is recorded (0 missed), so that reading predates the take. If the chunk re-reads it, the run uses cargo-mutants under `NEXTEST_PROFILE=mutants` with `--baseline=skip`, the unit's test command first read green on the unmutated tree, on the dev host alone. No mutation run is added to CI.
  > - **Mutation tier:** since 2026-10-16-fast-failing-mutants a mutation run is cargo-mutants with `--test-tool nextest` under the `mutants` profile of the root `.config/nextest.toml` — `fail-fast = { max-fail = 1, terminate = "immediate" }`, `test-threads = 4`, one `priority = 100` override naming the direct tests, the default profile untouched, selected by `NEXTEST_PROFILE=mutants`, a host tool's variable no workspace source reads — with `--baseline=skip`, each unit's mutant test command first read green on the unmutated tree (`NEXTEST_PROFILE=mutants cargo nextest run --locked`, a `-p` per package on the unit's list line, then `--lib` and its `--test=` targets): cargo-mutants 27.1.0's baseline tests the mutated package alone yet passes every `--cargo-test-arg`, so a unit's test set naming another package's targets cannot be its baseline, and a test phase that cannot pass would read every mutant caught. Under plain `cargo test` the six mutants the 0.1.0 audit read timeout-only still read `timeout`, because cargo runs the hung blitz-tests integration binaries (`stand_accessibility_ids`, `stand_act_diff`, `stand_mcp_masked`, `harness_smoke`) before the library tests — so the next audit runs with the profile. Run on the dev host alone, never in CI, cargo-mutants 27.1.0 and cargo-nextest 0.9.146 being host tools no lockfile resolves; over `timer.rs`, `mcp/framing.rs` and `mutation_writer.rs` the scoped runs read 137 mutants — 126 caught, 11 unviable, 0 missed, 0 timeout — and the six caught in 2.47 to 2.97 s (dev host, as measured at escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/report.md, its `evidence/mutants-{timer,framing,writer}.json` and `evidence/controls.md`)

### test-data-fixtures#3.7 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:28
- (.andromeda/test-plan-detail/test-data-fixtures.md:28) The style-fixture rows cite line ranges in `packages/blitz-dom/src/document.rs`, a file upstream's delta reaches (`06ecec886`, `3954ecb85`). Every cited range must still point at its fixture on the take's tree, re-pointed at the wrap where it moved.
  > | Style fixtures | inline user-agent stylesheets | (packages/blitz-dom/src/document.rs:3303-3306; packages/blitz-dom/src/document.rs:3598-3602) |

### test-data-fixtures#3.8 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:29
- (.andromeda/test-plan-detail/test-data-fixtures.md:29) The fixed viewports cite `mutator.rs`, one of the trial merge's four predicted conflicts, and `document.rs`. The fixture values (800x600 and 400x300, scale 1.0, Light) must stay unchanged through the conflict resolution, and the citations must be re-pointed where the take moves them.
  > | Viewports | fixed sizes (800x600 and 400x300, scale 1.0, Light) | (packages/blitz-dom/src/mutator.rs:1848; packages/blitz-dom/src/document.rs:3204) |

### test-data-fixtures#3.9 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:30
- (.andromeda/test-plan-detail/test-data-fixtures.md:30) The stand's pinned font payload is what keeps the headless-stand numbers deterministic. The take must leave the bundled DejaVu Sans registered for every generic, and the `document.rs` citation must be re-pointed where upstream's delta moves it.
  > | Font payload | the embedded bullet font; the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic | (packages/blitz-dom/src/document.rs:3691-3706; examples/seven_guis/src/lib.rs:7) |

### test-scope-summary#10.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:83
- (.andromeda/test-plan-detail/test-scope-summary.md:83) `286874cc1` syncs `packages/dioxus-native` too; the observed absence of tests there is re-read on the take's tree.
  > - **dioxus-native and stylo_taffy:** no tests — observed absent · searched: `#\[test\]|#\[cfg\(test\)\]` over the 21 files of slice s11

### test-strategy#5.1 · lost at strong · .andromeda/test-plan-detail/test-strategy.md:34
- (.andromeda/test-plan-detail/test-strategy.md:34) The headless-stand checks keep no skip path under the take: they assert unconditionally under the bundled DejaVu Sans with system fonts off, and `fonts-unavailable` stays refused with nothing painted.
  > the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx` (examples/seven_guis/src/stand.rs:52-54; tests/blitz-tests/tests/stand_boot.rs:51-65)

### test-strategy#6.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:40
- (.andromeda/test-plan-detail/test-strategy.md:40) Upstream's WPT runner change (`f8723f495`) and conformance numbers are out of scope: a WPT reading on the dev host is report-only and grades no criterion, so the chunk owes none.
  > it grades no criterion (as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/evidence/conformance.md, which holds its cost)

### unit-test-strategy#8.4 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:64
- (.andromeda/test-plan-detail/unit-test-strategy.md:64) Upstream's new blitz-tests files may falsify this recorded absence. It is re-read over the files the take carries.
  > In-file `#[cfg(test)]` unit-test modules in tests/blitz-tests · searched: `#\[cfg` over the 61 files of slice s12

## Held

- ci-integration#1.1
- ci-integration#4.1
- ci-integration#5.1
- ci-integration#6.1
- ci-integration#7.1
- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- integration-test-strategy#2.1
- integration-test-strategy#2.3
- mocking-stubbing-discipline#1.17
- mocking-stubbing-discipline#2.1
- quality-gates-coverage-targets#2.1
- quality-gates-coverage-targets#3.1
- test-data-fixtures#2.1
- test-scope-summary#1.1
- test-scope-summary#11.1
- test-scope-summary#12.1
- test-scope-summary#2.1
- test-scope-summary#2.2
- test-scope-summary#2.3
- test-scope-summary#3.2
- test-scope-summary#4.1
- test-scope-summary#7.1
- test-scope-summary#9.1
- test-strategy#2.1
- test-strategy#3.1
- test-strategy#4.1
- test-strategy#7.1
- unit-test-strategy#1.1
- unit-test-strategy#3.1
- unit-test-strategy#4.1
- unit-test-strategy#6.1
- unit-test-strategy#7.1

## Cascade only

- integration-test-strategy#1.1
- test-scope-summary#3.1
- test-scope-summary#5.1
- test-scope-summary#6.1
- test-scope-summary#7.2
- test-strategy#1.1
- unit-test-strategy#2.1
