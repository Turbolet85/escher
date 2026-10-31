# Scope — Select popup (2026-10-31-select-popup)

**Working entry:** «Select popup — a closed select opens a painted list of its options, chosen by pointer and key; the founder's fork (v030-17)». Last of Epoch 2, «Engine debts», before «Upstream sync ahead of the UI graph». Capability `v030-17`.

**Taken up on:** the founder's word lifting escher's pause for this one chunk, «Запускать Select popup (Рекомендую)» (inputs#I1); the pause stands again at this chunk's seam. Run terms: inputs#I1, inputs#I2.

## What it builds
- A closed single `select` opens a painted list of its options, in every document and renderer (F8 C, «Давай нарисовать в отдельном чанке», `chunks/2026-10-31-engine-debts/inputs/I12-engine-debts-founder-2026-10-31.md.txt`).
- A person or an agent chooses an option from the opened list by pointer and by key; the select then reads that choice, through the one write path `apply_select_choice` (`v030-17`'s acceptance).
- An option of the opened list can be acted on through the driver (`v030-17`'s acceptance).
- The list closes without a choice (`v030-17`'s acceptance).

## The CARRYs folded
- **The popup's reading of F9.** The lint's `overlaps` exclusion — today "the options of a closed single select, those under an `optgroup` included" — is restated once options read real boxes open: an open list's options read boxes of their own. Record: `.andromeda/runs/2026-10-31T03-24-40-phase/p5-card-1.md` rows 1-2.
- **`covered`'s closed-option reading ends here.** "An option of a closed single select stays refused `covered`" was his word until this chunk (`chunks/2026-10-31-engine-debts/inputs/I12-engine-debts-founder-2026-10-31.md.txt`, «Только строки multiple (Рекомендую)»). The refusal's reading and its schema text are this chunk's to restate.
- **A group's label and `size` are this entry's to paint or read.** A group's label is named in the tree and unpainted today (`content: attr()` unbuilt); `size` is unread, on his «В чанк с попапом (Рекомендую)» (same file).
- **The focus and key contract, reduced motion and engine colours come to the founder as this chunk's P4 forks** (same card, row 2). Colours are a new engine default, since the engine holds no tokens.
- **Playbook Boundary widening:** this entry's wrap halts once as the founder's answer card. A validated surface admits new input and pixels.
- **`.gitignore` gains `escher-0.4.0-incubator/`** beside the two incubator lines it holds, before this entry's wrap's P1. The directory stands excluded in `.git/info/exclude` alone today.
- watch: the free-disk step in `Stand checks repeated`, not shown to be the cure of CI#41190771843's out-of-disk red (0/3; since 2026-10-31-engine-debts; retires: a recurrence, or 3 green runs)

## Surfaces and boundaries
Drawn from the entry's words and the earlier fork's option text; closed against the code at P3 (research.md).
- The list is a new overlay in paint, hit testing and keys: `blitz-paint`, `blitz-dom` (`node/select.rs`, `events/{keyboard,pointer}.rs`, `assets/default.css`), for every document (F8 C's text: "a new overlay in paint, hit and keys").
- A closed select's options read `bounds` equal to its client rect in the snapshot today; an open list's options read real boxes, moving `dioxus-native-dom`'s `snapshot` and `escher-driver`'s `execute`, `lint` and schema help.
- [premise-corrected: `keyboard.rs:186` reads a pressed Space as a typed character today] Enter opens nothing on a single select today; a pressed Space does not open it either, and reaches typeahead as a character. Arrows and typeahead change the choice with no list open. The contract for those keys with a list open is his fork, and whether a closed select's arrows keep moving the choice is inside it.
- The chunk adds no verb, node key, cause, session error, exit status, carrier, listener, port, env var or workspace crate. Any that a fork would need is named on that fork. The driver's twelve key names already hold every key the list can take (`execute.rs:589-600`).
- A `multiple` select's own rows, keys and clicks stay as built by 2026-10-31-engine-debts, its `size` being read or not by this chunk's lean or fork. `size` is unread today (research.md, §Sweeps).
- `dioxus-native`'s windowed path shows the same list, the overlay living in the engine and not in the shell: the window reaches the one `paint_scene` (`blitz-shell/src/window.rs:424`). A separate OS window is not assumed. No test launches a window, so what the windowed path shows is read and not run.
- [premise-corrected: `stand_id_persistence.rs:96`, `stand_snapshot_state.rs:124` and `select_interaction.rs:113` also click the select, then press a key] The stand's flight booker `select#flight-type` is the stand's popup; stand tables that read it (`stand_contrast`, `stand_keyboard`, `stand_act_select`, `stand_lint`) move with any new row, and every check that clicks a select before a key moves with a click that opens.
- [premise-corrected: `accessibility.rs:238` reads `aria-expanded` from the attribute alone, and a native select holds none] A11y surface: the list's focus order and painted colours fall under SC 2.1.1 · 1.4.3 · 2.4.3; for the tree to say whether a select is open the builder needs a native write of `expanded`, on the shape of a native checkable's `toggled` (`accessibility.rs:205`), reading no author attribute there.

## Out of scope
- Non-adjacent `multiple` sets through the driver (`press` holds no Ctrl), a residual on his «Ограничение и residual (Рекомендую)».
- Lifting the pause past this chunk: his word covers this one alone (inputs#I1).
- Upstream sync work: the next epoch's.

## CI read at the take-up
- `da536ea7a714` — green · checks 17/17 · wall 1236 s · CI#41208321538.
- `5c2ef40434fa`, `3af5a9bcf078` — not on the remote (noted; the two records-door commits, unpushed). No red and no `not green` to disposition.
