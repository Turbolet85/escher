## 1. A11y Scope Summary

**A11y tier:** `0 — SC 2.1.1 · 1.4.3 · 2.4.3`

**A11y scope (entities needing assertions):**
- **Workspace AccessKit dependency** — accesskit "0.25" and the in-repo `accesskit_xplat` package are workspace dependencies/members (Cargo.toml:3; Cargo.toml:58; Cargo.toml:143)
- **accesskit_xplat** — provides the AccessKit platform adapter crate (packages/accesskit_xplat/Cargo.toml:2-6)
- **blitz-dom accessibility tree** — under the `accessibility` feature, `BaseDocument::build_accessibility_tree` builds an AccessKit tree from the DOM (packages/blitz-dom/src/lib.rs:83-84; packages/blitz-dom/src/accessibility.rs:5-62); blitz-dom's default features include `accessibility`, which enables `accesskit`, and `custom-widget` also requires it (packages/blitz-dom/Cargo.toml:14-23; packages/blitz-dom/Cargo.toml:28)
- **Custom widgets** — a custom widget accessibility-tree hook is commented out as a TODO (packages/blitz-dom/src/node/custom_widget.rs:6; packages/blitz-dom/src/node/custom_widget.rs:142-143)
- **blitz-html** — its `accessibility` feature forwards to `blitz-dom/accessibility` (packages/blitz-html/Cargo.toml:14)
- **blitz-shell** — accessibility is a default feature enabling accesskit, accesskit_xplat and blitz-dom/accessibility (packages/blitz-shell/Cargo.toml:14-19); each View holds an AccessibilityState wrapping an accesskit_xplat Adapter (packages/blitz-shell/src/window.rs:111-113; packages/blitz-shell/src/accessibility.rs:12-16)
- **blitz** — has an `accessibility` feature, on by default, forwarding to `blitz-shell/accessibility` (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:16)
- **blitz-traits node ids** — `NodeId::as_u64` is described as useful for interop with integer-id APIs such as AccessKit (packages/blitz-traits/src/node_id.rs:15-22)
- **Dioxus crates** — `accessibility` is a default feature of both Dioxus crates; dioxus-native-dom's forwards to blitz-dom and enables its own optional `accesskit`, and dioxus-native's forwards to blitz-dom, blitz-shell and dioxus-native-dom (packages/dioxus-native-dom/Cargo.toml:13; packages/dioxus-native-dom/Cargo.toml:18; packages/dioxus-native-dom/Cargo.toml:41; packages/dioxus-native/Cargo.toml:13; packages/dioxus-native/Cargo.toml:32); the workspace takes both with `default-features = false`, so a crate gets the feature only by naming it or by depending on a crate that names it — the seven_guis stand binary names none in its own manifest and builds no platform adapter (Cargo.toml:55-56; examples/seven_guis/Cargo.toml:22; examples/seven_guis/Cargo.toml:31) — as measured at escher-0.1.0/chunks/2026-10-06-accessibility-tree-identity/report.md; the session library `escher-driver`, which seven_guis depends on since 2026-10-07-driver-session, names `accessibility` on `dioxus-native-dom` since 2026-10-07-act-by-id — unconditionally, with no `[features]` table of its own: its executor reads the snapshot, which stands behind that feature — so the feature reaches seven_guis through that edge although seven_guis' own manifest still names none, and both its binaries, the windowed `seven_guis_native` and the `escher-session` host, now compile dioxus-native-dom's accessibility modules (the `accessibility_tree` override that carries `author_id`, the snapshot model, its text and its diff); neither builds a platform adapter — `accesskit_xplat` and `accesskit_winit` are in neither graph, `accesskit` was already in both through blitz-dom, and the lockfile did not change at that chunk — so no assistive technology is given a tree by the stand binary still, and the windowed stand boots as before; the library's third dependency, `tracing`, taken at 2026-10-07-driver-command-spans with no feature, adds one line to the lockfile and no package, and neither platform adapter joins either graph (packages/escher-driver/Cargo.toml:13-16; examples/seven_guis/Cargo.toml:35; as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md, the feature and its reach at escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/feature-reach.md, on the dev host's target, the `tracing` dependency at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md); re-read at 2026-10-10-upstream-sync-agent-surfaces on the merged lock, where the accesskit family moved — accesskit 0.25.1, accesskit_unix 0.24.0, accesskit_android 0.9.0: the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal`, 0 rows) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
- **Browser app** — the browser's `accessibility` feature (dioxus-native accessibility) is not in its default set (apps/browser/Cargo.toml:14; apps/browser/Cargo.toml:39)
- **blitz-tests** — `blitz-dom` and `dioxus-native-dom` are built with the `accessibility` feature and `accesskit` is a dev-dependency (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/Cargo.toml:21; tests/blitz-tests/Cargo.toml:33)
- **Input fixture** — exercises focusable divs with tabindex 0 and tabindex -1 beside a text input (examples/assets/input.html:4-9)

**A11y surfaces & assistive tech reach:** the AccessKit platform adapters and their per-platform gaps are recorded in §2 (Platform adapters).

**A11y assertion harness specification:** accessibility tests cover hidden-element exclusion and HTML-to-AccessKit role mapping (tests/blitz-tests/tests/accessibility_hidden.rs:8-137; tests/blitz-tests/tests/accessibility_roles.rs:1-7); the harness is detailed in §3.

**Observed absent (scope searches):**
- ARIA attributes in the Rust examples · searched: `role=|aria-|tabindex` over the 18 examples/*.rs files in the s03 slice
- accessibility tree, ARIA or screen-reader code · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files
- accessibility handling · searched: `aria|accessib` over the 8 s07 slice files
- ARIA, roles or keyboard handling · searched: `aria-|role=|keyboard|focus` (case-insensitive) over the 12 listed s13 files; the only hits are the quarantine reason `focus-events` and a quarantined test path (wpt/runner/src/test_runners/mod.rs:412; wpt/runner/src/test_runners/mod.rs:438)

> NOT YET MEASURED — must-be-accessible critical paths and a11y triggers: no slice recorded them, and s03 recorded accessibility scope as out of slice.

---

## 2. A11y Strategy

**Accessibility tree lifecycle:**
- The accessibility tree is built from the document on InitialTreeRequested and refreshed on change: on a poll that reported work the shell's `View::poll` takes the document's changed set (`BaseDocument::take_changed_nodes` — outside the `accessibility` cfg, so a shell built without the feature still drains it) and, under `accessibility`, rebuilds the platform tree when the set it took was non-empty; both paths go through the `Document::accessibility_tree` trait method, so a wrapper's override (DioxusDocument's stable ids) reaches the platform tree (packages/blitz-shell/src/application.rs:77-83; packages/blitz-shell/src/window.rs:376-382; packages/blitz-shell/src/window.rs:521-524; packages/blitz-shell/src/accessibility.rs:44-46; packages/blitz-dom/src/document.rs:152-160). Before 2026-10-07-change-tracking-and-diff the poll-time refresh did not run — the flag the poll read, `has_changes()`, read false after an in-document write on a document nothing had drained, and nothing drained the set — so the platform tree was built once (as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/evidence/control-changed-set.txt). The refresh on change is ratified by the founder (2026-10-07) — one item with the engine's changed-set contract in the next bullet. Its windowed witness is still owed: no windowed run witnesses the refresh on the dev host — its AT-SPI bus reads inactive, and the stand's windowed binary builds blitz-shell without `accessibility` — so it is proven at the document level (the `changed_set_` unit tests) and by both feature builds of blitz-shell, and its windowed witness is owed on the working route's "Stand a11y assertions" — as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md
- `changed_nodes` is the engine's changed set, the nodes written since the last `take_changed_nodes`: a mutation of an in-document node, a focus or checked change and a text control's input mark it; node creation, hover, active, scroll and layout do not; `has_changes()` reads true while it is non-empty. So a focus move, a checked change and a rewritten accessible name each reach the refresh above, and a pointer move alone does not (packages/blitz-dom/src/document.rs:326-329; packages/blitz-dom/src/document.rs:1007-1020) — ratified by the founder (2026-10-07), the same item — as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md. A harness settle (`Harness::settle`, the driver session's `act`) neither reads nor drains the set, so a step settled through it leaves the refresh above, and a later diff, exactly what the step marked; and the set is non-empty on a freshly booted harness document — the boot's own in-document mutations mark it and nothing has drained it — so a check that reads `has_changes()` as "this step changed something" drains first (packages/blitz-test-harness/src/settle.rs:129-191; tests/blitz-tests/tests/stand_settle.rs:83-124; as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md and its evidence/red-first.md)
- AccessibilityDeactivated and ActionRequested events are unhandled TODOs (packages/blitz-shell/src/application.rs:84-89)
- Window focus and outer/inner bounds are forwarded to the adapter on every window event before it is handled (packages/blitz-shell/src/accessibility.rs:48-74; packages/blitz-shell/src/window.rs:585-588)

**Role derivation (semantic HTML and ARIA):**
- A node's role comes from its `role` attribute, else from the HTML element mapping, else `Role::Unknown` (packages/blitz-dom/src/accessibility.rs:75-85)
- A TODO notes that elements with strong native semantics can currently have their role overridden, contrary to WAI-ARIA 1.2 (packages/blitz-dom/src/accessibility.rs:78-79)
- Role mapping follows the HTML-AAM spec, linked in the test doc; previously links, lists, tables, labels and landmarks arrived as `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:1-7)
- A semantic page asserts that only `<html>` and `<body>` map to `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:182-198)

**Platform adapters (accesskit_xplat):**
- The platform module picks windows, macos, unix (with `accesskit_unix`), android (with `accesskit_android`), else a null adapter (packages/accesskit_xplat/src/platform_impl/mod.rs:9-50)
- accesskit_xplat warns that AccessKit developers noted its approach may not be optimal (packages/accesskit_xplat/src/lib.rs:9-11)
- Android adapter `set_focus` and `set_window_bounds` are empty; Windows `set_focus` and macOS `set_window_bounds` are no-ops (packages/accesskit_xplat/src/platform_impl/android.rs:43-45; packages/accesskit_xplat/src/platform_impl/windows.rs:36-38; packages/accesskit_xplat/src/platform_impl/macos.rs:42)
- The Android adapter imports no `Rect` yet takes `Rect` parameters in `set_window_bounds` (packages/accesskit_xplat/src/platform_impl/android.rs:5; packages/accesskit_xplat/src/platform_impl/android.rs:45)
- The Android adapter reads the `mSurfaceView` field typed `GameActivity$InputEnabledSurfaceView` (packages/accesskit_xplat/src/platform_impl/android.rs:26-34)

**Feature exposure:**
- The crate docs state the `accessibility` feature enables accesskit support; dioxus-native-dom's feature also gates its own `accessibility_tree` override, its snapshot model (`DioxusDocument::snapshot`), that model's text form (`Snapshot::to_text`), its diff (`Snapshot::diff`) and its actionable-key check (`DioxusDocument::unkeyed_actionable`), all four of which its crate doc names, and dioxus-native forwards the feature to blitz and to dioxus-native-dom (packages/dioxus-native-dom/src/lib.rs:7-10; packages/dioxus-native-dom/src/lib.rs:15-16; packages/dioxus-native-dom/src/lib.rs:21-26; packages/dioxus-native/src/lib.rs:7; packages/dioxus-native/Cargo.toml:32; packages/dioxus-native-dom/src/dioxus_document.rs:297-313)

> NOT YET MEASURED — a stated accessibility strategy (POUR coverage depth, agent-runnable invariants, naming conventions): no slice states one, and s01, s02, s03, s06, s07, s08, s10 and s13 recorded accessibility strategy as out of slice.

---

## 3. A11y Assertion Harness Contract

**A11y testing tool pick:** accessibility assertions are Rust tests in blitz-tests over the AccessKit tree.
- Tests call `document.build_accessibility_tree()` after `resolve` and match `tree_update.nodes` with `test_that` matchers on `role()` and `is_hidden()` (tests/blitz-tests/tests/accessibility_hidden.rs:9-26)
- An `assert_role(html, element_id, expected)` helper maps the element's `NodeId` to an AccessKit `NodeId(node_id.as_u64())` and compares roles (tests/blitz-tests/tests/accessibility_roles.rs:39-72)
- An `unknown_tags(html)` helper lists the `html_tag` of nodes with `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:16-37)
- The test harness builds blitz-dom with the accessibility feature (packages/blitz-test-harness/Cargo.toml:15)
- accesskit_xplat ships a doc-comment usage example, not tests (packages/accesskit_xplat/src/lib.rs:13-86)

