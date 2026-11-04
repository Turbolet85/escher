@carried n=1 label=test-plan/test-data-fixtures fact=fct-iee65fcu
why: when the probe's served task beside the lean one is Cells, it must boot that screen through the stand's own boot and read it against this stated value, never a page of its own.
quote: the stand's own Cells screen, booted through `stand::boot_cells`: 47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary
@carried n=2 label=test-plan/test-data-fixtures fact=fct-qbzx34mx
why: when the probe's served task is the load page, its network answer must be the stand's own `LoadNet`, and every other boot the probe makes must stay offline.
quote: `stand::boot_load`'s stand-in `LoadNet` answers every fetch of the fixed URL `http://stand.test/load.css` with the fixed bytes `LOADED_CSS` (`.load-body .load-done { display: block; }`) from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms); every other stand boot keeps `net_provider: None`
@carried n=3 label=test-plan/test-data-fixtures fact=fct-afgwdd3i
why: the probe's headless stand runs must use the bundled font payload, so the readings measure the same on any runner.
quote: the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic
@carried n=4 label=test-plan/test-data-fixtures fact=fct-hutsqqej
why: any page the probe builds for itself (for example a key-handler page to re-verify the MountedData mechanism) must be test data stated in code, following the category's inline seed strategy.
quote: | Inline HTML | inline HTML string constants or `format!`-built pages |
@carried n=5 label=test-plan fact=fct-65u33zgx
why: the probe's lean task with state (Counter or CRUD) must boot through the stand's pinned boot, the same drive surface the stand checks use.
quote: The seven_guis headless stand boots each lean task (Counter, FlightBooker, Timer, Crud) through `seven_guis::stand::boot(LeanTask, HarnessOptions)` or `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each a fresh `VirtualDom` over `from_vdom` rendering the task in TaskShell, with `stand::options(incremental)` the pinned options
@carried n=6 label=test-plan fact=fct-552qmxvp
why: the probe must run in both layout modes on the stand's pinned options, offline except for the load page.
quote: `seven_guis::stand::options(incremental)` builds the stand's full `HarnessOptions` literal: 800×600, scale 1.0, Light, the bundled font, `net_provider: None` — kept for every caller but `stand::boot_load`
@carried n=7 label=test-plan fact=fct-6bs5asmb
why: the probe's check needs no seeding: its fixtures must be code, compiled by `agent-run.sh boot` and run by name or through `run stand`.
quote: nothing is seeded — the fixtures are code, the pages, the fixture boot and hold and the held network stand-in stated in `tests/blitz-tests/tests/fixtures/mod.rs` (§3 Crate-local test helpers) and the one-reader pages in their checks' own files, compiled into each target by `agent-run.sh boot` (`cargo test --no-run`), so `boot` is the bootstrap
@carried n=8 label=test-plan fact=fct-adt6dvk7
why: a probe page read by one check alone must stay in that check's file and not join the shared fixtures module.
quote: a page with one reader stays in its check's file
@carried n=9 label=test-plan fact=fct-osorhgow
why: a probe fixture booted through the shared module must use its pinned boot, and the module must keep holding no print, trace, env read, file, socket or listener (the scope's «nothing crosses»).
quote: `boot(app, incremental)`, a `Harness<DioxusDocument>` built from `VirtualDom::new(app)` with the stand's pinned options `stand::options(incremental)` (tests/blitz-tests/tests/fixtures/mod.rs:17-20)
@carried n=9 run=2 label=test-plan fact=fct-osorhgow
quote: the module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`
@carried n=10 label=test-plan fact=fct-3xrvezdp
why: all of the probe's test data must be stated in code, with no external page or file on disk.
quote: test data is stated in code — inline HTML, literals and byte arrays, images injected directly, fixed viewports, the embedded fonts and pages generated in the test
@carried n=11 label=test-plan fact=fct-i47d76zr
why: when P3 re-verifies the MountedData mechanism at bridge level, the reproduction must build its document this way, the same `poll(None)` path through which the panic was measured.
quote: the document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)`
@carried n=11 run=2 label=test-plan fact=fct-i47d76zr
quote: that helper's `DocumentConfig::default()` carries the no-op HTML parser, so `dangerous_inner_html` renders nothing at unit level
@carried n=12 label=test-plan fact=fct-bz62prox
why: the probe's stand checks must assert unconditionally under the bundled font, with no skip path.
quote: the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans with system fonts off
@binding n=1
**stand fixtures ↔ the scope's «Measured, not read»**: the lean task (Counter or CRUD) boots through `stand::boot` (test-plan fct-65u33zgx) and the served task through `stand::boot_cells` (test-plan/test-data-fixtures fct-iee65fcu) or `stand::boot_load` (test-plan/test-data-fixtures fct-qbzx34mx), each in both layout modes on `stand::options(incremental)` (test-plan fct-552qmxvp).
@binding n=2
**fixtures as code ↔ the scope's «Nothing crosses» (obs-plan, arch)**: probe pages are in-code seeds (test-plan/test-data-fixtures fct-hutsqqej; test-plan fct-3xrvezdp), and a page with one reader stays in its check's file (test-plan fct-adt6dvk7); the shared fixtures module keeps no print, trace, env read, file, socket or listener (test-plan fct-osorhgow), and nothing the probe reads is printed or logged.
@binding n=3
**network data ↔ §8 Mocking & Stubbing**: the only network answer the probe meets is the stand's `LoadNet` on the load page, with every other boot offline (test-plan/test-data-fixtures fct-qbzx34mx; test-plan fct-552qmxvp); the probe adds no stand-in, listener or port of its own.
@binding n=4
**font payload ↔ §2 font-dependent tests and §9 Fonts**: the bundled DejaVu Sans is registered for every generic (test-plan/test-data-fixtures fct-afgwdd3i), and the probe's checks have no skip path (test-plan fct-bz62prox).
@binding n=5
**MountedData re-verification ↔ the scope's folded CARRY**: P3's reproduction at HEAD follows the bridge test's `DocumentConfig::default()` · `initial_build` · `mark_dirty` · `poll(None)` build (test-plan fct-i47d76zr). Its page is in-code data in its check's own file (test-plan fct-adt6dvk7). It adds no engine change, because the fix is the founder's fork.
