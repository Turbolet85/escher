@row_add n=1 row=decision/select-list-colours-engine-constants area=color-palette rec=fct-nqadbrsf@1
name: Select list colours as engine constants
rule: The open select list's colours are engine constants, not design tokens and not in the default stylesheet; its highlight is the same pair as a multiple select's chosen row.
why: The engine holds no design tokens.
@row_check n=1 row=decision/select-list-colours-engine-constants sha=63d5dc0d36ac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:113
quote: pub const HIGHLIGHT_TEXT: [u8; 3] = [0x00, 0x00, 0x00];
at: packages/blitz-dom/src/node/select_list.rs:115
quote: pub const HIGHLIGHT_BACKGROUND: [u8; 3] = [0xc8, 0xc8, 0xc8];
at: packages/blitz-dom/assets/default.css:196-197
quote: background-color: #c8c8c8; color: #000000;
@row_add n=2 row=decision/screenshot-never-skips-text area=typography rec=fct-kjaf2qti@1
name: Screenshot never paints with text skipped
rule: A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
@row_check n=2 row=decision/screenshot-never-skips-text sha=3ba1affbe39a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/screenshot.rs:76-77
quote: if !doc.can_paint_text() { return Err(ScreenshotError::FontsUnavailable);
at: packages/escher-driver/src/execute.rs:307
quote: ScreenshotError::FontsUnavailable => Refusal::new(Cause::FontsUnavailable),
@row_add n=3 row=contract/driver-screenshot-refusals/fonts-unavailable area=typography rec=fct-kjaf2qti@1
boundary: driver screenshot refusals
item: fonts-unavailable
promise: A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
@row_check n=3 row=contract/driver-screenshot-refusals/fonts-unavailable sha=398a2d9b9ea0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:519-520
quote: if !harness.base().can_paint_text() { return Err(Refusal::new(Cause::FontsUnavailable));
at: packages/blitz-dom/src/document.rs:1532
quote: .generic_families(parley::fontique::GenericFamily::SansSerif)
@row_add n=4 row=decision/test-engine-with-system-fonts area=typography rec=fct-6tyrmhic@1
name: Test engine built with system fonts
rule: The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate.
why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
@row_check n=4 row=decision/test-engine-with-system-fonts sha=8de94f82d0f6 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/Cargo.toml:17
quote: blitz-dom = { workspace = true, features = ["accessibility", "autofocus", "floats", "system-fonts", "text-transform-icu"] }
@row_add n=5 row=decision/wasm-bundled-font-context area=typography rec=fct-v7kb2nps@1
name: Bundled font context on WASM
rule: On WASM a font context with bundled fonts must be provided, a single-font context for one font.
@row_check n=5 row=decision/wasm-bundled-font-context sha=8f90c68502e0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/lib.rs:140
quote: pub fn build_single_font_ctx(font_data: &[u8]) -> FontContext {
@row_add n=6 row=decision/hit-testing-stops-at-paint-clip area=depth-strategy rec=fct-zdoz4c52@1
name: Hit-testing stops at a paint clip
rule: Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box.
except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
@row_check n=6 row=decision/hit-testing-stops-at-paint-clip sha=f4bcd0f711ca v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1560-1561
quote: let clips_to_padding_box = self.containing_block().is_some() && self.primary_styles().is_some_and(|style| {
at: packages/blitz-dom/src/document.rs:2436
quote: if let Some(hit) = self.select_list_hit(x, y) {
@row_add n=7 row=decision/into-view-scroll-instant-in-nested-boxes area=motion-calibrated-to-expression-level-unrecorded rec=fct-tnwqvola@1
name: Into-view scroll instant in nested boxes
rule: A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, and the requested behaviour, Smooth included, applies to the viewport alone.
except: Into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.
@row_check n=7 row=decision/into-view-scroll-instant-in-nested-boxes sha=7cffb1946806 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
at: packages/blitz-dom/src/scrolling.rs:737
quote: self.scroll_to(root_id, x, y, behavior);
at: packages/blitz-dom/src/scrolling.rs:669-672
quote: // A row of an open list scrolls within the list, and nothing else moves. if self.scroll_select_list_to(node_id) { return; }
@row_add n=8 row=decision/settle-does-not-wait-on-animation area=motion-calibrated-to-expression-level-unrecorded rec=fct-vjxch6lk@1
name: Settle does not wait on animation
rule: A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
@row_check n=8 row=decision/settle-does-not-wait-on-animation sha=529286abe8f2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:178-181
quote: return Ok(Settled { passes, animating: doc.is_animating(), });
@row_add n=9 row=decision/animation-driven-by-virtual-clock area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
name: Animation driven by the virtual clock
rule: The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
@row_check n=9 row=decision/animation-driven-by-virtual-clock sha=21101e3b3151 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:158
quote: let time = self.time();
at: packages/blitz-test-harness/src/harness.rs:154
quote: self.clock.now().as_secs_f64()
@row_add n=10 row=contract/driver-verbs-and-virtual-clock/advance area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: advance
promise: The driver's advance moves the VirtualClock by exactly its ms and, through the step a session's caller hands it, the app's own time together.
@row_check n=10 row=contract/driver-verbs-and-virtual-clock/advance sha=1ad7cc2c861e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:268-271
quote: harness.advance_time(Duration::from_millis(u64::from(ms))); if let Some(step) = step.as_mut() { step(harness, ms); }
at: packages/blitz-test-harness/src/harness.rs:172
quote: self.clock.advance(by);
@row_add n=11 row=contract/driver-verbs-and-virtual-clock/click area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: click
promise: The driver's click leaves the VirtualClock where it was.
@row_check n=11 row=contract/driver-verbs-and-virtual-clock/click sha=07eb22aab839 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/input.rs:104-107
quote: pub fn click_at(&mut self, x: f32, y: f32) { let event = mouse_pointer_event(x, y); self.dispatch(UiEvent::PointerDown(event.clone())); self.dispatch(UiEvent::PointerUp(event));
@row_add n=12 row=contract/driver-verbs-and-virtual-clock/type area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: type
promise: The driver's type leaves the VirtualClock where it was.
@row_check n=12 row=contract/driver-verbs-and-virtual-clock/type sha=4001d11304fa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/input.rs:222-224
quote: pub fn type_text(&mut self, text: &str) { for ch in text.chars() { self.press(Key::Character(ch.to_string()));
@row_add n=13 row=contract/driver-verbs-and-virtual-clock/press area=motion-calibrated-to-expression-level-unrecorded rec=fct-za7xpowz@1
boundary: driver verbs and the virtual clock
item: press
promise: The driver's press leaves the VirtualClock where it was.
@row_check n=13 row=contract/driver-verbs-and-virtual-clock/press sha=95464dcfa3a9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/input.rs:208-212
quote: self.dispatch(UiEvent::KeyDown(key_event( key.clone(), KeyState::Pressed, modifiers, )));
at: packages/blitz-test-harness/src/input.rs:218
quote: self.pump();
@row_add n=14 row=decision/load-wait-moves-no-clock area=motion-calibrated-to-expression-level-unrecorded rec=fct-rxggiikl@1
name: Load wait moves no clock
rule: While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
@row_check n=14 row=decision/load-wait-moves-no-clock sha=2326133386b3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:26
quote: /// has passed. Nothing but the settles runs: no time moves and no time step is run.
at: packages/escher-driver/src/session.rs:19
quote: pub const LOAD_WAIT: Duration = Duration::from_secs(5);
@row_add n=15 row=decision/driver-output-for-agents area=surface-cli rec=fct-or2earye@1
name: Driver command line output read by agents
rule: The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not.
why: These surfaces are read by an agent, not a person.
@row_check n=15 row=decision/driver-output-for-agents sha=c5074a417244 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:408
quote: if let Some(line) = &written.stdout {
at: packages/escher-driver/src/cli.rs:410-412
quote: let _ = stdout .write_all(line.as_bytes()) .and_then(|()| stdout.write_all(b"\n"))
searched: is_terminal|isatty|IsTerminal|atty · in packages/escher-driver/**, examples/seven_guis/** (*.rs, *.toml)
@row_add n=16 row=contract/escher-session-stdout/command-line-line area=surface-cli rec=fct-or2earye@1
boundary: escher-session stdout
item: command line output
promise: One JSON line per command: the answer, the refusal or a session error's object.
@row_check n=16 row=contract/escher-session-stdout/command-line-line sha=1a1acb8fac91 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:408
quote: if let Some(line) = &written.stdout {
at: packages/escher-driver/src/cli.rs:410-412
quote: let _ = stdout .write_all(line.as_bytes()) .and_then(|()| stdout.write_all(b"\n"))
@row_add n=17 row=contract/escher-session-stdout/mcp-line area=surface-cli rec=fct-or2earye@1
boundary: escher-session stdout
item: mcp role output
promise: Newline-delimited JSON-RPC 2.0 lines.
@row_check n=17 row=contract/escher-session-stdout/mcp-line sha=325b095808f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:180
quote: object.text("jsonrpc", "2.0");
at: packages/escher-driver/src/mcp/framing.rs:198-199
quote: output.write_all(line.as_bytes())?; output.write_all(b"\n")?;
@row_add n=18 row=contract/escher-session-exit-codes/0 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 0
promise: The command was accepted.
@row_check n=18 row=contract/escher-session-exit-codes/0 sha=9818a25c9c5b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:596
quote: status: 0,
at: packages/escher-driver/src/cli.rs:343
quote: status: ACCEPTED,
@row_add n=19 row=contract/escher-session-exit-codes/1 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 1
promise: The command was refused.
@row_check n=19 row=contract/escher-session-exit-codes/1 sha=b7bf9c459a33 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:602
quote: status: 1,
at: packages/escher-driver/src/cli.rs:348
quote: status: REFUSED,
@row_add n=20 row=contract/escher-session-exit-codes/2 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 2
promise: A usage error.
@row_check n=20 row=contract/escher-session-exit-codes/2 sha=8d1713add3df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:609
quote: status: 2,
at: packages/escher-driver/src/cli.rs:353
quote: status: USAGE,
@row_add n=21 row=contract/escher-session-exit-codes/3 area=surface-cli rec=fct-or2earye@1
boundary: escher-session exit codes
item: 3
promise: A session error; an mcp role ending in Err also exits 3.
@row_check n=21 row=contract/escher-session-exit-codes/3 sha=90b7356a801d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:616
quote: status: 3,
at: packages/escher-driver/src/cli.rs:358
quote: status: SESSION_ERROR,
at: packages/escher-driver/src/cli.rs:366-369
quote: Ending::Spoken(Err(error)) => Written { stdout: None, stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=22 row=overview/surface-web-spa area=surface-web-spa rec=fct-x3p4qjpb@1
text: The web-spa surface is WASM pages rendering into a canvas: on wasm32 the dioxus-native surface is a canvas appended to the page body and sized from host CSS, so a page such as todomvc fills the window with that canvas.
@row_check n=22 row=overview/surface-web-spa sha=ed371682502f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/config.rs:32
quote: WindowAttributesWeb::default().with_append(true),
at: examples/todomvc/index.html:9
quote: canvas { display: block; width: 100%; height: 100%; }
@row_add n=23 row=contract/viewport-color-scheme/color-scheme area=color-palette rec=fct-bwfqqjae@1
boundary: The viewport's system color scheme
item: color scheme
promise: The system color scheme is Light by default or Dark, is set on the viewport, and feeds prefers-color-scheme.
@row_check n=23 row=contract/viewport-color-scheme/color-scheme sha=5cb83a3d0917 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:72
quote: Light,
at: packages/blitz-traits/src/shell.rs:73
quote: Dark,
at: packages/blitz-dom/src/stylo_device.rs:81
quote: ColorScheme::Light => PrefersColorScheme::Light,
at: packages/blitz-dom/src/stylo_device.rs:82
quote: ColorScheme::Dark => PrefersColorScheme::Dark,
@row_add n=24 row=decision/color-scheme-change-full-recascade area=color-palette rec=fct-tkymbjhz@1
name: Full recascade on color-scheme change
rule: A change of color scheme triggers a full recascade of the document.
why: light-dark() and system colors resolve at cascade time.
@row_check n=24 row=decision/color-scheme-change-full-recascade sha=af60ad6d8f0e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2716-2718
quote: if changes.contains(DeviceChanges::COLOR_SCHEME) { if let Some(root_id) = self.try_root_element().map(|el| el.id) { self.nodes[root_id].set_restyle_hint(RestyleHint::recascade_subtree());
@row_add n=25 row=contract/dioxus-native-config/base-color area=color-palette rec=fct-kmvrmuee@1
boundary: dioxus-native configuration
item: base_color
promise: The frame clear color is an optional base_color, a peniko Color, unset by default.
@row_check n=25 row=contract/dioxus-native-config/base-color sha=1d864126127e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/config.rs:12
quote: pub(crate) base_color: Option<Color>,
at: packages/dioxus-native/src/config.rs:40
quote: base_color: None,
@row_add n=26 row=decision/select-chosen-row-chromium-pair area=color-palette rec=fct-vx2ivc7b@1
name: Select chosen row takes Chromium's inactive selected pair
rule: A multiple select's chosen row is painted with Chromium's inactive selected pair, a #c8c8c8 background with #000000 text.
why: Follows Chromium's inactive selected pair; hardcoded as the engine holds no tokens.
guard: select_interaction
@row_check n=26 row=decision/select-chosen-row-chromium-pair sha=99b9d106f04a v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/assets/default.css:196-197
quote: background-color: #c8c8c8; color: #000000;
guard_at: tests/blitz-tests/tests/select_interaction.rs:1
@row_add n=27 row=decision/engine-colours-hardcoded area=color-palette rec=fct-vx2ivc7b@1,fct-4365wjwp@1,fct-4pd3zld4@1,fct-7xjrejiq@1
name: Engine colours hardcoded
rule: The engine's own select colours (chosen row, open list highlighted row, disabled row and border) are fixed hardcoded values; the open select list's are constants of the select list view, not in the default sheet.
why: The engine holds no tokens.
@row_check n=27 row=decision/engine-colours-hardcoded sha=c6631ab8f2df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:115
quote: pub const HIGHLIGHT_BACKGROUND: [u8; 3] = [0xc8, 0xc8, 0xc8];
at: packages/blitz-dom/src/node/select_list.rs:117
quote: pub const DISABLED_TEXT: [u8; 3] = [0x6d, 0x6d, 0x6d];
at: packages/blitz-dom/src/node/select_list.rs:120
quote: pub const BORDER: [u8; 3] = [0x99, 0x99, 0x99];
at: packages/blitz-dom/assets/default.css:196
quote: background-color: #c8c8c8;
@row_add n=28 row=absence/engine-color-tokens area=color-palette rec=fct-5lr3uypt@1
capability: Color tokens or CSS custom properties in the engine and integration crates
@row_check n=28 row=absence/engine-color-tokens sha=50a4cd76a789 v=holds judge=sonnet head=edd0cfaea7f0
searched: ^\s*--[a-z][a-z-]*\s*:|var\(--  · in packages/**/*.rs and packages/**/*.css
@row_add n=29 row=decision/hit-test-stops-at-paint-clip area=depth-strategy rec=fct-ynoy6voq@1
name: Hit-testing stops where paint clips
rule: Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box.
why: Ratified; a hit answers only what paint shows.
guard: hit_clipped_at_scrolling_box
except: The root element.
@row_check n=29 row=decision/hit-test-stops-at-paint-clip sha=b326748f4a41 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1567-1569
quote: style.clone_overflow_x() != Overflow::Visible || style.clone_overflow_y() != Overflow::Visible || contains_paint
guard_at: tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:1
@row_add n=30 row=decision/auto-z-index-shared-paint-level area=depth-strategy rec=fct-f2g7f7ak@1
name: Positioned auto z-index paint level
rule: Positioned descendants with z-index auto share one paint level and paint in tree order.
why: CSS 2.1 Appendix E
guard: paint_order
@row_check n=30 row=decision/auto-z-index-shared-paint-level sha=f3fc90153ce2 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/paint_order.rs:1-2
quote: //! CSS 2.1 Appendix E: all positioned descendants with z-index: auto share //! one paint level (step 8) and paint in tree order among themselves.
guard_at: tests/blitz-tests/tests/paint_order.rs:1
@row_add n=31 row=decision/cursor-icon-crate area=iconography rec=fct-yw4swvug@1
name: cursor-icon crate for cursor icons
rule: Cursor icons are cursor-icon crate values, CSS cursor keywords map one-to-one to them, and cursor: none maps to no cursor.
@row_check n=31 row=decision/cursor-icon-crate sha=78ee9c9d9b52 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/Cargo.toml:21
quote: cursor-icon = { workspace = true }
at: packages/blitz-dom/src/stylo_to_cursor_icon.rs:13
quote: StyloCursorKind::None => None,
@row_add n=32 row=contract/shell-provider/set-cursor area=iconography rec=fct-yw4swvug@1
boundary: ShellProvider
item: set_cursor
promise: Takes an optional CursorIcon; None hides the cursor and resets it to Default.
@row_check n=32 row=contract/shell-provider/set-cursor sha=214ffb539b60 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:13
quote: fn set_cursor(&self, icon: Option<CursorIcon>) {
at: packages/blitz-shell/src/lib.rs:115-116
quote: self.window.set_cursor(Cursor::Icon(CursorIcon::Default));                 self.window.set_cursor_visible(false)
@row_add n=33 row=decision/into-view-scroll-nested-boxes-instant area=motion rec=fct-vt52hpbo@1
name: Into-view scroll is instant in nested boxes
rule: A programmatic into-view scroll writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked; the requested behaviour, Smooth included, applies to the viewport alone.
why: The document holds one scroll animation at a time.
@row_check n=33 row=decision/into-view-scroll-nested-boxes-instant sha=c27427fe734b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
at: packages/blitz-dom/src/scrolling.rs:737
quote: self.scroll_to(root_id, x, y, behavior);
@row_add n=34 row=decision/overlay-scrollbar-fade-timings area=motion rec=fct-57yciv35@1
name: Overlay scrollbar fade timings
rule: Overlay scrollbars show at full opacity on scroll, stay opaque for 500 ms after last activity, then fade linearly over 200 ms.
why: Documented as Chromium's overlay timings.
@row_check n=34 row=decision/overlay-scrollbar-fade-timings sha=883a4036a664 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/scrollbar.rs:15
quote: pub(crate) const FADE_DELAY: Duration = Duration::from_millis(500);
at: packages/blitz-dom/src/node/scrollbar.rs:16
quote: pub(crate) const FADE_DURATION: Duration = Duration::from_millis(200);
at: packages/blitz-dom/src/node/scrollbar.rs:24
quote: Some(fading) => 1.0 - (fading.as_secs_f32() / FADE_DURATION.as_secs_f32()).min(1.0),
@row_add n=35 row=decision/driver-advance-moves-one-clock area=motion rec=fct-e2e2e6vg@1
name: Driver advance moves the one virtual clock
rule: The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was.
why: Ratified.
@row_check n=35 row=decision/driver-advance-moves-one-clock sha=cb6caac711ee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:268
quote: harness.advance_time(Duration::from_millis(u64::from(ms)));
at: packages/escher-driver/src/execute.rs:270
quote: step(harness, ms);
@row_add n=36 row=decision/body-right-bottom-margin-ignored area=spacing rec=fct-pzibjwj2@1
name: Body rightmargin and bottommargin ignored
rule: The body attributes rightmargin and bottommargin are deliberately ignored, while marginwidth, marginheight, leftmargin and topmargin map to pixel margins.
@row_check n=36 row=decision/body-right-bottom-margin-ignored sha=844bbeb6c8bb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:1184-1188
quote: "marginwidth" => b"lr",                     "marginheight" => b"tb",                     "leftmargin" => b"l",                     "topmargin" => b"t",                     _ => b"",
@row_add n=37 row=decision/select-list-engine-overlay area=surface rec=fct-ayatph22@1
name: Select list as an engine overlay
rule: On desktop-native a single select's open list is an engine overlay that reaches the screen through the scene paint alone, not a second OS window and not rows as boxes.
@row_check n=37 row=decision/select-list-engine-overlay sha=1e10cfcd9862 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:248
quote: self.paint_select_list(scene);
at: packages/blitz-paint/src/render/select_list.rs:22
quote: pub(super) fn paint_select_list(&self, scene: &mut impl PaintScene) {
@row_add n=38 row=contract/select-list-events/open area=surface rec=fct-gacdn4gi@1
boundary: A single select's list events
item: open
promise: The list opens on its chosen option with no input and no change event.
@row_check n=38 row=contract/select-list-events/open sha=ea4370c9fc6a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:837-839
quote: let current = select::last_chosen(&readings) .or_else(|| readings.iter().position(|option| !option.disabled)) .and_then(|index| options.get(index).copied());
@row_add n=39 row=contract/select-list-events/highlight-move area=surface rec=fct-gacdn4gi@1
boundary: A single select's list events
item: highlight move
promise: Keys and the pointer move the highlight over enabled rows and choose nothing.
@row_check n=39 row=contract/select-list-events/highlight-move sha=045a7e7f9a4f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:247-248
quote: if let Some(option) = target.and_then(|index| options.get(index).copied()) { doc.move_select_list_current(option);
at: packages/blitz-dom/src/events/pointer.rs:606
quote: list.current = Some(hit.node_id);
@row_add n=40 row=contract/select-list-events/choose area=surface rec=fct-hpj6y5dq@1
boundary: A single select's list events
item: choose
promise: Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event.
@row_check n=40 row=contract/select-list-events/choose sha=075b81cd3ad6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:228-229
quote: (Key::Named(NamedKey::Enter), None) => { choose_current_and_close(doc, node_id, dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:237-238
quote: if text == " " && !doc.select_text_stands(node_id) { choose_current_and_close(doc, node_id, dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:31-33
quote: if event.key == Key::Named(NamedKey::Tab) { if let Some(select) = doc.open_select() { choose_current_and_close(doc, select, &mut dispatch_event);
at: packages/blitz-dom/src/events/pointer.rs:654
quote: doc.apply_select_choice(select, &[row], dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:485-494
quote: dispatch_event(DomEvent::new( select, DomEventData::Input(BlitzInputEvent { value: value.clone(), }), )); dispatch_event(DomEvent::new( select, DomEventData::Change(BlitzInputEvent { value }), ));
@row_add n=41 row=contract/select-list-events/choose-disabled-row area=surface rec=fct-hpj6y5dq@1
boundary: A single select's list events
item: choose a disabled row
promise: Choosing a disabled row changes nothing and leaves the list open.
@row_check n=41 row=contract/select-list-events/choose-disabled-row sha=ebaa2f347e1f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:650-652
quote: if doc.get_node(row).is_some_and(select::option_disabled) { return; }
@row_add n=42 row=contract/select-list-events/choose-chosen-option area=surface rec=fct-hpj6y5dq@1
boundary: A single select's list events
item: choose the chosen option
promise: Choosing the option already chosen fires nothing.
@row_check n=42 row=contract/select-list-events/choose-chosen-option sha=2fa7ded2d734 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:480-482
quote: if !self.set_select_choice(&options, chosen) { return; }
at: packages/blitz-dom/src/document.rs:1135
quote: if node.element_data().is_none() || select::is_chosen(node) == chosen {
@row_add n=43 row=contract/select-list-events/close area=surface rec=fct-uazyshoy@1
boundary: A single select's list events
item: close
promise: Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event.
@row_check n=43 row=contract/select-list-events/close sha=9e8986467f78 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:232-233
quote: (Key::Named(NamedKey::Escape), None) => { doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:954-955
quote: if standing { doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:632
quote: doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:1201
quote: doc.close_select_list();
at: packages/blitz-dom/src/document.rs:915-916
quote: if self.live_select_list().is_none() { self.close_select_list();
@row_add n=44 row=contract/select-list-events/close-on-select-change area=surface rec=fct-uazyshoy@1
boundary: A single select's list events
item: close on its select's change
promise: Removing, disabling, hiding or making its select inert closes the list.
@row_check n=44 row=contract/select-list-events/close-on-select-change sha=b6b2225e75a1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:799-805
quote: self.focus_node_id == Some(select) && node.flags.is_in_document() && select::is_single_select(element) && !element.is_disabled() && !node.is_inert() && shown() && invertible()
at: packages/blitz-dom/src/document.rs:915-916
quote: if self.live_select_list().is_none() { self.close_select_list();
@row_add n=45 row=contract/select-accessibility/highlighted-row area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: highlighted row
promise: The highlighted row of an open list is in no snapshot and no tree.
@row_check n=45 row=contract/select-accessibility/highlighted-row sha=e40ba0c95040 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:241
quote: builder.set_expanded(self.select_list_for(node.id).is_some());
searched: highlight · in packages/blitz-dom/src/accessibility.rs
searched: highlight · in packages/dioxus-native-dom/src/snapshot.rs
@row_add n=46 row=contract/select-accessibility/open-select area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: open select
promise: The tree reads an open select as ComboBox and expanded.
@row_check n=46 row=contract/select-accessibility/open-select sha=956b3b1fb50a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:518
quote: None => Some(Role::ComboBox),
at: packages/blitz-dom/src/accessibility.rs:241
quote: builder.set_expanded(self.select_list_for(node.id).is_some());
@row_add n=47 row=contract/select-accessibility/chosen-option area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: chosen option
promise: The tree reads a select's chosen option as selected.
@row_check n=47 row=contract/select-accessibility/chosen-option sha=5a6ad61d81b4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:187
quote: builder.set_selected(select::is_chosen(node));
@row_add n=48 row=contract/select-accessibility/open-attribute area=surface rec=fct-6br6qi3d@1
boundary: The snapshot and accessibility tree of a select
item: open attribute
promise: An author's open attribute steers nothing on a select.
@row_check n=48 row=contract/select-accessibility/open-attribute sha=63e06b19c153 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
@row_add n=49 row=contract/snapshot/select-list-row-bounds area=surface rec=fct-grd6lupt@1
boundary: The snapshot
item: a row of an open select list
promise: A row of an open list carries its own bounds in the snapshot.
@row_check n=49 row=contract/snapshot/select-list-row-bounds sha=0017a133de4e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:163-164
quote: let bounds = match self.doc.select_list_bounds(node_id) { Some(bounds) => bounds,
@row_add n=50 row=contract/screenshot/select-list-row area=surface rec=fct-grd6lupt@1
boundary: The screenshot verb
item: a row of an open select list
promise: A screenshot of a row of an open list crops to the row and leaves the list open.
@row_check n=50 row=contract/screenshot/select-list-row sha=b4fc3ee4a2e3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:301
quote: Cleared::Rect(rect) => harness.screenshot_rect(rect),
@row_add n=51 row=contract/js-window/inner-width-height area=surface rec=fct-nodrgyrg@1
boundary: The JS window object
item: innerWidth / innerHeight
promise: innerWidth and innerHeight are the window size divided by the scale.
@row_check n=51 row=contract/js-window/inner-width-height sha=fbd7f4f42bb3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/window.rs:130
quote: viewport.window_size.0 as f64 / viewport.scale() as f64,
at: packages/blitz-vibey-script/src/runtime/window.rs:139
quote: viewport.window_size.1 as f64 / viewport.scale() as f64,
@row_add n=52 row=contract/js-window/outer-width-height area=surface rec=fct-nodrgyrg@1
boundary: The JS window object
item: outerWidth / outerHeight
promise: outerWidth and outerHeight alias innerWidth and innerHeight.
@row_check n=52 row=contract/js-window/outer-width-height sha=b9ee90283fba v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:138
quote: register_global_accessor(context, "outerWidth", window::inner_width);
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:139
quote: register_global_accessor(context, "outerHeight", window::inner_height);
@row_add n=53 row=contract/js-window/device-pixel-ratio area=surface rec=fct-nodrgyrg@1
boundary: The JS window object
item: devicePixelRatio
promise: devicePixelRatio is the scale.
@row_check n=53 row=contract/js-window/device-pixel-ratio sha=aeb9ee27d858 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/window.rs:149-150
quote: let scale = ctx.doc.borrow().viewport().scale(); Ok(JsValue::from(scale as f64))
@row_add n=54 row=contract/shell-provider/minimize area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: minimize
promise: The shell provider exposes minimize for custom titlebars.
@row_check n=54 row=contract/shell-provider/minimize sha=e76def61479d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:146-147
quote: fn set_window_minimized(&self, minimized: bool) { self.window.set_minimized(minimized);
@row_add n=55 row=contract/shell-provider/maximize area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: maximize
promise: The shell provider exposes maximize for custom titlebars.
@row_check n=55 row=contract/shell-provider/maximize sha=8d2b26b995f0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:149-150
quote: fn set_window_maximized(&self, maximized: bool) { self.window.set_maximized(maximized);
@row_add n=56 row=contract/shell-provider/decorations-toggle area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: decorations toggle
promise: The shell provider exposes a decorations toggle for custom titlebars.
@row_check n=56 row=contract/shell-provider/decorations-toggle sha=1f8ce72d4845 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:155-156
quote: fn set_window_decorations(&self, decorations: bool) { self.window.set_decorations(decorations);
@row_add n=57 row=contract/shell-provider/window-drag area=surface rec=fct-ryuge4oa@1
boundary: The shell provider
item: window drag
promise: The shell provider exposes window drag for custom titlebars.
@row_check n=57 row=contract/shell-provider/window-drag sha=bec3642191e0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:158-159
quote: fn drag_window(&self) { let _ = self.window.drag_window();
@row_add n=58 row=contract/dioxus-native-config/alpha-mode area=surface rec=fct-ls27dgne@1
boundary: The dioxus-native config
item: window compositing alpha mode
promise: The window compositing alpha mode is configurable, for example for transparent windows; the renderer ignores unsupported modes.
@row_check n=58 row=contract/dioxus-native-config/alpha-mode sha=baa39b420bb6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/config.rs:71-72
quote: pub fn with_alpha_mode(mut self, alpha_mode: CompositeAlphaMode) -> Self { self.alpha_mode = Some(alpha_mode);
@row_add n=59 row=decision/buffered-render-replay area=surface rec=fct-zgbhkh7f@1
name: Render buffered, then replayed under one borrow
rule: On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow.
why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic
@row_check n=59 row=decision/buffered-render-replay sha=2f868440afa1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:170
quote: self.vdom.rebuild(&mut self.buffered_mutations);
at: packages/dioxus-native-dom/src/dioxus_document.rs:318
quote: self.vdom.render_immediate(&mut self.buffered_mutations);
at: packages/dioxus-native-dom/src/dioxus_document.rs:319-321
quote: let mut inner = self.inner.borrow_mut(); let mut writer = MutationWriter::new(&mut inner, &mut self.vdom_state); self.buffered_mutations.replay(&mut writer);
@row_add n=60 row=contract/mounteddata-during-render/document-state area=surface rec=fct-zgbhkh7f@1
boundary: MountedData calls during a render
item: document state
promise: A MountedData call made during a render acts on the document as the previous render left it, so a node created in that same render is not yet in it.
@row_check n=60 row=contract/mounteddata-during-render/document-state sha=7a305ffbd534 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:318-321
quote: self.vdom.render_immediate(&mut self.buffered_mutations); let mut inner = self.inner.borrow_mut(); let mut writer = MutationWriter::new(&mut inner, &mut self.vdom_state); self.buffered_mutations.replay(&mut writer);
@row_add n=61 row=contract/mounteddata-during-render/set-focus-to area=surface rec=fct-zgbhkh7f@1
boundary: MountedData calls during a render
item: set_focus_to
promise: set_focus_to returns false for a node created in the same render.
@row_check n=61 row=contract/mounteddata-during-render/set-focus-to sha=a8dd10cbbeb9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2265-2266
quote: if !self.nodes[focus_node_id].flags.is_in_document() { return false;
@row_add n=62 row=decision/mobile-targets-built-not-tested area=surface rec=fct-zxgw7a6c@1
name: Mobile targets built, not tested
rule: CI builds the iOS and Android targets but runs no tests on them.
@row_check n=62 row=decision/mobile-targets-built-not-tested sha=ef3a9c9bb76d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:364
quote: command: "build",
at: .github/workflows/ci.yml:372
quote: command: "build",
@row_add n=63 row=contract/wpt-runner-result-colours/pass area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: PASS
promise: PASS prints green.
@row_check n=63 row=contract/wpt-runner-result-colours/pass sha=e0469db81d89 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:392
quote: TestStatus::Pass => write!(out, "{}", result_str.green()).unwrap(),
@row_add n=64 row=contract/wpt-runner-result-colours/fail-some-passing area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: FAIL with some passing subtests
promise: FAIL with some passing subtests prints yellow.
@row_check n=64 row=contract/wpt-runner-result-colours/fail-some-passing sha=90b92cf3ca21 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:394-395
quote: TestStatus::Fail if self.subtest_counts.pass > 0 => { write!(out, "{}", result_str.yellow()).unwrap()
@row_add n=65 row=contract/wpt-runner-result-colours/fail area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: FAIL
promise: FAIL prints red.
@row_check n=65 row=contract/wpt-runner-result-colours/fail sha=0cae858015d6 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:397
quote: TestStatus::Fail => write!(out, "{}", result_str.red()).unwrap(),
@row_add n=66 row=contract/wpt-runner-result-colours/timeout area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: TIMEOUT
promise: TIMEOUT prints bright red.
@row_check n=66 row=contract/wpt-runner-result-colours/timeout sha=8924020184a1 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:398
quote: TestStatus::Timeout => write!(out, "{}", result_str.bright_red()).unwrap(),
@row_add n=67 row=contract/wpt-runner-result-colours/skip area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: SKIP
promise: SKIP prints bright black.
@row_check n=67 row=contract/wpt-runner-result-colours/skip sha=06cbab6af640 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:399
quote: TestStatus::Skip => write!(out, "{}", result_str.bright_black()).unwrap(),
@row_add n=68 row=contract/wpt-runner-result-colours/crash area=surface rec=fct-he774etc@1
boundary: WPT runner result colours
item: CRASH
promise: CRASH prints bright magenta.
@row_check n=68 row=contract/wpt-runner-result-colours/crash sha=2f0de28ddcaa v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:400
quote: TestStatus::Crash => write!(out, "{}", result_str.bright_magenta()).unwrap(),
@row_add n=69 row=decision/wpt-runner-secondary-text-bright-black area=surface rec=fct-yhgw2pup@1
name: WPT runner secondary text in bright black
rule: The WPT runner prints test kind, flag markers and summary section headings in bright black.
@row_check n=69 row=decision/wpt-runner-secondary-text-bright-black sha=0956f895a66c v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:404
quote: write!(out, "{}", format_args!("{}", self.kind).bright_black()).unwrap();
at: wpt/runner/src/main.rs:417
quote: write!(out, "{}", "F".bright_black()).unwrap();
at: wpt/runner/src/main.rs:796
quote: println!("{}", "\nOf tests run:".bright_black());
@row_add n=70 row=contract/wpt-runner-output/test-name-hyperlink area=surface rec=fct-uompn4jm@1
boundary: WPT runner output
item: test name
promise: A test name is wrapped in an OSC 8 hyperlink to https://wpt.live/{name} only when stdout supports hyperlinks (a terminal, honouring FORCE_HYPERLINK).
@row_check n=70 row=contract/wpt-runner-output/test-name-hyperlink sha=0cee00ecd157 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:67-68
quote: static USE_HYPERLINKS: LazyLock<bool> = LazyLock::new(|| supports_hyperlinks::on(HyperlinkStream::Stdout));
at: wpt/runner/src/main.rs:370
quote: let url = format!("https://wpt.live/{}", self.name);
@row_add n=71 row=decision/escher-session-agent-read-surface area=surface rec=fct-6fb4pz6c@1
name: escher-session is read by an agent, not a person
rule: An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
@row_check n=71 row=decision/escher-session-agent-read-surface sha=c0fcc956aa4b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:408
quote: if let Some(line) = &written.stdout {
at: packages/escher-driver/src/cli.rs:410-413
quote: let _ = stdout             .write_all(line.as_bytes())             .and_then(|()| stdout.write_all(b"\n"))             .and_then(|()| stdout.flush());
@row_add n=72 row=decision/escher-session-no-colour area=surface rec=fct-6fb4pz6c@1
name: No colour and no terminal test on escher-session
rule: escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not.
guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
@row_check n=72 row=decision/escher-session-no-colour sha=9fef4f5d3235 v=holds guard=true judge=sonnet head=edd0cfaea7f0
searched: is_terminal|isatty|IsTerminal|owo-colors|colored|termcolor|anstyle|crossterm · in packages/escher-driver and examples/seven_guis (no match)
searched: colour dependency · in packages/escher-driver/Cargo.toml (deps: blitz-test-harness, dioxus-native-dom, png, serde_json, tracing)
guard_at: examples/seven_guis/tests/common/mod.rs:71
@row_add n=73 row=contract/escher-session-stderr/usage-error area=surface rec=fct-6fb4pz6c@1
boundary: escher-session stderr
item: usage error
promise: A usage error prints one fixed line on stderr with stdout empty.
@row_check n=73 row=contract/escher-session-stderr/usage-error sha=bc929c9fa796 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-354
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE, },
@row_add n=74 row=contract/escher-session-stderr/session-error area=surface rec=fct-6fb4pz6c@1
boundary: escher-session stderr
item: session error
promise: A session error prints its fixed message on stderr.
@row_check n=74 row=contract/escher-session-stderr/session-error sha=8892235da9f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:355-359
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR, },
@row_add n=75 row=contract/escher-session-exit-status/0 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 0
promise: Accepted.
@row_check n=75 row=contract/escher-session-exit-status/0 sha=b8b1b9261ed7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:596-597
quote: status: 0, name: "accepted",
@row_add n=76 row=contract/escher-session-exit-status/1 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 1
promise: Refused.
@row_check n=76 row=contract/escher-session-exit-status/1 sha=037d6deef190 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:602-603
quote: status: 1, name: "refused",
@row_add n=77 row=contract/escher-session-exit-status/2 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 2
promise: Usage.
@row_check n=77 row=contract/escher-session-exit-status/2 sha=0ca356236285 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:609-610
quote: status: 2, name: "usage",
@row_add n=78 row=contract/escher-session-exit-status/3 area=surface rec=fct-6fb4pz6c@1,fct-j6lb6pph@1
boundary: escher-session exit status
item: 3
promise: Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
@row_check n=78 row=contract/escher-session-exit-status/3 sha=271184ba952c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:616-617
quote: status: 3, name: "session-error",
at: packages/escher-driver/src/error.rs:164
quote: kind: "record-dir-not-private",
at: packages/escher-driver/src/error.rs:170
quote: kind: "recording-failed",
@row_add n=79 row=decision/exit-statuses-from-schema-table area=surface rec=fct-6fb4pz6c@1
name: Exit statuses read from the schema's table
rule: The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
@row_check n=79 row=decision/exit-statuses-from-schema-table sha=8ab83fcb0729 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:25
quote: const ACCEPTED: u8 = EXIT_STATUSES[0].status;
at: packages/escher-driver/src/describe.rs:38
quote: json::list(EXIT_STATUSES.iter().map(|ending| {
@row_add n=80 row=contract/escher-session-commands/help area=surface rec=fct-6fb4pz6c@1
boundary: escher-session commands
item: help
promise: help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
@row_check n=80 row=contract/escher-session-commands/help sha=ee56bfbe3759 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:432
quote: Line::Help => Ending::Accepted(describe()),
at: packages/escher-driver/src/cli.rs:343
quote: status: ACCEPTED,
@row_add n=81 row=contract/escher-session-session-error/remedy area=surface rec=fct-6fb4pz6c@1
boundary: escher-session session error object
item: remedy
promise: A session error's object carries its fixed remedy.
@row_check n=81 row=contract/escher-session-session-error/remedy sha=84dc9beba54c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:544
quote: error.text("remedy", self.remedy());
@row_add n=82 row=contract/escher-session-commands/screenshot area=surface rec=fct-6fb4pz6c@1
boundary: escher-session commands
item: screenshot [--id]
promise: screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
@row_check n=82 row=contract/escher-session-commands/screenshot sha=5b6711196f52 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
at: packages/escher-driver/src/json.rs:357
quote: object.count("width", u64::from(width));
@row_add n=83 row=contract/escher-session-commands/snapshot area=surface rec=fct-6fb4pz6c@1
boundary: escher-session commands
item: snapshot [--part <part>]
promise: snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts.
@row_check n=83 row=contract/escher-session-commands/snapshot sha=9ff1ab7abac3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:458
quote: object.text("text", text);
at: packages/escher-driver/src/json.rs:464-465
quote: object.count("part", u64::from(*part)); object.count("parts", u64::from(*parts));
@row_add n=84 row=contract/escher-session-mcp/stdout area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: stdout
promise: stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
@row_check n=84 row=contract/escher-session-mcp/stdout sha=911527dc8d5a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:151
quote: Envelope::Notification => return Ok(()),
at: packages/escher-driver/src/mcp/framing.rs:197-200
quote: pub(super) fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> { output.write_all(line.as_bytes())?; output.write_all(b"\n")?; output.flush()
at: packages/escher-driver/src/cli.rs:360
quote: Ending::Served | Ending::Spoken(Ok(())) => Written {
at: packages/escher-driver/src/cli.rs:361
quote: stdout: None,
@row_add n=85 row=contract/escher-session-mcp/snapshot-result area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: snapshot tool result
promise: A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts.
@row_check n=85 row=contract/escher-session-mcp/snapshot-result sha=4a5304c3ecbe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:230
quote: Ok(Outcome::Screen { text }) => ToolResult::accepted(text),
at: packages/escher-driver/src/mcp.rs:234
quote: Ok(outcome) => ToolResult::accepted(outcome.to_json()),
@row_add n=86 row=contract/escher-session-mcp/help-result area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: help tool result
promise: help's tool result is the self-description's line.
@row_check n=86 row=contract/escher-session-mcp/help-result sha=6ea2b0a036b7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:239
quote: Ok(_) => ToolResult::accepted(describe()),
at: packages/escher-driver/src/mcp.rs:253
quote: Ok(SessionCommand::Help) => ToolResult::accepted(describe()),
@row_add n=87 row=contract/escher-session-mcp/screenshot-result area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: screenshot tool result
promise: A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
@row_check n=87 row=contract/escher-session-mcp/screenshot-result sha=af5ac3210f20 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:88-93
quote: if let Some(data) = &self.image { let mut block = Object::new(); block.text("type", "image"); block.text("data", data); block.text("mimeType", "image/png"); blocks.push(block.end());
at: packages/escher-driver/src/mcp/result.rs:96
quote: block.text("type", "text");
at: packages/escher-driver/src/mcp/result.rs:50
quote: text: json::shot_without_image(width, height),
@row_add n=88 row=contract/escher-session-mcp/other-results area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: other tool results
promise: Any other tool result's text is the command's JSON line.
@row_check n=88 row=contract/escher-session-mcp/other-results sha=f08844bb68ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:234
quote: Ok(outcome) => ToolResult::accepted(outcome.to_json()),
@row_add n=89 row=contract/escher-session-mcp/err-ending area=surface rec=fct-6fb4pz6c@1
boundary: escher-session mcp role
item: Err ending
promise: An Err ending's fixed message goes to stderr with status 3.
@row_check n=89 row=contract/escher-session-mcp/err-ending sha=1a77955b5def v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:366
quote: Ending::Spoken(Err(error)) => Written {
at: packages/escher-driver/src/cli.rs:369
quote: status: SESSION_ERROR,
@row_add n=90 row=contract/escher-session-tasks/counter area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: counter
promise: counter is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_check n=90 row=contract/escher-session-tasks/counter sha=b33b6a132a90 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:24
quote: ("counter", Served::Lean(LeanTask::Counter)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=91 row=contract/escher-session-tasks/flight-booker area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: flight-booker
promise: flight-booker is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_check n=91 row=contract/escher-session-tasks/flight-booker sha=455e43568a83 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:25
quote: ("flight-booker", Served::Lean(LeanTask::FlightBooker)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=92 row=contract/escher-session-tasks/timer area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: timer
promise: timer is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_check n=92 row=contract/escher-session-tasks/timer sha=5c1417310c2b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:26
quote: ("timer", Served::Lean(LeanTask::Timer)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=93 row=contract/escher-session-tasks/crud area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: crud
promise: crud is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_check n=93 row=contract/escher-session-tasks/crud sha=f79d3fedf788 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:27
quote: ("crud", Served::Lean(LeanTask::Crud)),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=94 row=contract/escher-session-tasks/cells area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: cells
promise: cells is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_check n=94 row=contract/escher-session-tasks/cells sha=a3214ab96b86 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:28
quote: ("cells", Served::Cells),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=95 row=contract/escher-session-tasks/load area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: load
promise: load is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
@row_check n=95 row=contract/escher-session-tasks/load sha=bfe206f7598c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:29
quote: ("load", Served::Load),
at: packages/escher-driver/src/cli.rs:400
quote: apps.join(" | ")
at: packages/escher-driver/src/mcp/listing.rs:72
quote: property.json("enum", &strings(apps.iter().copied()));
@row_add n=96 row=contract/escher-session-tasks/unknown-task area=surface rec=fct-ufq6kvf4@1
boundary: escher-session tasks
item: unknown task
promise: An unknown task is a usage ending.
@row_check n=96 row=contract/escher-session-tasks/unknown-task sha=9f98ebc529d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:230
quote: Some(SessionCommand::Start { app }) if !apps.contains(&app.as_str()) => Line::Usage,
at: packages/escher-driver/src/cli.rs:163
quote: if option == SESSION_OPTION && apps.contains(&app.as_str()) =>
@row_add n=97 row=contract/escher-session-commands/graph-arguments area=surface rec=fct-gnekz77j@1
boundary: escher-session commands
item: graph arguments
promise: graph takes no argument; an argument is refused malformed with nothing run.
@row_check n=97 row=contract/escher-session-commands/graph-arguments sha=56526e89e9f2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:393
quote: args: &[],
at: packages/escher-driver/src/command.rs:324-325
quote: let Some((arg, slot)) = passed.iter_mut().find(|(arg, _)| arg.name == name) else { return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=98 row=contract/escher-session-commands/graph-answer area=surface rec=fct-gnekz77j@1
boundary: escher-session commands
item: graph answer
promise: graph answers the screen's whole UI graph as one uncoloured line of JSON {"nodes":[…],"states":[…]} on stdout, status 0, the same on the session socket's call and in a kept check; over the mcp role the tool result is the text block of that line.
@row_check n=98 row=contract/escher-session-commands/graph-answer sha=2cc205a2b028 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:505-506
quote: object.json("nodes", &list(graph.nodes.iter().map(graph_node))); object.json("states", &list(graph.states.iter().map(graph_state)));
at: packages/escher-driver/src/mcp.rs:234
quote: Ok(outcome) => ToolResult::accepted(outcome.to_json()),
@row_add n=99 row=contract/ui-graph-line/content area=surface rec=fct-gnekz77j@1
boundary: The UI graph line
item: content
promise: The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a value or typed text.
@row_check n=99 row=contract/ui-graph-line/content sha=67a847e9387f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:274-282
quote: fn graph_node(node: &GraphNode) -> String { let mut object = Object::new(); object.text("id", &node.id); if let Some(owner) = &node.owner { object.text("owner", owner); } if let Some(component) = &node.component {
at: packages/dioxus-native-dom/src/graph.rs:85-86
quote: Self::Undeclared => "undeclared", Self::Unconfirmed => "unconfirmed",
@row_add n=100 row=contract/escher-session-commands/actions-arguments area=surface rec=fct-rqe4xefm@1
boundary: escher-session commands
item: actions arguments
promise: actions takes no argument; an argument is refused malformed.
@row_check n=100 row=contract/escher-session-commands/actions-arguments sha=226b28a1eb7d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:440
quote: args: &[],
at: packages/escher-driver/src/command.rs:324-325
quote: let Some((arg, slot)) = passed.iter_mut().find(|(arg, _)| arg.name == name) else { return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=101 row=contract/escher-session-commands/actions-answer area=surface rec=fct-rqe4xefm@1
boundary: escher-session commands
item: actions answer
promise: actions answers one uncoloured line of JSON {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; owner and cause left out when absent.
@row_check n=101 row=contract/escher-session-commands/actions-answer sha=498cd65d0d91 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
at: packages/escher-driver/src/json.rs:264
quote: entry.text("verb", reading.verb);
at: packages/escher-driver/src/json.rs:266
quote: entry.text("cause", cause.name());
at: packages/escher-driver/src/json.rs:511
quote: object.json("nodes", &list(nodes.iter().map(action_node)));
@row_add n=102 row=contract/actions-line/nodes area=surface rec=fct-rqe4xefm@1
boundary: The actions line
item: nodes
promise: Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with.
@row_check n=102 row=contract/actions-line/nodes sha=6a0221fb793d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:79
quote: self.focusable || self.interactive_role || self.listener
at: packages/dioxus-native-dom/src/actionable.rs:132
quote: node.owner.is_some()
at: packages/escher-driver/src/actions.rs:77
quote: let cause = match validate(&call) {
at: packages/escher-driver/src/actions.rs:78
quote: Err(refusal) => Some(refusal.cause()),
@row_add n=103 row=contract/actions-line/content area=surface rec=fct-rqe4xefm@1
boundary: The actions line
item: content
promise: The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId.
@row_check n=103 row=contract/actions-line/content sha=ddcd72f39574 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255-259
quote: object.text("id", &node.id); if let Some(owner) = &node.owner { object.text("owner", owner); } object.flag("keyed", node.keyed);
at: packages/escher-driver/src/json.rs:264-266
quote: entry.text("verb", reading.verb); if let Some(cause) = reading.cause { entry.text("cause", cause.name());
@row_add n=104 row=decision/actions-share-the-verbs-checks area=surface rec=fct-cu455asz@1
name: actions decided by the verbs' own checks
rule: The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts.
why: so a listed cause is the cause the call returns
@row_check n=104 row=decision/actions-share-the-verbs-checks sha=c11af2dd44c5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:77
quote: let cause = match validate(&call) {
at: packages/escher-driver/src/actions.rs:83
quote: Some(target) => check_target(harness, &command, target)
at: packages/escher-driver/src/execute.rs:224
quote: let (x, y) = check_target(harness, &command, &target)?.point();
@row_add n=105 row=contract/actions-line/stale area=surface rec=fct-cu455asz@1
boundary: The actions line
item: stale
promise: A node that resolves to no element reads stale.
@row_check n=105 row=contract/actions-line/stale sha=f03edf28e9da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:82
quote: None => Some(Cause::Stale),
@row_add n=106 row=contract/escher-session-commands/coverage-answer area=surface rec=fct-j6lb6pph@1
boundary: escher-session commands
item: coverage answer
promise: coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step.
@row_check n=106 row=contract/escher-session-commands/coverage-answer sha=61d7142e8ae9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:426
quote: "unexercised",
at: packages/escher-driver/src/json.rs:437-438
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX)); object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
at: packages/escher-driver/src/coverage.rs:4
quote: //! The UI graph is read at boot and after every replayed step, and every handler it shows on a
@row_add n=107 row=contract/coverage-line/content area=surface rec=fct-j6lb6pph@1
boundary: The coverage line
item: content
promise: The line carries ids, owner ids, event names and two counts only, owner left out when absent.
@row_check n=107 row=contract/coverage-line/content sha=e2930ccbf370 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:429
quote: entry.text("id", &handler.id);
at: packages/escher-driver/src/json.rs:433
quote: entry.text("event", &handler.event);
at: packages/escher-driver/src/json.rs:437
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX));
at: packages/escher-driver/src/json.rs:438
quote: object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
@row_add n=108 row=contract/escher-session-commands/coverage-arguments area=surface rec=fct-y7unb4g4@1
boundary: escher-session commands
item: coverage arguments
promise: coverage's one shape is coverage <app> <check>..., an app the binary boots then one or more check paths, with no --session and no --record; any other shape is a usage ending, status 2.
@row_check n=108 row=contract/escher-session-commands/coverage-arguments sha=983ac6572a30 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:185
quote: [app, checks @ ..]
at: packages/escher-driver/src/cli.rs:187
quote: && !checks.is_empty()
at: packages/escher-driver/src/cli.rs:197
quote: _ => Line::Usage,
@row_add n=109 row=contract/coverage-check-files/read-bound area=surface rec=fct-y7unb4g4@1
boundary: coverage check files
item: read bound
promise: Each check path is read bounded at 1,048,576 + 1 bytes and parsed strictly.
@row_check n=109 row=contract/coverage-check-files/read-bound sha=e04bb447cdee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:498
quote: let bound = u64::try_from(MAX_CHECK_BYTES).ok()?.saturating_add(1);
at: packages/escher-driver/src/cli.rs:501
quote: .and_then(|file| file.take(bound).read_to_end(&mut bytes))
at: packages/escher-driver/src/recording.rs:32
quote: pub(crate) const MAX_CHECK_BYTES: usize = 1_048_576;
@row_add n=110 row=contract/coverage-check-files/refusal area=surface rec=fct-y7unb4g4@1
boundary: coverage check files
item: refusal
promise: A file that cannot be read, one past the bound, one that is no check and a check of another app are each refused malformed, status 1, the refusal holding nothing of the path or the file.
@row_check n=110 row=contract/coverage-check-files/refusal sha=cf8eea8289f0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:465
quote: .map(|path| read_check(path).filter(|check| check.app() == app))
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
@row_add n=111 row=contract/native-config/wasm-font-context area=surface rec=fct-4our3qxs@1,fct-pso5k3lx@1
boundary: dioxus-native configuration
item: font context on WASM
promise: On WASM a font context with bundled fonts must be provided; `build_single_font_ctx` serves where one font is bundled.
@row_check n=111 row=contract/native-config/wasm-font-context sha=ca4f953105f4 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: must be provided · in packages/dioxus-native/src
searched: font_ctx · in packages/dioxus-native/src
why: no evidence found; the WASM requirement is stated only in the doc comment of with_font_ctx (config.rs:58-60) and no code enforces it; build_single_font_ctx exists (packages/blitz-dom/src/lib.rs:140)
@row_add n=112 row=contract/native-config/font-context area=typography rec=fct-pso5k3lx@1
boundary: dioxus-native configuration
item: FontContext
promise: A custom `FontContext` can be set.
@row_check n=112 row=contract/native-config/font-context sha=dac4d9dfc7a9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/config.rs:61-62
quote: pub fn with_font_ctx(mut self, font_ctx: FontContext) -> Self {         self.font_ctx = Some(font_ctx);
@row_add n=113 row=contract/driver-screenshot/fonts-unavailable area=typography rec=fct-okan27km@1
boundary: driver screenshot
item: fonts-unavailable
promise: A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
@row_check n=113 row=contract/driver-screenshot/fonts-unavailable sha=3860c7486805 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:519-520
quote: if !harness.base().can_paint_text() { return Err(Refusal::new(Cause::FontsUnavailable));
at: packages/blitz-dom/src/document.rs:1530-1534
quote: font_ctx .collection .generic_families(parley::fontique::GenericFamily::SansSerif) .next() .is_some()
@row_add n=114 row=contract/script-runtime/document-fonts area=typography rec=fct-5cgh22bd@1
boundary: script runtime DOM
item: document.fonts
promise: `document.fonts` is a stub FontFaceSet in which all fonts report as loaded.
@row_check n=114 row=contract/script-runtime/document-fonts sha=c5ca26099d61 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js:1
quote: // `document.fonts` (FontFaceSet) stub: all fonts report as loaded
at: packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js:5
quote: check: () => true,
at: packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js:15
quote: document.fonts = fontFaceSet;
@row_add n=115 row=decision/tests-built-with-system-fonts area=typography rec=fct-zmi56wtl@1
name: Tests built with system fonts
rule: The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace.
why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
@row_check n=115 row=decision/tests-built-with-system-fonts sha=cea1c863c555 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/Cargo.toml:17
quote: blitz-dom = { workspace = true, features = ["accessibility", "autofocus", "floats", "system-fonts", "text-transform-icu"] }
at: tests/blitz-tests/tests/br_trailing_line.rs:11-13
quote: //! NOTE: These tests measure real text so they require a usable font. Without //! the `system-fonts` feature (enabled by default when testing the whole //! workspace) text measures 0x0 and the assertions pass vacuously.
@row_add n=116 row=decision/root-relative-units-track-root-font area=typography rec=fct-dp73pjt4@1
name: Root-relative units track the root font
rule: rem and rlh units resolve against the root element's font size and line height, also after viewport and hidpi changes, with an initial root font-size of 16px.
why: The stylist device is seeded with the root's values so rem/rlh units do not fall back to the 16px default.
guard: rem_after_viewport_change
@row_check n=116 row=decision/root-relative-units-track-root-font sha=8882ad6f4623 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2738
quote: device.set_root_font_size(root_style.effective_zoom.unzoom(font_size.px()));
at: packages/blitz-dom/src/document.rs:2743
quote: device.set_root_line_height(root_style.effective_zoom.unzoom(line_height.px()));
guard_at: tests/blitz-tests/tests/rem_after_viewport_change.rs:50
@row_add n=117 row=absence/project-type-scale area=typography rec=fct-65pzcpcs@1
capability: A project type scale or font tokens
@row_check n=117 row=absence/project-type-scale sha=3462630343a5 v=holds judge=sonnet head=edd0cfaea7f0
searched: ^\s*--[a-z][a-z-]*\s*:|var\(--  · in packages/**/*.rs and packages/**/*.css
searched: font-family · in packages/, tests/, apps/ (*.rs, *.css): only UA defaults in default.css, browser/readme app styles and test fixtures; no project type scale or font tokens
@row_add n=118 row=decision/driver-scroll-instant area=motion-calibrated-to-expression-level-unrecorded rec=fct-tnwqvola@1,fct-vt52hpbo@1
name: Driver scroll is instant
rule: The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step.
why: Ratified.
@row_check n=118 row=decision/driver-scroll-instant sha=cbd456f7581d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/input.rs:240
quote: ScrollBehavior::Instant,
at: packages/escher-driver/src/execute.rs:283-285
quote: let (busy, diff, after) = settled_step(harness, seen, span, before, |harness| { harness.scroll_into_view(target.node) });
@row_add n=119 row=absence/reduced-motion-handling area=motion-calibrated-to-expression-level-unrecorded rec=fct-edwhene2@1,fct-vvuq2clh@1
capability: Reduced-motion handling
@row_check n=119 row=absence/reduced-motion-handling sha=db8732438498 v=holds judge=sonnet head=edd0cfaea7f0
searched: prefers-reduced-motion|reduced.motion|prefers_reduced (case-insensitive) · in packages/**, examples/**, tests/** (*.rs, *.css, *.toml)
@row_add n=120 row=decision/cursor-resolution area=iconography rec=fct-hvyxlxhq@1,fct-guxvotwt@1
name: Cursor resolution order
rule: The cursor is taken from the CSS cursor keyword, else Text for text inputs, Pointer inside links, Text over selectable text, and Default otherwise, a pointer over no box included, so the shell keeps a visible arrow there.
why: Taken from upstream's #1220, ratified.
@row_check n=120 row=decision/cursor-resolution sha=f323d7f48d3d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2817-2818
quote: if keyword != CursorKind::Auto {             return stylo_to_cursor_icon(keyword);
at: packages/blitz-dom/src/document.rs:2826
quote: return Some(CursorIcon::Text);
at: packages/blitz-dom/src/document.rs:2833
quote: return Some(CursorIcon::Pointer);
at: packages/blitz-dom/src/document.rs:2850
quote: Some(CursorIcon::Default)
@row_none n=1 rec=fct-rnr6ebdp@1 type=S
@row_none n=2 rec=fct-jbi2ij6e@1 type=S
@row_none n=3 rec=fct-xfodco37@1 type=S
@row_none n=4 rec=fct-dgxjrhfr@1 type=S
@row_none n=5 rec=fct-iawtqlzw@1 type=S
@row_none n=6 rec=fct-oolgokdo@1 type=S
@row_none n=7 rec=fct-r74vd227@1 type=S
@row_none n=8 rec=fct-a5ni2awv@1 type=S
@row_none n=9 rec=fct-tqd5dldd@1 type=S
@row_none n=10 rec=fct-3tbpfog4@1 type=S
@row_none n=11 rec=fct-uwbv2oqg@1 type=S
@row_none n=12 rec=fct-435ixehz@1 type=S
@row_none n=13 rec=fct-d35vvhfz@1 type=S
@row_none n=14 rec=fct-c6md4fhl@1 type=S
@row_none n=15 rec=fct-ii7p3j24@1 type=S
@row_none n=16 rec=fct-q73in4ta@1 type=9
@row_none n=17 rec=fct-atzpfc54@1 type=S
@row_none n=18 rec=fct-woyfbmnb@1 type=S
@row_none n=19 rec=fct-tsm6w3f3@1 type=S
@row_none n=20 rec=fct-bn2nrqdn@1 type=S
@row_none n=21 rec=fct-iqz2qfcv@1 type=S
@row_none n=22 rec=fct-b37lobif@1 type=9
@row_none n=23 rec=fct-fdzevwkk@1 type=S
@row_none n=24 rec=fct-drur5box@1 type=S
@row_none n=25 rec=fct-pe4pylgg@1 type=S
@row_none n=26 rec=fct-4cmypdo2@1 type=S
@row_none n=27 rec=fct-vt4vwm5c@1 type=9
@row_none n=28 rec=fct-w6dpo7qk@1 type=S
@row_none n=29 rec=fct-buuw5o72@1 type=S
@row_none n=30 rec=fct-ibq7wjp3@1 type=S
@row_none n=31 rec=fct-hm6l7tmv@1 type=S
@row_none n=32 rec=fct-heaaydxr@1 type=S
@row_none n=33 rec=fct-vjurj4wq@1 type=5
@row_none n=34 rec=fct-spadupwu@1 type=S
@row_none n=35 rec=fct-7ewroczb@1 type=S
@row_none n=36 rec=fct-xuneycqw@1 type=S
@row_none n=37 rec=fct-izgk733s@1 type=S
@row_none n=38 rec=fct-kdmcvdbg@1 type=5
@row_none n=39 rec=fct-p4bhbrip@1 type=S
@row_none n=40 rec=fct-5kn46jcr@1 type=5
@row_none n=41 rec=fct-nd7wvb2o@1 type=S
@row_none n=42 rec=fct-gnv6fqzu@1 type=5
@row_none n=43 rec=fct-riuyhmj7@1 type=S
@row_none n=44 rec=fct-j6ghrsj3@1 type=S
@row_none n=45 rec=fct-5nzdvlmo@1 type=S
@row_none n=46 rec=fct-5jmivcud@1 type=S
@row_none n=47 rec=fct-zjo5o5zs@1 type=5
@row_none n=48 rec=fct-kvxteum3@1 type=5
@row_none n=49 rec=fct-vjavizn2@1 type=S
@row_none n=50 rec=fct-4wznelig@1 type=5
@row_none n=51 rec=fct-3rkpcvos@1 type=5
@row_none n=52 rec=fct-ft5gyw4z@1 type=5
@row_none n=53 rec=fct-wfthjucl@1 type=5
@row_none n=54 rec=fct-ddqph55d@1 type=4
@row_none n=55 rec=fct-cyybiebp@1 type=9
@row_none n=56 rec=fct-ytbrguse@1 type=4
@row_none n=57 rec=fct-prgo5gtm@1 type=5
@row_none n=58 rec=fct-pez3luu5@1 type=5
@row_none n=59 rec=fct-yqrg4ucs@1 type=5
@row_none n=60 rec=fct-2nxzibsi@1 type=5
@row_none n=61 rec=fct-vjqihnw3@1 type=7
@row_none n=62 rec=fct-ciqmyfr7@1 type=7
@row_none n=63 rec=fct-cejbuz7z@1 type=7
@row_none n=64 rec=fct-ls6yfpy7@1 type=7
@row_none n=65 rec=fct-sf23gvry@1 type=7
@row_none n=66 rec=fct-y5s6tvsm@1 type=7
@row_none n=67 rec=fct-3ejwfsuo@1 type=4
@row_none n=68 rec=fct-blf3mieo@1 type=4
relied: Iframe sub-documents take the parent viewport's color scheme.
@row_none n=69 rec=fct-or5fa5av@1 type=4
@row_none n=70 rec=fct-hsrlbfqh@1 type=4
@row_none n=71 rec=fct-obnrxxei@1 type=4
@row_none n=72 rec=fct-ipmksf32@1 type=5
@row_none n=73 rec=fct-lgq6vpvz@1 type=5
@row_none n=74 rec=fct-hu6iqe45@1 type=S
@row_none n=75 rec=fct-ldhghls4@1 type=5
@row_none n=76 rec=fct-itdq3zc7@1 type=9
@row_none n=77 rec=fct-6tygk7ef@1 type=5
@row_none n=78 rec=fct-46r6qldw@1 type=5
@row_none n=79 rec=fct-jgwhhaoy@1 type=5
@row_none n=80 rec=fct-etnvkm2s@1 type=5
@row_none n=81 rec=fct-y4llv7yc@1 type=5
@row_none n=82 rec=fct-4zglaolr@1 type=5
@row_none n=83 rec=fct-sjgxjva6@1 type=5
@row_none n=84 rec=fct-tsfy6s4v@1 type=5
@row_none n=85 rec=fct-anwoxb7v@1 type=5
@row_none n=86 rec=fct-xp75h5al@1 type=5
@row_none n=87 rec=fct-nzvpimut@1 type=5
@row_none n=88 rec=fct-eh5atpfx@1 type=5
@row_none n=89 rec=fct-63vaqdm2@1 type=5
@row_none n=90 rec=fct-xumrd476@1 type=5
@row_none n=91 rec=fct-hr2o5w4y@1 type=5
@row_none n=92 rec=fct-icetjnaz@1 type=5
@row_none n=93 rec=fct-cftb623n@1 type=5
@row_none n=94 rec=fct-fb6jxps4@1 type=5
@row_none n=95 rec=fct-o4xmo4yy@1 type=5
@row_none n=96 rec=fct-u4hnumsv@1 type=5
@row_none n=97 rec=fct-iebm2tqh@1 type=5
@row_none n=98 rec=fct-jyxo2y24@1 type=5
@row_none n=99 rec=fct-pinupjh4@1 type=5
@row_none n=100 rec=fct-27f4t2jl@1 type=5
@row_none n=101 rec=fct-my2w5twf@1 type=4
@row_none n=102 rec=fct-p7icj7bo@1 type=4
@row_none n=103 rec=fct-rgwceom5@1 type=5
@row_none n=104 rec=fct-okx3zhns@1 type=5
@row_none n=105 rec=fct-y37cc5gk@1 type=5
@row_none n=106 rec=fct-uyr6iib2@1 type=4
@row_none n=107 rec=fct-f7eh6bug@1 type=5
@row_none n=108 rec=fct-o4oj77dg@1 type=5
@row_none n=109 rec=fct-tzzpnj2i@1 type=5
@row_none n=110 rec=fct-egb2doim@1 type=5
@row_none n=111 rec=fct-avcjl5al@1 type=5
@row_none n=112 rec=fct-bas4rvdu@1 type=5
@row_none n=113 rec=fct-3chvitmv@1 type=5
@row_none n=114 rec=fct-zlqdkahl@1 type=5
@row_none n=115 rec=fct-mrf3xpbp@1 type=5
@row_none n=116 rec=fct-xbtaw3h6@1 type=5
@row_none n=117 rec=fct-drq5raj7@1 type=7
@row_none n=118 rec=fct-e4jigdc2@1 type=5
@row_none n=119 rec=fct-slgkqo7y@1 type=5
@row_none n=120 rec=fct-cma4e7o6@1 type=5
@row_none n=121 rec=fct-ujkkm46d@1 type=5
@row_none n=122 rec=fct-zetftvqr@1 type=5
@row_none n=123 rec=fct-toyw5jkw@1 type=5
@row_none n=124 rec=fct-zsfmji3l@1 type=5
@row_none n=125 rec=fct-rqdjf62x@1 type=S
@row_none n=126 rec=fct-7f5ztzr3@1 type=5
@row_none n=127 rec=fct-germ3vvh@1 type=5
@row_none n=128 rec=fct-dl5cu6xg@1 type=5
@row_none n=129 rec=fct-a76rgemn@1 type=4
@row_none n=130 rec=fct-g6x7bhkj@1 type=4
relied: Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root.
@row_none n=131 rec=fct-z3qfoykp@1 type=4
relied: hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered.
@row_none n=132 rec=fct-dqdo2wfr@1 type=4
@row_none n=133 rec=fct-kywr7h4v@1 type=4
@row_none n=134 rec=fct-cbe7vaww@1 type=4
relied: The open select list paints after the root's render and the blend layers, so it stands above every box of the page.
@row_none n=135 rec=fct-vhukioye@1 type=4
@row_none n=136 rec=fct-2ssdivf2@1 type=4
@row_none n=137 rec=fct-4lb3fws4@1 type=4
@row_none n=138 rec=fct-eqb53qnu@1 type=4
@row_none n=139 rec=fct-q6ap2crc@1 type=4
@row_none n=140 rec=fct-3j5hjl7t@1 type=5
@row_none n=141 rec=fct-u225y7ui@1 type=4
@row_none n=142 rec=fct-wepbw2vn@1 type=5
@row_none n=143 rec=fct-y3cpbowe@1 type=5
@row_none n=144 rec=fct-mf3536c6@1 type=5
@row_none n=145 rec=fct-ki3c3guz@1 type=7
@row_none n=146 rec=fct-4aaray2i@1 type=7
@row_none n=147 rec=fct-x42hwfvk@1 type=7
@row_none n=148 rec=fct-4ih5wyju@1 type=7
@row_none n=149 rec=fct-keknjqam@1 type=4
relied: The favicon read returns the href of the first link element whose rel contains icon.
@row_none n=150 rec=fct-bbr7kf2x@1 type=5
@row_none n=151 rec=fct-zl77wswc@1 type=5
@row_none n=152 rec=fct-vtmtilnu@1 type=5
@row_none n=153 rec=fct-hxngvsgt@1 type=5
@row_none n=154 rec=fct-5mc57xdd@1 type=5
@row_none n=155 rec=fct-o2mv7754@1 type=7
@row_none n=156 rec=fct-tzidfeny@1 type=7
@row_none n=157 rec=fct-xkcpmz2a@1 type=5
@row_none n=158 rec=fct-wr73fuj5@1 type=4
relied: scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll.
@row_none n=159 rec=fct-zbk2czd3@1 type=4
relied: A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once.
@row_none n=160 rec=fct-vb77s7g2@1 type=4
@row_none n=161 rec=fct-z2ugu37c@1 type=4
@row_none n=162 rec=fct-enbza3q2@1 type=4
@row_none n=163 rec=fct-dxtawno3@1 type=4
relied: Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first.
@row_none n=164 rec=fct-6zlaghie@1 type=4
@row_none n=165 rec=fct-qpwq4lud@1 type=4
relied: A harness settle neither waits on nor advances any animating member and returns Settled with animating reading the document's flag.
@row_none n=166 rec=fct-5pxpwwyq@1 type=4
relied: The select list has no motion, so a screen read after a step reads the new state without an advance.
@row_none n=167 rec=fct-335wqsvp@1 type=4
relied: While an acting step waits on a load, the harness virtual clock stands still, so no timer or animation runs during the wait.
@row_none n=168 rec=fct-nukfvayp@1 type=4
@row_none n=169 rec=fct-5d7ppgng@1 type=4
@row_none n=170 rec=fct-kez2bs3i@1 type=5
@row_none n=171 rec=fct-72ty63qd@1 type=7
@row_none n=172 rec=fct-u66y3ree@1 type=5
@row_none n=173 rec=fct-vxrvvp5i@1 type=5
@row_none n=174 rec=fct-bbe5tqa5@1 type=5
@row_none n=175 rec=fct-at5inxcy@1 type=5
@row_none n=176 rec=fct-wixqum32@1 type=5
@row_none n=177 rec=fct-je4n53zx@1 type=5
@row_none n=178 rec=fct-bfdjqju7@1 type=5
@row_none n=179 rec=fct-hajfgs6d@1 type=5
@row_none n=180 rec=fct-6t7vmkf6@1 type=5
@row_none n=181 rec=fct-d3mide3k@1 type=S
@row_none n=182 rec=fct-3gkxk3ju@1 type=5
@row_none n=183 rec=fct-qizuazwl@1 type=5
@row_none n=184 rec=fct-anxjke24@1 type=4
@row_none n=185 rec=fct-7onentxb@1 type=4
@row_none n=186 rec=fct-dqhs7pwt@1 type=4
@row_none n=187 rec=fct-utzmagou@1 type=4
@row_none n=188 rec=fct-lwnjcsh2@1 type=4
relied: An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite.
@row_none n=189 rec=fct-wi4mqvjs@1 type=4
@row_none n=190 rec=fct-tmv65jfj@1 type=4
@row_none n=191 rec=fct-ogsz23id@1 type=4
@row_none n=192 rec=fct-e5awoxxj@1 type=5
@row_none n=193 rec=fct-sdf4zybd@1 type=5
@row_none n=194 rec=fct-tge57kfq@1 type=5
@row_none n=195 rec=fct-otn6nfiv@1 type=5
@row_none n=196 rec=fct-misq73if@1 type=5
@row_none n=197 rec=fct-z4dmblpa@1 type=4
relied: Every document gets the blitz default user-agent stylesheet once, through the default document config.
@row_none n=198 rec=fct-aboeejrv@1 type=4
relied: A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token.
@row_none n=199 rec=fct-cbwqjv4z@1 type=4
@row_none n=200 rec=fct-c73v2tqd@1 type=4
@row_none n=201 rec=fct-eao27mhf@1 type=4
@row_none n=202 rec=fct-jperyeod@1 type=4
@row_none n=203 rec=fct-epbt6wng@1 type=4
relied: A single select's list opens on a click, Enter, or a Space with no typed text standing.
@row_none n=204 rec=fct-ypgr7m5t@1 type=4
relied: A disabled, inert or hidden select, a select with no row to show and a multiple select open no list.
@row_none n=205 rec=fct-ih466day@1 type=4
relied: A focused select whose container lost its box keeps focus and takes a key with no panic.
@row_none n=206 rec=fct-d3xc3mih@1 type=4
@row_none n=207 rec=fct-3zxhbi2k@1 type=4
@row_none n=208 rec=fct-lyaafumo@1 type=4
relied: The logical viewport size is the physical window size divided by hidpi scale times zoom.
@row_none n=209 rec=fct-f4jytyaa@1 type=4
@row_none n=210 rec=fct-4ylmz5ex@1 type=5
@row_none n=211 rec=fct-qhvfmv23@1 type=4
@row_none n=212 rec=fct-qmikpmeb@1 type=4
relied: The shell sets the window title from the document's title node when it was set before the window existed.
@row_none n=213 rec=fct-zhr7gldu@1 type=4
@row_none n=214 rec=fct-y72ncq2q@1 type=4
@row_none n=215 rec=fct-n5dyjnzk@1 type=5
@row_none n=216 rec=fct-d47jlprw@1 type=4
@row_none n=217 rec=fct-x6nmnnxs@1 type=6
@row_none n=218 rec=fct-gvo7figj@1 type=4
@row_none n=219 rec=fct-omhgtsu2@1 type=4
@row_none n=220 rec=fct-t4zdxxh3@1 type=4
@row_none n=221 rec=fct-ul626qzb@1 type=4
@row_none n=222 rec=fct-pibvawow@1 type=4
@row_none n=223 rec=fct-h7373ed2@1 type=4
@row_none n=224 rec=fct-qlphyfkz@1 type=4
@row_none n=225 rec=fct-bzczuomv@1 type=9
@row_none n=226 rec=fct-wvdzkcqu@1 type=9
@row_none n=227 rec=fct-axilysek@1 type=9
@row_none n=228 rec=fct-cp64cmtj@1 type=5
@row_none n=229 rec=fct-cv7cmt4u@1 type=5
@row_none n=230 rec=fct-clshjihy@1 type=5
@row_none n=231 rec=fct-fbgjjqxv@1 type=5
@row_none n=232 rec=fct-iotxo33d@1 type=5
@row_none n=233 rec=fct-swhtsbga@1 type=5
@row_none n=234 rec=fct-biisvyv5@1 type=5
@row_none n=235 rec=fct-lsdv4mfo@1 type=5
@row_none n=236 rec=fct-cqaj3usi@1 type=5
@row_none n=237 rec=fct-4vcrqhgt@1 type=5
@row_none n=238 rec=fct-nrlvlxxd@1 type=5
@row_none n=239 rec=fct-mpgheext@1 type=5
@row_none n=240 rec=fct-pxd6cx5y@1 type=5
@row_none n=241 rec=fct-qgjffy5g@1 type=5
@row_none n=242 rec=fct-pvknxsve@1 type=5
@row_none n=243 rec=fct-xmoyplif@1 type=4
relied: WASM builds render sans-serif, serif, monospace and system-ui text with the bundled DejaVu Sans.
@row_none n=244 rec=fct-vdyur6r3@1 type=S
@row_none n=245 rec=fct-nk2xx5j6@1 type=6
@row_none n=246 rec=fct-plskesan@1 type=4
relied: List bullets can always draw from the bullet font, which the default font context always registers.
@row_none n=247 rec=fct-mrtzanbs@1 type=4
relied: An element with no generic font family is laid out as sans-serif.
@row_none n=248 rec=fct-wgirrymc@1 type=4
@row_none n=249 rec=fct-zjuxxuk2@1 type=4
@row_none n=250 rec=fct-va3exjt4@1 type=4
relied: Layout treats `line-height: normal` as 1.2 times the font size.
@row_none n=251 rec=fct-tigq2ssu@1 type=4
@row_none n=252 rec=fct-qrvvxmts@1 type=4
@row_none n=253 rec=fct-ua34srkn@1 type=4
relied: The `lang` attribute decides the locale text is shaped and broken with.
@row_none n=254 rec=fct-nwz2uowu@1 type=4
@row_none n=255 rec=fct-gcmoldbw@1 type=4
@row_none n=256 rec=fct-jjpum73a@1 type=4
@row_none n=257 rec=fct-fwngn32o@1 type=4
@row_none n=258 rec=fct-etlx3awr@1 type=4
relied: Every caller other than the select group header passes the default run paint and adds no synthesized bold.
@row_none n=259 rec=fct-ty77ehpa@1 type=4
@row_none n=260 rec=fct-n7jzo7rd@1 type=5
@row_none n=261 rec=fct-6xglreut@1 type=5
@row_none n=262 rec=fct-4ofq4qli@1 type=5
@row_none n=263 rec=fct-ft4lxsdx@1 type=5
@row_none n=264 rec=fct-o6ppnb4n@1 type=5
@row_none n=265 rec=fct-cj7mlii5@1 type=5
@row_none n=266 rec=fct-yjq7pu4h@1 type=7
@row_none n=267 rec=fct-csqoaj2v@1 type=7
@row_none n=268 rec=fct-yjvyslbr@1 type=7
