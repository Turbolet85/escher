# facts-s04 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 86 files

## architecture §Design Philosophy
- The browser crate's doc comment states it is "A web browser with UI powered by Dioxus Native and content rendering powered by Blitz" (apps/browser/src/main.rs:5)
- The persistence crate states every write path is best-effort and may silently no-op, and that history is UX state, not load-bearing on the rest of the browser (apps/browser/persistence/src/lib.rs:5-19)
- The persistence crate states all methods are synchronous, the crate is runtime-agnostic, and callers keep disk work off latency-sensitive threads (apps/browser/persistence/src/lib.rs:21-25)
- `HistoryService` is documented as the single entry point for visit/favicon writes, owning the in-memory `Store` and the on-disk `HistoryStore` together (apps/browser/src/browser_history.rs:104-111)
- History dedupe policy: only consecutive visits to the same URL fold into the head entry, keeping a chronological log rather than a most-recently-visited set (apps/browser/src/browser_history.rs:51-72)
- The favicon cache stores only positive results so a transient probe failure does not poison the entry for the process lifetime (apps/browser/src/favicon.rs:9-15)
- Chrome writes for a loaded document run as one straight-line block so a render cannot sample a half-applied state (apps/browser/src/tab.rs:126-148)
- `accesskit_xplat` describes itself as a cross-platform AccessKit adapter similar to accesskit_winit but without depending on Winit (packages/accesskit_xplat/src/lib.rs:5-7)
- The transparent example states transparency requires three things: a transparent winit window, an alpha-aware compositing renderer with transparent base color, and CSS that paints no opaque background (examples/transparent/src/main.rs:4-13)
- `wasm_hello` describes itself as a minimal WASM proof driving `BlitzApplication` on `wasm32-unknown-unknown` (examples/wasm_hello/src/lib.rs:1-4)

## architecture §Stack and Technologies
- Rust edition 2024 for browser, browser-persistence, bump, rdme, counter, seven_guis, todomvc, transparent, wasm_hello (apps/browser/Cargo.toml:4; apps/browser/persistence/Cargo.toml:4; apps/bump/Cargo.toml:4; apps/readme/Cargo.toml:4; examples/counter/Cargo.toml:4; examples/seven_guis/Cargo.toml:4; examples/todomvc/Cargo.toml:4; examples/transparent/Cargo.toml:4; examples/wasm_hello/Cargo.toml:4)
- `wgpu_texture` uses edition 2021 (examples/wgpu_texture/Cargo.toml:7)
- `accesskit_xplat` and `blitz-dom` take edition and rust-version from the workspace (packages/accesskit_xplat/Cargo.toml:8-9; packages/blitz-dom/Cargo.toml:10-11)
- The browser UI is built on `dioxus-native` with features svg, system-fonts, net, clipboard, prelude, hot-reload (apps/browser/Cargo.toml:45-52)
- The browser depends on blitz-traits, blitz-dom (woff, parallel-construct, floats), blitz-net (http2), blitz-html, browser-persistence, and optional blitz-paint (apps/browser/Cargo.toml:53-58; apps/browser/Cargo.toml:70)
- Optional JavaScript execution via `blitz-vibey-script`, commented as a Boa-based engine, behind the `javascript` feature (apps/browser/Cargo.toml:34-35; apps/browser/Cargo.toml:57)
- Browser renderer features: vello, hybrid (vello-hybrid), skia, skia-raster variants, cpu variants; default features are hybrid, cookies, cache, screenshot, apple-font-embolden, scrollbars (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:18-26)
- The browser uses `nucleo` "0.5" for fuzzy URL suggestions (apps/browser/Cargo.toml:66; apps/browser/src/url_suggestions.rs:5-8)
- The browser uses tokio with time, rt, sync, macros (apps/browser/Cargo.toml:74)
- Optional `mimalloc` 0.1.48 global allocator (apps/browser/Cargo.toml:77; apps/browser/src/main.rs:7-9)
- `rfd` with xdg-portal on non-Android/iOS targets for save dialogs (apps/browser/Cargo.toml:79-80; apps/browser/src/capture.rs:112-118)
- `android-activity` 0.6.0 with native-activity on Android (apps/browser/Cargo.toml:82-83)
- browser-persistence depends on rusqlite 0.32 (bundled), rusqlite_migration 1.3, tracing, url, and `directories` on non-mobile targets (apps/browser/persistence/Cargo.toml:9-15)
- bump depends on toml_edit 0.22 and semver 1 (apps/bump/Cargo.toml:10-11)
- rdme is version 0.2.0 with `publish = true` (apps/readme/Cargo.toml:3; apps/readme/Cargo.toml:10)
- rdme default features are hybrid, comrak, floats, scrollbars; markdown via optional comrak 0.55 or pulldown-cmark 0.13; file watching via notify 8.0.0; blitz-shell; reqwest (apps/readme/Cargo.toml:13; apps/readme/Cargo.toml:50; apps/readme/Cargo.toml:57; apps/readme/Cargo.toml:60-63)
- rdme selects one `WindowRenderer` alias per renderer feature: skia, skia raster, vello, vello cpu, vello hybrid (apps/readme/src/main.rs:18-29)
- seven_guis uses futures-timer 3, and on wasm32 adds wasm-bindgen, console_error_panic_hook, anyrender_vello_hybrid with webgl, dioxus-native woff (examples/seven_guis/Cargo.toml:23; examples/seven_guis/Cargo.toml:33-39)
- todomvc on wasm32 uses dioxus-native with vello-hybrid and woff, anyrender_vello_hybrid webgl, wasm-bindgen, console_error_panic_hook (examples/todomvc/Cargo.toml:40-44)
- Examples pin `idna_adapter = "=1.0.0"` with the comment "Disable unicode URL support" (examples/counter/Cargo.toml:31-33; examples/seven_guis/Cargo.toml:25-27; examples/todomvc/Cargo.toml:33-35; examples/transparent/Cargo.toml:27-29)
- WASM pages are built with Trunk via a `data-trunk rel="rust"` link (examples/seven_guis/index.html:6; examples/todomvc/index.html:6; examples/wasm_hello/index.html:6; examples/wasm_hello/src/lib.rs:4)
- wasm_hello depends on anyrender_vello_hybrid (webgl), blitz-dom (woff), blitz-html, blitz-shell (tracing), winit, parley, wasm-bindgen, tracing-wasm, web-sys (examples/wasm_hello/Cargo.toml:10-27)
- wgpu_texture depends on anyrender, peniko, blitz crates with custom-widget, dioxus-native, wgpu_context, wgpu, color, bytemuck, pollster (examples/wgpu_texture/Cargo.toml:16-31)
- accesskit_xplat depends on accesskit 0.25 and raw-window-handle 0.6.2, with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.23.0, accesskit_android 0.8.0 per target, and dev-dependency winit-core 0.31.0-beta.2 (packages/accesskit_xplat/Cargo.toml:16-35)
- blitz-dom depends on Servo crates (style, selectors, cssparser, style_config, style_traits, style_dom), taffy, parley, skrifa, accesskit (optional), rayon, image, usvg, wuff, url, web-time (packages/blitz-dom/Cargo.toml:46-95)
- blitz-dom default features: svg, woff, accessibility, system-fonts, file-input, custom-widget (packages/blitz-dom/Cargo.toml:14-21)
- Android entry Activities are Kotlin classes extending `NativeActivity` (apps/browser/MainActivity.kt:9; examples/todomvc/MainActivity.kt:12)
- Dioxus bundle config names publisher DioxusLabs and identifier com.dioxuslabs.blitz (apps/browser/Dioxus.toml:4-7)

