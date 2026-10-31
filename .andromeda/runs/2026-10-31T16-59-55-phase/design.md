# design extract

## Relevance
relevant — the chunk paints a new engine overlay (colours, text, depth, hit, possible motion) in every document; the plan binds it through engine-default rows only, since no project token set, type scale, spacing scale, elevation scale or radius scale is recorded.

## Constraints
- design-system §Color Palette → Token sets records no colour tokens or custom properties in the engine and integration crates, so the list has no `--color-*` to cite: its colours are new engine defaults, and their values are the founder's P4 fork (scope, "The CARRYs folded"), not this extract's to pick.
- design-system §Color Palette requires the system colour scheme (`Light` default / `Dark`) to feed the viewport and resolve at cascade; whether the list's colours follow the scheme is part of the colour fork, and how `default.css` or paint can carry that is research's question.
- design-system §Typography (`blitz-dom default form controls` row) fixes the family for selects and states no size or weight; the list's option text and any painted group label are held to that row's family, and their size is research's question against what the closed select computes.
- design-system §Typography → Loading requires a driver `screenshot` to refuse `fonts-unavailable` rather than paint text skipped, and requires `blitz-dom` built with `system-fonts` in blitz-tests so that a measured option box is not 0x0 and vacuous; on WASM a bundled font context is required for the same text (per design-system §Surface: web-spa → Tokens).
- design-system §Depth Strategy → Engine depth behavior requires paint order and hit-test order to mirror each other (hoisted and paint children, positive z first when hitting) and stops hits at the padding box of a node paint clips, the root excepted; a list placed in the tree under a clipping ancestor would be neither shown nor hit outside it, so how the overlay escapes clipping is research's question and may be a fork.
- design-system §Surface: desktop-native → Tokens names the sheet rules a closed single select lays out by (hidden options, chosen option inline, a group `display: contents`) and those for `multiple` rows; the open list must leave both readings intact when closed and for `multiple`. Its "No select popup is painted" row names this entry its owner, so that row (group `label` unpainted, `size` unread, Enter and Space opening nothing) is this chunk's to restate at wrap; whether the code still reads so is research's question.
- design-system §Motion → Reduced motion records reduced-motion handling as observed absent, and design-system §Motion → Animation runtime requires `click`, `type` and `press` to leave the `VirtualClock` where it was and settle not to wait on any animation; any open or close motion would therefore not have run when a driver step returns. Reduced motion is the founder's P4 fork (scope).

## Patterns to follow
- A hardcoded engine-default colour pair, sourced to a named reference and with its contrast ratio measured by a test, per design-system §Color Palette → Core Colors (`blitz-dom default chosen row`).
- Select styling as user-agent sheet rules keyed on `select:not([multiple])` and `select[multiple]`, reaching every document through `DEFAULT_CSS`, per design-system §Surface: desktop-native → Tokens; rows and their padding per design-system §Spacing → Untokenized values (`blitz-dom default multiple select rows`).
- An engine-painted overlay with colours chosen per viewport scheme, painted as fill plus contrast stroke, per design-system §Color Palette → Engine-painted defaults (default scrollbar thumbs); and one that takes a pointerdown in its own band before dispatch, per design-system §Motion → This project's values (`hit_with_scrollbar`). The same row warns that a driver `click` in a shown scrollbar's band lands on the scrollbar, which binds a long list that scrolls.
- A highlighted option brought into view is written at once in a nested scrolling box, per design-system §Motion → This project's values (programmatic into-view scroll).
- The cursor over the list follows the CSS `cursor` keyword, else Default, with no new cursor rule, per design-system §Iconography → Cursors and favicons.

## Anti-patterns to avoid
- Introducing CSS custom properties or a token layer into `default.css` for the list: the engine holds none, per design-system §Color Palette → Token sets. The focus guide's "uses only design tokens" check does not apply here for that reason.
- Borrowing app or stand values as engine defaults: browser chrome's menu shadow and z-index (design-system §Depth Strategy → Observed values in the apps and examples), its radii (design-system §Border Radius, `Browser chrome (untokenized)` row) and its focus colour (design-system §Color Palette → Core Colors, `Browser chrome focus` row) belong to their apps; the stand's select values are held in layout-templates, per design-system §Color Palette → Core Colors (`seven_guis stand` row).
- Settling colours, reduced motion or the key contract in the plan ahead of his P4 answer: design-system §Motion → Reduced motion records nothing to inherit, and scope hands all three to the founder.

## Contract bindings
- Token contrast ↔ a11y §Contrast (SC 1.4.3, 4.5:1): each list text and background pair, the highlighted and chosen rows included; the stand's `stand_contrast` table moves with any new row (scope, "Surfaces and boundaries").
- Motion ↔ a11y §Animation (SC 2.3.3): any list animation the fork admits carries a reduce-motion override; the engine records none today per design-system §Motion → Reduced motion.
- State colour ↔ a11y §Use of Color (SC 1.4.1): highlighted and chosen options are told apart by more than colour; whether option text alone suffices or a mark is needed goes with the colour fork. The plan's §Color Palette → Semantic Colors holds no row for these states.
- Stand ↔ layouts: the flight-type select's border and radius are held in layout-templates §Surface: desktop-native → Primary screens, per design-system §Border Radius (`seven_guis stand` row); the stand's popup look is read there.
- Driver surface ↔ architecture: restated `covered` schema text and lint wording stay inside the uncoloured one-line JSON the driver answers, per design-system §Surface: cli → Component Patterns.

## Acceptance criteria contributions
- (design) Each text and background pair the open list paints is a hardcoded engine default from the founder's colour fork, adds no CSS custom property, and has its ratio measured by a test and recorded on the plan's palette, at or above 4.5:1 (per design-system §Color Palette → Core Colors).
- (design) With a select inside an `overflow: hidden` box, each option of the open list is hit at the place it paints, and a point the list does not cover answers what lies beneath as paint shows it (per design-system §Depth Strategy → Engine depth behavior).
- (design) A driver `screenshot` of an open list shows its option labels in the form-control family with non-zero option boxes in a `system-fonts` test build, and an app with no `sans-serif` family stays refused `fonts-unavailable` (per design-system §Typography → Loading).
- (design) A driver step that opens or closes the list reads the new state in its next `snapshot` with no `advance` between, the `VirtualClock` unmoved (per design-system §Motion → Animation runtime).
