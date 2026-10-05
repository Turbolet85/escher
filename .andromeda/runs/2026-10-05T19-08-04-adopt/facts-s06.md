# facts-s06 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 17 files

## architecture §Design Philosophy
- The scrolling module documents one scrolling primitive: user-initiated and programmatic scrolls differ only in the `ScrollRequest` they build (packages/blitz-dom/src/scrolling.rs:126-130)
- Stylo `Device` changes such as resize, zoom, color-scheme and media type are coalesced so the stylist device is rebuilt at most once per resolve (packages/blitz-dom/src/stylo_device.rs:1-5; packages/blitz-dom/src/stylo_device.rs:22-42)
- Text decoration styles are deliberately kept out of Parley so they can change without rebuilding the Parley layout; the `NodeId` travels in the `brush` field and decorations are read lazily at render time (packages/blitz-dom/src/stylo_to_parley.rs:506-521)
- Node storage is a versioned slotmap so that a dropped node's id stops resolving instead of aliasing a reused slot (packages/blitz-dom/src/tree.rs:27-35)
- The slotmap key is used only at the storage boundary; all public APIs use `NodeId` (packages/blitz-dom/src/tree.rs:11-15)
- Behaviour is written against cited web specifications: CSS Transforms current transformation matrix (packages/blitz-dom/src/stylo_to_kurbo.rs:11-24), HTML dimension-value parsing (packages/blitz-dom/src/stylo.rs:917-923), Pointer Events `touch-action` intersection (packages/blitz-dom/src/events/pointer.rs:158-166), HTML implicit form submission (packages/blitz-dom/src/events/keyboard.rs:138), CSS Fonts feature precedence (packages/blitz-dom/src/stylo_to_parley.rs:264-273)

## architecture §Stack and Technologies
- The slice's source is Rust and integrates Servo's Stylo `style` crate for styling (packages/blitz-dom/src/stylo.rs:1-2; packages/blitz-dom/src/stylo.rs:21-57)
- The `selectors` crate supplies selector matching and the bloom filter (packages/blitz-dom/src/stylo.rs:14-20)
- `markup5ever` supplies local names and namespaces (packages/blitz-dom/src/stylo.rs:13)
- `style_dom::ElementState` carries element state flags (packages/blitz-dom/src/stylo.rs:58)
- Parley is the text layout library; Stylo types are converted to Parley types (packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-dom/src/stylo_to_parley.rs:40-48)
- Parley's `FontContext` backs the Stylo font metrics provider (packages/blitz-dom/src/stylo_device.rs:11; packages/blitz-dom/src/stylo_device.rs:78)
- `kurbo::Affine` represents resolved 2D transforms, with `euclid` rects as input (packages/blitz-dom/src/stylo_to_kurbo.rs:1-2; packages/blitz-dom/src/stylo_to_kurbo.rs:25-28)
- `cursor_icon::CursorIcon` is the cursor output type (packages/blitz-dom/src/stylo_to_cursor_icon.rs:1-4)
- The `url` crate holds document URLs (packages/blitz-dom/src/url.rs:6)
- `slotmap::SlotMap` stores DOM nodes (packages/blitz-dom/src/tree.rs:7; packages/blitz-dom/src/tree.rs:34-35)
- `web_time` supplies `SystemTime`, `Instant` and `Duration` (packages/blitz-dom/src/scrolling.rs:7; packages/blitz-dom/src/events/pointer.rs:4)
- `taffy` types are used for points and axes (packages/blitz-dom/src/events/pointer.rs:17; packages/blitz-dom/src/events/pointer.rs:341-344)
- `keyboard_types` supplies `Key` and `Modifiers` (packages/blitz-dom/src/util.rs:3; packages/blitz-dom/src/events/keyboard.rs:8)
- The `color` crate's `AlphaColor<Srgb>` is the `Color` type (packages/blitz-dom/src/util.rs:2; packages/blitz-dom/src/util.rs:12)
- `bitflags` defines `DeviceChanges` (packages/blitz-dom/src/stylo_device.rs:9; packages/blitz-dom/src/stylo_device.rs:22-42)
- `usvg` with a `fontdb` database parses SVG images under the `svg` feature (packages/blitz-dom/src/util.rs:42-51; packages/blitz-dom/src/util.rs:156-164)
- `wuff` decompresses WOFF and WOFF2 fonts under the `woff` feature (packages/blitz-dom/src/util.rs:21-37)
- `percent_encoding` decodes URL fragments (packages/blitz-dom/src/scrolling.rs:663-666)
- `blitz_traits` supplies events, node ids, shell and navigation types (packages/blitz-dom/src/scrolling.rs:4-5; packages/blitz-dom/src/stylo_device.rs:10; packages/blitz-dom/src/events/pointer.rs:6-12)
- Parallel style traversal uses Stylo's global `STYLE_THREAD_POOL` rayon pool (packages/blitz-dom/src/stylo.rs:28; packages/blitz-dom/src/stylo.rs:149-153)

