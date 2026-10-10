# Fan-out results — 2026-10-10-scrolling-box-bounds-and-hit

Seven doc-agents, one batch, each sent the prompt of `amendment-flow.md` verbatim with its detectors (15 detector
names over the drift-base, 15 sent: architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 ·
test-plan 3 · obs-plan 3 · a11y-plan 2). Returns read for HTML entities: none in any of the seven. Each list below is
the return's proposals as parsed, a `change` shortened to the claim it makes where the return ran to a paragraph — the
shortening is the orchestrator's, the applied text is re-derived from the report, never from these lines.

Totals: 25 proposals — 23 applied · 2 rejected under Validate's opening rule and raised again by the orchestrator ·
0 escalated. Raised by the orchestrator beside them: 4 (design-system §Depth Strategy, by check 5; a11y-plan §5, by
check 6; the two rejected proposals' facts).

## architecture — 9 proposals

1. D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge, the `bounds` clause — retire "true of every
   element but a box that is itself scrolled … read shifted by its own offset … unfixed and owned by a route entry";
   say the reader's two bodies no longer subtract the node's own offset, the six callers kept, `offset_rect` and
   `inline_fragment_rects` re-based, held by `scrolled_box_client_rect`. basis `document.rs:2256-2261`.
   → **apply** — check 1: routine (Accurate this-chunk addition; the plan's first Expected amendment names the change).
   It settles the two `claim false` rows of `citation-dispositions.md` on `architecture.md:136`.
2. D-arch-resources · warning · §Standard Contracts → Scrolling, selection, tree — retire "the hit walk passes a point
   outside a scrolling box to that box's scrolled-out children … unfixed — the hit walk is not edited"; say the walk
   tests a node's own area where the box stands and stops at the padding box of a node that clips by `overflow`; what
   it does not stop at; the one-axis case; no check for a box both transformed and scrolled. basis `node.rs:1447-1472`.
   → **apply** — check 1: routine (the plan's second Expected amendment).
3. D-arch-resources · warning · same section, the `visible_region` clause — "a clipping box's own scroll offset is
   cancelled where that box is positioned" no longer holds; it takes the reader's position as the box's origin and
   answers as before. basis `scrolling.rs:774`.
   → **apply** — check 1: routine (same entry). It settles the `claim false` row on `architecture.md:124`.
4. D-arch-resources · warning · same section — add `Node::absolute_position` as measured: a scrolled box reads its
   position less its own scroll offset, (40, 50) scrolled by (70, 120) reads (−30, −70); the harness's `layout_rect`,
   `layout_rect_of`, `center_of` likewise; pinned by `scrolled_box_absolute_position`, left by option 2B. basis
   `scrolled_box_absolute_position.rs:48-78`.
   → **apply** — check 1: routine (same entry: "`Node::absolute_position` as step 3 measured it").
5. D-arch-resources · warning · §Standard Contracts → Driver session — the three texts as restated (`covered`'s
   meaning, the help of `changed` and of `snapshot`'s `text`), the crate docs, the cause set eight, logic unchanged.
   basis `refusal.rs:73-74`.
   → **apply** — check 1: routine (the plan's third Expected amendment).
6. D-arch-resources · warning · same section, dependent-of D-arch-resources — retire "The `covered` limit as measured
   … the engine is unchanged, the reading is pinned by `stand_act_obstructed` and its fix is owned by a route entry".
   → **apply** with its primary (2).
7. D-arch-resources · warning · same section, dependent-of D-arch-resources — retire "those `bounds` read shifted by
   the box's own scroll offset … lands off its centre … pinned by `stand_act_scroll`".
   → **apply** with its primary (1).
8. D-arch-resources · warning · §Existing Scopes → blitz-tests, dependent-of D-arch-resources — retire "the two
   measured engine limits pinned as they read"; name the three new engine checks (6 + 9 + 1).
   → **apply** with its primaries (1, 2).
9. D-arch-decisions · warning · §Established Decisions → Scrolling and input — the two engine behaviours for every
   document, the founder's ruling, the operator's answers (1B, 2B), upstreamable against `7832c177`.
   → **apply** — check 1: routine (the plan's fourth Expected amendment). Not a boundary widening by its subject:
   nothing new crosses a boundary — a client rect is corrected and a hit reaches less. The founder's ruling is recorded
   with its authority as the route line and the plan hold it.

## security-plan — 1 proposal

All three invariants reported holding (no input surface, no auth or secret change, no dependency). Stripped: a
comment block saying so, the sweep it ran (`scrolled out of` 1 hit at `:339`; `shifted`, `own scroll offset`,
`pinned as it/they read`, `contain: paint`, `hit walk` outside `:339` — 0), and a list of citations it thought might
have moved (the sweep had already re-pointed or listed them).

1. D-security-input · warning (filed under the nearest detector) · §Error Handling → Error format (typed errors), the
   escher-driver refusal item — replace "`covered`'s meaning now ends with the sentence that a hit reaches content
   scrolled out of a scrolling box, and three help texts say the scrolled-box bounds limit and that `type` replaces"
   with the texts as restated, the cause set eight. basis `refusal.rs:73-74`.
   → **apply** — check 1: routine (the plan's fifth Expected amendment; the detector reports its invariant holding).

## design-system — 0 proposals

`proposals: []`. Stripped: a comment that D-design-tokens holds (every changed surface `tokens n/a`), and a note that
the plan's Expected amendment for §Depth Strategy → Engine depth behavior is outside the detector and that the
sentence at `design-system.md:215` states the walk order only. Raw twin: `.raw-fanout-design-system.md`.

- Raised by the orchestrator (check 5): §Depth Strategy → Engine depth behavior — the hit-walk sentence gains its
  edge rule. → **apply**, routine: the report substantiates it (Symbols / APIs, the hit walk).

## layout-templates — 2 proposals

Stripped: a comment that D-layout-surface, read strictly, is not violated (no surface added), that both proposals
are additions resting on the report, and that a sweep for the two fixed limits found 0 hits.

1. D-layout-surface · warning · §Surface: desktop-native → IA notes — what `crud-list` reads after its content is
   scrolled; the click naming it at 12 Creates and at 14. basis `node.rs:1447-1472` · `document.rs:2256-2261`.
   → **apply** — check 1: routine (the plan's eighth Expected amendment).
2. D-layout-surface · warning · §Surface: desktop-native → Primary screens — Home is a scrolling box 600 high at the
   stand's viewport, six of seven cards inside it, Cells at 615 to 684 below it. basis
   `stand_actionable_keys.rs:161-164` · `evidence/readings.md`.
   → **apply** — check 6: it disposes the report's second `Spec claims disproved` entry. Routine: a measured reading
   of a surface the chunk did not edit, stated as measured; no fix is owed (Home scrolls by design, and the one
   check that leaned on the old hit is edited), so no CARRY.

## test-plan — 12 proposals

D-tests-framework and D-tests-obs-harness reported holding. Stripped: comments saying so; that the key file
`session-lifecycle.md` was read whole with no proposal; that §4 → escher-driver quotes none of the three restated
texts; and the admission that the agent read `stand_actionable_keys.rs` at 161-164 and grepped the three new check
files — outside the report.

1. D-tests-coverage · §5 → Session ↔ held instance, the closing clause — "two engine readings are pinned as they read,
   not fixed" becomes the two fixed readings; the `contain: paint` reading stays. → **apply**, routine (the plan's
   sixth Expected amendment).
2. dependent-of · same bullet, the `scroll` clause — retire "since a scrolled box's own `bounds` read shifted by its
   scroll offset". → **apply** with (1). The check still reads the list's box once, before the scroll
   (`stand_act_scroll.rs`, the CRUD test — read by the orchestrator), so "taken before the scroll" stays, its reason
   restated.
3. dependent-of · §1 → tests/blitz-tests, `stand_act_scroll` 6 — the two driver-cli tests restated. → **apply**.
4. dependent-of · §1 → tests/blitz-tests, `stand_act_obstructed` 5 — the pinned limit restated. → **apply**.
5. D-tests-coverage · §1 → tests/blitz-tests — add `scrolled_box_client_rect` 6, `hit_clipped_at_scrolling_box` 9,
   `scrolled_box_absolute_position` 1, with the gaps. → **apply**, routine (the plan's entry, "§1 and §9 — three new
   check files").
6. D-tests-coverage · §2 → Directory pattern — 106 `.rs` files, 105 test targets. → **apply**, routine.
7. D-tests-coverage · §9 → Local baseline — 165 result lines, 776 passed · 0 failed · 11 ignored. → **apply**, routine.
8. D-tests-coverage · §9 → Engine features by runner — both bodies of the reader hold a check of ours. → **apply**,
   routine. Its note that the two body citations need re-measuring is settled: the sweep wrote both ends of each
   (`stretched`, read `holds`).
9. D-tests-coverage · §1 → blitz-dom (layout) — the `writing_mode.rs` body is read by `scrolled_box_client_rect`; no
   check reads a scrolled box under a vertical writing mode. → **REJECTED** — the opening rule: its gap clause rests
   on the detector's own grep over three source files. **Raised by the orchestrator**, measured at this wrap: no
   fixture of the three new files sets a `writing-mode` style (the one mention is a doc comment,
   `scrolled_box_client_rect.rs:13`), the body returns before its vertical-chain path where no box on the chain is
   vertical (`writing_mode.rs:401-403`), and the one blitz-tests file that sets a vertical `writing-mode`,
   `inline_fragment_rects`, holds no `scroll`. → **apply**, routine; the gap gets a route owner at P5, and the report's
   Coverage line now carries it.
10. D-tests-coverage · §3 → Input helpers, the `scroll_into_view` clause — "exercised through the driver's `scroll`
    only" no longer holds. → **REJECTED** — the opening rule: the helper's name was read from a source file.
    **Raised by the orchestrator**: the chunk's own `evidence/readings.md` names `Harness::scroll_into_view`, and the
    report's Deviations carries the edit (new text 161-164). → **apply**, routine.
11. D-tests-coverage · §3 → Inspection helpers — `layout_rect`, `layout_rect_of`, `center_of` read a scrolled box's
    position less its own scroll offset. → **apply**, routine (the report's Symbols / APIs; Harness / gate surface).
12. D-tests-coverage · §2 → WPT conformance — the first conformance reading, 966 → 967 of 2292. → **apply**, routine
    (the plan's entry).

- The plan's entry also names "§4 → escher-driver — the restated table rows": **no change**. Searched
  `meaning|help of|held word|word for word|restated` over `test-plan.md`; two hits on driver unit tests (`:21`, `:143`),
  both read: each says the texts are held "against a stated table" and quotes none of them. The table is in the code.
- The key file `registries/contracts/test-plan/session-lifecycle.md`: **no change**. Its two hits (`:5`, the refusal
  order; `:11`, `stand_act_obstructed` 5 · `stand_act_scroll` 6) read true.

## obs-plan — 1 proposal

D-obs-instrumentation and D-obs-pii reported holding. Stripped: comments saying so, and a note on one `execute.rs`
citation (unmoved by the sweep).

1. D-obs-stack · warning · §3 → Logging stack, the no-subscriber census — the three new engine checks join it; no
   count in the bullet moves. → **apply** — check 1: routine (the plan's ninth Expected amendment).

## a11y-plan — 0 proposals

`proposals: []`. Stripped: comments that both invariants hold, that the key file is untouched, that §6's `node.rs`
citations may have moved (the sweep re-pointed them), and that §5 → Focus restoration holds no line on a click on a
plain button while the report now measures one. Raw twin: `.raw-fanout-a11y-plan.md`.

- Raised by the orchestrator (check 6, the report's first `Spec claims disproved` entry): §5 → Focus restoration gains
  the measured reading — after an accepted pointer click on a plain `button` no element reads focused. → **apply**,
  routine: the body states the measured reading and the question it leaves is pinned on "Stand keyboard harness",
  which already holds the hypothesis.

## The six checks

1. Playbook — every applied proposal matches a routine rule or a P5-approved Expected amendment naming the change;
   none is a boundary widening by its subject; no PROVISIONAL mark is added or discharged.
2. Cross-contradiction — none: no two proposals edit one section in opposing directions.
3. Intent-consistency — the scope record's one line (`stand_actionable_keys.rs` · companion · serves step 6 · self)
   holds: the check leaned on the hit defect step 6 fixes, its pinned counts did not move, no new behaviour. The
   report's deviations are each justified in it; two leave an unchecked path (a box both transformed and scrolled; a
   scrolled box under a vertical writing mode), both owned at P5.
4. Absence needs evidence — the two "no change" findings above cite their search and read every hit; the masters'
   line profile was read first (`splice.py summary`: lines over 2,000 chars — architecture 34 · security-plan 8 ·
   test-plan 16 · obs-plan 4 · layout-templates 2 · a11y-plan 2 · design-system 0), and every site was read by offset.
5. Expected amendments — nine entries: eight matched by proposals, one (design-system) raised by the orchestrator;
   the three `claim false` rows of the citation sweep are settled by architecture 1 and 3.
6. Disproved claims — two entries: the plain-button reading raised by the orchestrator into a11y-plan §5; the Home
   premise matched by layout-templates 2.
