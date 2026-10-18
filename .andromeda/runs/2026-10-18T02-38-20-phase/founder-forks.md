# Founder forks — 2026-10-18-engine-debts «Engine debts» (v020-03)

Nine forks, one per recorded engine debt. They are the founder's, and he is away. On the operator's directive
(inputs#I1) they are written here whole. The operator answers them in his stead, and each answer is recorded as a
delegate's (`delegate operator`), PROVISIONAL.

**How to answer.** Give one line per fork, e.g. `F1: A · F2: C · …`. A remark beside a letter is folded in as
written. A fork left unanswered is not guessed: P4 stops on it.

**What a delegate's answer does at the wrap.** Every «fix» option below is an engine behaviour change for every
document (`.andromeda/playbook.md:64`, «An engine behaviour change for every document»). On the founder's own word
that pattern is routine. On a delegate's word it is PROVISIONAL and stays marked until the founder discharges it in
one batch at an epoch boundary (`.andromeda/playbook.md:48`, «Provisional discharge», verdict escalate). This chunk
closes Epoch 2, so its own wrap is that boundary: it lists every PROVISIONAL mark this chunk leaves, for the founder
to rule on. Where an option also meets a second pattern, the fork names it.

**Shared by every «fix»:**
- the change is proven by a check on a fixture (or the stand) in both layout modes;
- the debt's standing pin is rewritten to the fixed reading, red on the unfixed tree;
- the merge surface is named by file against the merge base `23b9b26a9`, and owned by a CARRY on the next «Upstream
  sync» entry (Epoch 3);
- no upstream PR is sent (the founder's 2026-10-07 hold stands).

**Shared by every «pin»:** the limit is stated where an agent or a reader meets it, and a named check holds it.

**Shared by every «keep»:** nothing changes. The wrap appends a fresh `open` residuals line (target: next).

Recommendations are marked **(Recommended)** and argued in one line. Every coordinate cited is research.md's
(`escher-0.2.0/chunks/2026-10-18-engine-debts/research.md`).

---

## F1 — `inert` is not honoured
**Today:** a truthy `inert` removes nothing. Tab reaches a button under `inert`
(`stand_keyboard::the_engine_honours_no_inert` pins it); whether an inert subtree takes a hit is unmeasured; the
tree keeps it. The bridge already removes a framework-written `inert: false`. No stand file sets `inert`.
**His earlier word:** F3 «Оставить, перенести (Рекомендую)» (2026-10-14).

- **A — honour it fully, as HTML specifies (Recommended).** An element in an `inert` subtree:
  - is not focusable (the Tab walk, autofocus and `unkeyed_actionable` skip it);
  - takes no hit, which passes through it as under `pointer-events: none`;
  - is left out of the accessibility tree, as `hidden` is, so the snapshot no longer lists it.

  An `inert` write recomputes the subtree's focusability. Files: `node/element.rs`, `mutator.rs`, `node/node.rs`
  (`hit_inner`), `accessibility.rs`. A driver `click` on content under a modal then reads `not-found` (it is gone
  from the snapshot), not a false `covered`.
  *Why it wins:* the first real apps will use modal dialogs, and an agent should not see or act on what a user
  cannot reach. Leaving it out of the tree is one more check beside `hidden`.
- **B — focus and hit only.** As A, but the tree and the snapshot keep inert content. An agent still sees the
  controls under a modal, and a click on one reads `covered`. The tree reading is stated as a limit, pinned by a
  check. *Trade-off:* a smaller change, but the snapshot keeps listing what a user cannot reach.
- **C — pin as a stated limit.** Measure the hit on an inert subtree (today unmeasured), state the limit in
  `.claude/rules/a11y.md`'s source (a11y-plan §5) and keep the pin. No engine change.
- **D — keep.** The residual stands as written.

**Dependences:**
- A and B edit `flush_is_focussable` (also F7's site) and `hit_inner` (also F5's site). Each composes with
  either answer there.
- A changes what the snapshot lists, which no stand screen moves today (no stand `inert`).

## F2 — `aria-checked` is not read
**Today:** the builder reads `aria-selected`, `aria-pressed` and `aria-invalid` on every element, whatever its role
(no role consulted), and not `aria-checked`. An element stating `role="checkbox"` or `role="switch"` with
`aria-checked` reads its role and no checked state. AccessKit has one `toggled` property for pressed and checked.
`accessibility_states`' fourth test pins the gap. No stand task states `aria-checked`.
**His earlier word:** C7 «Отложить (Рекомендую)» (2026-10-14).

- **A — read it as `checked`, role-aware (Recommended).**
  - The builder writes `toggled` from `aria-checked` (`true` · `false` · `mixed`, the same closed set) on an element
    whose role is `checkbox`, `switch`, `radio`, `menuitemcheckbox` or `menuitemradio`. `aria-pressed` keeps its
    read on every other role.
  - The snapshot's `checked` takes that `toggled` when the native reader gives none, and its `pressed` reads
    `toggled` only off those roles.
  - Files: `accessibility.rs`, `dioxus-native-dom/src/snapshot.rs`. The node keys' help moves.

  *Why it wins:* custom switches are common in component libraries, and today they read as stateless. A closed
  boolean in the existing `checked` field is a field inside a ratified carrier (`.andromeda/playbook.md:40`,
  routine), not a widening.
- **B — fold it into `pressed`.** The builder writes `toggled` from `aria-checked` on every role, as it does from
  `aria-pressed`, and the snapshot reads it as `pressed`. No role is consulted. *Trade-off:* the smallest change,
  but a checkbox reads «pressed», a word that misleads an agent.
- **C — pin as a stated limit.** The gap is stated in the snapshot's node-key help (an agent learns it from `help`),
  and the fourth test stands.
- **D — keep.**

**Dependences:**
- A and B edit `accessibility.rs` beside F3's read.
- A moves dioxus-native-dom's snapshot reader. Its coverage floor is 83 %, read at 84.25 %, so the margin is thin.

## F3 — `aria-activedescendant` is not followed
**Today:** `crud-list` holds focus, and its keys move the selection. The tree's focus is the list, never the
option, and the builder reads no `aria-activedescendant`. This is SC 4.1.2 work outside the tier-0 target
(SC 2.1.1 · 1.4.3 · 2.4.3). The gap is stated in a11y-plan §5 and layout-templates. AccessKit 0.25.1 has
`set_active_descendant`. The stand binary builds no platform adapter, so no assistive technology reads the tree
today. That windowed witness is re-carried and out of scope.
**His earlier word:** F2 «Оставить записанным (Рекомендую)» (2026-10-14).

- **A — read it, adapter-side.**
  - The builder sets `active_descendant` on an element whose `aria-activedescendant` names a built node (an id
    naming none is ignored, as `aria-describedby`'s are). The tree's focus and the snapshot's `focused` stay on the
    list, so `stand_keyboard`'s three-way focus agreement holds.
  - The stand's `crud-list` states `aria-activedescendant` on the selected row, an attribute only.
  - Files: `accessibility.rs`, `examples/seven_guis/src/tasks/crud.rs`.
  - Its value is proven only headlessly, on the built tree.
  - A node relation the platform adapter did not receive before arguably widens the adapter crossing
    (`.andromeda/playbook.md:36`, «Boundary widening», verdict escalate).
- **B — pin as a stated limit (Recommended).** A named check holds the reading (tree focus on the list, no active
  descendant set). The stated gap stands where it is.
  *Why it wins:* it is outside the tier-0 target. The stand ships no adapter, so a fix reaches no assistive
  technology and nothing measures it past the headless tree. It is also the only fix option here that touches a
  boundary pattern.
- **C — keep.**

**Dependences:**
- A edits `accessibility.rs` beside F2.
- A's stand attribute is a stand markup change under layout-templates' attributes-only rule.

## F4 — `Node::absolute_position` misreads a self-scrolled box
**Today:** for a box that is itself scrolled, the reader subtracts the box's own scroll offset (`node.rs:1684-1685`).
A box at (40, 50) scrolled by (70, 120) reads (−30, −70) (pinned by `scrolled_box_absolute_position`).
- Its engine readers: `handle_dom_event`'s target position, the IME cursor area on focus, the synthetic click
  point, and `scroll_into_view`, which adds the offset back.
- Its harness readers: `layout_rect_of`, `layout_rect` and `center_of` (via `write_node`).

**His earlier word:** C14 «Отложить на следующую версию (Рекомендую)» (2026-10-14). This is that next version.

- **A — fix it (Recommended).** Subtract only the ancestors' scroll, drop `scroll_into_view`'s add-back, and move
  every reader with it. The pin is rewritten to (40, 50), and the harness readings on a scrolled box move. Files:
  `node/node.rs`, `scrolling.rs`; tests that read the harness on a scrolled box.
  *Why it wins:* it is a plain bug with one cause and a measured reading. A scrolled textarea's IME area and a
  scrolled box's synthetic click point are wrong today.
- **B — pin as a stated limit.** It is already pinned. The limit is stated in the harness docs.
- **C — keep.**

**Dependences:**
- None on another fork's answer.
- The geometry checks run under both builds (`writing-mode` decides which body compiles).
- F8 touches a different reader.

## F5 — the hit does not stop at a `contain: paint` box
**Today:** the hit stops at the padding box of a box that clips by `overflow` (`node.rs:1476-1495`). It does not
stop at a `contain: paint` box whose `overflow` is `visible`, so a control under such clipped content reads
`covered` though nothing shows over it. Paint already clips `contain: paint`, `strict` and `content`
(`blitz-paint/src/render.rs:327`). The limit is pinned by `hit_clipped_at_scrolling_box` and stated as the first limit
sentence of `covered`'s meaning. No stand file sets `contain`.
**His earlier word:** C4 «Отложить (Рекомендую)» (2026-10-14).

- **A — fix it (Recommended).** The clip test also stops at a box whose `contain` includes paint. The pin is
  rewritten, and the `contain: paint` sentence leaves `covered`'s meaning (the `help` line moves on the command line
  and MCP). Files: `node/node.rs`, `escher-driver/src/refusal.rs`, `lib.rs`.
  *Why it wins:* one condition beside an existing one. Hit and paint would then agree, and the tool loses a caveat
  it has to explain.
- **B — pin as a stated limit.** It is already pinned and stated. Nothing changes.
- **C — keep.**

**Dependences:**
- A edits `hit_inner` beside F1's A or B.
- A edits `covered`'s meaning, the same text F9's A edits. The two compose: one sentence leaves, one joins.

## F6 — a clicked caret in a password field reveals the width class
**Today:** a click inside a filled password places the caret by the real characters' widths (`pointer.rs:545`),
while paint draws bullets and the caret at the bullet of its index. Eight narrow characters give index 5 and eight
wide ones give 2 at the same click (`password_caret_click`). So a shot says something of what was typed.
security-plan states it a known 0.1.0 limit.
**His earlier word:** C11 «Замерить сейчас, исправить потом (Рекомендую)» (2026-10-14). It was measured then, and
this is the «later».

- **A — fix it (Recommended).** For a password input, place a click (and a shift-click, double-click and
  triple-click) by the bullets' layout (`layout_as_text_input`, the mask's own reader), then map the bullet index to
  the real text's position. The pin is rewritten: equal-length values give the same index. The security limit is
  retired. Files: `events/pointer.rs` (a private named function, as the existing arms are).
  *Why it wins:* it is a confidentiality leak in every document, the fix reuses the mask's own layout, and it closes
  a stated security limit.
- **B — pin as a stated limit.** It is already pinned and stated. Nothing changes.
- **C — keep.**

**Dependences:** none.

## F7 — `disabled` removes focusability on any element
**Today:** focusability reads `disabled` by presence on any element (`element.rs:703`), so `<div disabled
tabindex=0>` is not focusable where browsers focus it (pinned by
`stand_keyboard::a_disabled_attribute_of_any_value_is_skipped`). The same presence read on any element gates:
- the click default action (`ClickTarget::Disabled`, `pointer.rs:462`);
- Enter and Space activation of `a` and `summary` (`keyboard.rs`, `activates_on_enter` / `activates_on_space`).

The `:disabled` state and the snapshot's `enabled` already read it only on `button` · `input` · `select` ·
`textarea` (`can_be_disabled`). No stand file sets `disabled` outside those.
**His earlier word:** C15 «Оставить по наличию, отложить (Рекомендую)» (2026-10-14).

- **A — narrow every reader to form controls, as browsers do (Recommended).** Focusability, the click target and the
  activation gates read `disabled` only where `can_be_disabled()` holds, still by presence there. That gives one
  rule across the engine: `<div disabled tabindex=0>` focuses, `<a href disabled>` activates on Enter, and a
  disabled button stays unfocusable and inert to clicks. Files: `node/element.rs`, `events/pointer.rs`,
  `events/keyboard.rs`. No stand table moves.
  *Why it wins:* `:disabled` and the snapshot already follow this rule, so today the engine holds two rules and A
  leaves one. Keeping presence on form controls keeps his 2026-10-13 «По наличию» ruling where it applies.
- **B — narrow focusability alone.** As A for focusability only. The click target and activation keep presence on
  any element. *Trade-off:* a smaller diff, but a focusable `<div disabled>` then ignores its own click default.
- **C — pin as a stated limit.** It is already pinned and stated.
- **D — keep.**

**Dependences:**
- A and B edit `flush_is_focussable` beside F1's A or B, and they compose.
- `fieldset`, `optgroup` and `option` stay outside `can_be_disabled` either way. That is a stated limit, not this
  fork.

## F8 — a transformed box's client rect reads its untransformed position
**Today:** `get_client_bounding_rect` reads a box under `transform: translate(50px, 30px)` at its layout position
(0, 40, 300, 140), where CSSOM View includes transforms. The snapshot's `bounds` is that reader
(`snapshot.rs:139`), so a transformed control is listed, and clicked by the driver, where it is not drawn. The hit
does apply transforms, so such a click can read a false `covered` or land on a neighbour. The reading is pinned as a
fixture condition in `hit_clipped_at_scrolling_box`.
**His earlier word:** none. It was placed by the overseer's directive at the 2026-10-14 wrap.

- **A — fix the client rect (Recommended).** `get_client_bounding_rect` returns the bounding box of the border box
  under its own and its ancestors' transforms. The snapshot's bounds and the driver's click point move to where the
  box shows, and the script's `getBoundingClientRect` gets CSSOM's answer. `offset_rect` stays untransformed, as
  CSSOM's `offsetTop` is, so the change stays out of the shared `physical_unrounded_geometry`. Files: `document.rs`;
  the pin is rewritten.
  - **Stated limit it leaves:** `visible_region` still reads untransformed clip boxes, so a target inside a
    transformed scroller can misread `off-screen`. That is pinned, not fixed.

  *Why it wins:* the driver clicks where the hit then looks, so hit and click agree on transformed controls.
- **B — fix the client rect and `visible_region` together.** As A, and the visible-region reader also applies
  transforms. *Trade-off:* no limit is left, but it is the largest change of the nine, and `visible_region` is
  escher's own widening under the into-view rules (design-system §Motion).
- **C — pin as a stated limit.** The limit is stated in the snapshot `bounds` node-key help, and a named check holds
  it.
- **D — keep.**

**Dependences:**
- A and B move the snapshot's `bounds`, with no effect on the stand: no stand file sets `transform` (`transform`
  over examples/seven_guis/src: 0 matches).
