# facts-s07 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 8 files

## architecture §Design Philosophy
- The layout module's doc comment states that Blitz runs a style pass and then a separate layout pass (unlike Servo, which interleaves them), and accepts that this is slower because it is fast enough (packages/blitz-dom/src/layout/mod.rs:1-5)
- Inline (Parley) layout construction is deferred to a dedicated phase so that the expensive text-shaping step can be multithreaded; construction only queues a `ConstructionTask` and collects the embedded inline boxes (packages/blitz-dom/src/layout/construct.rs:619-624; packages/blitz-dom/src/layout/construct.rs:929-931)
- The paint-tree pass builds topology only (which node ids each node paints, which z-indexed descendants are hoisted); geometry is derived at use time from `final_layout()` and memoised per geometry generation so a cached list cannot paint a box where layout no longer puts it (packages/blitz-dom/src/layout/paint_tree.rs:1-9)
- Layout invalidation is incremental: style changes are classified into damage bits and only nodes whose subtree needs relayout have their layout caches cleared (packages/blitz-dom/src/layout/damage.rs:45-53; packages/blitz-dom/src/layout/damage.rs:118-125)
- Code comments cite CSS specifications for behaviour (css-images default object size, css-tables-3 abspos, CSS 2.1 Appendix E paint order, CSS2 §10.3.2, CSS Sizing 4) (packages/blitz-dom/src/layout/mod.rs:37-38; packages/blitz-dom/src/layout/mod.rs:371-374; packages/blitz-dom/src/layout/paint_tree.rs:293-297; packages/blitz-dom/src/layout/replaced.rs:149-158; packages/blitz-dom/src/layout/replaced.rs:132-141)

## architecture §Stack and Technologies
- Language is Rust (all eight slice files are `.rs` modules declared in the layout module) (packages/blitz-dom/src/layout/mod.rs:24-30)
- Box layout uses the `taffy` crate: block, flexbox, grid, leaf, cached and out-of-flow layout functions are imported from it (packages/blitz-dom/src/layout/mod.rs:16-22)
- Computed styles come from the Stylo `style` crate, bridged to Taffy through `stylo_taffy::TaffyStyloStyle` (packages/blitz-dom/src/layout/mod.rs:12-15)
- Text shaping and inline layout use the `parley` crate (`FontContext`, `LayoutContext`, `TreeBuilder`, `InlineBox`) (packages/blitz-dom/src/layout/construct.rs:6-9; packages/blitz-dom/src/layout/inline.rs:2)
- DOM names use `markup5ever` (`LocalName`, `local_name!`, `QualName`) (packages/blitz-dom/src/layout/mod.rs:9; packages/blitz-dom/src/layout/construct.rs:5)
- Other crates imported: `thin_vec` (packages/blitz-dom/src/layout/construct.rs:19), `atomic_refcell` (packages/blitz-dom/src/layout/table.rs:4), `style_traits` (packages/blitz-dom/src/layout/table.rs:20), `app_units` (packages/blitz-dom/src/layout/table.rs:137), `blitz_traits` for `NodeId` (packages/blitz-dom/src/layout/damage.rs:1)
- Cargo features referenced by the slice: `tracing` (packages/blitz-dom/src/layout/mod.rs:131), `svg` (packages/blitz-dom/src/layout/mod.rs:290), `custom-widget` (packages/blitz-dom/src/layout/mod.rs:321), `floats` (packages/blitz-dom/src/layout/inline.rs:16-19)

