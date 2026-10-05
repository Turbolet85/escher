# facts-s11 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 21 files

## architecture §Design Philosophy
- `dioxus-native-dom` describes itself as the "Core headless native renderer for Dioxus based on blitz" (packages/dioxus-native-dom/Cargo.toml:6; packages/dioxus-native-dom/src/lib.rs:3)
- `dioxus-native` describes itself as the "Native renderer for Dioxus based on blitz" (packages/dioxus-native/Cargo.toml:6; packages/dioxus-native/src/lib.rs:3)
- `stylo_taffy` states it holds conversion functions from Stylo types to Taffy types, is an implementation detail of `blitz-dom`, and can also be used standalone as a reference for integrating stylo with taffy (packages/stylo_taffy/src/lib.rs:1-4; packages/stylo_taffy/Cargo.toml:4)
- `DioxusDocument` integrates `BaseDocument` from blitz-dom with `VirtualDom` from dioxus-core; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:39; packages/dioxus-native-dom/src/dioxus_document.rs:65-66)
- `TaffyStyloStyle` wraps anything that derefs to stylo `ComputedValues` and implements taffy's layout traits so it can be used directly with taffy's layout algorithms (packages/stylo_taffy/src/wrapper.rs:26-33)
- `to_taffy_style` eagerly converts an entire stylo `ComputedValues` into a `taffy::Style` (packages/stylo_taffy/src/convert.rs:833-834)
- Anchor positioning is stated as "flagged off for time being"; its size variants hit `unreachable!()` (packages/stylo_taffy/src/convert.rs:116-118; packages/stylo_taffy/src/convert.rs:136-138; packages/stylo_taffy/src/convert.rs:191-194)

## architecture §Stack and Technologies
- Language Rust, edition 2024 for `dioxus-native-dom` and `dioxus-native` (packages/dioxus-native-dom/Cargo.toml:5; packages/dioxus-native/Cargo.toml:5)
- `stylo_taffy` takes version, edition and rust-version from the workspace (packages/stylo_taffy/Cargo.toml:6-11)
- `dioxus-native-dom` and `dioxus-native` are at version 0.7.0 (packages/dioxus-native-dom/Cargo.toml:3; packages/dioxus-native/Cargo.toml:3)
- `dioxus-native-dom` depends on blitz-dom (default features off, `custom-widget` on), blitz-traits, dioxus-core, dioxus-html, keyboard-types, optional tracing, rustc-hash and futures-util (packages/dioxus-native-dom/Cargo.toml:27-41)
- `dioxus-native` depends on blitz-dom, optional blitz-html, optional blitz-net, optional blitz-paint, blitz-traits and blitz-shell (packages/dioxus-native/Cargo.toml:71-76)
- `dioxus-native` depends on anyrender and peniko, with optional anyrender_vello, anyrender_vello_hybrid, anyrender_vello_cpu, anyrender_skia and wgpu_context (packages/dioxus-native/Cargo.toml:79-85)
- `dioxus-native` depends on dioxus-core, dioxus-html, dioxus-native-dom, dioxus-asset-resolver (feature `native`), dioxus-history, dioxus-document and dioxus-cli-config (packages/dioxus-native/Cargo.toml:88-94)
- `dioxus-native` optionally depends on dioxus-devtools for hot reload and on dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-core-macro and manganis for its prelude (packages/dioxus-native/Cargo.toml:96-104)
- `dioxus-native` uses winit for windowing and keyboard-types for input (packages/dioxus-native/Cargo.toml:106-108)
- `dioxus-native` uses optional tokio (feature `rt`, plus `rt-multi-thread` off wasm32) and webbrowser (packages/dioxus-native/Cargo.toml:111-112; packages/dioxus-native/Cargo.toml:121-122)
- `dioxus-native` also depends on tracing (optional), rustc-hash, futures-util, cfg-if "1.0.4" and slotmap (packages/dioxus-native/Cargo.toml:115-119)
- `dioxus-native` pulls android-activity "0.6" (default features off) on Android and anyrender_vello_cpu with `pixels_window_renderer` on the iOS simulator (packages/dioxus-native/Cargo.toml:124-128)
- `stylo_taffy` depends on bitflags, taffy, style and style_atoms (packages/stylo_taffy/Cargo.toml:15-18)
- The window renderer is chosen at compile time with `cfg_if` among vello, vello-cpu-base, skia, skia-raster-base and vello-hybrid, with a `compile_error!` when none is enabled (packages/dioxus-native/src/dioxus_renderer.rs:9-29)
- `dioxus-native-dom` has the `dioxus` crate as a dev-dependency (packages/dioxus-native-dom/Cargo.toml:43-44)