## architecture §Established Decisions
- Browser history is persisted to `history.sqlite3` in the `ProjectDirs` data dir for ("com", "DioxusLabs", "Blitz") (apps/browser/persistence/src/lib.rs:283-285)
- If the file-backed open fails the store falls back to an in-memory sqlite connection; on Android/iOS it is always in-memory (apps/browser/persistence/src/lib.rs:277-304)
- Schema migrations are append-only; the list index is the version stamped into `PRAGMA user_version` (apps/browser/persistence/src/lib.rs:35-50)
- `MAX_HISTORY_ENTRIES` is 1000 (apps/browser/persistence/src/lib.rs:52)
- Visit upsert and prune run in one transaction so a crash cannot leave the row count above the cap (apps/browser/persistence/src/lib.rs:206-259)
- Disk writes are dispatched with tokio `spawn_blocking`; with no runtime bound the write is dropped with a warning and a `debug_assert!` (apps/browser/src/browser_history.rs:147-164)
- The initial history read is synchronous on purpose so suggestions have entries on first render (apps/browser/src/main.rs:110-119)
- Favicon success is inferred by decoding the body because the net layer does not surface HTTP status or Content-Type (apps/browser/src/favicon.rs:44-68)
- Non-URL urlbar input becomes a DuckDuckGo search request to `https://html.duckduckgo.com/html/` (apps/browser/src/nav.rs:5-32; apps/browser/src/url_suggestions.rs:14)
- About pages (newtab, settings, history, bookmarks) are Dioxus components and never go through the document loader (apps/browser/src/about_pages.rs:20-77; apps/browser/src/tab.rs:72-82)
- Settings and Bookmarks about pages are stubs rendering "Coming soon." (apps/browser/src/about_pages.rs:73-75; apps/browser/src/about_pages.rs:189-197)
- With `javascript` enabled, external scripts are prefetched through the browser's net provider because the `ScriptFetcher` API is synchronous (apps/browser/src/document_loader.rs:207-255)
- The app always keeps at least one tab open; close controls appear only when more than one tab exists (apps/browser/src/tab.rs:116-124; apps/browser/src/tab_strip.rs:62-65; apps/browser/src/tab_strip.rs:91-97)
- bump versions blitz packages together and anyrender packages together (apps/bump/src/main.rs:5-23; apps/bump/src/main.rs:95-110)
- rdme leaks its file watcher deliberately so it lasts the whole program (apps/readme/src/main.rs:144-146)
- wasm_hello intentionally does not set a surface size on wasm so host CSS sizes the canvas (examples/wasm_hello/src/lib.rs:132-137)
- accesskit_xplat forbids enabling neither or both of `async-io` and `tokio` on Unix via `compile_error!` (packages/accesskit_xplat/src/lib.rs:97-125)

## architecture §Conventions
- Binaries set `windows_subsystem = "windows"` outside tests to hide the console window (apps/browser/src/main.rs:1-2; examples/counter/src/main.rs:1-2; examples/seven_guis/src/main.rs:1-2; examples/todomvc/src/main.rs:1-2; examples/transparent/src/main.rs:1-2)
- Android entry points are `#[unsafe(no_mangle)] pub fn android_main` calling `set_android_app` then launching (apps/browser/src/main.rs:50-55; examples/counter/src/main.rs:8-13; examples/todomvc/src/main.rs:8-13; examples/transparent/src/main.rs:17-22)
- `clippy::expect_used` / `clippy::unwrap_used` are allowed locally next to a justification comment or reason (apps/browser/src/about_pages.rs:60-63; apps/browser/src/tab.rs:46-49; apps/browser/src/tab.rs:105-107; apps/browser/src/tab.rs:117-119; apps/browser/src/favicon.rs:71)
- The browser crate allows `clippy::collapsible_if` crate-wide (apps/browser/src/main.rs:3)
- Reactive state uses `#[derive(Store)]` structs with `#[store]` impl blocks (apps/browser/src/history.rs:7; apps/browser/src/history.rs:22; apps/browser/src/tab.rs:27; apps/browser/src/tab.rs:39; apps/browser/src/browser_history.rs:13; apps/browser/src/browser_history.rs:27)
- Browser chrome CSS uses BEM-style names such as `tab__title`, `tab--active`, `iconbutton--disabled` (apps/browser/assets/browser.css:76; apps/browser/assets/browser.css:89; apps/browser/assets/browser.css:191)
- Browser assets are referenced through the `asset!` macro (apps/browser/src/main.rs:47; apps/browser/src/about_pages.rs:15-18; apps/browser/src/icons.rs:3-11)
- Examples embed CSS as a `const CSS: &str` raw string emitted in a `style` element (examples/counter/src/app.rs:9; examples/counter/src/app.rs:32; examples/seven_guis/src/tasks/counter.rs:9; examples/seven_guis/src/tasks/counter.rs:22; examples/transparent/src/app.rs:55; examples/transparent/src/app.rs:96)
- Unit tests live in a `#[cfg(test)] mod tests` block at the bottom of the source file (apps/browser/src/about_pages.rs:200-201; apps/browser/src/browser_history.rs:166-167; apps/browser/src/favicon.rs:70-72; apps/browser/src/url_suggestions.rs:339-340; apps/browser/persistence/src/lib.rs:310-311)
- Example and app crates set `publish = false` and `license.workspace = true` (apps/browser/Cargo.toml:5-6; examples/counter/Cargo.toml:5-6; examples/todomvc/Cargo.toml:5-6)
- `wasm_hello`'s manifest has no license field (examples/wasm_hello/Cargo.toml:1-5)
- `wgpu_texture` declares `license = "MIT"` with a SixtyFPS copyright header (examples/wgpu_texture/Cargo.toml:1-8; examples/wgpu_texture/src/demo_renderer.rs:1-2)
- `accesskit_xplat` declares `license = "Apache-2.0"` with AccessKit Authors headers (packages/accesskit_xplat/Cargo.toml:4; packages/accesskit_xplat/src/lib.rs:1-3)
- blitz-dom's `assets/default.css` carries a Mozilla Public License 2.0 header (packages/blitz-dom/assets/default.css:1-3)
- `apps/browser/src/util.rs` defines `is_shortcut_mod`, but the crate's module list does not declare a `util` module (apps/browser/src/util.rs:4-12; apps/browser/src/main.rs:21-35)