## architecture §Established Decisions
- HTML tables are laid out by Taffy's grid algorithm through a `TableTreeWrapper` whose children are the table's cells (packages/blitz-dom/src/layout/mod.rs:399-403; packages/blitz-dom/src/layout/table.rs:739-758)
- Table grids use `GridAutoFlow::RowDense` so cells backfill columns freed by earlier rowspans (packages/blitz-dom/src/layout/table.rs:197-202)
- The fixed table layout algorithm applies only when `table-layout: fixed` and the table width is non-auto (packages/blitz-dom/src/layout/table.rs:206-210); percentage column widths take effect only in fixed layout (packages/blitz-dom/src/layout/table.rs:222-229)
- Margins are zeroed on table-internal cells (packages/blitz-dom/src/layout/table.rs:675-676); `rowspan` is clamped to 1..=65534 (packages/blitz-dom/src/layout/table.rs:599-603)
- Anonymous table wrapper boxes are not generated: internal table displays outside a table are laid out as flow containers (packages/blitz-dom/src/layout/construct.rs:590-596)
- Replaced elements are `img`, `svg`, `canvas`, `video`, `embed`, `iframe`; `<object>` is deliberately excluded so its fallback children render (packages/blitz-dom/src/layout/replaced.rs:7-17)
- Atomic inline boxes are replaced elements plus `input`, `textarea`, `button` (packages/blitz-dom/src/layout/replaced.rs:435-442)
- Default object size for replaced elements is 300x150; an image with no loaded resource has zero default size (packages/blitz-dom/src/layout/mod.rs:37-42; packages/blitz-dom/src/layout/mod.rs:54-56)
- Text nodes are never laid out individually; one reached by dispatch returns `LayoutOutput::HIDDEN` (packages/blitz-dom/src/layout/mod.rs:128-141)
- Table scrollable overflow is capped at the node size, marked `HACK` (packages/blitz-dom/src/layout/mod.rs:434-442)
- Pseudo-element `content` supports only string items; other item types are skipped (packages/blitz-dom/src/layout/construct.rs:655-672)
- Text inputs (`textarea`, and `input` with no type or text/password/email/number/search/tel/url) get a Parley text editor; checkbox/radio inputs get checkbox data (packages/blitz-dom/src/layout/construct.rs:441-458)
- Inline `<svg>` is re-parsed from its outer HTML into an image during construction (packages/blitz-dom/src/layout/construct.rs:461-492) and rebuilt whenever its subtree's style changes (packages/blitz-dom/src/layout/damage.rs:127-133)
- Float support in inline layout is compiled only with the `floats` feature (packages/blitz-dom/src/layout/inline.rs:16-19; packages/blitz-dom/src/layout/inline.rs:361-364)
- `text-indent` `hanging`/`each-line` are stated not to work because their parsing is cfg'd out in Stylo (packages/blitz-dom/src/layout/inline.rs:437-439)
- Blitz-specific damage bits (`ONLY_RELAYOUT`, `REORDER_CHILDREN`, `CONSTRUCT_BOX`, `CONSTRUCT_FC`, `CONSTRUCT_DESCENDENT`, `ALL_DAMAGE`) are defined over Servo's `RestyleDamage`; `REORDER_CHILDREN` is deliberately not part of `ALL_DAMAGE` (packages/blitz-dom/src/layout/damage.rs:21-42)
- A change to display, float, position, contain, visibility, font, or text-shaping inherited properties yields `ALL_DAMAGE`; an `order` change on an in-flow box yields `RELAYOUT | REORDER_CHILDREN`; anything else yields `RELAYOUT` (packages/blitz-dom/src/layout/damage.rs:269-374)
- Flex/grid layout children are stable-sorted by `order`, skipping the sort when all orders are 0 (packages/blitz-dom/src/layout/damage.rs:376-387; packages/blitz-dom/src/layout/construct.rs:565-566)
- Paint order: z-index-auto positioned boxes share one paint level above in-flow content and floats; only stacking contexts with non-zero z-index are hoisted to the enclosing stacking context (packages/blitz-dom/src/layout/paint_tree.rs:420-462; packages/blitz-dom/src/layout/paint_tree.rs:338-346)