**Focus management test harness:**
- The harness exposes focused and hovered node queries (packages/blitz-test-harness/src/inspect.rs:107-115)

**Keyboard test harness:** a Tab and Shift+Tab key-press focus check exists on the stand, driven through the test harness's `press` / `press_with` (tests/blitz-tests/tests/stand_snapshot_state.rs:267-303) — see §5 (Test harness pattern).

**Contrast verification harness:** contrast checks are observed absent — see §6.

**CI integration:** ci.yml's `a11y` job runs the accessibility integration tests as their own leg — see §9.

**Observed absent (assertion searches):**
- accessibility tree or assertion tooling · searched: `accesskit|AccessKit` over the 32 s03 slice files
- accessibility assertions · searched: `#\[(tokio::)?test\]` over packages/accesskit_xplat/** and `aria-|role` over the 86 s04 slice files (role hits only in github-markdown.css)
- tests of the accessibility tree · searched: `#\[test\]` over packages/blitz-dom/src/accessibility.rs
- accessibility assertions in tests · searched: `accesskit|aria|role` over the 16 listed s08 files (only commented accesskit lines matched)
- accessibility assertions in tests · searched: `aria-|aria[A-Z]|\brole\b` over the 32 s10 slice files
- accessibility assertions in tests · searched: `aria|role|accesskit` over the 21 listed s11 files (matches only in crate docs and unrelated comments)

> NOT YET MEASURED — the structured violation JSON schema / log format, the WCAG criteria mapping of the existing tests and a screen-reader test pattern: no slice recorded them (s01, s02, s06, s07 and s13 recorded the a11y assertion harness as out of slice). The obs log format is now measured for escher's own sink (obs plan §3, §6 Log Coverage, §8): non-JSON text lines on stderr in two record classes — one line per printed event, `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…`, and, since 2026-10-07-driver-command-spans, one line per closed span of an admitted target, the same prefix then `span={the span's name}`, the span's recorded fields and the layer's close fields `message`, `time.busy` and `time.idle`, with no line when a span is created, recorded to, entered or exited (the second class was ratified by the founder, 2026-10-09, as obs plan §6 states) — both behind a target allowlist that, among others, prints only `node_id` / `status` / `waiting_nodes` / `property` / `log.*` fields for `blitz*` and `accesskit_xplat` targets, redacts `text`, `value`, `html` and `attrs` for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level — a closed span's every pair judged by the same rule, an outside target's span writing nothing; an a11y violation schema is not yet defined against it.

Contracts: .andromeda/registries/a11y-plan-contracts.toml — ask registry.py contracts; read one contracts/a11y-plan/{key}.md; never whole.

## 4. ARIA Patterns & Roles

**Landmark roles inventory (engine HTML-to-AccessKit mapping):**
- Native element mapping follows HTML-AAM: landmarks, headings, lists, tables, interactive and inline semantics (packages/blitz-dom/src/accessibility.rs:191-259)
- Landmarks: nav→Navigation, main→Main, aside→Complementary, footer→Footer, article→Article, blockquote→Blockquote, figure→Figure (tests/blitz-tests/tests/accessibility_roles.rs:74-93)
- header→Header, section→Section (tests/blitz-tests/tests/accessibility_roles.rs:157-180)

**Per-component pattern catalog (engine mapping):**
- **ARIA `role` attribute** — `role_from_name` maps ARIA role names (alert, button, checkbox, dialog, link, tab, textbox, landmark roles, etc.) to AccessKit roles (packages/blitz-dom/src/accessibility.rs:127-189)
- **Button** — button→Button, submit→Button (tests/blitz-tests/tests/accessibility_roles.rs:157-180; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
- **Link** — `<a>` is a Link only with `href`, else GenericContainer (packages/blitz-dom/src/accessibility.rs:226-243; tests/blitz-tests/tests/accessibility_roles.rs:117-126)
- **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:226-243; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
- **Lists and tables** — ul/ol→List, li→ListItem, table→Table, thead→RowGroup, tr→Row, th→ColumnHeader, th scope=row→RowHeader, td→Cell (tests/blitz-tests/tests/accessibility_roles.rs:95-115); `<th>` is RowHeader for `scope=row|rowgroup`, else ColumnHeader (packages/blitz-dom/src/accessibility.rs:226-243)
- **Form input** — `<input>` roles are mapped by `type`, defaulting to TextInput (packages/blitz-dom/src/accessibility.rs:260-278); label→Label, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput (tests/blitz-tests/tests/accessibility_roles.rs:128-155); text→TextInput, number→NumberInput, checkbox→CheckBox (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
- **Generic and text** — div→GenericContainer, h2→Heading, p→Paragraph (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
- **Hidden content** — `aria-hidden="true"` keeps the node but marks it hidden (`is_hidden`); `hidden`, `display: none` and `visibility: hidden` exclude it; children of a hidden element are excluded (packages/blitz-dom/src/accessibility.rs:96-99; tests/blitz-tests/tests/accessibility_hidden.rs:28-137)

**Application markup:**
- Browser controls are clickable `div`s rather than buttons: icon buttons, tabs, tab close, new tab, menu items (apps/browser/src/icons.rs:28-39; apps/browser/src/tab_strip.rs:83-106; apps/browser/src/toolbar.rs:419-441)
- Images: new-tab logo has `alt: "Blitz"`, favicons `alt: ""`, `IconButton` images no alt (apps/browser/src/about_pages.rs:86; apps/browser/src/tab.rs:278; apps/browser/src/icons.rs:38)
- rdme's stylesheet styles `[role=button]` and `[role=tabpanel]` focus states (apps/readme/assets/github-markdown.css:340-346; apps/readme/assets/github-markdown.css:1081)

**Fixture markup (examples/assets):**
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

**Observed absent (ARIA searches):**
- ARIA attributes or roles in app/example markup · searched: `aria-|role:|"role"|role=` over the 86 s04 slice files excluding github-markdown.css and default.css
- ARIA attributes or roles · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files
- ARIA attributes or roles · searched: `aria|accessib|role` over the 8 s07 slice files (only match is a paint-tree local `role_unchanged`)
- ARIA attributes or roles · searched: `accesskit|aria|role` over the 16 listed s08 files (only commented accesskit lines matched)
- ARIA attributes · searched: `aria-` over the 32 listed s09 files
- ARIA attributes or roles · searched: `aria-|aria[A-Z]|\brole\b` over the 32 s10 slice files
- ARIA attributes or roles · searched: `aria|role|accesskit` over the 21 listed s11 files
- ARIA attributes or roles · searched: `aria-|role=` over the 12 listed s13 files

---

## 5. Keyboard Navigation

**Focus order per layout (engine):**
- An element is focusable if it holds a sub-document, or if its `disabled` attribute does not parse as `true` and either has `tabindex >= 0` or, with no tabindex, is an `<a>`/`<area>` with `href` or a `button`, `input`, `select`, `textarea`, `frame`, `iframe` or `summary` (packages/blitz-dom/src/node/element.rs:634-666)
- Tab moves focus to the next node and Shift+Tab to the previous, dispatching focus events (packages/blitz-dom/src/events/keyboard.rs:22-41); `focus_next_node` / `focus_prev_node` move focus to the next/previous focussable node (packages/blitz-dom/src/document.rs:1661-1674)
- Cached focusability is recomputed when `tabindex`, `href` or `disabled` is set or removed (packages/blitz-dom/src/mutator.rs:337-346; packages/blitz-dom/src/mutator.rs:467-474); tests assert focusability follows `tabindex` set or cleared after creation (roving tabindex) (tests/blitz-tests/tests/focusability_updates.rs:1-6; tests/blitz-tests/tests/focusability_updates.rs:34-57)
- `button`, `input`, `select` and `textarea` can be disabled, which toggles the `DISABLED`/`ENABLED` states (packages/blitz-dom/src/node/element.rs:451-458; packages/blitz-dom/src/node/element.rs:482-484; packages/blitz-dom/src/node/node.rs:777-799); setting `disabled` removes a button's focusability (tests/blitz-tests/tests/focusability_updates.rs:63-76)
- Disabled-ness is keyed two ways: the `DISABLED`/`ENABLED` element state (hence `:disabled`) and the pointer click target on the attribute's PRESENCE, focusability on its value PARSED as a bool — so `disabled="false"` matches `:disabled` yet stays focusable, and a bare `disabled=""` is focusable too (packages/blitz-dom/src/node/element.rs:452-457; packages/blitz-dom/src/node/element.rs:635; packages/blitz-dom/src/events/pointer.rs:330; packages/blitz-dom/src/events/pointer.rs:457; tests/blitz-tests/tests/focusability_updates.rs:59-62), as measured at escher-0.1.0/chunks/2026-10-06-headless-stand/evidence/disabled-false-probe.txt
- The file input's generated inner button gets `tabindex="-1"` (packages/blitz-dom/src/mutator.rs:1316-1330)
- Under the `autofocus` feature, the latest mounted focussable node whose `autofocus` attribute is present with any value but "false" — bare, empty or "true" — is focused on flush, and `autofocus="false"` is ignored (packages/blitz-dom/src/mutator.rs:1021-1033; packages/blitz-dom/src/mutator.rs:941-946; tests/blitz-tests/tests/autofocus_attribute.rs:27-50) — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md
- Focusing sets the `FOCUS` and `FOCUSRING` element states; blurring removes them (packages/blitz-dom/src/document.rs:1710-1715; packages/blitz-dom/src/node/node.rs:713-751)
- `:focus` matches the focus element state while `:focus-visible` and `:focus-within` never match (packages/blitz-dom/src/stylo.rs:476-478)
- focus and blur do not bubble; focusin and focusout bubble (packages/blitz-traits/src/events.rs:441-444)

**Focus restoration:**
- Removing the focused node resets focus to the body (encoded as `None`) and runs blur side effects (packages/blitz-dom/src/document.rs:907-909; packages/blitz-dom/src/document.rs:925-929); removing a focused text input runs blur side-effects and disables IME (tests/blitz-tests/tests/interaction_state_teardown.rs:189-217)
- Clicking a non-interactive area clears focus (packages/blitz-dom/src/events/pointer.rs:814-817)
- Activating the first `summary` of a `details` toggles it open and focuses the summary (packages/blitz-dom/src/events/pointer.rs:696-722)
- Checkbox and radio clicks toggle state, dispatch `input` and move focus to the control (packages/blitz-dom/src/events/pointer.rs:645-695); clicking a checkbox focuses it (tests/blitz-tests/tests/harness_smoke.rs:33-44)

**Keyboard handling (engine and shell):**
- Text inputs handle arrow keys, Home/End, Delete/Backspace, Enter (newline or submit) and action-modifier copy/cut/paste/select-all, plus word movement with the action modifier (packages/blitz-dom/src/node/text.rs:202-370)
- On macOS, Backspace is left to the Apple standard keybindings, which map Cocoa selector commands to editor actions (packages/blitz-dom/src/node/text.rs:333-342; packages/blitz-dom/src/node/text.rs:372-761)
- The action modifier plus C copies selected text when no text input is focused (packages/blitz-dom/src/events/keyboard.rs:43-61)
- A text input's generated submit triggers implicit form submission unless the form has more than one blocking field type (packages/blitz-dom/src/events/keyboard.rs:131-134; packages/blitz-dom/src/events/keyboard.rs:139-175)
- Clicking a label runs the default click of its bound input (packages/blitz-dom/src/events/pointer.rs:723-734)
- `synthetic_click_event` builds a primary mouse click at the node's center (packages/blitz-dom/src/node/node.rs:1866-1896)
- winit key events are converted to keyboard-types Key, Code, Location and Modifiers, with is_composing always false (packages/blitz-shell/src/convert_events.rs:52-69; packages/blitz-shell/src/convert_events.rs:151-175)
- Every key press and release is dispatched as KeyDown or KeyUp after shell shortcuts are checked (packages/blitz-shell/src/window.rs:689-697)
- macOS standard key bindings are forwarded as `AppleStandardKeybinding` UI events (packages/blitz-shell/src/application.rs:189-202; packages/blitz-shell/src/window.rs:577-582; packages/blitz-traits/src/events.rs:71; packages/blitz-traits/src/events.rs:155)
- IME Enabled, Disabled, Preedit, Commit and DeleteSurrounding events are forwarded to the document, with shell hooks to enable IME and set its cursor area (packages/blitz-shell/src/convert_events.rs:31-50; packages/blitz-shell/src/window.rs:637-640; packages/blitz-traits/src/events.rs:734-779; packages/blitz-traits/src/shell.rs:19-27)

**Script and framework exposure:**
- Elements expose `focus()` and `blur()`; `document.activeElement` returns the focused node (packages/blitz-vibey-script/src/dom/element.rs:175-176; packages/blitz-vibey-script/src/dom/element.rs:952-964; packages/blitz-vibey-script/src/dom/document.rs:149-154)
- `autofocus` reflection writes the value "true", one of the values blitz-dom's autofocus handling takes — any present value but "false" (Focus order above); blitz-dom is used with feature `autofocus` (packages/blitz-vibey-script/src/dom/element.rs:475-492; packages/blitz-vibey-script/Cargo.toml:19)
- JS keyboard events carry key, code, location, repeat, isComposing and modifier flags (packages/blitz-vibey-script/src/dom/event.rs:128-153; packages/blitz-vibey-script/src/dom/event.rs:207-229)
- KeyDown, KeyUp and KeyPress events reach Dioxus as keyboard data with key, code, location, repeat, composing state and modifiers (packages/dioxus-native-dom/src/dioxus_document.rs:374-378; packages/dioxus-native-dom/src/events.rs:328-361)
- Focus, Blur, FocusIn and FocusOut events are forwarded as focus data (packages/dioxus-native-dom/src/dioxus_document.rs:369-372)
- Mounted elements can take or drop focus with `set_focus`; focus/blur events are not queued (TODO) (packages/dioxus-native-dom/src/events.rs:283-295)
- An `autofocus` feature forwards to blitz-dom (packages/dioxus-native-dom/Cargo.toml:23; packages/dioxus-native/Cargo.toml:19)
- IME events are not handled in dioxus-native-dom (TODO) (packages/dioxus-native-dom/src/dioxus_document.rs:385-386)
- A falsy Dioxus value (Bool false, Text `"false"`, Int 0, Float 0.0, None) of one of the bridge's 27 boolean attributes — `disabled`, `checked`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open` and `autofocus` among them, the `isBoolAttr` list of Dioxus 0.7.10's web interpreter — removes the attribute, on the dynamic and the static template attribute path alike, so an enabled Dioxus control does not match `:disabled` and an element rendered `hidden: false` carries no `hidden` attribute, is laid out and stays in the accessibility tree and the snapshot, while a truthy `hidden` still excludes it (§4); a truthy value of a listed name is still written, and an attribute outside the list holding `false` (`aria-hidden`, a `data-` attribute) is still written as `"false"` (packages/dioxus-native-dom/src/mutation_writer.rs:11-43; packages/dioxus-native-dom/src/mutation_writer.rs:439; tests/blitz-tests/tests/dioxus_falsy_disabled.rs:24-39; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:56-79; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:128-148; tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:169-197) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md

**Per-surface keyboard shortcuts:**
- Browser urlbar handles ArrowDown/ArrowUp to move selection, Escape to blur, Enter to submit (apps/browser/src/toolbar.rs:376-397)
- Browser: Cmd+T/W (macOS) or Ctrl+T/W open and close tabs (apps/browser/src/tab_strip.rs:42-71)
- Browser new-tab search input autofocuses and submits on Enter (apps/browser/src/about_pages.rs:87-105)
- rdme uses Ctrl/Cmd plus R (reload), T (theme), B (back) on key release (apps/readme/src/readme_application.rs:170-184)
- `use_back_button` handles `NamedKey::BrowserBack` presses, ignoring key repeats (packages/dioxus-native/src/hooks.rs:30-49)
- todomvc focuses the new-todo input on mount; Enter adds; Enter/Escape/Tab end editing (examples/todomvc/src/app.rs:115-133; examples/todomvc/src/app.rs:143-145; examples/todomvc/src/app.rs:207-212)
- Cells commits an edit on Enter or blur (examples/seven_guis/src/tasks/cells.rs:386-395)
- TodoMVC (Preact) input autofocuses and adds an item on `Enter` (examples/preact/index.html:80-86)
- Android Activities make the native view focusable in touch mode and request focus (apps/browser/MainActivity.kt:34-38; examples/todomvc/MainActivity.kt:11; examples/todomvc/MainActivity.kt:24-27)

**Fixture markup:**
- The docs.rs fixture uses `tabindex="-1"` on toolbar containers (examples/assets/docsrs_header.html:13; examples/assets/docsrs_header.html:16)
- The google fixture gives non-button elements role=button with tabindex=0 (examples/assets/google.html:3083-3084; examples/assets/google.html:3211-3214; examples/assets/google.html:3237-3238)
- The google fixture binds keydown handlers through jsaction (examples/assets/google.html:3171; examples/assets/google.html:3653)
- The google fixture styles :focus-visible with a 1px solid outline (examples/assets/google.html:469-474)
- The graphite fixture removes the outline on newsletter inputs and marks focus by a border-color custom property (examples/assets/graphite.html:342; examples/assets/graphite.html:347-349; examples/assets/graphite.html:358-361)
- servo.css removes `.button:focus` outline and draws a 0.125em box-shadow focus ring instead (examples/assets/servo.css:45-47; examples/assets/servo.css:3094-3100)
- servo.html headings carry `tabindex="-1"` (examples/assets/servo.html:252; examples/assets/servo.html:274; examples/assets/servo.html:309)
- Form labels associate with inputs by `for`/`id` and by nesting (examples/form.rs:27; examples/form.rs:36; examples/form.rs:39-46)

**Test harness pattern:**
- Focusability tests are listed under Focus order above (tests/blitz-tests/tests/focusability_updates.rs:34-57)
- A key-press focus check exists on the stand, in both layout modes: after a click on `crud-name` a `Key::Tab` press moves focus to `crud-surname` and Shift+Tab moves it back, the first Tab on a fresh boot focuses `back-btn`, nothing reads focused at boot or after a click on a heading, and each time exactly the focused control's snapshot node reads `focused` and it is the accessibility tree's focus (tests/blitz-tests/tests/stand_snapshot_state.rs:267-303) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md
- Observed absent — tabindex handling · searched: `tabindex` over the 17 listed s06 files
- Observed absent — tabindex handling · searched: `tabindex|tabIndex` over the 32 s10 slice files
- Observed absent — focus or keyboard handling · searched: `focus|tabindex|keyboard` over the 8 s07 slice files

> NOT YET MEASURED — skip links, modal focus traps and route-change focus restoration: no slice recorded them; s01 recorded keyboard focus handling code as out of slice and s13 recorded keyboard interaction as out of slice.

---

## 6. Visual Design Verification

**Color contrast:**
- A WCAG contrast ratio helper matching Chrome's GetContrastRatio is used for border bevels and scrollbar thumbs (packages/blitz-paint/src/color.rs:27-35; packages/blitz-paint/src/render/border.rs:72-78; packages/blitz-paint/src/color.rs:37-66)
- Default scrollbar thumbs get a thin contrast stroke so they read over same-coloured content (packages/blitz-paint/src/render.rs:747-749; packages/blitz-paint/src/render.rs:824-836); a test asserts default scrollbar thumbs carry a contrast stroke (tests/blitz-tests/tests/scrollbars.rs:194-202)
- A near-white author thumb must still visibly change on hover and drag (tests/blitz-tests/tests/scrollbar_drag.rs:224-292)
- Observed absent — contrast checks · searched: `contrast` over the 15 s05 files
- Observed absent — contrast or user-preference media handling · searched: `contrast|prefers-` over the 21 listed s11 files

**Focus ring:**
- Browser urlbar focus shows a `#5E9ED6` border and 1px outline (apps/browser/assets/browser.css:172-175)
- New-tab search input removes the outline and changes only border color on focus (apps/browser/assets/about-newtab.css:30-34)
- todomvc sets `:focus` outline to 0 (examples/todomvc/src/todomvc.css:36-38)
- seven_guis inputs replace the outline with a border color or box-shadow on focus (examples/seven_guis/src/tasks/temp_converter.rs:75-79; examples/seven_guis/src/tasks/flight_booker.rs:182-185)
- counter and transparent buttons show a 4px focus outline (examples/counter/src/app.rs:79-81; examples/transparent/src/app.rs:180-182)
- rdme stylesheet shows a 2px outline in its focus-outline color custom property on focus-visible (apps/readme/assets/github-markdown.css:356-363)
- blitz-dom default stylesheet gives inputs a 2px `#4D90FE` focus outline and suppresses outlines on iframe/body/html focus-visible (packages/blitz-dom/assets/default.css:93-96; packages/blitz-dom/assets/default.css:834-840)
- The google fixture switches focus outlines to currentcolor under @media (forced-colors:active) (examples/assets/google.html:475-479)
- Focused text inputs paint a caret honouring caret-color and a selection highlight (packages/blitz-paint/src/render.rs:915-941)

**Motion:**
- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)
- Observed absent — prefers-reduced-motion and prefers-color-scheme rules · searched: `prefers-reduced-motion|prefers-color-scheme` over the 21 s02 files

**State color (not-color-alone):**
- Disabled icon buttons render at 0.35 opacity (apps/browser/assets/browser.css:191-198)
- Each WPT runner result line prints its status word as text alongside its color (wpt/runner/src/main.rs:369-398)
- A comment in the form example notes the input color "Should be accent-color" (examples/form.rs:126-129)

**Color scheme, zoom and user preferences:**
- The viewport carries a light/dark color scheme and a document zoom level (`1.0` unzoomed) (packages/blitz-traits/src/shell.rs:68-82; packages/blitz-traits/src/shell.rs:134-150)
- Color-scheme changes on the viewport are tracked as a device change that rebuilds the stylist device (packages/blitz-dom/src/stylo_device.rs:37-38; packages/blitz-dom/src/stylo_device.rs:55-57)
- Total scale changes from hidpi or zoom are tracked and invalidate text shaping (packages/blitz-dom/src/stylo_device.rs:33-36; packages/blitz-dom/src/stylo_device.rs:52-54)
- Page zoom is adjustable from the keyboard in 0.1 steps and resettable to 1.0 (packages/blitz-shell/src/window.rs:654-662)
- rdme follows `prefers-color-scheme` for dark/light tokens and backgrounds (apps/readme/assets/github-markdown.css:13-124; apps/readme/assets/blitz-markdown-overrides.css:11-35)
- The TodoMVC page declares `color-scheme: light`; the reference page declares `color-scheme: light dark` (examples/preact/index.html:8; examples/preact/core_dom_apis.html:8)
- blitz-dom default stylesheet un-inverts images and video under `inverted-colors` (packages/blitz-dom/assets/default.css:1078-1089)
- Touch panning honours the `touch-action` property per axis (packages/blitz-dom/src/events/pointer.rs:158-204; packages/blitz-dom/src/events/pointer.rs:247-260)

**Visibility and hit-testing:**
- Nodes with `display: none` or `visibility: hidden`, and their descendants, are excluded from the accessibility tree (packages/blitz-dom/src/accessibility.rs:12-21; packages/blitz-dom/src/accessibility.rs:112-124)
- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1360-1368)
- `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1370-1374; packages/blitz-dom/src/node/node.rs:1532-1540)
- `scrollbar-width: none` suppresses overlay scrollbars (packages/blitz-dom/src/node/scrollbar.rs:76-87); a test asserts it paints no scrollbar (tests/blitz-tests/tests/scrollbars.rs:162-175)
- Devtools layout outlines and hover/node highlight overlays exist as settings (packages/blitz-traits/src/devtools.rs:5-34)

> NOT YET MEASURED — design-token contrast pairs and focus-ring, target-size and typography-readability tokens: no slice recorded them, and s01 and s07 recorded contrast and visual checks as out of slice.

---

## 7. Screen Reader Support

**Accessibility tree output:**
- Text nodes become `TextRun` nodes carrying their text, and the parent is labelled by them (packages/blitz-dom/src/accessibility.rs:100-104)
- An element's `aria-label`, when non-empty after trimming, becomes its `label` (accname-1.2 §2C); a whitespace-only one names nothing (packages/blitz-dom/src/accessibility.rs:88-94; tests/blitz-tests/tests/accessibility_names.rs:64-82)
- A `<label>` names the `<input>` it is bound to — its `for`-target by id, else its first nested `<input>` — by being pushed onto that input's `labelled_by` after the whole tree is built, so a label before or after its input both work; the bound input must be an `input` element with a built node (packages/blitz-dom/src/accessibility.rs:29-48; packages/blitz-dom/src/document.rs:641-678; tests/blitz-tests/tests/accessibility_names.rs:84-93)
- In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one; on the stand the 15 interactive controls carry an HTML-AAM role and a non-empty accessible name — the six inputs named by four `<label for>` and two `aria-label` attributes — in both layout modes, with the Tab order unchanged (packages/dioxus-native-dom/src/dioxus_document.rs:297-313; tests/blitz-tests/tests/stand_accessibility_ids.rs:1-4) — as measured at escher-0.1.0/chunks/2026-10-06-accessibility-tree-identity/report.md; the roles and input names those checks expect are stated once, as the tables `controls` and `INPUT_NAMES` of the stand checks' shared module, beside `rendered`, the ids rendered per task — stated expectation, never a second role or name mapping (tests/blitz-tests/tests/common/mod.rs:16-55; tests/blitz-tests/tests/common/mod.rs:60-97) — as measured at escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md
- The snapshot model is a consumer of this output: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` takes each node's role, its name (the `label`, else the names of its `labelled_by` targets) and the focused node from the tree `accessibility_tree` returns, with no second role or name mapping; on the stand every snapshot node's role and name equal its accessibility node's, nothing reads focused at boot, and after one focus move only `back-btn` does, in both layout modes (packages/dioxus-native-dom/src/snapshot.rs:79-97; packages/dioxus-native-dom/src/snapshot.rs:162-180; tests/blitz-tests/tests/stand_snapshot.rs:1-5) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-model/report.md; the tree gives an element node no value, checked or disabled property, so the snapshot reads those three from the element — `enabled` follows the presence of `disabled` on an element that can be disabled (a `disabled="false"` and a bare `disabled` read not enabled while both stay focusable, §5 Focus order; the driver's `disabled` refusal cause is stated on this same presence reading — its meaning reads "the element carries the `disabled` attribute, which the snapshot reads as not enabled" — and since 2026-10-07-refusal-detection the driver detects it on that reading and no other: `Session::run` refuses a `click` or a `type` as `disabled` when the snapshot node's `enabled` is `Some(false)`, the first of its three screen-level checks, with nothing dispatched and the ids reading `focused` and the tree's focus as they were before the call; on the stand `crud-delete` and `flight-return-date` are refused while they read not enabled and acted on once they read enabled, in both layout modes (packages/escher-driver/src/execute.rs:289-307; tests/blitz-tests/tests/stand_act_disabled.rs:1-8); the driver's schema defines no role or name set of its own, `role` and `name` being field names of its result nodes — the schema as measured at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md, the detection at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md), `checked` is the checkbox or radio state, `value` is the editor's text, else the `value` attribute — a textarea's as a text input's: on the in-file fixture of `stand_snapshot_state` a textarea named by `aria-label` reads role `MultilineTextInput`, a non-empty name, its HTML `id` as its stable id and a box wider and taller than 0, reads `""` untyped and its typed text back as its `value`, unmasked, and after the click exactly it reads `focused` and is the tree's focus, in both layout modes; it is a test-only element of that fixture, and no stand task or app gained one with it (tests/blitz-tests/tests/stand_snapshot_state.rs:331-338; tests/blitz-tests/tests/stand_snapshot_state.rs:509-536) — as measured at escher-0.1.0/chunks/2026-10-09-audit-corrections-agent-surfaces/report.md —, and a password input reads the fixed marker `MASKED_VALUE` where it holds text and `""` where it holds none, its typed text in no field of the snapshot or of the tree, and a file input reads the same marker where its `value` attribute is non-empty, its path in no field of the snapshot; and focus is proven three ways on the stand in both layout modes — after a pointer click, a Tab press and Shift+Tab exactly the focused control's node reads `focused` and it is the tree's focus (packages/dioxus-native-dom/src/snapshot.rs:140-147; packages/dioxus-native-dom/src/snapshot.rs:184-211; tests/blitz-tests/tests/stand_snapshot_state.rs:1-7) — as measured at escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/report.md, the file-input reading at escher-0.1.0/chunks/2026-10-06-compact-snapshot-serialization/report.md; the snapshot's text form `Snapshot::to_text` is a further reader of that model, never of the tree or of a control — each line's role and name are the snapshot node's, with no second role or name mapping, there is one line per snapshot node, and on the stand no line reads `focused` at boot and exactly `back-btn`'s does after a Tab press, in both layout modes (packages/dioxus-native-dom/src/snapshot_text.rs:36; tests/blitz-tests/tests/stand_snapshot_text.rs:1-7) — as measured at escher-0.1.0/chunks/2026-10-06-compact-snapshot-serialization/report.md; the snapshot's diff `Snapshot::diff` is one more reader of that model and of nothing else — a comparison of two snapshots by stable element id, its entries carrying the role and name the later snapshot holds, with no second role or name mapping — so on the stand, in both layout modes, a text change is reported on the element whose accessible name it changes, by that element's id and never as a text-run entry (`counter-value` after a click, the labelled input after its label is rewritten), an element hidden and shown by its `hidden` attribute leaves and returns with its descendants, and a pointer click, a Tab press and Shift+Tab each name exactly the controls whose `focused` reading moved (packages/dioxus-native-dom/src/snapshot_diff.rs:60; tests/blitz-tests/tests/stand_diff.rs:1-10) — as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md
- The actionable-key rule: every element an agent can act on — focusable, or carrying an interactive role, or carrying an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it (ratified by the founder, 2026-10-06); under dioxus-native-dom's `accessibility` feature `DioxusDocument::unkeyed_actionable` is the check — it reads focusability from `Node::is_focussable()`, the role from the tree `accessibility_tree` returns (a closed interactive list, so a disabled control is still held) and the listener mark `data-dioxus-id` (so a hidden control is still held), with no second role mapping, and returns each actionable element whose id is a positional path, with its tag, its role and the remedy; on the stand it returns nothing on the four lean tasks and on Home — at boot and after Book, Create and a row selection, with Book enabled and disabled, in both layout modes — CRUD's person rows reading `crud-person-{person.id}` and Home's seven cards `task-card-{slug}`, keyed by an `id` attribute alone with no `tabindex`, role, focusability or Tab-order change, and reads Temperature Converter 2 · Circle Drawer 3 · Cells 676 on the three tasks not yet keyed (packages/dioxus-native-dom/src/actionable.rs:55-89; packages/dioxus-native-dom/src/actionable.rs:91-122; tests/blitz-tests/tests/stand_actionable_keys.rs:1-4) — as measured at escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/report.md
- Element nodes carry their HTML tag name (packages/blitz-dom/src/accessibility.rs:86)
- The tree root is a `Window` node with id `u64::MAX`, and the focused DOM node is reported as tree focus (packages/blitz-dom/src/accessibility.rs:8; packages/blitz-dom/src/accessibility.rs:53-61)
- The role-mapping test states its goal as giving assistive technology something to navigate by (tests/blitz-tests/tests/accessibility_roles.rs:3-5; tests/blitz-tests/tests/accessibility_roles.rs:195-196)

**Landmark roles:** the engine's landmark mapping is recorded in §4.

**Platform adapter:**
- accesskit_xplat routes initial-tree requests, action requests and deactivation to one `EventHandler`, returning no initial tree synchronously (packages/accesskit_xplat/src/lib.rs:140-172)
- `Adapter` creation must happen before the window is first shown and panics if it is already visible (packages/accesskit_xplat/src/lib.rs:180-196)
- The AccessKit adapter is created with a combined handler from the raw window handle, or from the Android app on Android, and forwards events into the shell event loop (packages/blitz-shell/src/accessibility.rs:18-43)
- Screen-reader integration is behind blitz's `accessibility` feature via blitz-shell (packages/blitz/Cargo.toml:16); the Dioxus crates state screen-reader exposure only as accesskit support behind the `accessibility` feature (packages/dioxus-native-dom/src/lib.rs:7-10; packages/dioxus-native/src/lib.rs:7)

**Language and direction:**
- `lang`/`xml:lang` set the element language (`-x-lang`), inherited, with `lang=""` resetting to unknown (tests/blitz-tests/tests/lang_attribute.rs:1-2; tests/blitz-tests/tests/lang_attribute.rs:40-45)
- blitz-dom default stylesheet maps the `dir` attribute with attribute selectors for bidi isolation (packages/blitz-dom/assets/default.css:14-36)
- 11 fixtures set an html lang attribute (examples/assets/google.html:2; examples/assets/gosub.html:3; examples/assets/gosub_reduced.html:2; examples/assets/graphite.html:2; examples/assets/graphite_blog_section.html:2; examples/assets/graphite_software_overview.html:2; examples/assets/newservo.html:2; examples/assets/pseudo.html:2; examples/assets/servo-new-reduced-1.html:2; examples/assets/servo-new-reduced.html:2; examples/assets/servo-new.html:2)
- Pages declare `lang="en"` (examples/assets/servo.html:2; examples/preact/index.html:2; examples/preact/core_dom_apis.html:2)
- Observed absent — an html lang attribute in the other 10 s02 fixtures · searched: `lang=` over the 21 s02 files

**Text alternatives and hidden decoration (fixtures):**
- The captured GitHub fixture uses an `sr-only` tooltip element (examples/assets/github_profile_reduced2.html:68)
- The google fixture's links carry descriptive aria-labels such as "Gmail (opens a new tab)" (examples/assets/google.html:3052; examples/assets/google.html:3061)
- The google fixture hides decoration with aria-hidden and uses aria-atomic on suggestion options (examples/assets/google.html:3339; examples/assets/google.html:3318)
- The servo-new fixture hides the burger's bar spans with aria-hidden (examples/assets/servo-new.html:34-36)
- servo.html fixture images carry `alt` text ("Home", "Linux Foundation Europe logo") (examples/assets/servo.html:39; examples/assets/servo.html:331)
- servo_header_reduced.html fixture image has no `alt` attribute (examples/assets/servo_header_reduced.html:14)

**Input method (IME):**
- Focusing a text input enables IME and sets the IME cursor area to the input's content box; blurring disables IME (packages/blitz-dom/src/node/node.rs:720-733; packages/blitz-dom/src/node/node.rs:743-750)
- IME commit, preedit and disable events are applied to the text editor; `DeleteSurrounding` is a TODO (packages/blitz-dom/src/node/text.rs:763-803)

**Observed absent:**
- screen-reader or accessibility API integration · searched: `\baria-|accesskit|\brole\b|screen.?reader` over the 17 listed s06 files

> NOT YET MEASURED — live regions, heading-hierarchy handling and per-surface screen-reader test specs: no slice recorded them; s01, s07, s10 and s13 recorded screen-reader handling as out of slice, and s08 recorded the screen-reader tree output as out of slice.

---

## 8. Cognitive Accessibility

**Error recovery:**
- Error and 404 pages show "Failed to load page" and "404 Not found" (apps/browser/assets/error.html:19; apps/browser/assets/404.html:12)
- Invalid dates get an `invalid` class with red styling and the Book button disables (examples/seven_guis/src/tasks/flight_booker.rs:80-98; examples/seven_guis/src/tasks/flight_booker.rs:193-197); the headless-stand check asserts the cue without colour — a typed non-date sets `invalid` on the start field and gives `.flight-btn` a `disabled` attribute it did not carry before (tests/blitz-tests/tests/stand_flight_booker.rs:14-30); the stand asserts each lean task's forward focus sequence through `focus_next_node` (tests/blitz-tests/tests/stand_accessibility_ids.rs:86-101; tests/blitz-tests/tests/stand_accessibility_ids.rs:227-244) and, in the snapshot check, that only `back-btn` reads `focused` after one such move (tests/blitz-tests/tests/stand_snapshot.rs:210-263)

**Orientation and status cues:**
- History rows show relative time labels: "Just now", minutes, hours, days (apps/browser/src/browser_history.rs:88-102)
- Tabs show a tooltip with the full title on hover (apps/browser/assets/browser.css:56-74; apps/browser/src/tab_strip.rs:86-90)
- The status bar shows the hovered link target or a loading message (apps/browser/src/status_bar.rs:73-88)
- seven_guis cards carry a description and a tag per task (examples/seven_guis/src/app.rs:25-75; examples/seven_guis/src/app.rs:143-153)

> NOT YET MEASURED — timeout extensions, plain-language targets, redundant-entry avoidance and accessible authentication: every slice except s04 recorded cognitive accessibility as out of slice.

---

## 9. CI Integration

ci.yml's `a11y` job, "Accessibility (a11y) tests", runs `bash .github/scripts/ci-leg.sh a11y` — `cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates`, 6 + 6 + 3 tests — in the slow tier behind the four fast jobs, restoring the test job's cache and saving none (.github/workflows/ci.yml:247-268; .github/scripts/ci-leg.sh:34); it runs existing tests and adds no stand assertion. The search `a11y|accessib|axe` over .github/workflows/*.yml finds it — as measured at escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/gates.md.

---

## 10. SLO Invariants & A11y Budgets

> NO RECORDED INTENT

---

## 11. A11y Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

## 12. A11y Decisions Log

> NO RECORDED INTENT
