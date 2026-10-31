group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:16   class: omission
  leaf-text: «with `#000000` text (12.56:1) | engine defaults |»
  source: .andromeda/design-system.md:30 «The open select list's colours are engine constants, the associated constants of `SelectListView` (`HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT`, `BORDER`), not design tokens»
  replace-with: «with `#000000` text (12.56:1); an open select list's colours are in no sheet but `SelectListView`'s engine constants, no design token: highlight `#c8c8c8` background with `#000000` text, disabled row text `#6d6d6d`, border `#999999` | engine defaults |»
- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «default generic size 16px (13px monospace);»
  source: .andromeda/design-system-detail/typography.md:54 «A group's `label` in an open select list paints as a header heavier than a row on every build»
  replace-with: «default generic size 16px (13px monospace); an open select list's row height is the select's font on one line, a larger authored font clipping, a standing limit, and a group's `label` there paints as a header heavier than a row on every build;»
- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «in every box and in the viewport (ratified by the founder, 2026-10-07);»
  source: .andromeda/design-system.md:123 «into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport»
  replace-with: «in every box and in the viewport (ratified by the founder, 2026-10-07); into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport, and the list has no motion and needs no `advance`;»
- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «the windowed path not run); since 2026-10-19-visual-lint»
  source: .andromeda/layout-templates.md:16 «seven_guis flight booker's mode list, the engine's select list in every document, remote pages included — a click, Enter, or a Space with no typed text standing opens `select#flight-type`'s list»
  replace-with: «the windowed path not run); the flight booker's mode list is the engine's select list in every document, remote pages included: a click, Enter, or a Space with no typed text standing opens it, `paint_scene` painting it after the root's render and before the devtools overlays, whole past an `opacity` ancestor or a box that clips its select, in no second OS window and with no row a layout box; it opens where its rows fit and else on the side with more room, at least the select's width, at most the viewport's and inside it, shows up to twenty rows and scrolls by key and wheel, its thumb painted, taking no hit and not draggable, a group's `label` a painted header that chooses nothing; Enter, Space, Tab or a click on a row chooses, and Escape, a second click, a click outside, the page scrolling or a wheel outside the list closes it with the value unchanged; focus stays on the select and the Tab sequence holds with the list open and after each close, Tab choosing the current row and closing before focus moves; while it is open each row reads its own `bounds` in the snapshot and the diff, the highlight in neither the snapshot nor the tree, and after a close the rows read back at the select's client rect; no stand markup, id, class or style moved; the windowed list read and not run; since 2026-10-19-visual-lint»
- leaf: .claude/docs/design-summary.md:27   class: stale
  leaf-text: «`overlaps` skipping on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's), the snapshot giving each such option its select's client rect and a `multiple` select's rows linted as usual»
  source: .andromeda/layout-templates.md:62 «`overlaps` skips on either side an entry with `Entry.in_select_box`, true for an option of a closed select (through `closed_select_of`, which answers `None` for a select whose list is open»
  replace-with: «`overlaps` skipping on either side an entry with `Entry.in_select_box`, true for an option of a closed select (through `closed_select_of`, which answers `None` for a select whose list is open, reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's) and for an open list's row or group, the snapshot giving a closed select's option its select's client rect and an open list's row its own `bounds`, a `multiple` select's rows linted as usual and `clip_region` taking an open row's region from `visible_region` alone, no `contain: paint` cut»
- leaf: .claude/docs/design-summary.md:36   class: retired
  leaf-text: «No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted,»
  source: .andromeda/design-system-detail/surface.md:14 «A single select's list is painted: a click, Enter, or a Space with no typed text standing opens it, a group's `label` is named in the accessibility tree and painted as a header»
  replace-with: «A single select's list is painted, an engine overlay reaching the screen through `paint_scene` alone and no second OS window: a click, Enter, or a Space with no typed text standing opens it, a group's `label` is named in the accessibility tree and painted as a header,»
- leaf: .claude/docs/design-summary.md:36   class: stale
  leaf-text: «and `size` is not read;»
  source: .andromeda/design-system-detail/surface.md:14 «`size` is read on a `multiple` select and stays unread on a single select»
  replace-with: «and `size` is read on a `multiple` select and stays unread on a single select;»