## architecture §Standard Contracts
- `HistoryEntry` has fields id, url, title, favicon_url, visited_at (apps/browser/persistence/src/lib.rs:56-63)
- `HistoryStore` exposes `open`, `load_recent`, `record_visit`, `clear`, `set_favicon_by_url` (apps/browser/persistence/src/lib.rs:103-148)
- The `history_entries` table has id INTEGER PK AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL, with indexes on visited_at DESC and url (apps/browser/persistence/src/lib.rs:39-49)
- `BrowserNavProvider` implements `NavigationProvider::navigate_to` by pushing onto tab history (apps/browser/src/history.rs:101-109)
- rdme's `ReadmeNavigationProvider` forwards navigation as `BlitzShellEvent::Navigate` (apps/readme/src/main.rs:49-58)
- `LoadedDocument` carries document, html_source, title, favicon_candidate, is_error (apps/browser/src/document_loader.rs:21-34)
- `make_doc_config` builds a `DocumentConfig` with net, navigation, shell and HTML parser providers, font context and abort signal (apps/browser/src/document_loader.rs:45-65)
- Custom paint widgets implement blitz-dom's `Widget` trait (apps/browser/src/fps_overlay.rs:68-92; examples/wgpu_texture/src/demo_renderer.rs:21-80)
- `accesskit_xplat` public API: `WindowEvent` (InitialTreeRequested, ActionRequested, AccessibilityDeactivated), `EventHandler` trait, `Adapter` with `with_split_handlers`, `with_combined_handler`, `update_if_active`, `set_focus`, `set_window_bounds` (packages/accesskit_xplat/src/lib.rs:140-249)
- Each platform adapter exposes `new`, `update_if_active`, `set_focus`, `set_window_bounds` (packages/accesskit_xplat/src/platform_impl/null.rs:14-29; packages/accesskit_xplat/src/platform_impl/unix.rs:13-42)
- Browser CLI: the first argument that parses as a URL, or as a dotted space-free host prefixed with https, becomes the first tab's URL (apps/browser/src/main.rs:60-76; apps/browser/src/main.rs:126-130)
- rdme CLI: first argument is a URL or path, defaulting to the current directory; a directory resolves to the nearest README.md up the tree (apps/readme/src/main.rs:64-67; apps/readme/src/main.rs:153-170; apps/readme/src/main.rs:203-238)
- bump CLI: positional target `blitz` or `anyrender` then a semver version (apps/bump/src/main.rs:67-91)
- wgpu_texture CLI: `--html` selects the Blitz HTML path, otherwise dioxus-native (examples/wgpu_texture/src/main.rs:27-36)
- About URLs are `about:newtab`, `about:settings`, `about:history`, `about:bookmarks` (apps/browser/src/about_pages.rs:51-58)
- View Source writes `view-source://` plus the current URL into the urlbar (apps/browser/src/toolbar.rs:183-213)
- `UrlSuggester` is provided as context with `suggestions()` and `set_query()` (apps/browser/src/url_suggestions.rs:54-68; apps/browser/src/url_suggestions.rs:206-228)
- rdme handles `ReadmeEvent` embedder events, `Navigate` and `NavigationLoad` shell events (apps/readme/src/readme_application.rs:190-215)

## architecture §Occupied Resources
- On-disk file `history.sqlite3` under the DioxusLabs/Blitz data directory (apps/browser/persistence/src/lib.rs:283-285)
- Bundle identifier `com.dioxuslabs.blitz` (apps/browser/Dioxus.toml:6)
- Binary names `blitz`, `seven_guis_native`, `todomvc_native` (apps/browser/Cargo.toml:8-10; examples/seven_guis/Cargo.toml:8-10; examples/todomvc/Cargo.toml:8-10)
- Package names browser, browser-persistence, bump, rdme, counter, seven_guis, todomvc, transparent, wasm_hello, wgpu_texture, accesskit_xplat, blitz-dom (apps/browser/Cargo.toml:2; apps/browser/persistence/Cargo.toml:2; apps/bump/Cargo.toml:2; apps/readme/Cargo.toml:2; examples/counter/Cargo.toml:2; examples/seven_guis/Cargo.toml:2; examples/todomvc/Cargo.toml:2; examples/transparent/Cargo.toml:2; examples/wasm_hello/Cargo.toml:2; examples/wgpu_texture/Cargo.toml:5; packages/accesskit_xplat/Cargo.toml:2; packages/blitz-dom/Cargo.toml:2)
- Outbound search endpoint `https://html.duckduckgo.com/html/` (apps/browser/src/nav.rs:22)
- Windows window icon loaded from resource id 32512 (apps/browser/src/main.rs:79-81)
- Screenshot default file name pattern `blitz-screenshot-<unix-secs>.<ext>` (apps/browser/src/capture.rs:103-107)
- WASM canvas element id `blitz-target` (examples/wasm_hello/index.html:28; examples/wasm_hello/src/lib.rs:110)
- observed absent — any listening socket or port · searched: `TcpListener|bind\(|listen\(|localhost|127\.0\.0\.1` over the 86 slice files

## architecture §Infrastructure Patterns
- rdme builds a multi-thread tokio runtime and enters it before the event loop (apps/readme/src/main.rs:69-75; apps/readme/Cargo.toml:66-67)
- Polling loops: status bar every 100 ms, FPS overlay every 250 ms, history page clock every 30 s (apps/browser/src/status_bar.rs:43-48; apps/browser/src/fps_overlay.rs:105-108; apps/browser/src/about_pages.rs:12-13; apps/browser/src/about_pages.rs:116-122)
- URL suggestions run in a spawned worker fed by an unbounded mpsc channel (apps/browser/src/url_suggestions.rs:147-204; apps/browser/src/url_suggestions.rs:206-228)
- Each document load creates an `AbortController`, aborting the previous load; reload and drop also abort (apps/browser/src/document_loader.rs:84-98; apps/browser/src/document_loader.rs:105-115; apps/browser/src/document_loader.rs:183-187)
- Favicon cache is a process-wide `LazyLock<RwLock<HashMap>>` (apps/browser/src/favicon.rs:14-15)
- rdme watches the opened file with `notify` (NonRecursive) and reloads on change (apps/readme/src/main.rs:132-147; apps/readme/src/readme_application.rs:193-197)
- The browser's Android Activity searches the view tree for a `SurfaceView` and requests focus after layout (apps/browser/MainActivity.kt:11-39)
- todomvc's Android Activity reaches the native view through fixed child indexes (examples/todomvc/MainActivity.kt:13-19)
- Windows bundle sets `webview_install_mode = "Skip"` (apps/browser/Dioxus.toml:9-10)
- bump rewrites `Cargo.toml` files with toml_edit: package version under `./packages/`, workspace version, and workspace dependency versions (apps/bump/src/main.rs:33-62)
- WASM entry points use `#[wasm_bindgen(start)]`, install `console_error_panic_hook`, and load bundled DejaVuSans.woff2 (examples/seven_guis/src/lib.rs:4-20; examples/todomvc/src/wasm.rs:1-15; examples/wasm_hello/src/lib.rs:21-23; examples/wasm_hello/src/lib.rs:102-106)
- blitz-dom depends on objc2 with `disable-encoding-assertions` on Apple targets, commented as a HACK to stop debug builds panicking (packages/blitz-dom/Cargo.toml:97-100)