## architecture §Established Decisions
- `StyleThreading::Sequential` bypasses Stylo's global thread pool; only `Parallel` uses it (packages/blitz-dom/src/stylo.rs:149-153)
- Documents are always treated as HTML documents in no-quirks mode (packages/blitz-dom/src/stylo.rs:245-251; packages/blitz-dom/src/stylo_device.rs:74)
- Visited-link styling is disabled in the style context (packages/blitz-dom/src/stylo.rs:133; packages/blitz-dom/src/stylo.rs:213)
- Shadow DOM is not implemented: shadow-root methods are `todo!` and `as_shadow_root` returns `None` (packages/blitz-dom/src/stylo.rs:275-284; packages/blitz-dom/src/stylo.rs:356-359; packages/blitz-dom/src/stylo.rs:629-638)
- Container queries are disabled by returning a default size (packages/blitz-dom/src/stylo.rs:1210-1216)
- 3D transforms are not supported: non-z-axis `rotate` and non-2D `transform` matrices are dropped (packages/blitz-dom/src/stylo_to_kurbo.rs:40-46; packages/blitz-dom/src/stylo_to_kurbo.rs:64-73)
- The CSS `offset` property is not supported in transform resolution (packages/blitz-dom/src/stylo_to_kurbo.rs:81-82)
- The root element's scroll applies to the viewport, per CSS overflow propagation (packages/blitz-dom/src/scrolling.rs:28-32; packages/blitz-dom/src/scrolling.rs:272-283)
- The viewport's scroll offset lives on `NodeTree`, not on the root node (packages/blitz-dom/src/tree.rs:41-43)
- `overflow: hidden` axes are scrollable for programmatic scrolls but not for user scrolls (packages/blitz-dom/src/scrolling.rs:58-67; packages/blitz-dom/src/scrolling.rs:481-485)
- User scrolls chain unconsumed delta to the parent and finally the viewport; programmatic scrolls clamp to one scroller (packages/blitz-dom/src/scrolling.rs:177-193; packages/blitz-dom/src/scrolling.rs:551-586)
- A user-initiated scroll aborts any smooth scroll in progress (packages/blitz-dom/src/scrolling.rs:425-427; packages/blitz-dom/src/scrolling.rs:136-140)
- Smooth scrolls last 300 ms with a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:113-123; packages/blitz-dom/src/scrolling.rs:433-434)
- Fling deceleration is 0.95 per 60 fps frame normalised to frame time and stops below 0.1 velocity on both axes (packages/blitz-dom/src/scrolling.rs:732-746)
- Fling velocity is computed from pan samples of the last 100 ms on the dominant axis and doubled (packages/blitz-dom/src/events/pointer.rs:110-151)
- Wheel deltas in lines are multiplied by 20 to get pixels (packages/blitz-dom/src/events/pointer.rs:835-838)
- A drag counts as a selection or pan after moving more than 2 px from mousedown (packages/blitz-dom/src/events/pointer.rs:218-222)
- Repeated clicks count when within 500 ms and 2 px of the previous mousedown (packages/blitz-dom/src/events/pointer.rs:395-405)
- The platform action modifier is `SUPER` on macOS and `CONTROL` elsewhere (packages/blitz-dom/src/util.rs:7-10)
- Links whose URL differs from the document only by fragment scroll in-page instead of navigating (packages/blitz-dom/src/events/pointer.rs:737-750; packages/blitz-dom/src/url.rs:23-31)
- An empty fragment or an unmatched `top` fragment scrolls to the top of the document (packages/blitz-dom/src/scrolling.rs:674-678)
- The default `DocumentUrl` is an empty base64 `data:text/css` URL (packages/blitz-dom/src/url.rs:34-37)
- `font-feature-settings` is appended after `font-variant-*` features so it takes precedence, then features are sorted and deduplicated by tag (packages/blitz-dom/src/stylo_to_parley.rs:274-289)
- The generic font family `None` maps to sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-53)
- `-apple-system` and `BlinkMacSystemFont` map to the system-ui generic on Apple targets (packages/blitz-dom/src/stylo_to_parley.rs:68-76; packages/blitz-dom/src/stylo_to_parley.rs:444-456)
- `line-height: normal` is approximated as 1.2 times font size when resolving percentage baseline shifts (packages/blitz-dom/src/stylo_to_parley.rs:382-386)
- `baseline-shift: center` is approximated with middle alignment (packages/blitz-dom/src/stylo_to_parley.rs:373-377)
- Text selection endpoints for anonymous blocks store the parent id plus sibling index because anonymous block ids change across layout reconstruction (packages/blitz-dom/src/selection.rs:1-5; packages/blitz-dom/src/selection.rs:9-24)
- Animations of nodes no longer in the document are cleared during style resolution so an infinite animation cannot force a redraw every frame (packages/blitz-dom/src/stylo.rs:87-101)
- Pseudo-element style changes that need no reconstruction are flushed to the pseudo node during the style traversal (packages/blitz-dom/src/stylo.rs:1380-1400)
- Several non-tree-structural pseudo-classes always match false, including `:focus-visible`, `:focus-within`, `:valid`, `:invalid`, `:required` and `:target` (packages/blitz-dom/src/stylo.rs:460-506)