- F4 is a different reader.
- Both readers run under both builds.

## F9 — `covered` on a zero-height box
**Today:** a driver `click` on `task-header-spacer` in each lean task is refused `covered`. The box is 0 high, and
the hit at its centre answers the header that holds it. `timer-progress`, 0 wide at boot, is hit at its own centre
and accepted, so a zero size does not predict the reading (pinned by `stand_act_ids::refused_at_boot`). The two
counts his instruction asks for at take-up both read 0:
- 0 spacer calls of 43 in the 0.1.0 cold run (escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-run.md:55);
- 0 in the 2026-10-10 live trial (.andromeda/residuals.md:28).

**His earlier word:** «Отложить на 0.2.0, наблюдать (Рекомендую)» (2026-10-15): «read both counts, then decide the
words or the box». This fork changes the driver and the stand, not the engine.

- **A — the words (Recommended).** `covered`'s meaning gains a limit sentence: a box with no height or no width is
  hit at its centre on what holds it, so it can read `covered`. The pin stands. Files: `escher-driver/src/refusal.rs`,
  `lib.rs`; the `help` line moves on the command line and MCP.
  *Why it wins:* agents made 0 calls on it in two live runs. The words cost one fixed sentence and tell an agent the
  truth, while the box exists only in the stand.
