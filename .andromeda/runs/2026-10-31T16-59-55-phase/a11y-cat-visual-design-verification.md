# a11y · Visual Design Verification — category read

## Carried
- (.andromeda/a11y-plan.md:5) The opened list must meet the tier's three criteria: its painted colours SC 1.4.3, its keys SC 2.1.1 and its focus order SC 2.4.3.
  > **A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`
- (.andromeda/a11y-plan-detail/visual-design-verification.md:11) The list's option text must hold these thresholds under this one ratio, with no second formula, whatever colours the founder's fork picks.
  > `contrast_ratio` is the WCAG 2.x ratio over the `color` crate's relative luminance (Chrome's `GetContrastRatio`, blitz-paint's helper restated, blitz-paint untouched), and `TextContrast::required_ratio` asks 3.0 of large text (at least 24 px, or 14 pt at weight 700 or more) and 4.5 of other text.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:11) An open list's option must be readable by this walk, its background and text coming from computed styles rather than from paint alone, or its pair reads wrongly or as `None` and goes unmeasured.
  > the background behind it: each ancestor's `background-color` composited source-over down to the first opaque layer, else the root-else-`body` canvas over opaque white; it reads `None` where an element on that walk has `opacity` below 1, a non-empty `filter` or a `background-image` layer.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:11) The control-row count must stay true at boot, or move with its reading where the chunk adds a row for the stand's popup.
  > The stand check `stand_contrast` records one row per `controls(task)` row over the four lean tasks at boot (800 × 600 · scale 1 · Light · DejaVu Sans · offline), identical in both layout modes — 17 rows.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:12) Any pair the chunk measures on the list must also be witnessed against the painted pixel at this tolerance, since the list is painted by a new overlay.
  > the painted pixel matches the measured background within 1 per channel on all 16 measured rows.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:12) A row added for the list must be classified by this one reader, a disabled option exempt on exactly this condition and on no other.
  > Beside the record one shared reader classifies two more tables, its exempt class still exactly `enabled == Some(false)` from the snapshot and a `label[for=…]` row never exempt
- (.andromeda/a11y-plan-detail/visual-design-verification.md:12) An open list's plain, hovered, key-current and chosen options are colour states reached by input, so they must join this table with its count and breakdown restated; the existing `flight-type` readings must hold.
  > `STATE_ROWS`, 18 colour states reached by input — return-chosen 2 (`flight-type` and `flight-return-date`, each 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — pass 17 and exempt 1 (`flight-book` disabled, `#ffffff` on `#a0aec0`, 2.26), the lowest pass 5.11 (`flight-start` invalid, `#c53030` on `#fff5f5`).
- (.andromeda/a11y-plan-detail/visual-design-verification.md:13) A hover or active row on an option must be reached and read this way, through the same pointer steps.
  > `text_contrast` reads the declared `:hover` background after `move_mouse_to` and the `:active` one after `mouse_down_at`, the same in both layout modes
- (.andromeda/a11y-plan-detail/visual-design-verification.md:13) The list's rows must be asserted in both layout modes and run in these legs, so the claim of SC 1.4.3 stays true with the list open.
  > Three tests assert the three tables in both layout modes, so SC 1.4.3 is met on every measured non-exempt stand row; the check runs in the workspace `test` leg, per package and in the `a11y` leg (§9)
- (.andromeda/a11y-plan-detail/visual-design-verification.md:15) The lint's `contrast` reading must take an open list's options under this same rule and no other.
  > reported below `required_ratio()`, exempt exactly where the snapshot's `enabled` reads `Some(false)`, a `None` reading not measured — the exempt rule `stand_contrast` asserts, with no second formula or threshold
- (.andromeda/a11y-plan-detail/visual-design-verification.md:15) The stand at boot, every select closed, must keep reading no lint finding after the chunk.
  > The stand at boot reads no `contrast` finding on the four lean tasks in both layout modes, the nodes outside `stand_contrast`'s rows included — and no finding of any kind at boot, `at_boot` reading `[0, 0, 0, 0]`
- (.andromeda/a11y-plan-detail/visual-design-verification.md:15) These counts must be restated at the wrap to what the chunk's new tests measure.
  > the `a11y` leg reads 112 tests over 14 targets, `stand_contrast`'s 7 among them