## architecture §Conventions
- Modules open with `//!` doc comments describing their purpose (packages/blitz-dom/src/scrolling.rs:1-2; packages/blitz-dom/src/selection.rs:1-5; packages/blitz-dom/src/stylo_device.rs:1-5; packages/blitz-dom/src/tree.rs:1; packages/blitz-dom/src/stylo_to_parley.rs:1)
- Optional functionality is gated by cargo features named `tracing`, `woff`, `svg`, `custom-widget` and `file-input` (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9; packages/blitz-dom/src/util.rs:22; packages/blitz-dom/src/util.rs:42; packages/blitz-dom/src/events/mod.rs:146; packages/blitz-dom/src/events/pointer.rs:768)
- Crate-internal items are declared with crate-restricted `pub` visibility (packages/blitz-dom/src/scrolling.rs:34; packages/blitz-dom/src/stylo_device.rs:29; packages/blitz-dom/src/url.rs:9)
- Event-producing functions take a `dispatch_event` callback over `DomEvent` instead of returning events (packages/blitz-dom/src/scrolling.rs:131-135; packages/blitz-dom/src/events/keyboard.rs:16-21; packages/blitz-dom/src/events/focus.rs:5-9)
- Unsafe blocks carry an `allow unsafe_code` attribute or `SAFETY` comments stating the exclusivity argument (packages/blitz-dom/src/stylo.rs:749-756; packages/blitz-dom/src/stylo.rs:1324; packages/blitz-dom/src/stylo.rs:1396-1401)
- Children are iterated by taking the child list out of the node and restoring it afterwards, via macros and helpers (packages/blitz-dom/src/traversal.rs:8-44; packages/blitz-dom/src/traversal.rs:140-150)
- Known gaps are marked with `TODO` and `FIXME` comments (packages/blitz-dom/src/scrolling.rs:213; packages/blitz-dom/src/scrolling.rs:574; packages/blitz-dom/src/stylo.rs:1214; packages/blitz-dom/src/events/pointer.rs:101)
- Spec URLs are cited inline in comments next to the behaviour they govern (packages/blitz-dom/src/stylo.rs:989; packages/blitz-dom/src/stylo.rs:1019-1022; packages/blitz-dom/src/stylo.rs:1101)
- Unit tests sit inline in `test`-gated `mod tests` blocks at the end of the module (packages/blitz-dom/src/stylo_to_parley.rs:525-526; packages/blitz-dom/src/util.rs:180-181)
- An exported `qual_name!` macro builds qualified names (packages/blitz-dom/src/util.rs:245-256)
- Stylo types are aliased under a `stylo` module and Parley types under a `parley` module for readable conversions (packages/blitz-dom/src/stylo_to_parley.rs:9-48)