## architecture §Established Decisions
- `dioxus-native` default features are accessibility, hot-reload, net, html, svg, system-fonts, clipboard, file-dialog, vello-hybrid, woff and apple-font-embolden (packages/dioxus-native/Cargo.toml:13)
- `dioxus-native-dom` default features are accessibility, svg and system-fonts (packages/dioxus-native-dom/Cargo.toml:13)
- `stylo_taffy` default features are std, block, flexbox and grid; floats is opt-in (packages/stylo_taffy/Cargo.toml:21-26)
- The `net` feature is stated to be a no-op on wasm32, where `launch_cfg` falls back to the in-process `DioxusNativeNetProvider` (packages/dioxus-native/Cargo.toml:50-52; packages/dioxus-native/src/lib.rs:197-198)
- `woff` is default-on so wasm callers, which cannot use system fonts, can bundle compressed fonts (packages/dioxus-native/Cargo.toml:23-25)
- The document `base_url` defaults to `dioxus://index.html` when none is configured (packages/dioxus-native-dom/src/dioxus_document.rs:88-92)
- The blitz `DEFAULT_CSS` user-agent stylesheet is always added to a `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:95-96)
- Hot reload is wired only under `hot-reload` with `debug_assertions` and off wasm32 (packages/dioxus-native/src/lib.rs:166-175)
- Navigation history is an in-memory `MemoryHistory` (packages/dioxus-native/src/dioxus_application.rs:161-164)
- JavaScript `eval` on the native document delegates to `NoOpDocument` (packages/dioxus-native/src/contexts.rs:19-21)
- `overflow: auto` maps to taffy `Overflow::Scroll`, with a TODO to support Auto in taffy (packages/stylo_taffy/src/convert.rs:367-368)
- `display: table` maps to taffy Grid under the `grid` feature, with TODOs for display:contents and table layout (packages/stylo_taffy/src/convert.rs:222-225)
- Grid subgrid and masonry are not implemented and convert to none / empty (packages/stylo_taffy/src/wrapper.rs:453-455; packages/stylo_taffy/src/convert.rs:698-700)
- `scrollbar_width` is always 0.0 in the taffy style (packages/stylo_taffy/src/wrapper.rs:99-102; packages/stylo_taffy/src/convert.rs:853)
- `gap: normal` resolves to 0 for flexbox and grid (packages/stylo_taffy/src/convert.rs:539-541)
- Sizing keywords (max-content, min-content, fit-content, stretch) on min/max size properties resolve to auto (packages/stylo_taffy/src/convert.rs:128-134; packages/stylo_taffy/src/convert.rs:148-154)
- Baseline content-alignment falls back to start/end, and last-baseline item alignment maps to end (packages/stylo_taffy/src/convert.rs:432-435; packages/stylo_taffy/src/convert.rs:510-512)
- In block containers, positional content-alignment keywords default to `safe` unless `unsafe` is explicit (packages/stylo_taffy/src/convert.rs:407-447)
- Container `align-items`/`justify-items: normal` are left unset so taffy applies its per-item default (packages/stylo_taffy/src/convert.rs:522-534)
- Insets are reported as auto for `position: static` boxes (packages/stylo_taffy/src/convert.rs:264-270)

## architecture §Conventions
- Dependencies are inherited from the workspace with `workspace = true` (packages/dioxus-native-dom/Cargo.toml:27-44; packages/dioxus-native/Cargo.toml:71-117; packages/stylo_taffy/Cargo.toml:15-18)
- docs.rs builds with all features, and crates enable `doc_cfg` under `docsrs` (packages/dioxus-native-dom/Cargo.toml:46-47; packages/dioxus-native/Cargo.toml:130-131; packages/dioxus-native-dom/src/lib.rs:1; packages/dioxus-native/src/lib.rs:1)
- Tracing calls are gated with `#[cfg(feature = "tracing")]` at each call site (packages/dioxus-native/src/assets.rs:47; packages/dioxus-native/src/assets.rs:52; packages/dioxus-native/src/assets.rs:59; packages/dioxus-native/src/link_handler.rs:14; packages/dioxus-native/src/dioxus_application.rs:137)
- `dioxus-native-dom` defines a crate-local `trace!` macro that expands to `tracing::debug!` only under the `tracing` feature (packages/dioxus-native-dom/src/lib.rs:32-55)
- The 4-argument arm of that `trace!` macro passes only the first two items to `tracing::debug!` (packages/dioxus-native-dom/src/lib.rs:46-49)
- The crate-level docs of both crates list feature flags `default`, `accessibility`, `hot-reload`, `menu` (muda menubar) and `tracing` (packages/dioxus-native-dom/src/lib.rs:5-10; packages/dioxus-native/src/lib.rs:5-10)
- observed absent — a `menu` feature or `muda` dependency in any manifest, and a `hot-reload` feature in the `dioxus-native-dom` manifest · searched: `menu|hot-reload|muda` over the 3 s11 Cargo.toml files
- An `unsafe` block carries a `// SAFETY:` comment (packages/stylo_taffy/src/convert.rs:84-85)
- Unfinished work is marked with `TODO`/`FIXME`/`todo` comments (packages/dioxus-native-dom/src/mutation_writer.rs:125; packages/dioxus-native-dom/src/mutation_writer.rs:285; packages/dioxus-native-dom/src/mutation_writer.rs:291; packages/dioxus-native/src/lib.rs:107; packages/dioxus-native/src/dioxus_application.rs:180)
- An ordering-sensitive block is marked "WARNING: DO NOT REORDER" (packages/dioxus-native-dom/src/mutation_writer.rs:181-185)
- Stylo types are aliased through a private `stylo` module for shorter names (packages/stylo_taffy/src/convert.rs:3-58)
- Conversion functions are marked `#[inline]` and gated per layout feature with `#[cfg(feature = ...)]` (packages/stylo_taffy/src/convert.rs:556-599; packages/stylo_taffy/src/convert.rs:628-663)

