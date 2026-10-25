# Task — a11y · 2026-10-25-capability-statement

## The chunk
Writes one capability statement, current with 0.2.0: a document, not code. It says what escher can do (the driver's twelve verbs on their carriers, stable element ids, the snapshot and its text, parts and diff, the screenshot, the lint's six defects, recording and replay, the load wait, the select model, the range input, keyboard activation and the one virtual clock) and gathers the limits already stated in one place.
It states escher's a11y reach: what the tier-0 target (SC 2.1.1 · 1.4.3 · 2.4.3) is proven on (the stand's Tab order, its Enter/Space activation, its contrast rows and the fourteen-file `a11y` leg), and what is not measured (an assistive technology on a windowed run, which a residuals line owns). These claims are checked against the a11y plan and the residuals at P3.
It also states what an MCP client carries onward, and it fixes README's MCP verb count from eleven to twelve (CARRY 1). It states as a limit that the diff and the lint findings are not paged (CARRY 2).
No code, engine, schema, verb, carrier, test, log field or span changes. Where the statement lives is a P4 fork: README, a tracked escher document that README points to, or Viola's incubator.

## Reached files
- README.md · edits its MCP-role line (the verb count) and its 0.2.0 stage list

## Needs
- The a11y tier and its three success criteria, SC 2.1.1 · 1.4.3 · 2.4.3, which the statement names as its a11y reach
- The stand's keyboard proof: `stand_keyboard`'s Tab and Shift+Tab walk over `tab_order` and `ENABLED_TAB_ORDER`, with the engine's focus, the snapshot's `focused` node and the tree's focus agreeing; and Enter and Space on each stand button counting as its click (SC 2.1.1, SC 2.4.3)
- Keyboard activation's reach: which focused elements Enter and a released Space activate, gated so that no disabled form control is activated. The statement names this as a capability
- The range input's keyboard and pointer model (arrows, Home/End, pointerdown, a click that focuses without moving the value, and one `input` event per change), named as a capability
- The single-choice select's keyboard model (arrows, Home/End, 1 s typeahead on the document's clock, `input` then `change`), plus its unmodelled parts as stated limits: `multiple`, `optgroup` and no popup
- Focusability as stated: `disabled` counts by presence on a form control alone, and nothing inert or under an inert element is focusable. HTML's focus fixup is not built (a node keeps focus while it turns inert), a stated limit owned by a residuals line
- The CRUD list `crud-list` named `People` as a keyboard region; its rows are not focusable and `aria-activedescendant` is left unread. This is a reach gap the statement may state
- SC 1.4.3 on the stand: `stand_contrast`'s three tables (18 control rows, 14 `TEXT_ROWS`, 19 `STATE_ROWS`) in both layout modes, met on every measured non-exempt row; the exempt rule, where `enabled` reads `Some(false)`; and the accent value that brought every row to the target
- `Harness::text_contrast` with the WCAG 2.x ratio (4.5, or 3.0 for large text), and the cases where it reads `None` (opacity below 1, a filter, a background image). This reaches the statement through the lint's `contrast` defect, which uses the same exempt rule
- The fourteen-file `a11y` CI leg and the eight of those files that also run three rounds in `stand-repeat`, cited as the reach's evidence
- The platform-adapter gap: no stand binary builds a platform adapter, so no assistive technology is given a tree. The windowed AT witness, the tree's adapter refresh and the description reads at an assistive technology are all unmeasured, each owned by a residuals line. This is the statement's "not measured" half
- The accessibility tree's state reads that feed the snapshot: `selected` · `toggled` · `invalid` from `aria-selected` · `aria-pressed` / `aria-checked` (by role) · `aria-invalid`. The tree never states a native checkbox or radio as checked, a stated limit owned by a residuals line
- The boolean model's two limits, which reach the statement through the snapshot's node-key help: a mixed toggle reads false and not pressed, and a grammar or spelling error reads invalid
- The snapshot consumes the tree for role, name, focus and states. `enabled` reads the `disabled` attribute, so `aria-disabled` moves no reading, and the snapshot reads no description. These are stated limits of what an agent sees
- The name computation the snapshot's `name` carries: `aria-labelledby` outranks a `<label>` and an `aria-label`, and an `aria-label` that is only whitespace names nothing
- Stable element ids carried as AccessKit `author_id`, and the actionable-key rule checked by `unkeyed_actionable`. No snapshot field says an element reacts to a click, a gap owned by a residuals line
- Masked values: a password's or a file input's value reads `MASKED_VALUE` in the snapshot, and a password is painted as bullets in a shot. These are part of what an MCP client carries onward
- Hidden and inert exclusion: `hidden`, `display: none`, `visibility: hidden` and `inert` keep a node out of the tree and the snapshot, with a select's option the exception to `display: none`. A hit passes through an inert element
- The flight booker's invalid dates, stated by `aria-invalid` as well as colour, so the cue reaches the tree and the snapshot. The statement may cite this as state carried without colour alone

## Not needed
- The assertion harness's internals (`assert_role`, `unknown_tags`, the `test_that` / `verify_that!` matchers, the per-file test counts): the chunk writes and changes no test
- The role-derivation TODO on overriding strong native semantics, and the HTML-AAM landmark mapping inventory: the statement claims no landmark or role-mapping conformance
- The changed set's lifecycle (what marks `changed_nodes`, when `take_changed_nodes` drains it, the shell's poll refresh): the statement does not describe an engine-internal invalidation contract
- The Dioxus bridge's falsy boolean-attribute handling (the 28 `is_bool_attr` attributes removed on a false value): the statement names no framework-bridge behaviour
- The rule that the AccessKit `Adapter` must be created before the window is shown: no window or adapter is built or described as built
- Language and direction (the missing `html lang` in fixtures), text alternatives in fixtures, and IME: not among the capabilities or limits the statement gathers
- The focus ring, motion, colour scheme, zoom and user-preference sections: they are not measured, and the statement makes no claim about them
- The cognitive orientation and status cues, and the cognitive items marked NOT YET MEASURED (timeouts, plain language, redundant entry, authentication): outside the tier-0 reach the statement states
- The a11y violation schema and log format marked NOT YET MEASURED, and the obs sink's line format: no log field or span changes
- The empty sections: SLO invariants and budgets, anti-patterns and the decisions log