## architecture §Standard Contracts
- `EventHandler` trait: `handle_event` receives the node chain, the mutable `DomEvent`, the document and an `EventState` (packages/blitz-dom/src/events/driver.rs:9-17)
- `NoopEventHandler` implements `EventHandler` and does nothing (packages/blitz-dom/src/events/driver.rs:19-30)
- `EventDriver::new` takes a document and a handler; `handle_ui_event` and `handle_dom_event` are its public entry points (packages/blitz-dom/src/events/driver.rs:38-45; packages/blitz-dom/src/events/driver.rs:148; packages/blitz-dom/src/events/driver.rs:271-274)
- The events module re-exports `EventDriver`, `EventHandler` and `NoopEventHandler` (packages/blitz-dom/src/events/mod.rs:9)
- Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen input; the default action runs on the pointer event when not cancelled (packages/blitz-dom/src/events/driver.rs:288-310)
- Handlers see the bubbling node chain only when the event bubbles, else the target alone (packages/blitz-dom/src/events/driver.rs:339-344)
- Focus change dispatches `blur` then `focusout` on the old node and `focus` then `focusin` on the new node (packages/blitz-dom/src/events/focus.rs:15-39)
- UI events target the hovered node for pointer and wheel input and the focused node for keyboard and IME input, falling back to the root element (packages/blitz-dom/src/events/driver.rs:188-204)
- Public scroll API on `BaseDocument`: `scroll_to`, `scroll_by`, `scroll_into_view`, `scroll_to_fragment`, `scroll_to_fragment_smooth`, `resolve_scroll_animation`, `scroll_node_by` and `scroll_viewport_by` (packages/blitz-dom/src/scrolling.rs:354-406; packages/blitz-dom/src/scrolling.rs:555-562; packages/blitz-dom/src/scrolling.rs:620-626; packages/blitz-dom/src/scrolling.rs:712-722)
- `ScrollBehavior` is `Auto`, `Instant` or `Smooth`; `ScrollLogicalPosition` is `Start`, `Center`, `End` or `Nearest` (packages/blitz-dom/src/scrolling.rs:12-26)
- Scroll offsets written to a node or the viewport dispatch a `scroll` event with scroll and client dimensions (packages/blitz-dom/src/scrolling.rs:285-352)
- `TextSelection` and `SelectionEndpoint` are public types with anchor and focus endpoints (packages/blitz-dom/src/selection.rs:14-24; packages/blitz-dom/src/selection.rs:80-86)
- `NodeTree` exposes `get`, `get_mut`, `contains_key`, `len`, `iter` and `iter_mut`; indexing a stale id panics (packages/blitz-dom/src/tree.rs:75-117; packages/blitz-dom/src/tree.rs:120-136)
- `TreeTraverser` is a pre-order traverser and `AncestorTraverser` walks parents (packages/blitz-dom/src/traversal.rs:46-100)
- `compare_document_order` returns `Ordering` of two nodes in document order (packages/blitz-dom/src/traversal.rs:307-362)
- `resolve_undisplayed_style` computes styles inside `display: none` subtrees on demand for `getComputedStyle` without storing them (packages/blitz-dom/src/stylo.rs:187-229)
- `resolve_stylist` takes the current animation time and runs the style traversal (packages/blitz-dom/src/stylo.rs:63)
- `resolve_2d_transform` returns an `Affine` or `None` when the result is identity (packages/blitz-dom/src/stylo_to_kurbo.rs:25-28; packages/blitz-dom/src/stylo_to_kurbo.rs:124-128)
- `decode_font_bytes` returns decompressed bytes for WOFF and WOFF2 and the input unchanged otherwise (packages/blitz-dom/src/util.rs:14-40)
- `ToColorColor` converts a Stylo `AbsoluteColor` to sRGB `Color` (packages/blitz-dom/src/util.rs:166-178)