## architecture §Cross-cutting Patterns
- `tracing` is the logging facade; `tracing_subscriber::fmt::init()` runs only under the `tracing` feature (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- Optional capabilities are gated with `cfg(feature = ...)`: capture module, screenshot, capture, cache menu items (apps/browser/src/main.rs:23-24; apps/browser/src/toolbar.rs:215-258; apps/browser/src/toolbar.rs:278-312)
- Platform behavior is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const for Android/iOS (apps/browser/src/main.rs:48; apps/browser/src/main.rs:79-89; apps/browser/src/tab_strip.rs:9-12)
- Keyboard shortcuts use Cmd on macOS and Ctrl elsewhere (apps/browser/src/tab_strip.rs:42-56; apps/browser/src/util.rs:4-12)
- Shared context values are provided with `use_context_provider` and read with `use_context` (apps/browser/src/main.rs:121-123; apps/browser/src/tab.rs:200; apps/browser/src/toolbar.rs:46)

## architecture §Project Intent
- rdme's manifest description is "Markdown renderering app" (apps/readme/Cargo.toml:5)
- bump's manifest description is "Utility to aid publishing blitz" (apps/bump/Cargo.toml:5)
- blitz-dom's manifest description is "Blitz DOM implementation" (packages/blitz-dom/Cargo.toml:3)
- accesskit_xplat's manifest description is "AccessKit UI accessibility infrastructure: cross-platform adapter" (packages/accesskit_xplat/Cargo.toml:5)
- seven_guis presents itself as "Seven benchmark tasks for GUI frameworks" (examples/seven_guis/src/app.rs:119-120)
- todomvc states it is "The typical TodoMVC app, implemented in Dioxus." (examples/todomvc/src/app.rs:1)
- wgpu_texture's page text states custom WGPU content can render beneath and above HTML layers (examples/wgpu_texture/src/html.rs:64-71)

## architecture §Existing Scopes
- apps/browser: modules about_pages, browser_history, capture, document_loader, favicon, fps_overlay, history, icons, nav, status_bar, tab, tab_strip, toolbar, url_suggestions (apps/browser/src/main.rs:21-35)
- apps/browser/persistence: browser-persistence crate exposing `HistoryStore` (apps/browser/persistence/Cargo.toml:2; apps/browser/persistence/src/lib.rs:1-3)
- apps/bump: release version bump tool (apps/bump/Cargo.toml:2-5)
- apps/readme: rdme markdown viewer with comrak and pulldown_cmark backends (apps/readme/src/main.rs:3-16)
- examples/seven_guis tasks: cells, circle_drawer, counter, crud, flight_booker, temp_converter, timer (examples/seven_guis/src/tasks/mod.rs:1-7)
- examples counter, todomvc, transparent, wasm_hello, wgpu_texture are separate crates (examples/counter/Cargo.toml:2; examples/todomvc/Cargo.toml:2; examples/transparent/Cargo.toml:2; examples/wasm_hello/Cargo.toml:2; examples/wgpu_texture/Cargo.toml:5)
- packages/accesskit_xplat has platform implementations for windows, macos, unix, android and a null fallback (packages/accesskit_xplat/src/platform_impl/mod.rs:9-50)
- out of slice — blitz-dom's Rust sources; only its manifest and `assets/default.css` are in this slice

## security-plan §Threat Model Summary
- The browser fetches and parses arbitrary remote HTML into a document (apps/browser/src/document_loader.rs:117-140)
- JavaScript execution is opt-in behind the non-default `javascript` feature (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:34-35)
- rdme's comrak renderer sets `unsafe: true`, passing raw HTML in markdown through (apps/readme/src/markdown/comrak.rs:27-31)
- rdme fetches remote URLs and treats a `.md` suffix as markdown (apps/readme/src/main.rs:172-201)

## security-plan §Authentication & Authorization
- observed absent — authentication or authorization code · searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 slice files

## security-plan §Input Validation
- Urlbar input is parsed as a URL, then as a dotted space-free host with https, else turned into a search query (apps/browser/src/nav.rs:5-19)
- Opening in an external browser is limited to GET requests with http, https or mailto schemes (apps/browser/src/nav.rs:34-40)
- Favicon bytes must decode as a raster image or SVG to be accepted; tests reject HTML payloads, truncated PNGs and garbage (apps/browser/src/favicon.rs:53-68; apps/browser/src/favicon.rs:95-112)
- Persisted URLs are re-parsed on load and unparsable rows are skipped (apps/browser/persistence/src/lib.rs:184-188)
- SQL statements bind values through `params!` placeholders (apps/browser/persistence/src/lib.rs:165; apps/browser/persistence/src/lib.rs:231-243; apps/browser/persistence/src/lib.rs:249-256; apps/browser/persistence/src/lib.rs:268-271)
- bump rejects a target other than blitz/anyrender and a version that fails semver parsing (apps/bump/src/main.rs:71-91)
- Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59)
- Cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232)
- Circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:107-110)

## security-plan §Data Protection
- History is written to a sqlite file opened with `Connection::open` (apps/browser/persistence/src/lib.rs:280-291)
- observed absent — database encryption · searched: `encrypt|cipher|sqlcipher` (case-insensitive) over the 86 slice files
- On mobile, history is in-memory only and does not persist across launches (apps/browser/persistence/src/lib.rs:12-13)
- The about:history page offers "Clear history", which deletes all rows in memory and on disk (apps/browser/src/about_pages.rs:135-141; apps/browser/src/browser_history.rs:140-144; apps/browser/persistence/src/lib.rs:261-265)
- Cookies and HTTP cache are default browser features; a "Clear Cache" menu item calls `clear_cache` (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:30-31; apps/browser/src/toolbar.rs:298-310)

## security-plan §API Security
- out of slice — no server or API endpoint is defined in these files; outbound HTTP goes through blitz-net with http2 and, in rdme, reqwest (apps/browser/Cargo.toml:55; apps/readme/Cargo.toml:57)

## security-plan §Dependency Security
- rusqlite is built with its `bundled` feature (apps/browser/persistence/Cargo.toml:9)
- Examples pin `idna_adapter` to exactly 1.0.0 (examples/counter/Cargo.toml:31-33)
- Most dependency versions are inherited with `workspace = true` (apps/browser/Cargo.toml:45-74; packages/blitz-dom/Cargo.toml:42-95)
- out of slice — dependency audit or deny configuration and the workspace root manifest

## security-plan §Secret Management
- observed absent — secret-bearing fields or credentials · searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 slice files; only the formula `Token` enum in cells.rs matched

## security-plan §Error Handling
- Persistence errors are logged at warn and swallowed; callers never see a `Result` (apps/browser/persistence/src/lib.rs:14-15; apps/browser/persistence/src/lib.rs:106-108; apps/browser/persistence/src/lib.rs:200-204)
- A failed fetch renders `error.html` with the error's Debug text in the `#error` paragraph (apps/browser/src/document_loader.rs:151-177; apps/browser/assets/error.html:19-20)
- An empty response body renders the bundled 404 page and is flagged `is_error` (apps/browser/src/document_loader.rs:131-137; apps/browser/assets/404.html:12)
- Error pages skip history recording and favicon probing (apps/browser/src/tab.rs:134-143)
- bump's `bail!` macro prints to stderr and exits with status 1 (apps/bump/src/main.rs:25-31)
- rdme exits with status 1 after an stderr message when the argument is neither URL nor file, or no README.md is found (apps/readme/src/main.rs:166-168; apps/readme/src/main.rs:218-221)
- rdme unwraps fetch results and UTF-8 decoding (apps/readme/src/main.rs:182-198; apps/readme/src/readme_application.rs:83-84)
- AnyRender scene capture unwraps archive and file errors (apps/browser/src/capture.rs:75-78)
- accesskit_xplat macOS and Windows adapters call `unimplemented!()` for UiKit and WinRt handles (packages/accesskit_xplat/src/platform_impl/macos.rs:20-24; packages/accesskit_xplat/src/platform_impl/windows.rs:20-24)
- wasm_hello maps event-loop errors into `JsValue` (examples/wasm_hello/src/lib.rs:120; examples/wasm_hello/src/lib.rs:143-145)

