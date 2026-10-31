# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan.md:351) Any stand-in the popup's checks need must be hand-written, in the concerns this line names, and a new one must gain its own row in this category.
  > - **Stand-ins:** hand-written stand-ins replace the shell, the network, a document's poll, navigation, event handlers, widgets, the renderer, time, timer instrumentation, the WPT JS harness, worker publication, notify and the GitHub API
- (.andromeda/test-plan.md:365) The chunk's tests must add no mocking library; the absence stands through this chunk.
  > - Observed absent, each by its search: a mocking library in CI, the build configuration and the apps, and mocks in slices s03, s06–s09 and s11
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:45) The absence of a mocking library must still read true under this same search after any manifest change the chunk makes.
  > - Mocking libraries in CI and build configuration · searched: `mock` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:10) A check that the list's opening, closing or moved highlight asks for a redraw must read it through a hand-written `ShellProvider` fake that counts requests, since no test launches a window; this row's `mutator.rs` and `document.rs` citations must be re-read if the chunk moves those files.
  > | Shell | hand-written `ShellProvider` fakes count redraw requests |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:11) The open list is new interaction state, so a check of its teardown, or of what its focus and keys ask of the shell's IME, must read the shell through this recording stand-in.
  > | Shell | `RecordingShell` implements `ShellProvider` and records `set_ime_enabled` calls |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:22) The list must open, paint, take its choice and close under the no-op shell and net a headless document holds, the overlay living in the engine with no OS window assumed; a shell need of its own would be a new method on the no-op provider, to be named on a fork.
  > | Net, navigation, shell | no-op provider implementations |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:23) A check of which DOM events a pointer or key choice from the open list dispatches, and that closing without a choice dispatches none of the choice's, must read names through this recorder.
  > | Events | `RecordingHandler` EventHandler installed by dispatch_recorded records each dispatched DOM event name |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) This stand-in serves timings alone: the list's pixels and engine colours must be read on a real raster, and any new overlay paint call must still run through it in `paint_tree_bench`.
  > | Renderer | `NullScenePainter` stands in for a renderer in timings |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17) The stand's popup, the flight booker's `select#flight-type`, must be driven on an offline boot, and `the_stand_options_stay_offline` must stay green with no network stand-in added for this chunk.
  > every other boot keeping `net_provider: None` (`the_stand_options_stay_offline`)
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:28) If the reduced-motion fork leaves any timed opening or closing, a check must move it on this one clock through `advance`, with no sleep and no wall-clock read; a list with no motion needs no time stand-in.
  > the driver moves that time by `advance`, which moves the session's clock — the harness's `VirtualClock`, the clock every harness-built document reads — by exactly `ms` inside one settled step
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:39) A blitz-dom unit test over the list's text-sized boxes must assert its font precondition and take no skip, and its fixture must use the two-argument `qual_name!(…, html)` so the default sheet, which this chunk's `assets/default.css` work moves, reaches it; the row's `document.rs` line citations must be re-read if the chunk moves that file.
  > No font-metric test of `document.rs` skips: since 2026-10-14-quality-gates the two `hover_state_tests` assert their precondition through `#[track_caller] fn assert_text_has_size` — the container's height over 50 px — where they skipped with an `eprintln!` when text measured 0x0; the skip was the fixture's, not missing system fonts: built with the one-argument `qual_name!`, a no-namespace element the default sheet misses, both skipped under `--features system-fonts` too, and with the two-argument `qual_name!(…, html)` for `html`, `body` and both `div`s both assert and pass
- (.andromeda/test-plan.md:361) The stand's popup checks, contrast and option boxes included, must measure the list's text under this pinned font, asserting unconditionally with no font stand-in and no skip.
  > - The headless-stand checks keep a real but pinned font — the bundled DejaVu Sans woff2, decoded through seven_guis' native `dioxus-native` `woff` feature, with system fonts off and no skip

## Bindings
- **Redraw fakes ↔ §6 E2E**: with no test launching a window, the shell fake is the one reading of the list's redraw ask, and the windowed path's list stays unread by a windowed check, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:10) with (.andromeda/test-plan.md:302).
- **Event recorder ↔ §5 Integration**: `dispatch_recorded` bypasses Dioxus VirtualDom forwarding, so the recorder reads the engine's dispatch only and a Dioxus handler's reaction to a choice must be read through a held session, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:23) with (.andromeda/test-plan.md:253).
- **Null painter ↔ §2 Pixel tests**: the list's pixels must follow the pixel-test level, a CPU buffer sampled and a shot compared on one host, never the timing stand-in, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) with (.andromeda/test-plan.md:89).
- **Virtual clock ↔ §5 Wall time**: a timed list must not add a third check holding a sleep or wall-clock read, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:28) with (.andromeda/test-plan.md:262).
- **Pinned font ↔ §2 Font-dependent tests and §9 Fonts**: the stand's list must measure the same on any runner, with no skip path, (.andromeda/test-plan.md:361) with (.andromeda/test-plan.md:94) and (.andromeda/test-plan.md:390).
- **No mocking library ↔ §3 Third-party test frameworks**: any dev-dependency the chunk proposes must answer both absences, (.andromeda/test-plan.md:365) with (.andromeda/test-plan.md:126).