## architecture §Occupied Resources
- A process-wide static `FONT_DB` loads system fonts once under the `svg` feature (packages/blitz-dom/src/util.rs:46-51)
- Parallel styling uses Stylo's global style thread pool (packages/blitz-dom/src/stylo.rs:149-152)
- The system clipboard is written through the shell provider on copy (packages/blitz-dom/src/events/keyboard.rs:57-58)
- A native file dialog is opened through the shell provider for file inputs (packages/blitz-dom/src/events/pointer.rs:768-773)
- observed absent — network listeners or ports · searched: `TcpListener|localhost|0\.0\.0\.0|\.bind` over the 17 listed s06 files

## architecture §Infrastructure Patterns
- out of slice — the s06 files hold no build, deploy, container or workspace configuration

## architecture §Cross-cutting Patterns
- A `shell_provider` handles redraw requests, clipboard and file dialogs (packages/blitz-dom/src/scrolling.rs:321; packages/blitz-dom/src/events/keyboard.rs:58; packages/blitz-dom/src/events/pointer.rs:773)
- A `navigation_provider` receives full navigations from link clicks (packages/blitz-dom/src/events/pointer.rs:745-749)
- Events generated by default actions are queued and processed in order, each running handlers then its default action unless cancelled (packages/blitz-dom/src/events/driver.rs:313-320; packages/blitz-dom/src/events/driver.rs:384-387)
- Events targeting a sub-document or custom widget are mapped to UI events and forwarded, with coordinates adjusted (packages/blitz-dom/src/events/mod.rs:18-27; packages/blitz-dom/src/events/mod.rs:118-173)
- Geometry-derived memoised values are invalidated by bumping a geometry generation counter on layout and scroll writes (packages/blitz-dom/src/tree.rs:36-40; packages/blitz-dom/src/tree.rs:59-73; packages/blitz-dom/src/scrolling.rs:336)
- State changes that affect selectors are wrapped in element snapshots for restyle invalidation (packages/blitz-dom/src/events/pointer.rs:647-652)
- Elements with non-empty restyle damage mark their ancestor chain for the damage propagation pass (packages/blitz-dom/src/stylo.rs:1339-1343)
- Legacy HTML presentational attributes `align`, `width`, `height`, `hspace`, `vspace`, `border`, body margin attributes, `bgcolor`, `hidden` and `lang` are mapped to CSS declarations at the presentational-hints cascade level (packages/blitz-dom/src/stylo.rs:870-1200)
- Focused text inputs are re-clamped to keep the caret visible after events that may move it (packages/blitz-dom/src/events/mod.rs:105-116; packages/blitz-dom/src/events/mod.rs:301-308)

## architecture §Project Intent
- out of slice — the s06 files state module purposes but no project-level intent

## architecture §Existing Scopes
- The `events` module is split into `driver`, `focus`, `ime`, `keyboard` and `pointer` submodules (packages/blitz-dom/src/events/mod.rs:1-5)
- Scrolling covers user and programmatic scrolling of nodes and the viewport plus scroll animations (packages/blitz-dom/src/scrolling.rs:1-2)
- Text selection state for non-input elements (packages/blitz-dom/src/selection.rs:1-5)
- Stylo integration lets the DOM participate in Servo styling (packages/blitz-dom/src/stylo.rs:1)
- Stylo `Device` management (packages/blitz-dom/src/stylo_device.rs:1-5)
- Stylo-to-Parley, Stylo-to-kurbo and Stylo-to-cursor conversions (packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-dom/src/stylo_to_kurbo.rs:25; packages/blitz-dom/src/stylo_to_cursor_icon.rs:4)
- Versioned node storage (packages/blitz-dom/src/tree.rs:1)
- Tree traversal and inline-root collection for selections (packages/blitz-dom/src/traversal.rs:46-48; packages/blitz-dom/src/traversal.rs:376-383)
- Document URL handling (packages/blitz-dom/src/url.rs:8-32)