- **B — a reading: refuse a zero-area target `off-screen`.** It is checked before the hit, and `off-screen`'s
  meaning gains «or has no area». *Trade-off:* the refusal is truer, but `timer-progress` is refused at boot where it
  is accepted today, `stand_act_ids`' rows move, and the remedy («scroll it into view») does not fit a box with no
  area.
- **C — the box: take the spacer out of the stand's TaskShell.** It is the header's last child, so `flex: 1` lays
  out nothing visible. Its id leaves the screens, and `stand_act_ids`' refused rows go 7 → 3. *Trade-off:* the stand
  reads clean, but a real app's zero-size boxes still read `covered` with no word said.
- **D — keep, observe one more version.**

**Dependences:**
- A edits `covered`'s meaning beside F5's A. The two compose.
- B and C move `stand_act_ids`' refused-row count (7) and its asserted totals.

---

## Answers (filled in on the operator's reply — `delegate operator`, PROVISIONAL)
Answered 2026-10-18 02:53Z by the operator in the founder's stead (`delegate operator`), PROVISIONAL, each the
recommended option «for the reason the file argues» — relay-2.md, snapped as inputs#I2:

| Fork | Answer | Outcome for the debt |
|---|---|---|
| F1 `inert` | A | fixed — not focusable, takes no hit, out of the tree and the snapshot |
| F2 `aria-checked` | A | fixed — role-aware, read as `checked` |
| F3 `aria-activedescendant` | B | pinned — a named check holds the tree focus on the list, no active descendant |
| F4 `Node::absolute_position` | A | fixed — only the ancestors' scroll subtracted |
| F5 `contain: paint` hit | A | fixed — the hit stops at a paint-containing box; its `covered` sentence leaves |
| F6 password click caret | A | fixed — placed by the bullets' layout |
| F7 `disabled` focusability | A | fixed — every reader narrowed to `can_be_disabled()` elements |
| F8 transformed client rect | A | fixed — `get_client_bounding_rect` only; `visible_region` left as a pinned limit |
| F9 `covered` zero-height | A | pinned by words — a limit sentence joins `covered`'s meaning |

Seven engine fixes (F1, F2, F4–F8) are each «An engine behaviour change for every document» on a delegate's word —
PROVISIONAL under «Provisional discharge» (`.andromeda/playbook.md:48`, escalate) until the founder's own word at the
Epoch 2 boundary, this chunk's wrap. F3's answer leaves the one boundary-pattern option unshipped.