## architecture §Standard Contracts
- `launch(app)` launches with empty contexts and configs; `launch_cfg` and `launch_cfg_with_props` accept root-context factories and `Box<dyn Any>` configs (packages/dioxus-native/src/lib.rs:94-113)
- Launch configs are read by downcasting each `Box<dyn Any>` to `Features`, `Limits` (vello/vello-hybrid only), `WindowAttributes` or `Config`; unmatched configs are dropped (packages/dioxus-native/src/lib.rs:114-145)
- `Config` implements `LaunchConfig` and offers `with_window_attributes`, `with_font_ctx`, `with_alpha_mode` and `with_base_color` (packages/dioxus-native/src/config.rs:7-15; packages/dioxus-native/src/config.rs:45-80)
- `RendererOptions` carries `base_color`, `alpha_mode`, and, for vello/vello-hybrid, wgpu `features` and `limits` (packages/dioxus-native/src/dioxus_renderer.rs:31-47)
- `DioxusNativeWindowRenderer` implements anyrender `RenderContext` and `WindowRenderer` by delegating to an inner renderer (packages/dioxus-native/src/dioxus_renderer.rs:110-169)
- Public hooks: `use_window`, `use_raw_window_handle` (packages/dioxus-native/src/lib.rs:81-92); `use_window_event` and `use_back_button`, each returning a `WinitEventHandlerId` (packages/dioxus-native/src/hooks.rs:11-49)
- `WinitEventHandlerId::remove` unregisters a window event handler (packages/dioxus-native/src/event_handlers.rs:6-16)
- `DioxusNativeEvent` has a `DevserverEvent` variant (hot-reload, debug only) and a `CreateHeadElement` variant with window, name, attributes and contents (packages/dioxus-native/src/dioxus_application.rs:18-33)
- The native `Document` context turns title, meta, script, style and link requests into `CreateHeadElement` events sent through the shell proxy (packages/dioxus-native/src/contexts.rs:23-65)
- `DioxusDocument` implements the blitz `Document` trait: `id`, `inner`, `inner_mut`, `poll`, `handle_ui_event` (packages/dioxus-native-dom/src/dioxus_document.rs:206-260)
- DOM events are routed to the vdom by finding the nearest node in the chain with a `data-dioxus-id` attribute parsed as `usize` (packages/dioxus-native-dom/src/dioxus_document.rs:29-37; packages/dioxus-native-dom/src/dioxus_document.rs:342-348)
- Registering a listener sets a `"<rust func>"` placeholder attribute and a `data-dioxus-id` attribute on the element (packages/dioxus-native-dom/src/mutation_writer.rs:316-323)
- The `__webview_document` attribute sets or removes a sub-document; the `data` attribute on `<object>` sets or removes a custom widget (packages/dioxus-native-dom/src/mutation_writer.rs:226-262)
- Attributes in the `style` namespace become style properties; falsy `checked` clears the attribute; `dangerous_inner_html` sets inner HTML (packages/dioxus-native-dom/src/mutation_writer.rs:390-413)
- `SubDocumentAttr` and `CustomWidgetAttr` are write-once attribute values compared by id (packages/dioxus-native-dom/src/write_once_attr.rs:6-70)
- The HTML event converter maps form, mouse, keyboard, focus, mounted, pointer, scroll, touch and wheel data (packages/dioxus-native-dom/src/events.rs:41-137)
- `NodeHandle` backs `MountedData` with scroll offset, scroll size, client rect, scroll_to, scroll and set_focus (packages/dioxus-native-dom/src/events.rs:194-296)
- `stylo_taffy` exports `to_taffy_style`, `TaffyStyloStyle`, `StyleFlags` and `Atom` (packages/stylo_taffy/src/lib.rs:6-13)
- `TaffyStyloStyle` implements taffy `CoreStyle`, `BlockContainerStyle`, `BlockItemStyle`, `FlexboxContainerStyle`, `FlexboxItemStyle`, `GridContainerStyle`, `GridItemStyle` and `OofItemStyle` (packages/stylo_taffy/src/wrapper.rs:62; packages/stylo_taffy/src/wrapper.rs:205; packages/stylo_taffy/src/wrapper.rs:222; packages/stylo_taffy/src/wrapper.rs:243; packages/stylo_taffy/src/wrapper.rs:295; packages/stylo_taffy/src/wrapper.rs:408; packages/stylo_taffy/src/wrapper.rs:596; packages/stylo_taffy/src/wrapper.rs:629)
- `StyleFlags` has one flag, `IS_REPLACED`, for replaced elements (packages/stylo_taffy/src/wrapper.rs:16-24)
- `dioxus-native` re-exports all of `dioxus_native_dom`, plus `CompositeAlphaMode`, `Color`, `FontContext`, `Widget`, `build_single_font_ctx`, `winit`, `LogicalSize`, `PhysicalSize` and `WindowAttributes` (packages/dioxus-native/src/lib.rs:26-69)
- A `prelude` module, behind the `prelude` feature, re-exports dioxus-core, the RSX macros, dioxus-html, manganis, hooks, signals, stores, document and history items (packages/dioxus-native/src/lib.rs:21-22; packages/dioxus-native/src/prelude.rs:1-28)