## security-plan §Threat Model Summary
- out of slice — the s06 files state no threat model

## security-plan §Authentication & Authorization
- observed absent — authentication or authorization logic · searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files

## security-plan §Input Validation
- Legacy `bgcolor` values are accepted only as `#` followed by 3 or 6 hex digits; anything else is ignored (packages/blitz-dom/src/stylo.rs:894-915; packages/blitz-dom/src/stylo.rs:1182-1189)
- Legacy dimension attributes go through Stylo's HTML dimension-value parser, with a nonzero variant for table width and cells (packages/blitz-dom/src/stylo.rs:917-946; packages/blitz-dom/src/stylo.rs:1056-1062)
- SVG `width` and `height` attributes reject negative and unparseable values (packages/blitz-dom/src/stylo.rs:948-978)
- `border` and body margin attributes are parsed as unsigned integers (packages/blitz-dom/src/stylo.rs:1129; packages/blitz-dom/src/stylo.rs:1163)
- Link `href` values are resolved against the document URL and an unparseable href is not navigated (packages/blitz-dom/src/events/pointer.rs:736-754; packages/blitz-dom/src/url.rs:19-21)
- `DocumentUrl` parsing returns the URL parse error instead of panicking (packages/blitz-dom/src/url.rs:39-45)
- URL fragments are percent-decoded lossily before matching (packages/blitz-dom/src/scrolling.rs:663-666)
- Font bytes shorter than 4 bytes pass through, and failed WOFF decompression falls back to the original bytes (packages/blitz-dom/src/util.rs:17-39)
- Scroll targets are clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:165-168)
- Programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:570-572)
- Elements with a `disabled` attribute ignore pointer selection and click default actions (packages/blitz-dom/src/events/pointer.rs:330-333; packages/blitz-dom/src/events/pointer.rs:457; packages/blitz-dom/src/events/pointer.rs:635-638)
- File inputs do not apply the `accept` attribute as a filter (packages/blitz-dom/src/events/pointer.rs:771-773)

## security-plan §Data Protection
- Copy writes the selected document text to the clipboard only when no text input is focused (packages/blitz-dom/src/events/keyboard.rs:43-61)
- A chosen file's full path is stored in the file input's `value` attribute (packages/blitz-dom/src/events/pointer.rs:775-778)

## security-plan §API Security
- out of slice — the s06 files expose no network API

## security-plan §Dependency Security
- out of slice — no dependency manifest is in the s06 file list

## security-plan §Secret Management
- observed absent — environment or secret reads · searched: `env::var|std::env|api_key` over the 17 listed s06 files

## security-plan §Error Handling
- Time reads `unwrap` the duration since the Unix epoch (packages/blitz-dom/src/scrolling.rs:515-518; packages/blitz-dom/src/events/pointer.rs:588-591)
- Invariant violations use `expect` with messages (packages/blitz-dom/src/events/keyboard.rs:111-116; packages/blitz-dom/src/stylo.rs:434)
- Finding a non-anonymous ancestor panics when none exists (packages/blitz-dom/src/traversal.rs:130-134)
- Unimplemented Stylo hooks use `todo!`, `unimplemented!` or `panic!` (packages/blitz-dom/src/stylo.rs:741-747; packages/blitz-dom/src/stylo.rs:1218-1223; packages/blitz-dom/src/stylo.rs:1366-1372)
- `stylo_to_cursor_icon` returns a default cursor for `Auto` instead of panicking (packages/blitz-dom/src/stylo_to_cursor_icon.rs:6-12)
- Stale node ids resolve to `None` through `get` (packages/blitz-dom/src/tree.rs:89-97)
- `handle_ui_event` returns early when a document has no event target (packages/blitz-dom/src/events/driver.rs:199-204)
- The clipboard write result is discarded (packages/blitz-dom/src/events/keyboard.rs:58)
- A `debug_assert` checks that compared nodes share the root (packages/blitz-dom/src/traversal.rs:340-343)