## architecture §Conventions
- Submodules are `pub(crate)` (packages/blitz-dom/src/layout/mod.rs:24-30)
- Optional tracing calls are wrapped in `#[cfg(feature = "tracing")]` with a `#[cfg(not(feature = "tracing"))] let _ = …;` branch to silence unused variables (packages/blitz-dom/src/layout/mod.rs:131-139; packages/blitz-dom/src/layout/construct.rs:480-488)
- Clippy lints are allowed locally, one with a `reason` (packages/blitz-dom/src/layout/damage.rs:350-353; packages/blitz-dom/src/layout/table.rs:485; packages/blitz-dom/src/layout/construct.rs:571)
- An `unsafe` block carries a `// SAFETY:` comment (packages/blitz-dom/src/layout/table.rs:165-168)
- `TODO` comments mark known gaps in the code (packages/blitz-dom/src/layout/mod.rs:162; packages/blitz-dom/src/layout/construct.rs:428; packages/blitz-dom/src/layout/construct.rs:612; packages/blitz-dom/src/layout/inline.rs:243; packages/blitz-dom/src/layout/inline.rs:451)
- Doc comments are written on functions and structs, often citing spec URLs (packages/blitz-dom/src/layout/replaced.rs:19-22; packages/blitz-dom/src/layout/replaced.rs:34-37)
- Borrow conflicts are worked around by `std::mem::take` of a child list or layout and putting it back afterwards (packages/blitz-dom/src/layout/damage.rs:76; packages/blitz-dom/src/layout/damage.rs:103-104; packages/blitz-dom/src/layout/construct.rs:263-272; packages/blitz-dom/src/layout/inline.rs:210-216)

## architecture §Standard Contracts
- `BaseDocument` implements Taffy's tree traits: `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutContainingBlock`, `CacheTree`, `LayoutBlockContainer`, `LayoutFlexboxContainer`, `LayoutGridContainer`, `RoundTree`, `PrintTree` (packages/blitz-dom/src/layout/mod.rs:467-742)
- All Taffy style accessors return `TaffyStyloStyle<ComputedStyleRef>`; the custom ident type is `style::Atom` (packages/blitz-dom/src/layout/mod.rs:498-508)
- `TableTreeWrapper` implements `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutGridContainer` over `&taffy::Style<Atom>` (packages/blitz-dom/src/layout/table.rs:739-820)
- Public replaced-element API: `IntrinsicSizes { width, height, ratio }`, `ReplacedContext { intrinsic_sizes, default_object_size }`, `compute_replaced_layout(inputs, style, resolve_calc_value, context) -> LayoutOutput` (packages/blitz-dom/src/layout/replaced.rs:23-57)
- Public paint-tree API: `hoisted_child_position`, `HoistedPaintChild { node_id, z_index }`, `StackingContext { children, negative_z_count }` with `neg_z_range`/`pos_z_range` iterators (packages/blitz-dom/src/layout/paint_tree.rs:33; packages/blitz-dom/src/layout/paint_tree.rs:69-76; packages/blitz-dom/src/layout/paint_tree.rs:107-203)
- Public table types: `TableContext`, `TableCell`, `TableColumn`, `TableRow` (packages/blitz-dom/src/layout/table.rs:36-132)
- Crate-internal construction task types: `ConstructionTask`, `ConstructionTaskData::InlineLayout`, `ConstructionTaskResult` (packages/blitz-dom/src/layout/construct.rs:43-61)
- `resolve_calc_value(calc_ptr, parent_size)` dereferences a raw pointer to Stylo's `CalcLengthPercentage` (packages/blitz-dom/src/layout/mod.rs:72-76)

## architecture §Occupied Resources
- observed absent — ports, sockets or listeners · searched: `port|bind\(|listen` over the 8 slice files

