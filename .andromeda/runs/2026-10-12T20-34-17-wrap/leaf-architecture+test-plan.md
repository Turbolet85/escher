group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:58   class: omission
  leaf-text: «`advance --ms <ms>` · `scroll --id <id>` · `status` each run one command against it»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:9 «`screenshot` the seventh since 2026-10-12-headless-screenshot, its `--id` optional, its accepted answer one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`»
  replace-with: «`advance --ms <ms>` · `scroll --id <id>` · `screenshot [--id <id>]` (since 2026-10-12-headless-screenshot; its accepted answer one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`) · `status` each run one command against it»

- leaf: .claude/docs/commands.md:59   class: stale
  leaf-text: «its tools the ten driver verbs»
  source: .andromeda/architecture.md:133 «`tools/list` answers the eleven verbs of `VERBS` in order (`screenshot` after `scroll` since 2026-10-12-headless-screenshot), `help` last»
  replace-with: «its tools the eleven driver verbs (`screenshot` after `scroll` since 2026-10-12-headless-screenshot, an accepted shot answered as an `image` block (`image/png`, base64) then a text block)»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «`cargo test -p blitz-tests --locked --test stand_act_ids --test stand_act_diff --test stand_act_timer --test stand_act_refused --test stand_act_keys --test stand_act_range` · `cargo test -p seven_guis --locked --lib stand::` — the driver's calls run on a held stand session (`Session::run`, both layout modes) · the stand's time step (milliseconds onto Timer ticks)»
  source: .andromeda/test-plan.md:26 «the thirteenth, since 2026-10-12-headless-screenshot, `stand_act_screenshot` 6»
  replace-with: «`cargo test -p blitz-tests --locked --test stand_act_ids --test stand_act_diff --test stand_act_timer --test stand_act_refused --test stand_act_keys --test stand_act_range --test stand_act_screenshot --test stand_screenshot_repeat` · `cargo test -p seven_guis --locked --lib stand::` · `cargo test -p blitz-tests --locked --test password_painted_masked` — the driver's calls run on a held stand session (`Session::run`, both layout modes; since 2026-10-12-headless-screenshot the `screenshot` verb, and a shot of one state byte-identical in two child runs and the parent) · the stand's time step (milliseconds onto Timer ticks) · the engine's password mask read on frames, both layout modes (since 2026-10-12-headless-screenshot)»

- leaf: .claude/docs/services/blitz-test-harness.md:6   class: omission
  leaf-text: «input synthesis through the real event-dispatch pipeline, and inspection helpers. No window, GPU or compositor.»
  source: .andromeda/test-plan.md:47 «a paint with no display — `Harness::screenshot` / `screenshot_rect` into `VelloCpuImageRenderer`, composited over opaque white, at the harness's own time with no tick or pump»
  replace-with: «input synthesis through the real event-dispatch pipeline, inspection helpers and, since 2026-10-12-headless-screenshot, a screenshot — the viewport painted through blitz-paint into `VelloCpuImageRenderer` and composited over opaque white, a paint with no display. No window, GPU or compositor.»

- leaf: .claude/docs/services/blitz-test-harness.md:11   class: omission
  leaf-text: «blitz-dom (built with `accessibility`), blitz-html (`HtmlProvider`), dioxus-native-dom, keyboard-types.»
  source: .andromeda/architecture.md:130 «the harness depends on three more crates — `blitz-paint` (`default-features = false`, so no `svg` and no `scrollbars` in a per-package build), `anyrender` and `anyrender_vello_cpu`»
  replace-with: «blitz-dom (built with `accessibility`), blitz-html (`HtmlProvider`), dioxus-native-dom, keyboard-types and, since 2026-10-12-headless-screenshot, for the screenshot `blitz-paint` (`default-features = false`, so no `svg` and no `scrollbars` in a per-package build), `anyrender` and `anyrender_vello_cpu` (no `multithreading`).»

- leaf: .claude/docs/services/blitz-test-harness.md:14   class: omission
  leaf-text: «`Busy`, `NotSettled`, `SETTLE_PASS_LIMIT`, `Settled`.»
  source: .andromeda/architecture.md:130 «`PixelRect`, `Raster` and `ScreenshotError` from its private module `screenshot`»
  replace-with: «`Busy`, `NotSettled`, `SETTLE_PASS_LIMIT`, `Settled` and, since 2026-10-12-headless-screenshot, `PixelRect`, `Raster` and `ScreenshotError` (`FontsUnavailable` · `EmptyRect`) from the private module `screenshot`.»

- leaf: .claude/docs/services/blitz-test-harness.md:15   class: omission
  leaf-text: «The outcome names a class only — no node, id, name, URL or value.»
  source: .andromeda/test-plan.md:88 «`screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` in the private module `screenshot`»
  replace-with: «The outcome names a class only — no node, id, name, URL or value. Since 2026-10-12-headless-screenshot `Harness::screenshot(&mut self) -> Result<Raster, ScreenshotError>` and `screenshot_rect(&mut self, PixelRect) -> Result<Raster, ScreenshotError>` resolve at the harness's own time (no tick, no pump), paint the viewport at its scale and device size into `VelloCpuImageRenderer` and composite over opaque white (premultiplied source-over); a rectangle is clipped to the viewport and an empty one is `EmptyRect`; a document whose `can_paint_text` reads false is `FontsUnavailable` with nothing painted; no path prints, panics or logs.»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: omission
  leaf-text: «Consumed by seven_guis' headless stand (`seven_guis::stand::{boot, boot_timer, options}`), which builds its pinned options as a full `HarnessOptions` literal.»
  source: .andromeda/architecture.md:133 «the frame is the harness's `screenshot` (the Test harness contract above)»
  replace-with: «Consumed by seven_guis' headless stand (`seven_guis::stand::{boot, boot_timer, options}`), which builds its pinned options as a full `HarnessOptions` literal, and by escher-driver, whose `screenshot` verb's frame is the harness's `screenshot` (since 2026-10-12-headless-screenshot).»

- leaf: .claude/docs/services/blitz-test-harness.md:30   class: omission
  leaf-text: «`src/inspect.rs` (queries, `dom_string`)»
  source: .andromeda/architecture.md:255 «the private screenshot (`Harness::screenshot`, `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError`, the three types re-exported from the crate root»
  replace-with: «`src/inspect.rs` (queries, `dom_string`) · `src/screenshot.rs` (the screenshot: `Harness::screenshot`, `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError`, since 2026-10-12-headless-screenshot)»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «`cargo test -p blitz-test-harness --lib settle` — the crate's 11 unit tests (the load counter, the loop over a scripted document, the messages);»
  source: .andromeda/test-plan.md:19 «17 since 2026-10-12-headless-screenshot, 6 more in `screenshot.rs`»
  replace-with: «`cargo test -p blitz-test-harness --lib settle` — the crate's 11 settle unit tests (the load counter, the loop over a scripted document, the messages); `cargo test -p blitz-test-harness --lib screenshot` — its 6 screenshot unit tests (17 in the crate since 2026-10-12-headless-screenshot: the whole viewport at its device size, unpainted reading opaque white, a rectangle clipped or an error, a document with no font refused, a shot the same twice);»