## security-plan §Logging & Monitoring
- Logging uses `tracing` macros compiled only with the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27)
- `walk_tree` prints a debug dump of the DOM to stdout (packages/blitz-dom/src/util.rs:90-154)
- A devtools hover-highlight mode logs the clicked node instead of handling the click (packages/blitz-dom/src/events/pointer.rs:561-571)

## design-system §Color Palette
- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
- The viewport's `ColorScheme` Light or Dark feeds `prefers-color-scheme` (packages/blitz-dom/src/stylo_device.rs:80-83)
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1182-1189)

## design-system §Typography
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412)
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1197-1199)
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329)
- Word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)

## design-system §Spacing
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1073-1099)
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1143-1180)

## design-system §Depth Strategy
- observed absent — z-index or shadow handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files

## design-system §Border Radius
- observed absent — border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1120-1141)

## design-system §Motion
- Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:113-123; packages/blitz-dom/src/scrolling.rs:433-434)
- `scroll-behavior: smooth` in computed style makes `Auto` scrolls animate (packages/blitz-dom/src/scrolling.rs:532-549)
- Touch flings decelerate per frame until velocity drops below 0.1 (packages/blitz-dom/src/scrolling.rs:724-747)
- CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:105-123)
- observed absent — reduced-motion preference handling · searched: `prefers-reduced-motion|reduced_motion|prefers_reduced` over the 17 listed s06 files

## design-system §Iconography
- CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49)

## design-system §Surface: none observed
- observed absent — an application entry point or window creation · searched: `fn main|winit|EventLoop` over the 17 listed s06 files

## layout-templates §Surface: none observed
- observed absent — an application entry point or window creation · searched: `fn main|winit|EventLoop` over the 17 listed s06 files

## test-plan §Test Scope Summary
- `stylo_to_parley.rs` holds nine unit tests of font feature mapping (packages/blitz-dom/src/stylo_to_parley.rs:525-655)
- `util.rs` holds seven SVG parsing tests compiled only with the `svg` feature (packages/blitz-dom/src/util.rs:180-243)
- `stylo.rs` holds two `#[test]` functions whose bodies are entirely commented out (packages/blitz-dom/src/stylo.rs:1492-1518)

## test-plan §Test Strategy
- Tests are inline Rust unit tests next to the code they cover (packages/blitz-dom/src/stylo_to_parley.rs:525-528; packages/blitz-dom/src/util.rs:180-182)

## test-plan §Test Harness Contract
- Tests use the built-in Rust `#[test]` attribute with `assert!` and `assert_eq!` (packages/blitz-dom/src/stylo_to_parley.rs:550-564; packages/blitz-dom/src/util.rs:191-200)
- SVG tests are gated on both `test` and the `svg` feature (packages/blitz-dom/src/util.rs:180)

## test-plan §Unit Test Strategy
- Feature-mapping tests assert exact OpenType tag and value pairs per `font-variant-*` input (packages/blitz-dom/src/stylo_to_parley.rs:550-635)
- A test asserts `font-feature-settings` entries come after variant-derived ones so they win (packages/blitz-dom/src/stylo_to_parley.rs:637-654)
- SVG tests assert intrinsic width, height and aspect ratio for viewBox, absolute, percentage, unit and non-numeric dimensions (packages/blitz-dom/src/util.rs:184-242)

## test-plan §Integration Test Strategy
- out of slice — no integration tests are in the s06 files

## test-plan §E2E Test Strategy
- out of slice — no end-to-end tests are in the s06 files

## test-plan §Test Data & Fixtures
- SVG test inputs are inline byte-string literals (packages/blitz-dom/src/util.rs:186; packages/blitz-dom/src/util.rs:193)
- Test helpers `pairs` and `feature_settings` build and flatten feature lists (packages/blitz-dom/src/stylo_to_parley.rs:530-548)

## test-plan §Mocking & Stubbing Discipline
- observed absent — mocks or fixtures · searched: `mock|fixture` over the 17 listed s06 files

## test-plan §CI Integration
- out of slice — no CI configuration is in the s06 files

## obs-plan §Obs Scope Summary
- Six `tracing` log call sites exist in the slice, all behind the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/util.rs:34-35)

## obs-plan §Telemetry Strategy
- Telemetry is the `tracing` crate's event macros, compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28)

