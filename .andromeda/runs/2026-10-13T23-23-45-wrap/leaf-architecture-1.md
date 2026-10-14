group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «read `Some(false)` while both stay focusable)»
  source: .andromeda/architecture.md:136 «a `disabled="false"` and a bare `disabled` both read `Some(false)` and, since 2026-10-13-stand-keyboard-harness, are not focusable either — the engine's focusability reader keys on presence too»
  replace-with: «read `Some(false)` and, since 2026-10-13-stand-keyboard-harness, are not focusable either — the engine's focusability reader keys on presence too)»

- leaf: .claude/docs/services/dioxus-native-dom.md:23   class: omission
  leaf-text: «a framework-written `inert: false` is now cleared)»
  source: .andromeda/architecture.md:136 «a framework-written `inert: false` is cleared — and the engine honours no `inert`, so neither a truthy nor a falsy one makes content inert»
  replace-with: «a framework-written `inert: false` is cleared — and the engine honours no `inert`, so neither a truthy nor a falsy one makes content inert)»

- leaf: .claude/docs/services/dioxus-native-dom.md:41   class: stale
  leaf-text: «`dioxus_falsy_boolean_attrs.rs` the 27 names of its own list (the bridge's predicate is upstream's 28-name `is_bool_attr` since 2026-10-13-upstream-sync-polish-and-ship; whether a check reads a falsy `inert` is not measured).»
  source: .andromeda/architecture.md:263 «`inert` the 28th name the check reads since 2026-10-13-stand-keyboard-harness, the attribute half only — `stand_keyboard` pins that the engine honours neither a truthy nor a falsy `inert`»
  replace-with: «`dioxus_falsy_boolean_attrs.rs` the 28 names of its own list (the bridge's predicate is upstream's 28-name `is_bool_attr` since 2026-10-13-upstream-sync-polish-and-ship; `inert` the 28th name the check reads since 2026-10-13-stand-keyboard-harness, the attribute half only — `stand_keyboard` pins that the engine honours neither a truthy nor a falsy `inert`).»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «form submission sends the formatted current value.»
  source: .andromeda/architecture.md:87 «three more direct engine changes for every document stand beside those since 2026-10-13-stand-keyboard-harness, the sixth to the eighth: keyboard activation»
  replace-with: «form submission sends the formatted current value. Since 2026-10-13-stand-keyboard-harness three more direct engine changes for every document (answered by the founder, flagged upstreamable), the sixth to the eighth: keyboard activation — a pressed `Enter` on the focused node, after the text-input and range arms, dispatches a Click on a `button`, a `summary`, an `input` of type `submit`, `reset` or `button`, or an `a` with `href`, and a released Space (`DomEventData::KeyUp`'s new default action, routed by `events/mod.rs` to the crate-internal `handle_key_up`) the same Click on a `button`, a `summary`, or an `input` of type `submit`, `reset`, `button`, `checkbox` or `radio` — neither with a `disabled` attribute, a link taking no Space and a text or range input neither key — both through `dispatch_activation_click` (data `Node::synthetic_click_event_data(modifiers)`, queued by the event driver so the app's handlers run before the default action `handle_click`); a click focuses a button — `handle_click`'s button arm focuses a `button` of any type or an `input` of type `submit`, `reset` or `button` through `generate_focus_events` / `set_focus_to` before it submits a submit button's form owner, a click on any other element still clearing focus; and focusability reads `disabled` by presence (`ElementData::flush_is_focussable` reads `has_attr(disabled)`), so a bare, a `"false"` and a `"true"` `disabled` on any element remove focusability — a `<div disabled tabindex=0>` included, a known limit pinned by `stand_keyboard`.»

- leaf: .claude/docs/gotchas.md:41   class: omission
  leaf-text: «subgrid/masonry are absent.»
  source: .andromeda/architecture.md:86 «`inert` is not implemented — a truthy `inert` takes no focusability away, so Tab reaches a button under `inert: true` as under `inert: false`»
  replace-with: «subgrid/masonry are absent; `inert` is not implemented — a truthy `inert` takes no focusability away, so Tab reaches a button under `inert: true` as under `inert: false`.»
