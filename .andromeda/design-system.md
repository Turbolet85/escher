## Brand Identity

> NO RECORDED INTENT

---

## Color Palette

Detail: .andromeda/design-system-detail/color-palette.md — read whole.

**Color model and scheme (engine):**

### Core Colors
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.

### Surface Scale (elevation hierarchy)
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

### Text Hierarchy
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

### Semantic Colors
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

### Border Progression
Detail: .andromeda/design-system-detail/color-palette.md — read whole.

**Engine-painted defaults:**

**Token sets:**

**Example fixtures:**

> NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.

---

## Typography

Detail: .andromeda/design-system-detail/typography.md — read whole.

**Loading:**

A driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped.

`blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate: without it text measures 0x0 and font-dependent assertions pass vacuously.

On WASM a font context with bundled fonts must be provided, `build_single_font_ctx` for one font.

**Engine text mapping:**

**Example fixtures:**

**Observed absent:**

> NOT YET MEASURED — a project-wide type scale: the examples slice (s01) recorded the project's own type scale as out of slice, and the WPT runner slice (s13) recorded typography as out of slice.

---

## Spacing

Detail: .andromeda/design-system-detail/spacing.md — read whole.

**Untokenized values in the apps and examples:**

**Engine spacing:**

**Example fixtures:**

> NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

---

## Depth Strategy

Detail: .andromeda/design-system-detail/depth-strategy.md — read whole.

**Chosen approach:**
> NO RECORDED INTENT

**Observed values in the apps and examples:**

**Engine depth behavior:**

Hit-testing stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, and the node itself is still answered inside its border box.

**Example fixtures:**

**Observed absent:**

> NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.

---

## Border Radius

Detail: .andromeda/design-system-detail/border-radius.md — read whole.

**Personality:**
> NO RECORDED INTENT

**Engine radius handling:**

**Example fixtures:**

**Observed absent:**

> NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

---

## Motion (calibrated to expression level `unrecorded`)

Detail: .andromeda/design-system-detail/motion.md — read whole.

**Expression level:**
> NO RECORDED INTENT

**This project's values:**

A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, whatever behaviour was asked, and the requested behaviour, `Smooth` included, applies to the viewport alone; the driver's `scroll` is instant in every box and in the viewport.

**Animation runtime:**

A harness settle answers every member of the document's animating set the same way — none holds it open, none is waited on or advanced: it returns `Settled`, `animating` reading the document's animating flag.

The test harness drives animation from the `VirtualClock` its documents read, and settle reads animation time only through it, never advancing it; the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.

The driver's load wait moves no clock: while an acting step waits on a load in flight, for at most `LOAD_WAIT`, the harness's `VirtualClock` stands still, so a timer or an animation does not run during the wait.

**Reduced motion:**

Observed absent — reduced-motion handling.

**Example fixtures:**

---

## Iconography

Detail: .andromeda/design-system-detail/iconography.md — read whole.

**Style:** Browser toolbar icons are SVG assets rotate-cw, house, arrow-left, arrow-right, ellipsis-vertical, external-link, code, and camera (feature-gated) (apps/browser/src/icons.rs:3-11); tab close and new-tab controls are text glyphs "×" and "+" (apps/browser/src/tab_strip.rs:92-96; apps/browser/src/tab_strip.rs:102-106)
**Library:** `IconButton` renders an `img.urlbar-icon` inside a clickable div (apps/browser/src/icons.rs:13-40; apps/browser/assets/browser.css:200-202); bundle icons are `blitz-logo.png` and `blitz-logo.ico` (apps/browser/Dioxus.toml:7); rdme styles GitHub octicons (apps/readme/assets/github-markdown.css:139-157)
**Size grid:** toolbar icon images 20px high (apps/browser/src/icons.rs:13-40; apps/browser/assets/browser.css:200-202); menu item icons 16x16 (apps/browser/assets/browser.css:243-246); favicons 16x16 (apps/browser/src/tab.rs:269-281)

**Cursors and favicons (engine):**

The cursor is the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there.

**Example fixtures:**

**Observed absent:**

---

## Surface: desktop-native

**Platform:** Windows (NSIS .exe), macOS (.dmg) and Linux (.AppImage) on x86_64 and aarch64 (.github/workflows/publish-browser.yml:47-82); the flake's default package is the browser app whose binary is `blitz`, wrapped with winit/wgpu runtime libraries on Linux (flake.nix:40-53; flake.nix:117-121)
**Toolkit / Framework:** winit windows (packages/blitz-shell/src/window.rs:200-206); HTML launches into a native window with `WindowConfig` and the Vello window renderer (packages/blitz/src/lib.rs:106-137; examples/inner_html.rs:28-30; examples/preact_script.rs:37); Dioxus apps open through `dioxus_native::launch` (examples/box_shadow.rs:4; examples/custom_widget.rs:18; examples/counter/src/main.rs:15-17; examples/seven_guis/src/main.rs:4-10; examples/todomvc/src/main.rs:15-19), built from `WindowAttributes` and titled from dioxus-cli-config or "Dioxus App" (packages/dioxus-native/src/config.rs:17-20; packages/dioxus-native/src/lib.rs:238-239)

### Tokens (platform-specific)
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Component Patterns
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Navigation Pattern
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Platform-Specific Notes
Detail: .andromeda/design-system-detail/surface.md — read whole.

---

## Surface: mobile-native

**Platform:** Android aarch64 APK bundled with `--android --package-types apk --no-default-features --features android-defaults` (.github/workflows/publish-browser.yml:83-89); CI builds (does not test) for `aarch64-apple-ios` and `aarch64-linux-android` (.github/workflows/ci.yml:353-368)
**Toolkit / Framework:** the browser app; `IS_MOBILE` is true for Android and iOS and adds the `mobile` class to the frame (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172)

### Tokens (platform-specific)
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Component Patterns
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Navigation Pattern
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Platform-Specific Notes
Detail: .andromeda/design-system-detail/surface.md — read whole.

---

## Surface: cli

**Platform:** command-line binaries — the WPT runner has `fn main` (wpt/runner/src/main.rs:466); `paint_bench` and `screenshot` examples print to the terminal (examples/paint_bench.rs:113-115; examples/screenshot.rs:31)
**Toolkit / Framework:** output colors come from `owo-colors` (wpt/runner/src/main.rs:27; wpt/runner/Cargo.toml:41)

### Tokens (platform-specific)
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Component Patterns
Detail: .andromeda/design-system-detail/surface.md — read whole.

`escher-session`, the driver's command line, and its `mcp` role are surfaces an agent reads, not a person: stdout holds only JSON — one line per command (the answer, the refusal or a session error's object), or newline-delimited JSON-RPC 2.0 lines — never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not; the command line's exit status carries the class (`0` accepted · `1` refused · `2` usage · `3` session error), and an `mcp` `Err` ending exits `3`.

### Navigation Pattern
> NOT YET MEASURED — no slice recorded argument parsing or command structure for the CLI binaries.

### Platform-Specific Notes
> NOT YET MEASURED — no slice recorded terminal-width handling or platform differences for the CLI binaries.

---

## Surface: web-spa

**Platform:** WASM pages rendering into a canvas — seven_guis and todomvc fill the window with a canvas on `#f5f5f5` (examples/seven_guis/index.html:7-10; examples/todomvc/index.html:7-10)
**Toolkit / Framework:** on wasm32 dioxus-native's surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)

### Tokens (platform-specific)
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Component Patterns
Detail: .andromeda/design-system-detail/surface.md — read whole.

### Navigation Pattern
> NOT YET MEASURED — no slice recorded routing or navigation for the WASM pages.

### Platform-Specific Notes
> NOT YET MEASURED — no slice recorded browser-specific limitations for the WASM pages.

---

## Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

## Self-Validation Protocol

> NO RECORDED INTENT

---

<!-- subjects: palette · status-states · typography · spacing · depth · radius · expression-motion · icons · desktop-native-tokens · desktop-native-titlebar · desktop-native-custom-widget · desktop-native-devtools-overlay · desktop-native-window · desktop-native-stack · mobile-native-tokens · mobile-native-urlbar · mobile-native-back-button · mobile-native-capture · mobile-native-stack · cli-tokens · cli-output · web-spa-tokens · web-spa-canvas -->

## Design Decisions Log

> NO RECORDED INTENT
