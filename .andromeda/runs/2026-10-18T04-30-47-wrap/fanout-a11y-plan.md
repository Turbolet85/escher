proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§2 A11y Strategy → Role derivation (the three-states bullet)"
    change: "Restate the toggle read: since 2026-10-18-engine-debts the builder consults the role for `toggled` — a native `input` whose `type` is exactly `checkbox` or `radio` (private `is_native_checkable`) reads no author toggle at all (neither `aria-checked` nor `aria-pressed`, HTML-AAM / ARIA in HTML); else a role in the private `is_checkable` set (`CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio`) writes `toggled` from `aria-checked` (`true` · `false` · `mixed`, trimmed, exact); every other role reads `aria-pressed` as before — one read per attribute, nothing else writes `toggled`; `role_from_name` maps `switch` → `Role::Switch` (no role before); drop «`aria-checked` is not read, so a node's `toggled` is exactly its `aria-pressed` reading» and «with no role consulted» for `toggled`; add the measured limit: native checkedness is unread by the builder, so the tree the shell hands the adapter never states a native checkbox or radio checked (the snapshot's `checked` reads the element and is right); PROVISIONAL on the operator's word as the founder's delegate (F2 A, its two native corrections), standing for the founder's own word; cite packages/blitz-dom/src/accessibility.rs:171-184, :330, :378-388, :390-398 and tests/blitz-tests/tests/accessibility_states.rs:245-299, :301-316 in place of :188-220 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §2 Role derivation: `aria-checked` read as `toggled` on the five checkable roles, `switch` mapped, native checkbox/radio read neither attribute, native checkedness unread by the tree — PROVISIONAL (F2 A)"
    rationale: "Report Changes → Symbols / APIs, F2 the toggle read (accessibility.rs:171-184, :330, :378-388, :390-398), Deviation 5 (native exclusion), Decisions & corrections (native-checkedness residuals line); Counts / qualifiers: `aria-checked` stale at a11y-plan.md:46, `acc-states` stale at a11y-plan.md:46"
    basis: ".andromeda/a11y-plan.md:46"
  - detector: D-a11y-surface
    severity: warning
    section: "§4 ARIA Patterns & Roles → Per-component pattern catalog (ARIA state attributes)"
    change: "Replace «`aria-checked` sets none of the three, so an ARIA checkbox or switch states no checked reading through it — owned by a `.andromeda/residuals.md` line on the founder's C7 …, `accessibility_states`' fourth test standing as its pin» with: `aria-checked` → `toggled` (`true` · `false` · `mixed`) on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone, `aria-pressed` → `toggled` on every other role, a native checkbox or radio reading neither; `role=\"switch\"` → `Switch`; the fourth test rewritten as `aria_checked_reads_its_three_values_on_a_checkable_role_alone` and a fifth `aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere`; PROVISIONAL (F2 A) for the founder's word; cite accessibility_states.rs:245-299, :301-316 in place of :188-220 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §4 ARIA state attributes: `aria-checked` now read on checkable roles; the C7 residual's pin rewritten — PROVISIONAL (F2 A)"
    rationale: "Report Changes → F2; Counts / qualifiers: `aria-checked` stale and `acc-states` stale at a11y-plan.md:104 («the fourth test standing as its pin» and its `:128-220` coordinates)"
    basis: ".andromeda/a11y-plan.md:104"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract → A11y testing tool pick (the state-check bullet)"
    change: "The state check `accessibility_states` holds 5 tests since 2026-10-18-engine-debts — its fourth rewritten to read `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute, its fifth that `aria-pressed` on a checkable role reads nothing and a 5,500-character `aria-checked` is nowhere in the tree; still in the workspace test leg and per package, not in the a11y leg; cite tests/blitz-tests/tests/accessibility_states.rs:194-209, :245-299, :301-316 beside the earlier tables — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §3: `accessibility_states` 4 → 5 tests, its fourth rewritten"
    rationale: "Report Counts / qualifiers: `acc-states` 4 → 5, stale at a11y-plan.md:69; Deviation 5; Outcome (a11y · F2) `accessibility_states` 5 passed"
    basis: ".andromeda/a11y-plan.md:69"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output (the snapshot-model bullet)"
    change: "Restate the snapshot's `checked` and `pressed` sources: `NodeState::checked` takes the native checkbox or radio reader, else, for a node whose tree role is one of the five checkable roles (its own private `is_checkable`, a copy of blitz-dom's), `toggled == Toggled::True` — `None` where the role states nothing, a mixed one `Some(false)`; `NodeState::pressed` reads `toggled` only off those five roles; so «the tree gives an element node no … checked property» and «`checked` is the checkbox or radio state» narrow to native controls, and the node keys' help that `help` serves states `checked`'s new source and its mixed-reads-false beside `pressed`'s; PROVISIONAL (F2 A); cite packages/dioxus-native-dom/src/snapshot.rs:163-175, :213-225 and packages/escher-driver/src/schema.rs:74-77, :93-94 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §7 snapshot model: `checked` reads `aria-checked`'s toggle on the five checkable roles, `pressed` off them; the node-key help restated — PROVISIONAL (F2 A)"
    rationale: "Report Changes → F2, the snapshot paragraph (snapshot.rs:163-175, :213-225) and the escher-driver node-key help (schema.rs:74-77, :93-94); Deviations 2-4"
    basis: ".andromeda/a11y-plan.md:283"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Focus order per layout (engine) (the focusability bullet)"
    change: "Restate focusability: an element is focusable if it holds a sub-document, or if it is not inert (by presence, on itself or an ancestor element — `Node::is_inert`), is not a disabled form control and has `tabindex >= 0` or is one of the natively focusable elements; `disabled` (by presence) now counts only on `button` · `input` · `select` · `textarea` (`ElementData::is_disabled` = `can_be_disabled()` and the attribute present), read by `flush_is_focussable`, so a `<div disabled tabindex=0>` takes focus — the C15 limit and its residuals line discharged by this narrowing, PROVISIONAL on the operator's word as the founder's delegate (F7 A), standing for the founder's own word; the Tab walk, autofocus and `DioxusDocument::unkeyed_actionable` read it; cite packages/blitz-dom/src/node/element.rs:490-494, :709 and packages/blitz-dom/src/node/node.rs:558-575 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5 Focus order: `disabled` read on form controls only (a `<div disabled tabindex=0>` focusable), an inert element unfocusable — PROVISIONAL (F7 A, F1 A)"
    rationale: "Report Changes → F7 `disabled` on form controls only (element.rs:490-494, :709) and F1 `inert` honoured (node.rs:558-575); Counts / qualifiers: `disabled-any` stale at a11y-plan.md:146"
    basis: ".andromeda/a11y-plan.md:146"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Focus order per layout (engine) (the «Disabled-ness is keyed one way» bullet)"
    change: "Narrow the one-way keying: the `DISABLED`/`ENABLED` state (`:disabled`), the pointer-move gate, `ClickTarget::Disabled`, `handle_click`'s walk gate, Enter and Space activation and focusability all read `ElementData::is_disabled` — the attribute's presence on a `button`, `input`, `select` or `textarea` alone — since 2026-10-18-engine-debts, PROVISIONAL (F7 A); cite packages/blitz-dom/src/node/element.rs:490-494, :709, packages/blitz-dom/src/events/pointer.rs:330, :463, :764 and packages/blitz-dom/src/events/keyboard.rs:163, :180 in place of the presence reads it names — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5: disabled-ness keyed by `is_disabled` (form controls only) — PROVISIONAL (F7 A)"
    rationale: "Report Changes → F7: its readers listed (pointer.rs:330, :463, :764; keyboard.rs:163, :180); the bullet states the same presence-on-any-element fact in other words"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Focus restoration (the button-click focus bullet)"
    change: "Replace «a `disabled` attribute still stops the walk first» with: a disabled form control (`is_disabled`) still stops `handle_click`'s walk first, while since 2026-10-18-engine-debts a disabled ancestor `div` no longer stops it, PROVISIONAL (F7 A) (packages/blitz-dom/src/events/pointer.rs:764) — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5 Focus restoration: a disabled ancestor div no longer stops the click walk — PROVISIONAL (F7 A)"
    rationale: "Report Changes → F7 consequences: «a disabled ancestor `div` no longer stops `handle_click`'s walk»"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Keyboard handling (engine and shell) (the Enter / Space activation bullet)"
    change: "Restate the activation gates: Enter and Space dispatch the Click unless the focused node is a disabled form control (`activates_on_enter` / `activates_on_space` read `is_disabled`), so since 2026-10-18-engine-debts a `<summary disabled>` or an `<a href disabled>` takes Enter / Space activation and a click, as HTML does, PROVISIONAL (F7 A); cite packages/blitz-dom/src/events/keyboard.rs:163, :180 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5 Keyboard handling: a disabled-attributed summary or link now activates — PROVISIONAL (F7 A)"
    rationale: "Report Changes → F7 (keyboard.rs:163, :180) and Deviation 9 («a `<summary disabled>` or `<a href disabled>` now takes activation and a click»)"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Focus order per layout (engine) (new bullet: `inert`)"
    change: "Add: since 2026-10-18-engine-debts the engine honours `inert` by presence on the element or an ancestor element, in every document — `Node::is_focussable` reads false for an inert node through `Node::is_inert` (an ancestor walk; the cached `ElementData::is_focussable` stays per element, no mutator hook), so the Tab walk, autofocus and `DioxusDocument::unkeyed_actionable` skip an inert subtree; known limit: HTML's focus fixup is not built — a node holding focus when it becomes inert keeps it until focus moves; PROVISIONAL on the operator's word as the founder's delegate (F1 A), standing for the founder's own word; pinned by `inert_subtree` (tests/blitz-tests/tests/inert_subtree.rs:68-93, :144-194) and `stand_keyboard::an_inert_subtree_is_skipped_by_tab`; cite packages/blitz-dom/src/node/node.rs:558-575 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5 Focus order: `inert` honoured (Tab, autofocus, actionable check skip it), the focus-fixup limit — PROVISIONAL (F1 A)"
    rationale: "Report Changes → F1 `inert` honoured (node.rs:558-575), its known focus-fixup limit; Coverage of new surfaces: `inert` → a11y focus order and tree (SC 2.4.3); Counts / qualifiers: `inert` stale at a11y-plan.md:188, :215, and «focus fixup» has 0 master rows"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Script and framework exposure (the falsy-boolean-attribute bullet)"
    change: "Replace «while the engine honours no `inert`: a truthy one takes no focusability away, and Tab reaches the buttons under `inert: true` and `inert: false` alike, owned by a `.andromeda/residuals.md` line … `stand_keyboard::the_engine_honours_no_inert` staying its pin and whether an inert subtree takes a hit unmeasured» with: since 2026-10-18-engine-debts a truthy `inert` takes focusability, hits and the tree away from its subtree (Focus order above, §6 Visibility and hit-testing) while a framework-written `inert: false` is removed and leaves the buttons reachable by Tab — the pin renamed `stand_keyboard::an_inert_subtree_is_skipped_by_tab`, the hit measured by `inert_subtree`; PROVISIONAL (F1 A); the bridge `mutation_writer.rs` untouched — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5: «the engine honours no `inert`» retired; pin renamed `an_inert_subtree_is_skipped_by_tab` — PROVISIONAL (F1 A)"
    rationale: "Report Changes → F1; Renamed tests: `stand_keyboard::the_engine_honours_no_inert` → `an_inert_subtree_is_skipped_by_tab`; Counts / qualifiers: `inert` stale at a11y-plan.md:188 (@c1163-1597)"
    basis: ".andromeda/a11y-plan.md:188"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Test harness pattern (the `stand_keyboard` bullet)"
    change: "Restate two of `stand_keyboard`'s 14 tests: «a `disabled` attribute of any value skipped, with the `<div disabled tabindex=0>` limit» → a `disabled` attribute skipping a form control and not a `div` (`a_disabled_attribute_skips_a_form_control_and_not_a_div`), and «the engine honouring no `inert`» → an inert subtree skipped by Tab (`an_inert_subtree_is_skipped_by_tab`), both since 2026-10-18-engine-debts, PROVISIONAL (F7 A, F1 A); name tests/blitz-tests/tests/stand_keyboard.rs without the two retired line ranges — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5 Test harness: `stand_keyboard`'s disabled and inert tests renamed and restated"
    rationale: "Report Renamed tests (two `stand_keyboard` renames); `stand_keyboard` 14 unchanged; Counts / qualifiers: `inert` stale and `disabled-any` stale at a11y-plan.md:215"
    basis: ".andromeda/a11y-plan.md:215"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§6 Visual Design Verification → Visibility and hit-testing (new bullet: `inert`)"
    change: "Add: since 2026-10-18-engine-debts an element carrying `inert` and its subtree leave the accessibility tree — `is_hidden_from_accessibility_tree` reads true for it, so the snapshot drops it too — and a hit returns `None` at an element carrying `inert` before any other test, so a hit passes through it; removing `inert` restores focus, hit and tree; PROVISIONAL (F1 A); cite packages/blitz-dom/src/accessibility.rs:243, :245-256, packages/blitz-dom/src/node/node.rs:1401-1410 and tests/blitz-tests/tests/inert_subtree.rs:95-122, :124-142, :144-194 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §6 Visibility and hit-testing: an inert subtree leaves the tree and takes no hit — PROVISIONAL (F1 A)"
    rationale: "Report Changes → F1 (`hit_inner` node.rs:1401-1410; accessibility.rs:243, :245-256, «the subtree leaves the tree, and the snapshot with it»); Coverage of new surfaces: `inert` → a11y focus order and tree"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output (the snapshot-model bullet, password caret clause)"
    change: "Replace «a click inside a field holding a value places the caret by the real characters' widths … index 5 after eight narrow characters and at 2 after eight wide ones … placing it by the bullet layout is owned by a `.andromeda/residuals.md` line on the founder's C11 …» with: since 2026-10-18-engine-debts a click in a password input places the caret by the bullets — `click_password_input` lays out one U+2022 per character through `BaseDocument::layout_as_text_input` (its second caller), scrolls it as paint does, finds the bullet under the point and moves the real editor to that character (a shift-click extends, two or more clicks select all), so no width of the real characters reaches the caret: the two fillings read one index, inside 1–7, in both layout modes; PROVISIONAL (F6 A); cite packages/blitz-dom/src/events/pointer.rs:586-652 and tests/blitz-tests/tests/password_caret_click.rs:106-121 in place of :83-104 — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §7: the password click caret placed by the bullets, the C11 limit fixed — PROVISIONAL (F6 A)"
    rationale: "Report Changes → F6 (pointer.rs:586-652); Deviations 1, 11; `password_caret_click` 1 → 2 tests, renamed; Counts / qualifiers: `pw-caret` stale at a11y-plan.md:283"
    basis: ".andromeda/a11y-plan.md:283"
  - detector: D-a11y-surface
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output (the stand state-by-attribute bullet)"
    change: "Replace «the missing `aria-activedescendant` is owned by a `.andromeda/residuals.md` line (target: next) on the founder's F2 …» with: the builder still reads no `aria-activedescendant`, kept unread and pinned since 2026-10-18-engine-debts by `accessibility_active_descendant_unread` (a focused listbox reads focus and no active descendant, both layout modes; tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81), on the operator's word as the founder's delegate (F3 B, no engine change), PROVISIONAL — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §7: the unread `aria-activedescendant` pinned by `accessibility_active_descendant_unread` (F3 B)"
    rationale: "Report Changes → F3 B «`aria-activedescendant` stays unread, pinned by `accessibility_active_descendant_unread`»; Counts / qualifiers: `active-desc` 1 stale owner at a11y-plan.md:285"
    basis: ".andromeda/a11y-plan.md:285"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Per-surface keyboard shortcuts (the seven_guis CRUD bullet)"
    change: "Beside «the builder reads no `aria-activedescendant` … recorded, not closed», name its pin: kept unread on the operator's F3 B (PROVISIONAL), pinned since 2026-10-18-engine-debts by `accessibility_active_descendant_unread` (tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81) — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §5 CRUD list: the `aria-activedescendant` gap now named with its pin"
    rationale: "Report Counts / qualifiers: `active-desc` true at a11y-plan.md:198, «the check to name»"
    basis: ".andromeda/a11y-plan.md:198"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract → A11y testing tool pick (new bullet after the upstream name and description files)"
    change: "Add: since 2026-10-18-engine-debts three more files assert the builder off the built tree in both layout modes — `accessibility_merged_states`, 3 tests, the merged `aria-expanded` (true and false and nothing else), `aria-disabled` (`true` alone sets disabled) and `aria-level` (its number less one, and the heading defaults), each a stated table with its count asserted (tests/blitz-tests/tests/accessibility_merged_states.rs:88-104, :106-121, :123-157); `inert_subtree`, 4 (Tab and Shift+Tab, hit, tree, and removal restoring all three; tests/blitz-tests/tests/inert_subtree.rs:68-194); and `accessibility_active_descendant_unread`, 1 (tests/blitz-tests/tests/accessibility_active_descendant_unread.rs:39-81); they run in the workspace test leg and per package, outside the a11y leg's six files (§9) — as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md"
    sidecar: "a11y-plan §3: three new a11y checks — `accessibility_merged_states` (3), `inert_subtree` (4), `accessibility_active_descendant_unread` (1)"
    rationale: "Report Changes → Files (five new checks), the CARRY «observed absent» → present (`accessibility_merged_states`, 3 tests), Outcome (a11y · F1) `inert_subtree` 4 passed, Harness / gate surface unchanged (a11y leg not widened)"
note: "D-a11y-obs-schema not fired — the report states no violation schema, log field, span or scrub set changed. Every F1/F2/F3/F6/F7 restatement above is PROVISIONAL on the operator's word as the founder's delegate (inputs#I2), standing for the founder's own word under the Provisional discharge card (inputs#I7); F7 overrides the founder's ratified presence rule and his C15 deferral, and F2 his C7 deferral, so the apply should carry the PROVISIONAL mark rather than a ratification. a11y-plan.md:47 (merged-aria) and :167 (range gate) read true and are left. Five sites above are cited by section and wording alone: their line numbers are not stated in the report."
