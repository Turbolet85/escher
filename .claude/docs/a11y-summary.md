# Accessibility Summary — escher

_Distilled from `.andromeda/a11y-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## A11y tier

**Tier:** 0
**WCAG target:** SC 2.1.1 Keyboard · SC 1.4.3 Contrast (Minimum) · SC 2.4.3 Focus Order

## Harness contract (§3)

- **Tool:** Rust tests in `tests/blitz-tests` over the AccessKit tree — `build_accessibility_tree()` (or `Document::accessibility_tree()`, which a `DioxusDocument` enriches with each element's stable id as `author_id`) after `resolve`, `test_that` matchers on `role()` / `is_hidden()`, `assert_role(html, id, expected)` and `unknown_tags(html)` helpers.
- **Focus harness:** `Harness::focused()` / `hovered()`.
- **Keyboard harness:** a Tab and Shift+Tab key-press focus check exists on the stand, driven through the harness's `press` / `press_with` (`stand_snapshot_state.rs`); the headless keyboard harness itself is still to come.
- **Contrast harness:** observed absent (a Chrome-matching contrast-ratio helper exists in `blitz-paint/src/color.rs`).
- **Structured violation JSON / WCAG mapping of existing tests:** NOT YET MEASURED.

## Engine behaviour (measured)
- Roles: `role` attribute → HTML-AAM mapping → `Role::Unknown`; landmarks, lists, tables, form inputs by `type`, links only with `href`.
- Hidden: `hidden` / `display:none` / `visibility:hidden` exclude the subtree; `aria-hidden` marks hidden.
- Focus: Tab / Shift+Tab via `focus_next_node` / `focus_prev_node`; focusability recomputed on `tabindex` / `href` / `disabled`; focus resets to body on removal; `:focus-visible` / `:focus-within` never match.
- Disabled-ness is keyed two ways: the `DISABLED` state (`:disabled`) and click targeting on the attribute's presence, focusability on its value parsed as a bool — `disabled="false"` matches `:disabled` yet stays focusable. dioxus-native-dom removes a falsy value of its 27 boolean attributes (`disabled`, `checked`, `hidden` among them), so a `hidden: false` element stays displayed, in the tree and in the snapshot; an attribute outside the list holding `false` is still written as `"false"`. The snapshot reads `enabled` by presence — the driver's `disabled` refusal cause is stated on the same presence reading ("the element carries the `disabled` attribute, which the snapshot reads as not enabled") and is detected on it and on nothing else: the driver refuses a `click` or a `type` as `disabled` when the snapshot reads the element not enabled, with nothing dispatched and focus as it was, and acts on the same control once it reads enabled (`stand_act_disabled.rs`), and the driver's schema defines no role or name set of its own — `checked` and `value` from the element (a password or file-input value reads the fixed mask `MASKED_VALUE`), and `focused` from the tree — proven after a click, a Tab press and Shift+Tab (`stand_snapshot_state.rs`). The snapshot's text form (`Snapshot::to_text`) reads that model, never the tree or a control: one line per node, its role and name the node's, with no second mapping (`stand_snapshot_text.rs`). The snapshot's diff (`Snapshot::diff`) is one more reader of that model: a text change is reported on the element whose accessible name it changes, by its id and never as a text-run entry, a hidden element leaves and returns with its descendants, and a focus move names exactly the controls whose `focused` reading moved (`stand_diff.rs`).
- Stand cue: `stand_flight_booker.rs` asserts the invalid-date cue (`invalid` class + `disabled` Book) without colour; `stand_accessibility_ids.rs` pins each lean task's Tab sequence (`focus_next_node` from the root element) and asserts the 15 stand controls' roles and non-empty names; `stand_actionable_keys.rs` asserts every actionable element of the lean tasks and Home reads an author key (`DioxusDocument::unkeyed_actionable()` empty, in every state), with no `tabindex`, role or Tab-order change; no test asserts SC-level keyboard behaviour.
- Platform: accesskit_xplat adapters (windows, macos, unix, android, null); ActionRequested and AccessibilityDeactivated are unhandled TODOs; the tree is built on InitialTreeRequested and rebuilt on a poll that reported work when the document's changed set, taken by that poll, was non-empty — a focus move, a checked change, a rewritten name and other mutations mark it, a pointer move alone does not — through `Document::accessibility_tree` (ratified by the founder, 2026-10-07; before 2026-10-07 the poll-time refresh never ran; no windowed witness on the dev host — still owed, on "Stand a11y assertions"). A harness settle neither reads nor drains that changed set, and the set is non-empty on a freshly booted harness document — a check reading the flag as "this step changed something" drains first. Names come from text children, a trimmed-non-empty `aria-label` and `<label>` association (`for`-target or nested `<input>`). The seven_guis stand binary enables no `accessibility` feature, so the windowed stand builds no platform adapter.

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
