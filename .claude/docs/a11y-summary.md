# Accessibility Summary — escher

_Distilled from `.andromeda/a11y-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## A11y tier

**Tier:** 0
**WCAG target:** SC 2.1.1 Keyboard · SC 1.4.3 Contrast (Minimum) · SC 2.4.3 Focus Order

## Harness contract (§3)

- **Tool:** Rust tests in `tests/blitz-tests` over the AccessKit tree — `build_accessibility_tree()` (or `Document::accessibility_tree()`, which a `DioxusDocument` enriches with each element's stable id as `author_id`) after `resolve`, `test_that` matchers on `role()` / `is_hidden()`, `assert_role(html, id, expected)` and `unknown_tags(html)` helpers.
- **Focus harness:** `Harness::focused()` / `hovered()`.
- **Keyboard harness:** observed absent (no Tab-order or key-event tests).
- **Contrast harness:** observed absent (a Chrome-matching contrast-ratio helper exists in `blitz-paint/src/color.rs`).
- **Structured violation JSON / WCAG mapping of existing tests:** NOT YET MEASURED.

## Engine behaviour (measured)
- Roles: `role` attribute → HTML-AAM mapping → `Role::Unknown`; landmarks, lists, tables, form inputs by `type`, links only with `href`.
- Hidden: `hidden` / `display:none` / `visibility:hidden` exclude the subtree; `aria-hidden` marks hidden.
- Focus: Tab / Shift+Tab via `focus_next_node` / `focus_prev_node`; focusability recomputed on `tabindex` / `href` / `disabled`; focus resets to body on removal; `:focus-visible` / `:focus-within` never match.
- Disabled-ness is keyed two ways: the `DISABLED` state (`:disabled`) and click targeting on the attribute's presence, focusability on its value parsed as a bool — `disabled="false"` matches `:disabled` yet stays focusable. dioxus-native-dom now removes a falsy `disabled` / `checked`; its other boolean attributes still write `"false"`.
- Stand cue: `stand_flight_booker.rs` asserts the invalid-date cue (`invalid` class + `disabled` Book) without colour; `stand_accessibility_ids.rs` pins each lean task's Tab sequence (`focus_next_node` from the root element) and asserts the 15 stand controls' roles and non-empty names; `stand_actionable_keys.rs` asserts every actionable element of the lean tasks and Home reads an author key (`DioxusDocument::unkeyed_actionable()` empty, in every state), with no `tabindex`, role or Tab-order change; no test asserts SC-level keyboard behaviour.
- Platform: accesskit_xplat adapters (windows, macos, unix, android, null); ActionRequested and AccessibilityDeactivated are unhandled TODOs; the tree is rebuilt on poll when the document changed, through `Document::accessibility_tree`. Names come from text children, a trimmed-non-empty `aria-label` and `<label>` association (`for`-target or nested `<input>`). The seven_guis stand binary enables no `accessibility` feature, so the windowed stand builds no platform adapter.

## Critical paths (must-be-accessible)
> NOT YET MEASURED — the 0.1.0 route makes the stand controls the target: every stand control reachable (2.1.1), in order (2.4.3), with measured contrast (1.4.3).

## Bootstrap phases (owners on the working route)
1. `contrast-verification-harness-setup` → "Stand contrast harness"
2. Headless keyboard dispatch with focused-node read-back → "Stand keyboard harness"
3. Stable id + role + name on every accessibility node → "Accessibility-tree identity" (tree leg built: `author_id` on every element node, 15 stand controls named; snapshot leg built: `DioxusDocument::snapshot` takes its ids, roles, names and focus from that tree; the driver leg waits on Epoch 4)
4. SC assertions on every stand control, gating merges through the a11y CI leg (the `a11y` job runs the accessibility integration tests — `a11y-ci-gate-wire` discharged) → "Stand a11y assertions"

## Universal anti-patterns
> NO RECORDED INTENT.

## Critical decisions
> NO RECORDED INTENT.

---

**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `.claude/rules/a11y.md`.