## architecture §Occupied Resources
- The `dioxus` URL scheme is the default document base (`dioxus://index.html`) and is served from the file system by `dioxus_asset_resolver::native::serve_asset` (packages/dioxus-native-dom/src/dioxus_document.rs:91; packages/dioxus-native/src/assets.rs:43-45)
- The default window title is the dioxus-cli-config app title, else "Dioxus App" (packages/dioxus-native/src/config.rs:19-20)
- The app renders into a `<main id="main">` element under `<body>` (packages/dioxus-native-dom/src/dioxus_document.rs:123-130)
- Event-handler counts are kept in a fixed array of 64 slots indexed by `DomEventKind` discriminant (packages/dioxus-native-dom/src/mutation_writer.rs:22-23; packages/dioxus-native-dom/src/mutation_writer.rs:36)
- observed absent — any network port, host or socket binding · searched: `port|127\.0\.0\.1|localhost` over the 21 listed s11 files

## architecture §Infrastructure Patterns
- A winit event loop is created with `create_default_event_loop`, wrapped in a `BlitzShellProxy`, and run with `run_app` (packages/dioxus-native/src/lib.rs:151-153; packages/dioxus-native/src/lib.rs:242-243)
- With `net` off wasm32, a multi-threaded tokio runtime is built and entered before launch (packages/dioxus-native/src/lib.rs:155-164)
- With `net` off wasm32, a `blitz_net::Provider` is created with the shell proxy as waker, provided as a root context, and wrapped by `DioxusNativeNetProvider` (packages/dioxus-native/src/lib.rs:185-195)
- Without `net`, the inner net provider is a data-URI provider when `data-uri` is on, else none (packages/dioxus-native/src/assets.rs:16-28)
- Hot reload connects through `dioxus_devtools::connect`, forwarding devserver messages as embedder events (packages/dioxus-native/src/lib.rs:171-174)
- On a hot-reload message every window applies vdom changes and reloads changed assets by href; a devserver `Shutdown` exits the event loop (packages/dioxus-native/src/dioxus_application.rs:64-87)
- `DioxusNativeApplication` delegates winit `ApplicationHandler` callbacks to an inner `BlitzApplication` and drains embedder events from the event queue on `proxy_wake_up` (packages/dioxus-native/src/dioxus_application.rs:114-213)
- Android support exposes `set_android_app`/`current_android_app` and re-exports `AndroidApp` (packages/dioxus-native/src/lib.rs:38-55)
- On wasm32 the default window attributes append a canvas to the document body and do not seed a surface size (packages/dioxus-native/src/config.rs:22-34)
- On macOS the application forwards the macOS handler extension (packages/dioxus-native/src/dioxus_application.rs:11-12; packages/dioxus-native/src/dioxus_application.rs:115-118)