## architecture §Infrastructure Patterns
- Layout results are cached per node through Taffy's `CacheTree` (`cache_get`/`cache_store`/`cache_clear`) (packages/blitz-dom/src/layout/mod.rs:576-605)
- The out-of-flow pass runs inside the cache wrapper so cache hits do not re-run it (packages/blitz-dom/src/layout/mod.rs:87-102)
- Background and mask images referenced from style are fetched through `net_provider.fetch` with a `stamped_request` and the document's abort signal; results are looked up in `image_cache` first and concurrent requests for one URL are queued in `pending_images` (packages/blitz-dom/src/layout/damage.rs:488-527)
- The font context is shared behind a lock (`font_ctx.lock().unwrap()`) (packages/blitz-dom/src/layout/damage.rs:415; packages/blitz-dom/src/layout/list.rs:88; packages/blitz-dom/src/layout/mod.rs:204)
- Hoisted-child positions and stacking-context bounding boxes are memoised in `Cell`s keyed by geometry generation (packages/blitz-dom/src/layout/paint_tree.rs:87-99; packages/blitz-dom/src/layout/paint_tree.rs:134-177)

## architecture §Cross-cutting Patterns
- Damage propagates up the DOM (children plus `::before`/`::after`), and anonymous ancestors are invalidated by walking `layout_parent` (packages/blitz-dom/src/layout/damage.rs:54-181)
- Non-incremental mode marks every node with `ALL_DAMAGE` (packages/blitz-dom/src/layout/damage.rs:183-192)
- Damage and dirty flags are cleared in one visit per node, skipping clean subtrees (packages/blitz-dom/src/layout/damage.rs:194-239)
- Stale slab ids (freed anonymous boxes, pseudo-elements) are tolerated by `get`/`contains_key` checks (packages/blitz-dom/src/layout/damage.rs:204-208; packages/blitz-dom/src/layout/damage.rs:435-438)
- `display: contents` is handled transparently in construction by hoisting children into the nearest non-contents container with that container's wrap policy (packages/blitz-dom/src/layout/construct.rs:199-276)
- Viewport scale is applied to Parley measurements and divided back out of layout sizes (packages/blitz-dom/src/layout/inline.rs:201; packages/blitz-dom/src/layout/inline.rs:805-808)

## architecture §Project Intent
- The layout module's stated purpose is to "Enable the dom to lay itself out using taffy" (packages/blitz-dom/src/layout/mod.rs:1)

## architecture §Existing Scopes
- `layout::construct` — box-tree construction: layout-children collection, anonymous blocks, pseudo-elements, text inputs, inline layout building (packages/blitz-dom/src/layout/construct.rs:408-653; packages/blitz-dom/src/layout/construct.rs:1035-1108)
- `layout::damage` — damage bits, propagation, layout-damage computation, style image flushing (packages/blitz-dom/src/layout/damage.rs:21-42; packages/blitz-dom/src/layout/damage.rs:269; packages/blitz-dom/src/layout/damage.rs:428-443)
- `layout::inline` — inline formatting context layout (packages/blitz-dom/src/layout/inline.rs:82-193)
- `layout::list` — list-item markers (packages/blitz-dom/src/layout/list.rs:17-41)
- `layout::paint_tree` — paint children and stacking contexts (packages/blitz-dom/src/layout/paint_tree.rs:205-218)
- `layout::replaced` — replaced-element sizing (packages/blitz-dom/src/layout/replaced.rs:52)
- `layout::table` — table context construction and grid wrapper (packages/blitz-dom/src/layout/table.rs:177-180)
- `layout` (mod) — layout dispatch by element kind and display (packages/blitz-dom/src/layout/mod.rs:104-464)

## security-plan §Threat Model Summary
- out of slice — the slice is layout code and states no threat model

## security-plan §Authentication & Authorization
- observed absent — authentication or authorization · searched: `auth|session|login|permission` over the 8 slice files

## security-plan §Input Validation
- Numeric attributes from markup are parsed with `.parse().ok()` and fall back to defaults: textarea `rows` (default 2) and `cols` (packages/blitz-dom/src/layout/mod.rs:164-171), replaced-element `width`/`height` (packages/blitz-dom/src/layout/mod.rs:267-274), `<col span>` floored at 1 (packages/blitz-dom/src/layout/table.rs:441-445), `colspan` default 1 (packages/blitz-dom/src/layout/table.rs:595-598)
- `rowspan` is clamped to 1..=65534 (packages/blitz-dom/src/layout/table.rs:599-603)
- `<ol start>` is parsed as `usize` and has 1 subtracted (packages/blitz-dom/src/layout/construct.rs:494-500)
- Degenerate aspect ratios (zero, infinite, NaN) are discarded before use (packages/blitz-dom/src/layout/replaced.rs:132-147)
- Inline-box heights are kept finite (`min(f32::MAX)`) so huge author lengths cannot stall the line breaker (packages/blitz-dom/src/layout/inline.rs:414-422)
- SVG source is parsed with `parse_svg_image`; a parse error is logged and the element left without image data (packages/blitz-dom/src/layout/construct.rs:470-490)