## security-plan §Logging & Monitoring
- Successful loads are logged at info with the resolved URL (apps/browser/src/document_loader.rs:121)
- Load failures, script fetch failures and JS errors are logged at error (apps/browser/src/document_loader.rs:152; apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244)
- External-browser open failures log at error (apps/browser/src/nav.rs:36-38)
- Persistence warnings are prefixed `history_store:` (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:298-300)
- Screenshot success and wgpu demo warnings go to stdout via `println!` (apps/browser/src/capture.rs:60; examples/wgpu_texture/src/demo_renderer.rs:30-33; examples/wgpu_texture/src/demo_renderer.rs:75)

## design-system §Color Palette
- Browser chrome: tabstrip `#E0E0E0`, tab `#D0D0D0`, tab hover `#E8E8E8`, active tab and urlbar `#F9F9F9`, urlbar border `#EEE` (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135)
- Browser focus color `#5E9ED6`; icon button hover `#CCC`, active `#BBB` (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189)
- Tooltip `#333` on `#fff`; FPS overlay `rgb(0, 255, 0)` on `rgba(0, 0, 0, 0.7)`; status bar `rgba(240, 240, 240, 0.95)` with `#333` text (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303)
- About pages use `#f0f0f0` backgrounds; history link color `#1a73e8`; muted text `#888`/`#777`/`#aaa` (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2)
- Error page paragraph color `#666` (apps/browser/assets/error.html:14)
- rdme uses GitHub markdown color tokens (`--fgColor-*`, `--bgColor-*`, `--borderColor-*`, syntax colors) defined separately for dark and light schemes (apps/readme/assets/github-markdown.css:13-124)
- rdme override sets dark-mode page background `#0d1117` and light-mode white (apps/readme/assets/blitz-markdown-overrides.css:11-27)
- seven_guis accent `#4a6cf7`, hover `#3a5ce5`, active `#2a4cd3`, page `#f5f5f5`, text `#1a1a1a` (examples/seven_guis/src/tasks/counter.rs:48; examples/seven_guis/src/tasks/counter.rs:58-71; examples/seven_guis/src/app.rs:173)
- seven_guis invalid state `#e53e3e`/`#fff5f5`/`#c53030` and success message `#ebf8ee`/`#68d391`/`#276749` (examples/seven_guis/src/tasks/flight_booker.rs:186-190; examples/seven_guis/src/tasks/flight_booker.rs:221-229)
- todomvc heading `rgba(175, 47, 47, 1.0)`, body text `#4d4d4d` on `#f5f5f5` (examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76)
- counter and transparent buttons use named green, red, blue with white text (examples/counter/src/app.rs:83-111; examples/transparent/src/app.rs:184-212)
- wasm_hello dark palette `#0f1226`, `#1a1d3a`, heading `#ffd166`, code `#ff7b9c` (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68)
- wgpu_texture main background `#f4e8d2` (examples/wgpu_texture/src/styles.css:15)
- blitz-dom default stylesheet: link `rgb(0, 0, 238)`, input focus outline `#4D90FE`, button background `#EFEFEF` (packages/blitz-dom/assets/default.css:42-45; packages/blitz-dom/assets/default.css:92-95; packages/blitz-dom/assets/default.css:105)

## design-system §Typography
- Browser chrome text is sans-serif: tab title 12px, urlbar 14px (16px on mobile), menu items 14px, status bar 12px (apps/browser/assets/browser.css:93-94; apps/browser/assets/browser.css:157-160; apps/browser/assets/browser.css:170; apps/browser/assets/browser.css:225-226; apps/browser/assets/browser.css:297-298)
- Suggestion section headers are 11px, weight 600, uppercase, letter-spacing 0.04em (apps/browser/assets/browser.css:329-336)
- FPS overlay is monospace 12px (apps/browser/assets/browser.css:269-270)
- about:history uses `system-ui, sans-serif` with a 24px/600 heading (apps/browser/assets/about-history.css:7; apps/browser/assets/about-history.css:9-11)
- rdme body font stack begins `-apple-system,BlinkMacSystemFont,"Segoe UI"` at 16px/1.5; monospace stack in `--fontStack-monospace`; weights in `--base-text-weight-*` (apps/readme/assets/github-markdown.css:7-10; apps/readme/assets/github-markdown.css:132-134)
- blitz-dom default stylesheet: inputs/selects/buttons `system-ui, sans-serif`, textarea monospace, h1–h6 from 2em down to 0.67em bold, code/pre `-moz-fixed` (packages/blitz-dom/assets/default.css:82-90; packages/blitz-dom/assets/default.css:301-347; packages/blitz-dom/assets/default.css:358-366; packages/blitz-dom/assets/default.css:662-667)
- seven_guis base is sans-serif 14px with a 36px/700 home title (examples/seven_guis/src/app.rs:169-195)
- todomvc body font `14px 'Helvetica Neue', Helvetica, Arial, sans-serif` weight 300; h1 100px weight 100 (examples/todomvc/src/todomvc.css:24; examples/todomvc/src/todomvc.css:33; examples/todomvc/src/todomvc.css:73-74)
- wgpu_texture uses `system-ui, sans` (examples/wgpu_texture/src/styles.css:7)
- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)

## design-system §Spacing
- Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
- about:history padding 32px 48px; list items 12px 16px with gap 12px and margin-bottom 8px (apps/browser/assets/about-history.css:4; apps/browser/assets/about-history.css:49-54)
- rdme spacing tokens `--base-size-4/8/16/24/40` equal 0.25–2.5rem (apps/readme/assets/github-markdown.css:2-6)
- rdme overrides markdown body to max-width 892px with padding 16px 32px (apps/readme/assets/blitz-markdown-overrides.css:1-5)
- seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:37-41; examples/seven_guis/src/app.rs:181)
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:264-267)

## design-system §Depth Strategy
- Browser z-index: tooltip, menu and suggestions 100; FPS overlay 50; status bar 10 (apps/browser/assets/browser.css:69; apps/browser/assets/browser.css:218; apps/browser/assets/browser.css:274; apps/browser/assets/browser.css:304; apps/browser/assets/browser.css:324)
- Browser menu and suggestions share `box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15)` (apps/browser/assets/browser.css:217; apps/browser/assets/browser.css:323)
- seven_guis cards use `0 2px 8px rgba(0, 0, 0, 0.08)`; circle dialog uses 0.10 (examples/seven_guis/src/tasks/counter.rs:42; examples/seven_guis/src/tasks/timer.rs:76; examples/seven_guis/src/tasks/flight_booker.rs:129; examples/seven_guis/src/tasks/circle_drawer.rs:205)
- todomvc uses layered shadows on the app and footer (examples/todomvc/src/todomvc.css:48; examples/todomvc/src/todomvc.css:279)
- wgpu_texture layers overlay z-index 10, underlay -10, header 100 (examples/wgpu_texture/src/styles.css:26; examples/wgpu_texture/src/styles.css:34; examples/wgpu_texture/src/styles.css:45)
- blitz-dom default dialog backdrop `rgba(0, 0, 0, 0.1)` (packages/blitz-dom/assets/default.css:989-992)