## architecture §Cross-cutting Patterns
- Services are provided to components as root contexts: net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer and winit window (packages/dioxus-native/src/lib.rs:181-205; packages/dioxus-native/src/dioxus_application.rs:147-172)
- Event handling is skipped entirely for an event kind whose handler count is zero; counts rise on listener creation and fall on removal (packages/dioxus-native-dom/src/dioxus_document.rs:275-281; packages/dioxus-native-dom/src/mutation_writer.rs:327-337)
- `mounted` listeners are queued and fired after `initial_build` and after each `poll` render (packages/dioxus-native-dom/src/mutation_writer.rs:309-314; packages/dioxus-native-dom/src/dioxus_document.rs:145-153; packages/dioxus-native-dom/src/dioxus_document.rs:183-203)
- The blitz document is shared as `Rc<RefCell<BaseDocument>>` (packages/dioxus-native-dom/src/dioxus_document.rs:68; packages/dioxus-native-dom/src/events.rs:139-143)
- Window event handlers are stored in a `SlotMap` and applied per window id before inner handling (packages/dioxus-native/src/event_handlers.rs:25-63; packages/dioxus-native/src/dioxus_application.rs:197-199)
- Each feature flag forwards to the same-named feature of the underlying blitz crates (packages/dioxus-native/Cargo.toml:16-31; packages/dioxus-native-dom/Cargo.toml:15-23)

## architecture §Project Intent
- Both Dioxus crates use keywords dom, ui, gui and react, with repository `https://github.com/DioxusLabs/dioxus/` and homepage `https://dioxuslabs.com/learn/0.7/getting_started` (packages/dioxus-native-dom/Cargo.toml:8-10; packages/dioxus-native/Cargo.toml:8-10)
- `stylo_taffy` uses keywords css and layout (packages/stylo_taffy/Cargo.toml:5)
- `launch` is documented as launching "an interactive HTML/CSS renderer driven by the Dioxus virtualdom" (packages/dioxus-native/src/lib.rs:94)

## architecture §Existing Scopes
- `dioxus-native-dom` has modules dioxus_document, events, mutation_writer and write_once_attr (packages/dioxus-native-dom/src/lib.rs:12-15)
- `dioxus-native` has modules assets, config, contexts, dioxus_application, dioxus_renderer, event_handlers, hooks, link_handler and an optional prelude (packages/dioxus-native/src/lib.rs:12-22)
- `stylo_taffy` has modules wrapper and convert (packages/stylo_taffy/src/lib.rs:6-9)
- `dioxus-native` depends on `dioxus-native-dom` (packages/dioxus-native/Cargo.toml:90)

## security-plan §Threat Model Summary
- out of slice — no threat model is stated in these files

## security-plan §Authentication & Authorization
- observed absent — any authentication or authorization code or credential field · searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)

## security-plan §Input Validation
- `NativeFormData::valid` always returns true, with the comment "todo: actually implement validation here" (packages/dioxus-native-dom/src/events.rs:316-319)
- The `data-dioxus-id` attribute value is parsed as `usize`, and a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:30-37)
- Navigation opens a URL only for GET requests with an `http`, `https` or `mailto` scheme (packages/dioxus-native/src/link_handler.rs:9-13)
- A `dangerous_inner_html` attribute value is applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:408-409)
- Attribute values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:284-286)