## security-plan §Data Protection
- out of slice — the slice handles no stored or user data beyond DOM layout state

## security-plan §API Security
- out of slice — the slice exposes no network API; it only issues image fetches through `net_provider` (packages/blitz-dom/src/layout/damage.rs:511-524)

## security-plan §Dependency Security
- out of slice — no dependency manifest is in this slice

## security-plan §Secret Management
- observed absent — secrets or credentials read · searched: `env::var|secret|token` over the 8 slice files

## security-plan §Error Handling
- A table root without styles panics ("Ignoring table because it has no styles") (packages/blitz-dom/src/layout/table.rs:191-193)
- A node flagged table root without a `TableContext` panics (packages/blitz-dom/src/layout/mod.rs:389-396)
- `unreachable!()` is used for states the code treats as impossible (packages/blitz-dom/src/layout/mod.rs:354; packages/blitz-dom/src/layout/construct.rs:886; packages/blitz-dom/src/layout/construct.rs:1030; packages/blitz-dom/src/layout/inline.rs:670)
- Two `unsafe` raw-pointer operations exist for calc values (packages/blitz-dom/src/layout/mod.rs:73; packages/blitz-dom/src/layout/table.rs:165-168)
- Table descendants without styles are skipped with an info log rather than a panic (packages/blitz-dom/src/layout/table.rs:506-510)

## security-plan §Logging & Monitoring
- Logging uses the `tracing` crate behind the `tracing` feature (packages/blitz-dom/src/layout/mod.rs:131-136); see obs-plan §Log Coverage

## design-system §Color Palette
- observed absent — color tokens or CSS custom properties · searched: `color|Color` and `--[a-z]+-|var\(` over the 8 slice files

## design-system §Typography
- The layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:116-120)
- List bullets use the font family `"Bullet, monospace, sans-serif"` (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/construct.rs:1086-1092)
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)

## design-system §Spacing
- out of slice — the slice states no spacing scale

## design-system §Depth Strategy
- out of slice — the slice implements CSS z-index paint order (packages/blitz-dom/src/layout/paint_tree.rs:420-462) but states no project depth scale

## design-system §Border Radius
- observed absent — border radius values · searched: `radius` over the 8 slice files

## design-system §Motion
- observed absent — transitions or animation · searched: `transition|animation` over the 8 slice files

## design-system §Iconography
- out of slice — no icon set is referenced

## design-system §Surface: none observed
- observed absent — an application surface (entry point, CLI, window, webview) · searched: `fn main|clap|winit|wasm_bindgen|webview` over the 8 slice files

## layout-templates §Surface: none observed
- observed absent — an application surface (entry point, CLI, window, webview) · searched: `fn main|clap|winit|wasm_bindgen|webview` over the 8 slice files

## test-plan §Test Scope Summary
- The slice holds one inline unit-test module, in list.rs, covering `marker_for_style` (packages/blitz-dom/src/layout/list.rs:185-237)

## test-plan §Test Strategy
- out of slice — the slice states no test strategy beyond the one inline module

## test-plan §Test Harness Contract
- Tests use Rust's built-in harness: `#[cfg(test)] mod tests` with `#[test]` functions and `assert_eq!` (packages/blitz-dom/src/layout/list.rs:185-204)