- (.andromeda/a11y-plan-detail/visual-design-verification.md:16) The chunk's folded F9 CARRY restates this rule: an open list's options must read boxes of their own and be linted as such, a closed select's options must stay excluded, the read must stay on the element and never the role, and CRUD's rows and a `multiple` select's rows must keep their reading.
  > The driver's visual lint leaves a closed select's options out of its overlap reading in every document, the snapshot giving each such `ListBoxOption` its select's client rect: `overlaps` skips such an entry on either side, `Entry.in_closed_select` reading the element through `closed_select_of` and never the role, so CRUD's `role="option"` rows are not read as a select's and a `multiple` select's rows are linted as usual; the stand's flight booker at boot reads no `overlap` in both layout modes, `stand_lint`'s `the_stand_at_boot_reads_its_measured_findings` reading `[0, 0, 0, 0]`
- (.andromeda/a11y-plan-detail/visual-design-verification.md:17) The closed single select's layout must stay as this states when the list is shut; the list's own default box must be a new statement beside it and must not disturb it.
  > The user-agent single `select` lays out as an `inline-block` box holding its chosen option, a group not `hidden` reading `display: contents` and the chosen option under it `display: inline`, with the `input, textarea` border (`1px solid #999`), padding (`2px`) and background (`white`)
- (.andromeda/a11y-plan-detail/visual-design-verification.md:17) The `multiple` select's rows are a carve-out: they must stay as built, their own boxes unchanged by the list.
  > a `multiple` select lays out its groups and rows as blocks, each row (`padding: 0 2px`, `white-space: nowrap`) with a box of its own
- (.andromeda/a11y-plan-detail/visual-design-verification.md:17) A single select's user-agent pair must be measured once the list paints it, since this gap sits on the chunk's own surface.
  > that user-agent pair is not measured on a single select, while `select_interaction` measures a `multiple` select's plain and chosen rows at the user agent's pair
- (.andromeda/a11y-plan-detail/visual-design-verification.md:17) `select#flight-type` is the stand's popup: its opened list must meet 4.5 under this author CSS, and this pair and both its rows must keep their reading.
  > the stand's one select, `select#flight-type`, replaces it by author CSS (15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10), and `stand_contrast` reads its pair at 17.40, 15px 400, against 4.5, as a control row and as a return-chosen `STATE_ROWS` row
- (.andromeda/a11y-plan-detail/visual-design-verification.md:18) The engine colours fork must be set against this held precedent: a hardcoded default sheet pair, since the engine holds no tokens, measured in both layout modes; the `multiple` pair itself must not move unless his fork says so.
  > `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`, Chromium's inactive selected pair, hardcoded by design as the engine holds no tokens, 12.56:1 measured by `select_interaction` in both layout modes
- (.andromeda/a11y-plan-detail/visual-design-verification.md:34) The stand's select draws its focus by this border, so the focus fork must say what the select shows while its list is open, and the closed reading must hold.
  > the flight booker's `select#flight-type` dropping it for the `#3a5ce5` border as the date inputs do
- (.andromeda/a11y-plan-detail/visual-design-verification.md:38) The engine's one default focus indication stands here, so the fork on the list's current option must match it or state its departure.
  > blitz-dom default stylesheet gives inputs a 2px `#4D90FE` focus outline
- (.andromeda/a11y-plan-detail/visual-design-verification.md:52) The reduced-motion fork must be put knowing no such rule is observed on these surfaces: any opening or closing animation would need its own reduce override, and an instant list needs none.
  > Observed absent — prefers-reduced-motion and prefers-color-scheme rules · searched: `prefers-reduced-motion|prefers-color-scheme` over the 21 s02 files
- (.andromeda/a11y-plan-detail/visual-design-verification.md:59) If an open select takes the `OPEN` state, this rule must be restated to name it and `open_state`'s coverage must grow; if it does not, `:open` must stay on `details` and `dialog` alone and the plan must say a select does not match.
  > Author styles keyed on `:open` apply to an open `details` or `dialog` in every document, `:open` reading the element's `OPEN` state (§5 Focus restoration)
- (.andromeda/a11y-plan-detail/visual-design-verification.md:67) The engine colours fork must say whether the list's default colours follow the viewport's scheme, and the list must paint and hit true at any zoom.
  > The viewport carries a light/dark color scheme and a document zoom level (`1.0` unzoomed)