## security-plan §Data Protection
- observed absent — encryption or transport-security code · searched: `encrypt|crypt|tls|rustls|cert` over the 21 listed s11 files

## security-plan §API Security
- Requests with the `dioxus` scheme are answered from the local file system by `serve_asset` on the URL path; other schemes go to the inner net provider (packages/dioxus-native/src/assets.rs:42-61)
- Allowed navigation targets are handed to the system browser through `webbrowser::open` (packages/dioxus-native/src/link_handler.rs:12)
- JavaScript evaluation requested through the document context goes to `NoOpDocument` (packages/dioxus-native/src/contexts.rs:19-21)

## security-plan §Dependency Security
- All but two dependencies are workspace-inherited; `cfg-if` is pinned at "1.0.4" and `android-activity` at "0.6" in the manifest (packages/dioxus-native/Cargo.toml:118; packages/dioxus-native/Cargo.toml:125)
- One `unsafe` block builds a taffy `LengthPercentage` from a raw calc pointer (packages/stylo_taffy/src/convert.rs:81-86)
- out of slice — lockfile, audit or deny configuration

## security-plan §Secret Management
- observed absent — secrets or environment-variable reads · searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files

## security-plan §Error Handling
- Twelve event-data conversions (cancel, animation, clipboard, composition, drag, image, media, selection, toggle, transition, resize, visible) call `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:42-44; packages/dioxus-native-dom/src/events.rs:70-92; packages/dioxus-native-dom/src/events.rs:110-116; packages/dioxus-native-dom/src/events.rs:122-124; packages/dioxus-native-dom/src/events.rs:130-136)
- Supported event-data conversions `unwrap` the platform-data downcast (packages/dioxus-native-dom/src/events.rs:46-128)
- Building the tokio runtime, running the event loop and getting the raw window handle all `unwrap` (packages/dioxus-native/src/lib.rs:158-161; packages/dioxus-native/src/lib.rs:243; packages/dioxus-native/src/lib.rs:86-91)
- `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document"; `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:154-176)
- Missing nodes in mounted-element operations return `MountedError::OperationFailed` wrapping `NodeNotExistErr` (packages/dioxus-native-dom/src/events.rs:178-192; packages/dioxus-native-dom/src/events.rs:211-214; packages/dioxus-native-dom/src/events.rs:228-233)
- `element_to_node_id` unwraps; `try_element_to_node_id` returns an option (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
- A failed asset fetch is logged at warn and the handler is not called (packages/dioxus-native/src/assets.rs:51-54)
- A failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:12-16)
- `current_android_app` is documented to panic if the activity has not been set up (packages/dioxus-native/src/lib.rs:47-51)
- Anchor-positioning sizes and non-breadth `fit-content()` track limits hit `unreachable!()` (packages/stylo_taffy/src/convert.rs:116-118; packages/stylo_taffy/src/convert.rs:788-792)
- Unknown alignment flags are mapped to none rather than panicking (packages/stylo_taffy/src/convert.rs:436-437; packages/stylo_taffy/src/convert.rs:513-514)

## security-plan §Logging & Monitoring
- Logging uses the optional `tracing` crate behind a `tracing` feature that also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net (packages/dioxus-native/Cargo.toml:67; packages/dioxus-native-dom/Cargo.toml:19)
- Asset-fetch logs print the full request with Debug formatting (packages/dioxus-native/src/assets.rs:48; packages/dioxus-native/src/assets.rs:53; packages/dioxus-native/src/assets.rs:60)

## design-system §Color Palette
- The frame clear color is an optional `base_color` (a `peniko::Color`), unset by default (packages/dioxus-native/src/config.rs:12; packages/dioxus-native/src/config.rs:40; packages/dioxus-native/src/config.rs:76-80; packages/dioxus-native/src/dioxus_renderer.rs:37-38)
- observed absent — color tokens or CSS custom properties · searched: `var\(--|--[a-z]+-` over the 21 listed s11 files

## design-system §Typography
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64)
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:45-46)

## design-system §Spacing
- out of slice — no spacing scale or tokens; these files only convert CSS margin, padding and gap values to taffy

