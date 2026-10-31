# Apply — design-system · part 2 of 2

## Verdicts
- fanout-design-system--cascade-g2.md 7 · applied · .andromeda/design-system-detail/motion.md:9+#1
- fanout-design-system--cascade-g2.md 8 · applied · .andromeda/design-system-detail/motion.md:12#1
- fanout-design-system--cascade-g2.md 10 · applied · .andromeda/design-system-detail/motion.md:28+#1
- fanout-design-system--cascade-g2.md 11 · applied · .andromeda/design-system-detail/surface.md:11+#1
- fanout-design-system--cascade-g2.md 12 · applied · .andromeda/design-system-detail/surface.md:13#1
- fanout-design-system--cascade-g2.md 13 · applied · .andromeda/design-system-detail/surface.md:13+#1
- fanout-design-system--cascade-g2.md 14 · applied · .andromeda/design-system-detail/surface.md:13+#2
- fanout-design-system--cascade-g2.md 15 · applied · .andromeda/design-system-detail/surface.md:13+#3
- fanout-design-system--cascade-g2.md 16 · applied · .andromeda/design-system-detail/surface.md:13+#4
- fanout-design-system--cascade-g2.md 17 · applied · .andromeda/design-system-detail/surface.md:13+#5
- fanout-design-system--cascade-g2.md 18 · applied · .andromeda/design-system-detail/surface.md:13+#6
- fanout-design-system--cascade-g2.md 19 · applied · .andromeda/design-system-detail/surface.md:63+#1
- fanout-design-system--cascade-g2.md 20 · applied · .andromeda/design-system-detail/surface.md:13+#7
- fanout-design-system--cascade-g2.md 21 · applied · .andromeda/design-system-detail/surface.md:13+#8

## Edits
- after: .andromeda/design-system-detail/motion.md:9
  anchor: «- Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:112-122; packages/blitz-dom/src/scrolling.rs:432-433); `scroll-behavior: smooth` makes `Auto` scrolls animate and `auto` behavior jumps (packages/blitz-dom/src/scrolling.rs:528-545; tests/blitz-tests/tests/fragment_navigation.rs:264-293; tests/blitz-tests/tests/fragment_navigation.rs:374-383); a wheel event over a scroller cancels an in-progress smooth scroll (tests/blitz-tests/tests/fragment_navigation.rs:356-372)»
  add: «- A wheel over the open select list scrolls the list; a wheel elsewhere closes the list and then scrolls; a key brings the current row into view at once (packages/blitz-dom/src/document.rs:947-960; packages/blitz-dom/src/scrolling.rs:669-672; tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/design-system-detail/motion.md:12
  old: «applies to the viewport alone; the driver's»
  new: «applies to the viewport alone; on a row of an open select list `scroll_into_view` writes that list's `scroll_y` at once and returns, moving no chain and no viewport (packages/blitz-dom/src/scrolling.rs:669-672; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); the driver's»
- after: .andromeda/design-system-detail/motion.md:28
  anchor: «- The shell's animation clock is seconds since the first animation-time query, and frames keep redrawing while the document is animating (packages/blitz-shell/src/window.rs:281-289; packages/blitz-shell/src/window.rs:438-440); the test harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance` (packages/blitz-test-harness/src/harness.rs:152-174), and settle reads animation time only through that clock, never advancing it; the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was: a driver click beside a running CSS animation returns settled without waiting on it (tests/blitz-tests/tests/stand_act_timer.rs:1-12; tests/blitz-tests/tests/stand_virtual_clock.rs:50-96 — as measured at escher-0.2.0/chunks/2026-10-18-one-virtual-clock/report.md)»
  add: «- The select list has no motion and needs no `advance`: a screen read after a step reads the new state (tests/blitz-tests/tests/stand_act_popup.rs:408-423; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:11
  anchor: «- In that sheet the rule `select[multiple] > option:checked, select[multiple] > optgroup > option:checked` colours a `multiple` select's chosen row — its pair and ratio at §Color Palette → Core Colors, the `blitz-dom default chosen row` row (packages/blitz-dom/assets/default.css:161-199)»
  add: «- The open select list's colours are in no sheet: the highlight's text and background, a disabled row's text and the box's border are associated constants on `SelectListView` — engine constants, no design token; their values at §Color Palette → Core Colors (packages/blitz-dom/src/node/select_list.rs:110-123; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/design-system-detail/surface.md:13
  old: «No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «A single select's list is painted: a click, Enter, or a Space with no typed text standing opens it, a group's `label` is named in the accessibility tree and painted as a header, and `size` is read on a `multiple` select and stays unread on a single select (docs/capabilities.md:135-140; packages/blitz-dom/src/node/select_list.rs:194-208; packages/blitz-dom/src/stylo.rs:1124-1139; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- On desktop-native a single select's list is an engine overlay reaching the screen through `paint_scene` alone, not a second OS window and not rows as boxes; the windowed list stands read and not run (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- A single select's list opens on its chosen option with no `input` or `change`; keys and the pointer move the highlight over enabled rows and choose nothing (packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/pointer.rs:595-611; tests/blitz-tests/tests/select_popup.rs:202-230; tests/blitz-tests/tests/select_popup.rs:249-273; tests/blitz-tests/tests/select_popup.rs:751-781; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- Enter, Space, Tab in either direction or a click on a row chooses the row with one `input` then one `change`; a disabled row changes nothing and leaves the list open; a choice of the chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/select_popup.rs:363-375; tests/blitz-tests/tests/select_popup.rs:377-388; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no `input` or `change`; removing, disabling, hiding or making its select inert closes it too (packages/blitz-dom/src/document.rs:881-907; packages/blitz-dom/src/events/pointer.rs:619-633; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:478-520; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- A disabled, inert or hidden select, or one with no row to show, opens nothing; a `multiple` select opens no list; a label's click focuses a single select and opens nothing (tests/blitz-tests/tests/select_popup.rs:428-441; tests/blitz-tests/tests/select_popup.rs:443-472; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- In the list a group reads as a header with its options under it, and a header chooses nothing; a group with no label reads the whole list's box (tests/blitz-tests/tests/select_popup.rs:805-855; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:63
  anchor: «- The optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:49)»
  add: «- A scrolled select list paints a thumb under `feature = "scrollbars"` that takes no hit and is not draggable; keys, the wheel and `scroll` move the rows (packages/blitz-paint/src/render/select_list.rs:130-154; tests/blitz-tests/tests/select_popup_painted.rs:312-351; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- The highlighted row is in no snapshot and no tree; the tree reads the open select `ComboBox` and `expanded`, and its chosen option `selected`; an author's `open` steers nothing on a select (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/surface.md:13
  anchor: «- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «- A row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it and leaves the list open (packages/dioxus-native-dom/src/snapshot.rs:152-155; packages/blitz-dom/src/document.rs:995-1012; tests/blitz-tests/tests/stand_act_popup.rs:287-316; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»

## Patterns
[[pattern]]
id = "ds2-01"
regex = 'No select popup is painted|Enter and Space open nothing|left unpainted'
ignore_case = true