## obs-plan §Observability Harness Contract
- observed absent — a tracing subscriber or exporter setup · searched: `tracing_subscriber|subscriber` over the 17 listed s06 files

## obs-plan §Span / Trace Coverage
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 17 listed s06 files

## obs-plan §Metric Coverage
- observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files

## obs-plan §Log Coverage
- `debug` logs a sent IME event with a `node_id` field (packages/blitz-dom/src/events/ime.rs:28)
- `warn` logs an unparseable link href together with the document URL (packages/blitz-dom/src/events/pointer.rs:753)
- `info` logs a click on a link without href together with the element's attributes (packages/blitz-dom/src/events/pointer.rs:758)
- `warn` logs WOFF1 and WOFF2 decompression failures (packages/blitz-dom/src/util.rs:27; packages/blitz-dom/src/util.rs:35)
- `warn` logs a call to `stylo_to_cursor_icon` with `CursorKind::Auto` (packages/blitz-dom/src/stylo_to_cursor_icon.rs:10)

## obs-plan §Error Capture & Reporting
- Recoverable failures are logged and a fallback is used, as with font decompression (packages/blitz-dom/src/util.rs:25-29)
- observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files

## obs-plan §PII Scrubbing & Compliance
- Log lines include the raw href, the document URL and element attributes unfiltered (packages/blitz-dom/src/events/pointer.rs:753; packages/blitz-dom/src/events/pointer.rs:758)
- observed absent — redaction or scrubbing of logged values · searched: `redact|scrub` over the 17 listed s06 files

## obs-plan §CI Integration
- out of slice — no CI configuration is in the s06 files

## a11y-plan §A11y Scope Summary
- observed absent — accessibility tree, ARIA or screen-reader code · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files

## a11y-plan §A11y Strategy
- out of slice — the s06 files state no accessibility strategy

## a11y-plan §A11y Assertion Harness Contract
- out of slice — no accessibility assertions are in the s06 files

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files

## a11y-plan §Keyboard Navigation
- Tab moves focus to the next node and Shift+Tab to the previous, dispatching focus events (packages/blitz-dom/src/events/keyboard.rs:22-41)
- The action modifier plus C copies selected text when no text input is focused (packages/blitz-dom/src/events/keyboard.rs:43-61)
- A text input's generated submit triggers implicit form submission unless the form has more than one blocking field type (packages/blitz-dom/src/events/keyboard.rs:130-133; packages/blitz-dom/src/events/keyboard.rs:138-174)
- Clicking a label runs the default click of its bound input (packages/blitz-dom/src/events/pointer.rs:723-734)
- Activating the first `summary` of a `details` toggles it open and focuses the summary (packages/blitz-dom/src/events/pointer.rs:696-722)
- Checkbox and radio clicks toggle state, dispatch `input` and move focus to the control (packages/blitz-dom/src/events/pointer.rs:645-695)
- Clicking a non-interactive area clears focus (packages/blitz-dom/src/events/pointer.rs:814-817)
- `:focus` matches the focus element state while `:focus-visible` and `:focus-within` never match (packages/blitz-dom/src/stylo.rs:476-478)
- observed absent — tabindex handling · searched: `tabindex` over the 17 listed s06 files

## a11y-plan §Visual Design Verification
- Color-scheme changes on the viewport are tracked as a device change that rebuilds the stylist device (packages/blitz-dom/src/stylo_device.rs:37-38; packages/blitz-dom/src/stylo_device.rs:55-57)
- Total scale changes from hidpi or zoom are tracked and invalidate text shaping (packages/blitz-dom/src/stylo_device.rs:33-36; packages/blitz-dom/src/stylo_device.rs:52-54)
- Touch panning honours the `touch-action` property per axis (packages/blitz-dom/src/events/pointer.rs:158-204; packages/blitz-dom/src/events/pointer.rs:247-260)

## a11y-plan §Screen Reader Support
- observed absent — screen-reader or accessibility API integration · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files

## a11y-plan §Cognitive Accessibility
- out of slice — the s06 files show no cognitive accessibility handling

## a11y-plan §CI Integration
- out of slice — no CI configuration is in the s06 files