## design-system §Depth Strategy
- observed absent — shadows, elevation or z-index tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

## design-system §Border Radius
- observed absent — border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

## design-system §Motion
- Programmatic scrolling supports `Smooth` and `Instant` behaviour (packages/dioxus-native-dom/src/events.rs:235-238; packages/dioxus-native-dom/src/events.rs:274-277)
- Animation and transition event data are `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:70-72; packages/dioxus-native-dom/src/events.rs:122-124)

## design-system §Iconography
- observed absent — icons or icon sets · searched: `icon` over the 21 listed s11 files

## design-system §Surface: desktop-native
- The surface is a winit window built from `WindowAttributes`, titled from dioxus-cli-config or "Dioxus App" (packages/dioxus-native/src/config.rs:17-20; packages/dioxus-native/src/lib.rs:236-237)
- Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet (packages/dioxus-native-dom/src/dioxus_document.rs:95-96)
- On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
- Optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:47)

## layout-templates §Surface: desktop-native
- Each document starts with the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>` and the app mounts into `main` (packages/dioxus-native-dom/src/dioxus_document.rs:98-133)
- Arbitrary `index.html` templates are not supported (a TODO) (packages/dioxus-native-dom/src/dioxus_document.rs:108)
- Head elements (title, meta, script, style, link) are appended to `<head>`, with optional text contents (packages/dioxus-native-dom/src/dioxus_document.rs:155-181)

## test-plan §Test Scope Summary
- `dioxus-native-dom` has three unit tests: `keyed_nodes_do_not_crash`, `touches_reports_all_active_pointers` and `touches_is_empty_when_no_pointers_are_active` (packages/dioxus-native-dom/src/dioxus_document.rs:375-411; packages/dioxus-native-dom/src/events.rs:684-728)
- observed absent — tests in `dioxus-native` and `stylo_taffy` · searched: `#\[test\]|#\[cfg\(test\)\]` over the 21 listed s11 files (matches only in dioxus_document.rs and events.rs)

## test-plan §Test Strategy
- Tests are Rust `#[cfg(test)]` modules inside the source files (packages/dioxus-native-dom/src/dioxus_document.rs:366-367; packages/dioxus-native-dom/src/events.rs:656-657)
- `keyed_nodes_do_not_crash` is a regression test for a panic when keyed nodes are reordered (packages/dioxus-native-dom/src/dioxus_document.rs:375-378)
- A rustdoc usage example for `DioxusDocument` sits in the doc comment (packages/dioxus-native-dom/src/dioxus_document.rs:41-63)

## test-plan §Test Harness Contract
- The dioxus-native-dom tests use the `dioxus` crate as a dev-dependency (packages/dioxus-native-dom/Cargo.toml:43-44; packages/dioxus-native-dom/src/dioxus_document.rs:370)
- The document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)` (packages/dioxus-native-dom/src/dioxus_document.rs:394-404)

## test-plan §Unit Test Strategy
- Touch-data tests build `BlitzPointerEvent` values directly and check `touches`/`touches_changed` counts and coordinates (packages/dioxus-native-dom/src/events.rs:684-728)

## test-plan §Integration Test Strategy
- `keyed_nodes_do_not_crash` runs a real `VirtualDom` against a `DioxusDocument` through 100 inserts, then checks that `<main>` has 100 children (packages/dioxus-native-dom/src/dioxus_document.rs:394-410)

## test-plan §E2E Test Strategy
- out of slice — no end-to-end tests in these files

## test-plan §Test Data & Fixtures
- `finger_event(id, x, y)` builds a finger `BlitzPointerEvent` fixture (packages/dioxus-native-dom/src/events.rs:663-682)
- The keyed-nodes test uses a shared `Rc<RefCell<HashMap<usize, usize>>>` as app props (packages/dioxus-native-dom/src/dioxus_document.rs:379-394)

## test-plan §Mocking & Stubbing Discipline
- observed absent — mocks, stubs or fakes · searched: `mock|stub|fake` over the 21 listed s11 files

## test-plan §CI Integration
- out of slice — no CI configuration in these files

## obs-plan §Obs Scope Summary
- Observability is optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:58-67; packages/dioxus-native-dom/Cargo.toml:19)

## obs-plan §Telemetry Strategy
- The `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:67)
- `log-frame-times` turns on `log_frame_times` in whichever anyrender backend is enabled; `log-phase-times` forwards to blitz-dom (packages/dioxus-native/Cargo.toml:59-66)

