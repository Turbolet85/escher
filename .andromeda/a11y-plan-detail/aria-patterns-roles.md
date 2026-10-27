# a11y-plan · ARIA Patterns & Roles
Read whole. The master's broad view is .andromeda/a11y-plan.md; this file holds its ARIA Patterns & Roles detail.

## 4. ARIA Patterns & Roles

<!-- detail: values · aria-landmarks -->

- Landmarks: nav→Navigation, main→Main, aside→Complementary, footer→Footer, article→Article, blockquote→Blockquote, figure→Figure (tests/blitz-tests/tests/accessibility_roles.rs:74-93)
- header→Header, section→Section (tests/blitz-tests/tests/accessibility_roles.rs:157-180)

<!-- /detail -->

<!-- detail: values · aria-patterns -->

- **ARIA `role` attribute** — `role_from_name` maps ARIA role names (alert, button, checkbox, dialog, link, tab, textbox, landmark roles, etc.) to AccessKit roles (packages/blitz-dom/src/accessibility.rs:330-393)
- **ARIA state attributes** — `aria-selected` → `selected`, `aria-pressed` → `toggled` (a toggle button; `mixed` kept as Mixed), `aria-invalid` → `invalid` (True / Grammar / Spelling; a value outside the list reads True), read by value on any element — a select's option excepted, its `selected` its chosen state (packages/blitz-dom/src/accessibility.rs:171-181) — since 2026-10-10-snapshot-states; `aria-checked` → `toggled` (`true` · `false` · `mixed`) on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone, `aria-pressed` read on every other role, a native checkbox or radio reading neither, and `role="switch"` mapping to `Switch` (ratified by the founder, 2026-10-18) — `accessibility_states`' fourth test its pin (packages/blitz-dom/src/accessibility.rs:169-213; tests/blitz-tests/tests/accessibility_states.rs:134-151; tests/blitz-tests/tests/accessibility_states.rs:153-171; tests/blitz-tests/tests/accessibility_states.rs:173-192; tests/blitz-tests/tests/accessibility_states.rs:245-316; packages/blitz-dom/src/accessibility.rs:395-415) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md
- **Button** — button→Button, submit→Button (tests/blitz-tests/tests/accessibility_roles.rs:157-180; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
- **Link** — `<a>` is a Link only with `href`, else GenericContainer (packages/blitz-dom/src/accessibility.rs:475-492; tests/blitz-tests/tests/accessibility_roles.rs:117-126)
- **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:475-492; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
- **Lists and tables** — ul/ol→List, li→ListItem, table→Table, thead→RowGroup, tr→Row, th→ColumnHeader, th scope=row→RowHeader, td→Cell (tests/blitz-tests/tests/accessibility_roles.rs:95-115); `<th>` is RowHeader for `scope=row|rowgroup`, else ColumnHeader (packages/blitz-dom/src/accessibility.rs:475-492)
- **Form input** — `<input>` roles are mapped by `type`, defaulting to TextInput (packages/blitz-dom/src/accessibility.rs:509-527); label→Label, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput (tests/blitz-tests/tests/accessibility_roles.rs:128-155); text→TextInput, number→NumberInput, checkbox→CheckBox (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
- **Generic and text** — div→GenericContainer, h2→Heading, p→Paragraph (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
- **Hidden content** — `aria-hidden="true"` keeps the node but marks it hidden (`is_hidden`); `hidden`, `display: none` and `visibility: hidden` exclude it — an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree, `hidden`, `inert` and `visibility: hidden` still excluding it (packages/blitz-dom/src/accessibility.rs:257-258; packages/blitz-dom/src/accessibility.rs:260-273); children of a hidden element are excluded (packages/blitz-dom/src/accessibility.rs:165-168; tests/blitz-tests/tests/accessibility_hidden.rs:28-137)

<!-- /detail -->

<!-- detail: values · aria-patterns -->

- Browser controls are clickable `div`s rather than buttons: icon buttons, tabs, tab close, new tab, menu items (apps/browser/src/icons.rs:28-39; apps/browser/src/tab_strip.rs:83-106; apps/browser/src/toolbar.rs:419-441)
- Images: new-tab logo has `alt: "Blitz"`, favicons `alt: ""`, `IconButton` images no alt (apps/browser/src/about_pages.rs:86; apps/browser/src/tab.rs:289; apps/browser/src/icons.rs:38)
- rdme's stylesheet styles `[role=button]` and `[role=tabpanel]` focus states (apps/readme/assets/github-markdown.css:340-346; apps/readme/assets/github-markdown.css:1081)

<!-- /detail -->

<!-- detail: values · aria-patterns -->

- The captured GitHub fixtures carry `aria-hidden`, `role="tooltip"`, `role="img"` with `aria-label`, and `aria-labelledby` (examples/assets/github_profile_reduced2.html:65; examples/assets/github_profile_reduced2.html:68; examples/assets/github_profile_reduced2.html:78)
- The Google fixture carries `role="contentinfo"` (examples/assets/bottom_only.html:14)
- The google fixture's query textarea is a combobox with aria-autocomplete=both, aria-controls/aria-owns Alh6id and aria-haspopup=both (examples/assets/google.html:3184-3199)
- The google fixture uses role=listbox and role=option for suggestions, and role=menu, menuitem and separator for its settings menu (examples/assets/google.html:3294; examples/assets/google.html:3318; examples/assets/google.html:3667; examples/assets/google.html:3680; examples/assets/google.html:3753)
- The google fixture marks landmark regions with role=navigation, role=search and role=contentinfo (examples/assets/google.html:3027; examples/assets/google.html:3134; examples/assets/google.html:3583)
- The servo-new fixture's hamburger button carries aria-label and aria-expanded, which its script flips on click (examples/assets/servo-new.html:33; examples/assets/servo-new.html:232-239)
- The servo-new fixture labels nav and sections with aria-label (examples/assets/servo-new.html:25; examples/assets/servo-new.html:244; examples/assets/servo-new.html:300; examples/assets/servo-new.html:338; examples/assets/servo-new.html:348)
- The graphite fixture's carousel buttons carry aria-label text (examples/assets/graphite.html:1892; examples/assets/graphite.html:1898; examples/assets/graphite.html:1899)
- servo.html fixture: `nav` with `role="navigation" aria-label="main navigation"`; burger `a` with `role="button" aria-label="menu" aria-expanded="false"` and `aria-hidden` bars (examples/assets/servo.html:35; examples/assets/servo.html:42-45)
- servo.html fixture download links carry `role="button"` (examples/assets/servo.html:280; examples/assets/servo.html:287)
- svg.html fixture icons use `aria-hidden="true"` and `focusable="false"` (examples/assets/svg.html:4; examples/assets/svg.html:6; examples/assets/svg.html:10)

<!-- /detail -->

<!-- detail: absence · aria-patterns -->

- ARIA attributes or roles in app/example markup · searched: `aria-|role:|"role"|role=` over the 86 s04 slice files excluding github-markdown.css and default.css
- ARIA attributes or roles · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files
- ARIA attributes or roles · searched: `aria|accessib|role` over the 8 s07 slice files (only match is a paint-tree local `role_unchanged`)
- ARIA attributes or roles · searched: `accesskit|aria|role` over the 16 listed s08 files (only commented accesskit lines matched)
- ARIA attributes · searched: `aria-` over the 32 listed s09 files
- ARIA attributes or roles · searched: `aria-|aria[A-Z]|\brole\b` over the 32 s10 slice files
- ARIA attributes or roles · searched: `aria|role|accesskit` over the 21 listed s11 files
- ARIA attributes or roles · searched: `aria-|role=` over the 12 listed s13 files

<!-- /detail -->