## design-system §Border Radius
- Browser: tabs 4px top corners, tooltip 4px, close button 3px, urlbar input 4px, icon buttons 4px, menu 4px, status bar 3px top-right (apps/browser/assets/browser.css:45-46; apps/browser/assets/browser.css:67; apps/browser/assets/browser.css:104; apps/browser/assets/browser.css:166; apps/browser/assets/browser.css:180; apps/browser/assets/browser.css:216; apps/browser/assets/browser.css:295)
- About pages: clear button 6px, history rows 8px, newtab search input 8px (apps/browser/assets/about-history.css:27; apps/browser/assets/about-history.css:48; apps/browser/assets/about-newtab.css:29)
- seven_guis: cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px (examples/seven_guis/src/tasks/counter.rs:40; examples/seven_guis/src/tasks/counter.rs:60; examples/seven_guis/src/tasks/temp_converter.rs:69; examples/seven_guis/src/app.rs:218; examples/seven_guis/src/app.rs:259)
- transparent card 16px (examples/transparent/src/app.rs:121)
- blitz-dom default button radius 1px (packages/blitz-dom/assets/default.css:102)

## design-system §Motion
- todomvc transitions label color over 0.4s and destroy button color over 0.2s ease-out (examples/todomvc/src/todomvc.css:214; examples/todomvc/src/todomvc.css:234)
- observed absent — transitions or animations in browser chrome and seven_guis styles · searched: `transition|animation|@keyframes` over apps/browser/assets/*.css and examples/seven_guis/src/**/*.rs
- observed absent — reduced-motion handling · searched: `prefers-reduced-motion` over the 86 slice files

## design-system §Iconography
- Browser toolbar icons are SVG assets rotate-cw, house, arrow-left, arrow-right, ellipsis-vertical, external-link, code, and camera (feature-gated) (apps/browser/src/icons.rs:3-11)
- `IconButton` renders an `img.urlbar-icon` (20px high) inside a clickable div (apps/browser/src/icons.rs:13-40; apps/browser/assets/browser.css:200-202)
- Menu item icons are 16x16 (apps/browser/assets/browser.css:243-246)
- Favicons render as 16x16 images (apps/browser/src/tab.rs:269-281)
- Tab close and new-tab controls are text glyphs "×" and "+" (apps/browser/src/tab_strip.rs:92-96; apps/browser/src/tab_strip.rs:102-106)
- Bundle icons are `blitz-logo.png` and `blitz-logo.ico` (apps/browser/Dioxus.toml:7)
- rdme styles GitHub octicons (apps/readme/assets/github-markdown.css:139-157)

## design-system §Surface: desktop-native
- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
- On macOS the tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173)
- rdme titles its window "README for" plus the last path segment, and toggles light/dark theme with Ctrl/Cmd+T (apps/readme/src/main.rs:95-98; apps/readme/src/readme_application.rs:124-131; apps/readme/src/readme_application.rs:175)
- The transparent example opens a 360x300 decoration-less transparent window and closes via the shell provider (examples/transparent/src/app.rs:9-31; examples/transparent/src/app.rs:49-52)
- counter, seven_guis and todomvc launch through `dioxus_native::launch` (examples/counter/src/main.rs:15-17; examples/seven_guis/src/main.rs:4-7; examples/todomvc/src/main.rs:15-19)

## design-system §Surface: mobile-native
- `IS_MOBILE` is true for Android and iOS and adds the `mobile` class to the frame (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172)
- On mobile the urlbar input grows to 16px font with 8px 6px padding (apps/browser/assets/browser.css:140-161)
- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)
- On mobile, screenshots save to a default file name without a dialog (apps/browser/src/capture.rs:109-110)

## design-system §Surface: web-spa
- seven_guis and todomvc WASM pages fill the window with a canvas on `#f5f5f5` (examples/seven_guis/index.html:7-10; examples/todomvc/index.html:7-10)
- wasm_hello's canvas is 80% of the page with a 4px radius, focusable via tabindex 0 and outline removed (examples/wasm_hello/index.html:17-24; examples/wasm_hello/src/lib.rs:113-118)

## design-system §Surface: cli
- bump prints "Bumped anyrender versions" or "Bumped blitz versions" on success and errors to stderr (apps/bump/src/main.rs:25-31; apps/bump/src/main.rs:100; apps/bump/src/main.rs:109)

## layout-templates §Surface: desktop-native
- Browser frame is a full-height flex column: TabStrip, Toolbar, one TabWebView per tab, optional FPS overlay, StatusBar (apps/browser/src/main.rs:168-198; apps/browser/assets/browser.css:5-18)
- Tab content flexes to fill; inactive tabs are `display: none` (apps/browser/assets/browser.css:255-261; apps/browser/src/tab.rs:219-223)
- Toolbar row: back, forward, refresh, home, urlbar with suggestion dropdown, overflow menu (apps/browser/src/toolbar.rs:318-446)
- Status bar is fixed bottom-left at max 60% width; menu dropdown and suggestions are absolutely positioned popovers (apps/browser/assets/browser.css:205-220; apps/browser/assets/browser.css:287-305; apps/browser/assets/browser.css:315-328)
- about:newtab is a centered column of logo and 520px search input (apps/browser/src/about_pages.rs:79-109; apps/browser/assets/about-newtab.css:1-34)
- about:history is a header toolbar with clear button over a list of favicon/title/url/time rows (apps/browser/src/about_pages.rs:111-187; apps/browser/assets/about-history.css:15-85)
- seven_guis Home is a centered 640px column of task cards; TaskShell is a header (back button, title, spacer) over a scrolling body (examples/seven_guis/src/app.rs:114-164; examples/seven_guis/src/app.rs:202-209; examples/seven_guis/src/app.rs:274-319)
- rdme centers a `.markdown-body` column (apps/readme/src/markdown/comrak.rs:40-49; apps/readme/assets/blitz-markdown-overrides.css:1-5)
- todomvc centers a 230–550px body column (examples/todomvc/src/todomvc.css:23-34)
- wgpu_texture main is a grid with a 100px header row over a 1fr canvas row (examples/wgpu_texture/src/styles.css:11-16)

## layout-templates §Surface: mobile-native
- On Android the frame pads 30px top and 44px bottom as a hardcoded safe area (apps/browser/src/main.rs:147-158; apps/browser/src/main.rs:171)
- On mobile the toolbar omits the forward and home buttons (apps/browser/src/toolbar.rs:325-335)
- A bottom-toolbar (`column-reverse`) mobile layout is present only as commented-out CSS (apps/browser/assets/browser.css:141-155)

## layout-templates §Surface: web-spa
- The WASM canvas fills the full body (examples/seven_guis/index.html:8-9; examples/todomvc/index.html:8-9)
- wasm_hello centers its canvas with grid and renders content in a 640px card (examples/wasm_hello/index.html:9-24; examples/wasm_hello/src/lib.rs:51-59)

## layout-templates §Surface: cli
- bump takes two positional arguments and emits single-line output (apps/bump/src/main.rs:67-91; apps/bump/src/main.rs:100; apps/bump/src/main.rs:109)