## obs-plan §Observability Harness Contract
- observed absent — a tracing subscriber or exporter set up in these crates · searched: `subscriber` over the 21 listed s11 files

## obs-plan §Span / Trace Coverage
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 21 listed s11 files

## obs-plan §Metric Coverage
- observed absent — metrics or telemetry backends · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files
- Frame and phase timing are exposed only as log features (packages/dioxus-native/Cargo.toml:58-66)

## obs-plan §Log Coverage
- Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:119-205; packages/dioxus-native-dom/src/mutation_writer.rs:305; packages/dioxus-native-dom/src/mutation_writer.rs:388)
- Asset fetch success is logged at trace and failure at warn; fetches without a net provider are logged at warn (packages/dioxus-native/src/assets.rs:47-60)
- A failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:14-15)
- Injecting the document provider into windows is logged at debug (packages/dioxus-native/src/dioxus_application.rs:137-138)

## obs-plan §Error Capture & Reporting
- Errors are reported only as `tracing` warn/error events (packages/dioxus-native/src/assets.rs:52-53; packages/dioxus-native/src/link_handler.rs:14-15)
- observed absent — an error-reporting service · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files

## obs-plan §PII Scrubbing & Compliance
- observed absent — scrubbing or redaction · searched: `sanitiz|scrub|redact|pii` over the 21 listed s11 files
- Debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:150; packages/dioxus-native-dom/src/mutation_writer.rs:202; packages/dioxus-native-dom/src/mutation_writer.rs:388; packages/dioxus-native/src/assets.rs:48)

## obs-plan §CI Integration
- out of slice — no CI configuration in these files

## a11y-plan §A11y Scope Summary
- `accessibility` is a default feature of both Dioxus crates and forwards to blitz-dom (and blitz-shell in dioxus-native) (packages/dioxus-native-dom/Cargo.toml:13; packages/dioxus-native-dom/Cargo.toml:18; packages/dioxus-native/Cargo.toml:13; packages/dioxus-native/Cargo.toml:30)

## a11y-plan §A11y Strategy
- The crate docs state the `accessibility` feature enables accesskit support; these crates only forward the feature to blitz (packages/dioxus-native-dom/src/lib.rs:7; packages/dioxus-native/src/lib.rs:7; packages/dioxus-native/Cargo.toml:30)

## a11y-plan §A11y Assertion Harness Contract
- observed absent — accessibility assertions in tests · searched: `aria|role|accesskit` over the 21 listed s11 files (matches only in crate docs and unrelated comments)

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles · searched: `aria|role|accesskit` over the 21 listed s11 files

## a11y-plan §Keyboard Navigation
- KeyDown, KeyUp and KeyPress events reach Dioxus as keyboard data with key, code, location, repeat, composing state and modifiers (packages/dioxus-native-dom/src/dioxus_document.rs:320-324; packages/dioxus-native-dom/src/events.rs:328-361)
- Focus, Blur, FocusIn and FocusOut events are forwarded as focus data (packages/dioxus-native-dom/src/dioxus_document.rs:315-318)
- Mounted elements can take or drop focus with `set_focus`; focus/blur events are not queued (TODO) (packages/dioxus-native-dom/src/events.rs:283-295)
- An `autofocus` feature forwards to blitz-dom (packages/dioxus-native-dom/Cargo.toml:23; packages/dioxus-native/Cargo.toml:19)
- IME events are not handled (TODO) (packages/dioxus-native-dom/src/dioxus_document.rs:331-332)
- `use_back_button` handles `NamedKey::BrowserBack` presses, ignoring key repeats (packages/dioxus-native/src/hooks.rs:30-49)

## a11y-plan §Visual Design Verification
- observed absent — contrast or user-preference media handling · searched: `contrast|prefers-` over the 21 listed s11 files

## a11y-plan §Screen Reader Support
- Screen-reader exposure is stated only as accesskit support behind the `accessibility` feature (packages/dioxus-native-dom/src/lib.rs:7; packages/dioxus-native/src/lib.rs:7)

## a11y-plan §Cognitive Accessibility
- out of slice — nothing in these files addresses cognitive accessibility

## a11y-plan §CI Integration
- out of slice — no CI configuration in these files