- (.andromeda/a11y-plan-detail/visual-design-verification.md:80) The options must stay in the tree and the snapshot both open and closed, under a group too; only their boxes change on opening.
  > a select's option excepted from `display: none` (§4 Hidden content), so a select's options are under it in the tree and the snapshot, directly or through an `optgroup`, the exception holding under a group
- (.andromeda/a11y-plan-detail/visual-design-verification.md:80) The list's hit path must honour `inert` first: an inert select or option must take no hit and open no list.
  > a hit answers nothing at an element carrying `inert`, before any other test, so a hit passes through it, and removing `inert` restores the tree and the hit
- (.andromeda/a11y-plan-detail/visual-design-verification.md:81) The overlay's hit path must keep this rule for an option or group in the list.
  > Elements with `visibility: hidden` or `collapse` are never hit-test targets
- (.andromeda/a11y-plan-detail/visual-design-verification.md:82) A select under such a transform must take no hit through its list either, so the overlay must not open a second route around `Node::hit_inner`'s answer.
  > A transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit
- (.andromeda/a11y-plan-detail/visual-design-verification.md:83) The overlay's hit path must keep this rule for the list's elements.
  > `pointer-events: none` makes an element transparent to hits while its descendants are still tested
- (.andromeda/a11y-plan-detail/visual-design-verification.md:85) Where an opened list scrolls and shows an overlay scrollbar, an option under that band takes no click by this rule, so a person's and a driver's choice there must allow for it.
  > Right after a `scroll`, a main-button pointerdown inside the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing
- (.andromeda/a11y-plan.md:178) No token exists to match for the list's focus ring, target size or type, so whatever the forks settle must be recorded as a measured value of the chunk.
  > NOT YET MEASURED — focus-ring, target-size and typography-readability tokens: no slice recorded them

## Bindings
- **Closed-option overlap rule ↔ §5 Keyboard handling and Focus restoration:** the line ending "no popup is painted" (.andromeda/a11y-plan.md:123) and "an option of a closed single select takes no click" (.andromeda/a11y-plan.md:113) must move with the overlap rule's restatement (.andromeda/a11y-plan-detail/visual-design-verification.md:16), one reading of a closed and an open select across the sections.
- **Options in the tree ↔ §4 Hidden content and §6's broad line:** the exception (.andromeda/a11y-plan-detail/visual-design-verification.md:80) is stated with (.andromeda/a11y-plan.md:91) and (.andromeda/a11y-plan.md:176); all three must hold together with the list open.
- **Contrast rows ↔ §3 Contrast verification harness and §9 CI:** a new row or test on the list moves the counts in (.andromeda/a11y-plan-detail/visual-design-verification.md:12) and (.andromeda/a11y-plan-detail/visual-design-verification.md:15) along with (.andromeda/a11y-plan.md:64) and the `a11y` job's fourteen files at (.andromeda/a11y-plan.md:241).
- **Group label painted ↔ §7 Accessibility tree output:** if the chunk paints a group's label, "named in the tree and unpainted" (.andromeda/a11y-plan.md:202) must be restated and that label's pair measured under the ratio at (.andromeda/a11y-plan-detail/visual-design-verification.md:11).
- **`:open` ↔ §2 builder reads:** the select's open state in styles (.andromeda/a11y-plan-detail/visual-design-verification.md:59) and in the tree through `aria-expanded` (.andromeda/a11y-plan.md:37) must not tell two stories.
- **Engine colours ↔ design system colour palette:** the new default pair is hardcoded beside (.andromeda/a11y-plan-detail/visual-design-verification.md:18), the engine holding no tokens, and must meet SC 1.4.3 on both sides.
- **Reduced motion ↔ design system motion:** (.andromeda/a11y-plan-detail/visual-design-verification.md:52) and (.andromeda/a11y-plan.md:168) bind with whatever the founder's fork settles for motion.
- **Focus indication ↔ layout templates' focus management:** the list's focus order and current-option indication bind with (.andromeda/a11y-plan-detail/visual-design-verification.md:34) and (.andromeda/a11y-plan-detail/visual-design-verification.md:38).