## test-plan §Test Scope Summary
- Tests exist only in the browser crates: about_pages (3), browser_history (4), favicon (5), url_suggestions (12 `#[test]` plus 4 `#[tokio::test]`), persistence (10) (apps/browser/src/about_pages.rs:204-239; apps/browser/src/browser_history.rs:178-225; apps/browser/src/favicon.rs:83-112; apps/browser/src/url_suggestions.rs:415-629; apps/browser/persistence/src/lib.rs:341-506)
- observed absent — tests in rdme, bump, examples, accesskit_xplat · searched: `#\[(tokio::)?test\]` over the 86 slice files (hits only in the five browser files above)

## test-plan §Test Strategy
- Persistence tests target the inner SQL functions with a freshly migrated in-memory connection rather than the `HistoryStore` wrapper (apps/browser/persistence/src/lib.rs:306-326)
- In-memory history tests call `record_visit_inner` directly on a `VecDeque` (apps/browser/src/browser_history.rs:178-207)
- URL suggestion tests split a synchronous nucleo driver from worker-level tokio tests covering the command state machine (apps/browser/src/url_suggestions.rs:390-406; apps/browser/src/url_suggestions.rs:547-549)

## test-plan §Test Harness Contract
- Tests use the standard `#[test]` harness and `#[tokio::test]` for async worker tests (apps/browser/src/url_suggestions.rs:415; apps/browser/src/url_suggestions.rs:551)
- `drive_worker` queues messages, drops the sender, captures publications, and bounds the run with a 2-second timeout (apps/browser/src/url_suggestions.rs:364-388)
- `make_conn` opens an in-memory sqlite connection and migrates it to latest (apps/browser/persistence/src/lib.rs:322-326)

## test-plan §Unit Test Strategy
- About-page URL parsing: known paths, unknown rejection, round-trip (apps/browser/src/about_pages.rs:204-239)
- History fold, non-consecutive revisit, truncation to the cap, elapsed-label buckets (apps/browser/src/browser_history.rs:178-225)
- Favicon decode acceptance and rejection cases (apps/browser/src/favicon.rs:83-112)
- Suggestions: empty query, literal-first, search-last, case-insensitive and fuzzy matching, cap of six history rows, URL dedup, ranking (apps/browser/src/url_suggestions.rs:415-545)
- Persistence: schema bootstrap, migration validation, round trip, ordering, clear, fold, NULL-only favicon patching, prune cap (apps/browser/persistence/src/lib.rs:341-506)

## test-plan §Integration Test Strategy
- observed absent — integration tests outside `src` · searched: `#\[(tokio::)?test\]` over the 86 slice files; every hit sits inside a `#[cfg(test)] mod tests` in `src`

## test-plan §E2E Test Strategy
- observed absent — end-to-end or UI-driving tests · searched: `#\[(tokio::)?test\]` over the 86 slice files; no test launches a window or document

## test-plan §Test Data & Fixtures
- Fixture helpers build entries from URL strings, with titles or fixed timestamps (apps/browser/src/url_suggestions.rs:348-362; apps/browser/persistence/src/lib.rs:328-339; apps/browser/src/browser_history.rs:170-176)
- Test URLs use the `.test` TLD (apps/browser/src/browser_history.rs:181; apps/browser/persistence/src/lib.rs:377-379)
- A 1x1 PNG is encoded in-test for favicon checks (apps/browser/src/favicon.rs:75-81)

## test-plan §Mocking & Stubbing Discipline
- Worker tests inject a capturing `publish` closure instead of a Dioxus signal (apps/browser/src/url_suggestions.rs:374-378)
- The synchronous nucleo driver uses a no-op notify closure (apps/browser/src/url_suggestions.rs:398)
- observed absent — mocking libraries · searched: `mockall|mock` (case-insensitive) over the 86 slice files

## test-plan §CI Integration
- A test comment states the worker timeout exists so a regression would not hang CI (apps/browser/src/url_suggestions.rs:380-382)
- out of slice — CI workflow files

## obs-plan §Obs Scope Summary
- Observability in this slice is `tracing` logging, optional frame/phase timing features, and an in-app FPS overlay (apps/browser/Cargo.toml:27-29; apps/browser/Cargo.toml:36; apps/browser/src/fps_overlay.rs:94-124)

## obs-plan §Telemetry Strategy
- The browser `tracing` feature enables tracing in dioxus-native, blitz-html, blitz-net, blitz-paint and pulls in tracing-subscriber (apps/browser/Cargo.toml:36)
- rdme's `tracing` feature enables it in blitz-shell, blitz-net, blitz-html (apps/readme/Cargo.toml:42)
- `log-frame-times` and `log-phase-times` features forward to renderer and DOM crates (apps/browser/Cargo.toml:27-29; apps/readme/Cargo.toml:34-41; examples/todomvc/Cargo.toml:23-25; examples/counter/Cargo.toml:21-22)
- blitz-dom's `log-phase-times` enables `debug_timer/enable` (packages/blitz-dom/Cargo.toml:38)