## test-plan §Unit Test Strategy
- Four unit tests check list markers: disc, decimal, lower-alpha (including `aa.`/`ab.` past 26), upper-alpha (packages/blitz-dom/src/layout/list.rs:200-236)
- observed absent — tests in the other seven slice files · searched: `#\[test\]` over the 8 slice files

## test-plan §Integration Test Strategy
- out of slice — no integration tests are in this slice

## test-plan §E2E Test Strategy
- out of slice — no end-to-end tests are in this slice

## test-plan §Test Data & Fixtures
- Unit tests build `ListStyleType` values inline with a `list_style` helper (packages/blitz-dom/src/layout/list.rs:196-198)

## test-plan §Mocking & Stubbing Discipline
- observed absent — mocks or fixtures · searched: `mock|fixture` over the 8 slice files

## test-plan §CI Integration
- out of slice — no CI configuration is in this slice

## obs-plan §Obs Scope Summary
- The slice emits `tracing` events only when the `tracing` feature is enabled (packages/blitz-dom/src/layout/mod.rs:131; packages/blitz-dom/src/layout/construct.rs:480; packages/blitz-dom/src/layout/damage.rs:491; packages/blitz-dom/src/layout/table.rs:507)

## obs-plan §Telemetry Strategy
- Telemetry is feature-gated `tracing` events; each call has a no-op path when the feature is off (packages/blitz-dom/src/layout/mod.rs:131-139; packages/blitz-dom/src/layout/construct.rs:480-488)

## obs-plan §Observability Harness Contract
- out of slice — no subscriber or exporter setup is in this slice

## obs-plan §Span / Trace Coverage
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 8 slice files

## obs-plan §Metric Coverage
- observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)

## obs-plan §Log Coverage
- `tracing::error!` "Tried to lay out text node individually" with fields `node_id`, `data` (packages/blitz-dom/src/layout/mod.rs:131-136)
- `tracing::warn!` "SVG parse failed" with fields `node_id`, `html`, `error` (packages/blitz-dom/src/layout/construct.rs:480-486)
- `tracing::info!` for image loading from cache, image already pending, and image fetch start, each including the image URL (packages/blitz-dom/src/layout/damage.rs:491-492; packages/blitz-dom/src/layout/damage.rs:500-501; packages/blitz-dom/src/layout/damage.rs:506-507)
- `tracing::info!` "Ignoring table descendent because it has no styles" (packages/blitz-dom/src/layout/table.rs:507-508)

## obs-plan §Error Capture & Reporting
- SVG parse errors are captured into the `error` field of a warn event and not propagated (packages/blitz-dom/src/layout/construct.rs:479-489)

## obs-plan §PII Scrubbing & Compliance
- The SVG parse-failure event carries the element's full outer HTML in field `html` (packages/blitz-dom/src/layout/construct.rs:463; packages/blitz-dom/src/layout/construct.rs:481-486)
- Image-fetch info events carry the full image URL (packages/blitz-dom/src/layout/damage.rs:489-507)
- observed absent — scrubbing or redaction · searched: `scrub|redact|sanitize` over the 8 slice files

## obs-plan §CI Integration
- out of slice — no CI configuration is in this slice

## a11y-plan §A11y Scope Summary
- observed absent — accessibility handling · searched: `aria|accessib` over the 8 slice files

## a11y-plan §A11y Strategy
- out of slice — the slice states no accessibility strategy

## a11y-plan §A11y Assertion Harness Contract
- out of slice — no a11y test harness is in this slice

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles · searched: `aria|accessib|role` over the 8 slice files (only match is a paint-tree local `role_unchanged`)

## a11y-plan §Keyboard Navigation
- observed absent — focus or keyboard handling · searched: `focus|tabindex|keyboard` over the 8 slice files

## a11y-plan §Visual Design Verification
- out of slice — the slice states no contrast or visual checks

## a11y-plan §Screen Reader Support
- out of slice — the slice states no screen-reader handling

## a11y-plan §Cognitive Accessibility
- out of slice — the slice states no cognitive-accessibility handling

## a11y-plan §CI Integration
- out of slice — no CI configuration is in this slice
