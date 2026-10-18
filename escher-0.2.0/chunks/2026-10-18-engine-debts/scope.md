# Scope — 2026-10-18-engine-debts · Engine debts

**Working entry (escher-0.2.0/working-route.md:20, verbatim):** Engine debts — nine recorded engine deferrals, each the
founder's fork: fixed and proven on stand or fixture, pinned as a limit, or kept (v020-03)  CARRY: a test of the
builder's merged `aria-expanded` · `aria-disabled` · `aria-level` reads, observed absent (test-plan §1) — upstream's
reads, adapter-only, merged at 2026-10-17-upstream-sync-engine-debts

**Capability:** v020-03 «Engine debts dispositioned» (escher-0.2.0/verification-matrix.json, unclaimed) — intent §2:
each debt taken to the founder as a fork, then fixed, pinned as a stated limit, or kept as a residual by his word;
every fix proven on the stand or a fixture, its upstream merge surface owned.

**How the forks are answered (inputs#I1):** the founder is away. Every founder fork of this chunk is written into ONE
file in the run dir, `.andromeda/runs/2026-10-18T02-38-20-phase/founder-forks.md` — each fork with its options whole,
its dependences and a marked recommendation — and the turn ends there. The operator answers them in his stead; each
answer is recorded as a delegate's (`delegate operator`), PROVISIONAL. Technical forks stay in the question dialog
to the operator as usual. A delegate's answer on a fork whose option changes engine behaviour for every document is
PROVISIONAL (.andromeda/playbook.md:64-66, «An engine behaviour change for every document»: «A delegate's answer or a
relay not verbatim is PROVISIONAL (Provisional discharge rule)»), discharged on the founder's own word in one batch at
an epoch boundary (.andromeda/playbook.md:48-50) — verified at P3.

## What it builds

The disposition of each of the nine engine debts that `.andromeda/residuals.md` marks `absorbed:v020-03`. Each one
ends in exactly one of three outcomes, chosen per debt by the answered fork:
- **fixed** — the engine (or bridge) changed, proven by a check on the stand or a fixture in both layout modes, its
  standing pin rewritten to the fixed reading, and an owner named for its upstream merge surface;
- **pinned** — the limit stated (in the doc that already states it, or the tool's own help where it is a driver
  reading) and a check that holds it;
- **kept** — the residual line stands as written, re-targeted by the wrap.

The nine (folded from residuals.md as hypotheses — each coordinate re-verified at P3):
1. **`inert` honoured** (residuals.md:16, the founder's F3 2026-10-14): a truthy `inert` takes no focusability away;
   pin `stand_keyboard::the_engine_honours_no_inert`; whether an inert subtree still takes a hit is not measured; the
   bridge clears a framework-written `inert: false`.
2. **`aria-checked` read** (residuals.md:20, C7): the builder reads `aria-selected` · `aria-pressed` · `aria-invalid`,
   not `aria-checked`; AccessKit holds one `toggled` property for pressed and checked, so a role-aware reader is
   needed; pin `accessibility_states`' fourth test; no stand task states `aria-checked`.
3. **`aria-activedescendant` followed** (residuals.md:15, F2): `crud-list` holds focus, the tree's focus is the list,
   never the option; the builder reads no `aria-activedescendant`; SC 4.1.2 work outside the tier-0 target; the gap
   stated in a11y-plan §5 and layout-templates.
4. **`Node::absolute_position` for a self-scrolled box** (residuals.md:18, C14): it reads the box less its own
   scroll offset — (40, 50) scrolled by (70, 120) reads (−30, −70); seven readers (harness `layout_rect`,
   `layout_rect_of`, `center_of`, `Node::synthetic_click_event_data`, the IME cursor area, `handle_dom_event`'s
   target position, `scroll_into_view`, which adds the offset back); pin `scrolled_box_absolute_position`;
   upstream-owned merge surface `packages/blitz-dom/src/node/node.rs`.
5. **The hit's stop at a `contain: paint` box** (residuals.md:17, C4): the hit stops at an `overflow` clip, not at
   `contain: paint` with `overflow: visible`; pinned by `hit_clipped_at_scrolling_box` and said by `covered`'s
   meaning in the schema; no stand file sets `contain`; upstream-owned merge surface `node.rs`.
6. **The password click caret** (residuals.md:22, C11): a click places the caret by the real characters' widths
   (index 5 after eight narrow characters, 2 after eight wide — `password_caret_click`), leaking the width class
   into a shot; security-plan states it a known 0.1.0 limit; a fix is in blitz-dom's pointer path.
7. **`disabled` focusability narrowed to form controls** (residuals.md:21, C15): presence on any element removes
   focusability (`<div disabled tabindex=0>` unfocusable, unlike browsers); pin
   `stand_keyboard::a_disabled_attribute_of_any_value_is_skipped`.
8. **The transformed box's client rect** (residuals.md:25): `BaseDocument::get_client_bounding_rect` reads a box under
   `transform: translate(50px, 30px)` at its layout position (0, 40, 300, 140); pinned as the fixture condition of
   `hit_clipped_at_scrolling_box::a_box_both_transformed_and_scrolled_is_hit_where_its_rows_show`; CSSOM View includes
   transforms. The residual's own hypothesis, kept verbatim: «hypothesis, not measured: a transformed element's
   snapshot `bounds`, read through the same reader, read its layout position too» — verified at P3 by construction:
   the snapshot's `bounds` IS `get_client_bounding_rect` (packages/dioxus-native-dom/src/snapshot.rs:139), so it reads
   whatever that reader reads; no separate measurement owed.
9. **`covered` on a zero-height box** (residuals.md:28, the founder's «Отложить на 0.2.0, наблюдать (Рекомендую)»,
   2026-10-15): a driver `click` on `task-header-spacer` in each lean task is refused `covered` (0 high, the hit at its
   centre answers the header), while `timer-progress`, 0 wide at boot, is accepted; pinned by
   `stand_act_ids::refused_at_boot`. Its own take-up instruction: «At take-up: read both counts, then decide the words
   or the box» — the counts: 0 spacer calls of 43 in the 0.1.0 cold run
   (escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/live-run.md) and 0 in the 2026-10-10 live trial.

**The CARRY (folded):** a test of the builder's merged `aria-expanded` · `aria-disabled` · `aria-level` reads,
recorded observed absent in test-plan §1 at the last wrap — upstream's reads, adapter-only (the snapshot reads none;
`aria-disabled` moves no snapshot reading). This chunk adds the test of the accessibility tree the builder writes for
them; it is an obligation of this chunk, not a fork.

## The answered outcomes (P4, inputs#I2 — `delegate operator`, PROVISIONAL)

F1 A · F2 A · F3 B · F4 A · F5 A · F6 A · F7 A · F8 A · F9 A (`.andromeda/runs/2026-10-18T02-38-20-phase/founder-forks.md`
§Answers): seven fixed (F1, F2, F4–F8), two pinned (F3 by a check, F9 by `covered`'s words), none kept. Two parts
ride those answers past the debt lines' letter (val-1, intent-incomplete, amended here):
- F2 A names `role="switch"`, which `role_from_name` maps to no role today (packages/blitz-dom/src/accessibility.rs:298-330),
  so the fix adds the `switch` → `Role::Switch` mapping;
- F8 A leaves `visible_region` untransformed as a pinned limit; where the check measures it misreading, `off-screen`'s
  meaning states it, as `covered`'s states its limits.

## Boundaries

- In: the nine debts above, each to one outcome; the CARRY's test; the docs that state each debt's reading (the
  residuals line is the wrap's to flip; the masters are the wrap's to amend).
- Out: the residuals lines this chunk does not own — `absorbed:v020-06` (the scrollbar fade, «One virtual clock»),
  and every `re-carried` or `open` line; upstream PRs (the founder's 2026-10-07 hold on new PRs until #1083 gets a
  reaction stands — «an owner named for its upstream merge surface» names who carries the merge surface, not a PR
  sent); the windowed AT witness (residuals.md:14, re-carried).
- A fix to an engine reading for every document is a direct engine change of the same class the founder ratified
  case by case in 0.1.0 (range input, keyboard activation, the password mask) — each fix here needs his (here: the
  delegate's PROVISIONAL) word, and the snapshot, driver refusals or schema help that read it move with it — verified
  at P3 (.andromeda/playbook.md:64-66; the precedents in the arch and security histories).
- A fork may be answered «keep»: then the chunk changes nothing for that debt and the acceptance holds for it by the
  recorded answer alone. [premise-corrected: .andromeda/residuals.md:3-6 — only /andromeda-route flips a disposition
  and the wrap only APPENDS `open` entries, so the wrap does not "re-carry" the `absorbed:v020-03` line; a kept debt
  gets a new `open` line appended by the wrap's route-resolve, the absorbed line standing as route wrote it]

## Surfaces and contracts touched (to be measured at P3)

- blitz-dom: `accessibility.rs` (debts 2, 3, CARRY), focusability (1, 7), `node/node.rs` hit and position (4, 5),
  `document.rs` client rect (8), the pointer path for a text input's caret (6); dioxus-native-dom: the bridge's
  `inert` handling (1), the snapshot's `bounds`/state readers (2, 8); escher-driver: `covered`'s meaning and remedy
  text, `schema.rs`/`refusal.rs` (5, 9); stand markup `examples/seven_guis/src/app.rs` (9, if the box changes).
- The stand's a11y tables (`stand_keyboard`, `stand_contrast`) move with any focus-order change (CLAUDE.md
  invariant); incremental and non-incremental layout stay identical.
- Every ENGINE fix lands in an upstream-owned file (blitz-dom's `node/element.rs`, `node/node.rs`, `accessibility.rs`,
  `document.rs`, `events/pointer.rs`, `scrolling.rs`, `mutator.rs`), its merge surface owned by a CARRY on the next
  «Upstream sync» entry (.andromeda/playbook.md:66). [premise-corrected: research.md §Files inspected — the driver text
  (`escher-driver/src/refusal.rs`, `lib.rs`), the snapshot readers (`dioxus-native-dom/src/snapshot.rs`) and the stand
  are escher's own files with no upstream merge surface]

## CI read at take-up (Setup 5a)

- `34bcf71c678c` (the last wrap's commit, HEAD): green · checks 17/17 · wall 923 s · CI#39089107338 push
  completed/success (`.andromeda/runs/2026-10-18T02-38-20-phase/ci-no-marker.json`). Nothing to fold.