## obs-plan §Observability Harness Contract
- Native subscribers are installed with `tracing_subscriber::fmt::init()` under the `tracing` feature (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- wasm_hello installs `tracing_wasm::set_as_global_default()` (examples/wasm_hello/src/lib.rs:105)

## obs-plan §Span / Trace Coverage
- observed absent — spans or instrumented functions · searched: `span!|#\[instrument|info_span|debug_span` over the 86 slice files

## obs-plan §Metric Coverage
- The FPS overlay records frame deltas in a 60-entry ring, polls every 250 ms, and shows average FPS and ms (apps/browser/src/fps_overlay.rs:7; apps/browser/src/fps_overlay.rs:27-49; apps/browser/src/fps_overlay.rs:105-123)
- The overlay is toggled from the menu item "Toggle FPS" (apps/browser/src/toolbar.rs:437-440)
- observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files

## obs-plan §Log Coverage
- Document loading logs info on success and error on failure (apps/browser/src/document_loader.rs:121; apps/browser/src/document_loader.rs:152)
- Script prefetch failures and JS errors log at error (apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244)
- Urlbar parse failures log at warn (apps/browser/src/toolbar.rs:130)
- Every persistence failure path logs at warn (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:176; apps/browser/persistence/src/lib.rs:202; apps/browser/persistence/src/lib.rs:263; apps/browser/persistence/src/lib.rs:273; apps/browser/persistence/src/lib.rs:298-300)
- Dropped disk writes log at warn (apps/browser/src/browser_history.rs:161)
- wasm_hello logs "Starting app..." at info (examples/wasm_hello/src/lib.rs:107)

## obs-plan §Error Capture & Reporting
- WASM builds install `console_error_panic_hook` (examples/seven_guis/src/lib.rs:13; examples/todomvc/src/wasm.rs:8; examples/wasm_hello/src/lib.rs:104)
- JS errors are drained with `take_js_errors` and logged (apps/browser/src/document_loader.rs:241-244)
- Load errors are shown to the user on an error page with the Debug-formatted error (apps/browser/src/document_loader.rs:154-166)

## obs-plan §PII Scrubbing & Compliance
- Visited URLs are logged at info and urlbar text at warn (apps/browser/src/document_loader.rs:121; apps/browser/src/toolbar.rs:130)
- observed absent — redaction or scrubbing · searched: `redact|scrub|mask` (case-insensitive) over the 86 slice files; hits are a comment and CSS `mask-image` only

## obs-plan §CI Integration
- out of slice — CI workflow files

## a11y-plan §A11y Scope Summary
- `accesskit_xplat` provides the AccessKit platform adapter crate (packages/accesskit_xplat/Cargo.toml:2-6)
- blitz-dom's default features include `accessibility`, which enables `accesskit`; `custom-widget` also requires it (packages/blitz-dom/Cargo.toml:14-22; packages/blitz-dom/Cargo.toml:27)
- The browser's `accessibility` feature (dioxus-native accessibility) is not in its default set (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:38)

## a11y-plan §A11y Strategy
- accesskit_xplat warns that AccessKit developers noted its approach may not be optimal (packages/accesskit_xplat/src/lib.rs:9-11)
- The platform module picks windows, macos, unix (with `accesskit_unix`), android (with `accesskit_android`), else a null adapter (packages/accesskit_xplat/src/platform_impl/mod.rs:9-50)
- Android adapter `set_focus` and `set_window_bounds` are empty; Windows `set_focus` and macOS `set_window_bounds` are no-ops (packages/accesskit_xplat/src/platform_impl/android.rs:43-45; packages/accesskit_xplat/src/platform_impl/windows.rs:36-38; packages/accesskit_xplat/src/platform_impl/macos.rs:42)
- The Android adapter imports no `Rect` yet takes `Rect` parameters in `set_window_bounds` (packages/accesskit_xplat/src/platform_impl/android.rs:5; packages/accesskit_xplat/src/platform_impl/android.rs:45)
- The Android adapter reads the `mSurfaceView` field typed `GameActivity$InputEnabledSurfaceView` (packages/accesskit_xplat/src/platform_impl/android.rs:26-34)

## a11y-plan §A11y Assertion Harness Contract
- accesskit_xplat ships a doc-comment usage example, not tests (packages/accesskit_xplat/src/lib.rs:13-86)
- observed absent — accessibility assertions · searched: `#\[(tokio::)?test\]` over packages/accesskit_xplat/** and `aria-|role` over the 86 slice files (role hits only in github-markdown.css)

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles in app/example markup · searched: `aria-|role:|"role"|role=` over the 86 slice files excluding github-markdown.css and default.css
- rdme's stylesheet styles `[role=button]` and `[role=tabpanel]` focus states (apps/readme/assets/github-markdown.css:340-346; apps/readme/assets/github-markdown.css:1081)
- Images: new-tab logo has `alt: "Blitz"`, favicons `alt: ""`, `IconButton` images no alt (apps/browser/src/about_pages.rs:86; apps/browser/src/tab.rs:278; apps/browser/src/icons.rs:38)
- Browser controls are clickable `div`s rather than buttons: icon buttons, tabs, tab close, new tab, menu items (apps/browser/src/icons.rs:28-39; apps/browser/src/tab_strip.rs:83-106; apps/browser/src/toolbar.rs:419-441)

## a11y-plan §Keyboard Navigation
- Urlbar handles ArrowDown/ArrowUp to move selection, Escape to blur, Enter to submit (apps/browser/src/toolbar.rs:376-397)
- Cmd+T/W (macOS) or Ctrl+T/W open and close tabs (apps/browser/src/tab_strip.rs:42-71)
- rdme uses Ctrl/Cmd plus R (reload), T (theme), B (back) on key release (apps/readme/src/readme_application.rs:170-184)
- The new-tab search input autofocuses and submits on Enter (apps/browser/src/about_pages.rs:87-105)
- todomvc focuses the new-todo input on mount; Enter adds; Enter/Escape/Tab end editing (examples/todomvc/src/app.rs:115-133; examples/todomvc/src/app.rs:143-145; examples/todomvc/src/app.rs:207-212)
- Cells commits an edit on Enter or blur (examples/seven_guis/src/tasks/cells.rs:386-395)
- Android Activities make the native view focusable in touch mode and request focus (apps/browser/MainActivity.kt:34-38; examples/todomvc/MainActivity.kt:11; examples/todomvc/MainActivity.kt:24-27)

## a11y-plan §Visual Design Verification
- Browser urlbar focus shows a `#5E9ED6` border and 1px outline (apps/browser/assets/browser.css:172-175)
- New-tab search input removes the outline and changes only border color on focus (apps/browser/assets/about-newtab.css:30-34)
- todomvc sets `:focus` outline to 0 (examples/todomvc/src/todomvc.css:36-38)
- seven_guis inputs replace the outline with a border color or box-shadow on focus (examples/seven_guis/src/tasks/temp_converter.rs:75-79; examples/seven_guis/src/tasks/flight_booker.rs:175-178)
- counter and transparent buttons show a 4px focus outline (examples/counter/src/app.rs:79-81; examples/transparent/src/app.rs:180-182)
- rdme stylesheet shows a 2px `--focus-outlineColor` outline on focus-visible (apps/readme/assets/github-markdown.css:356-363)
- blitz-dom default stylesheet gives inputs a 2px `#4D90FE` focus outline and suppresses outlines on iframe/body/html focus-visible (packages/blitz-dom/assets/default.css:92-95; packages/blitz-dom/assets/default.css:833-839)
- Disabled icon buttons render at 0.35 opacity (apps/browser/assets/browser.css:191-198)
- rdme follows `prefers-color-scheme` for dark/light tokens and backgrounds (apps/readme/assets/github-markdown.css:13-124; apps/readme/assets/blitz-markdown-overrides.css:11-35)
- blitz-dom default stylesheet un-inverts images and video under `inverted-colors` (packages/blitz-dom/assets/default.css:1077-1088)

## a11y-plan §Screen Reader Support
- accesskit_xplat routes initial-tree requests, action requests and deactivation to one `EventHandler`, returning no initial tree synchronously (packages/accesskit_xplat/src/lib.rs:140-172)
- `Adapter` creation must happen before the window is first shown and panics if it is already visible (packages/accesskit_xplat/src/lib.rs:180-196)
- blitz-dom default stylesheet maps the `dir` attribute with attribute selectors for bidi isolation (packages/blitz-dom/assets/default.css:14-36)

## a11y-plan §Cognitive Accessibility
- History rows show relative time labels: "Just now", minutes, hours, days (apps/browser/src/browser_history.rs:88-102)
- Error and 404 pages show "Failed to load page" and "404 Not found" (apps/browser/assets/error.html:19; apps/browser/assets/404.html:12)
- Tabs show a tooltip with the full title on hover (apps/browser/assets/browser.css:56-74; apps/browser/src/tab_strip.rs:86-90)
- The status bar shows the hovered link target or a loading message (apps/browser/src/status_bar.rs:73-88)
- seven_guis cards carry a description and a tag per task (examples/seven_guis/src/app.rs:23-66; examples/seven_guis/src/app.rs:127-136)
- Invalid dates get an `invalid` class with red styling and the Book button disables (examples/seven_guis/src/tasks/flight_booker.rs:78-91; examples/seven_guis/src/tasks/flight_booker.rs:186-190)

## a11y-plan §CI Integration
- out of slice — CI workflow files and accessibility checks in CI
