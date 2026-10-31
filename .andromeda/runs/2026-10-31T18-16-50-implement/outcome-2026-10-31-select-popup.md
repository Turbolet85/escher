# Outcome — 2026-10-31-select-popup: green + smoke, one finding surfaced, one CI red fixed

**Outcome word: green + smoke.** The last full block on the pre-CI tree reads `entries 31 · green 27 · red 0 · recorded 0 · timeout 0 · not-run 4`. The four not run are the operator's. `v030-17` reads `implemented`.

**Corrected after the first CI read.** CI#41233234207 on `8cd9c1286` ended red on `Test (macos)` alone, every other job green: `a_groups_header_is_darker_than_a_row_by_weight` read the header and the row equal, because this chunk's paint gave a synthesized bold the same embolden an emboldening build gives every run. A defect of this chunk's, on the operator's word fixed in `packages/blitz-paint/src/text.rs` with the test as written. The block's green above did not witness it: no local gate builds `blitz-paint`'s `font-embolden`. The plan's "no OS-sensitive case" did not hold for this feature on macOS. The record, both builds' readings and the next CI read are in `chunks/2026-10-31-select-popup/evidence/ci.md`. **Next: the fix commit's entries 29, 30 and 31, then `/andromeda-wrap-session`.**

## What needs a decision

**A panic the chunk found and did not fix.** A key press panics at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`) when the focused control's anonymous block was dropped because an ancestor gained `hidden`.
- **It is not the list's.** It reproduced with no list ever opened, on a `select` and on a plain `button`. The stack runs through `handle_dom_event`'s `absolute_position`, before any list code.
- **Basis, and its limit.** Those controls ran on this tree. It was not read at the base commit, so "predates this chunk" rests on the stack and on that path holding no line of this chunk.
- **Why it matters.** A remote page can reach it, and it sits beside the plan's security criterion: "a select removed, disabled, hidden or made inert with its list open closes with no panic". That criterion holds for the select itself in each of those states. The list closes with an ancestor hidden too, but a key pressed after that panics, as it does for any control. `select_popup` skips the key press in that one row, and its comment says why.
- **Why it was left.** The fix is in `node.rs`, on no list and in no intent of this chunk, so it was left rather than widened. The wrap needs to give it an owner.

## Files changed

41 changed: the plan's 5 new and 35 modified, plus 1 recorded.
- **New:** `packages/blitz-dom/src/node/select_list.rs` · `packages/blitz-paint/src/render/select_list.rs` · `tests/blitz-tests/tests/{select_popup,select_popup_painted,stand_act_popup}.rs`.
- **Modified:** as the plan lists them.
- **Evidence:** `chunks/2026-10-31-select-popup/evidence/{counts,merge-surface}.md`.

## Deviations from the plan

Scope record: `packages/blitz-paint/src/text.rs` · in-intent · serves step 10 · self. `gate.py scope` reads `clean — changed 41 · listed 40 · recorded 1`.

1. **`text.rs`, outside the lists.** Step 4's "brush its row state gives" cannot carry a colour: `TextBrush` holds only a node id and `stroke_text` reads the colour from that node. `stroke_text` now delegates to `stroke_text_as`, which takes a `RunPaint` (a colour in place of the computed one; synthesized bold on request, without which a font with no bold face paints a header no darker than a row). A synthesized bold adds to a build's own embolden, the first CI read's fix. Every other caller paints as before, read by frame hashes on both builds (`evidence/ci.md`).
2. **`SelectList` has a seventh field, `fresh`.** A click focuses and opens in one step, before a `:focus` rule's restyle. A rule that moves the box, such as a thicker focus border, would otherwise read as the select having moved and close the list at once. The first reconcile takes the anchor again. `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened` pins it.
3. **The public read surface is `SelectListView` and `SelectListRow`,** where the plan says one type. Paint needs the row's fields. The colour constants are associated constants of the view, so they add no type.
4. **The `select_interaction` `KEYS` table moved one row.** Step 14 says the closed-key tables read as they stand after `focused_pick`'s Escape, but its Enter row now opens and marks the set. That row became `PageDown`; the count stays 14.
5. **OS-conditional sites.** The plan says none lands. The new `host_log` test carries `#[cfg(unix)]`, as every host-driving test in that file does, because the socket is unix-only. The CI entry's note, "no OS-conditional site is added", is therefore not true to the letter, though the new test only joins a gate the file already had. `cargo check --tests --target x86_64-pc-windows-gnu -p seven_guis` passes; its one warning is in `host_binary.rs`, which this chunk does not edit.
6. **No `ml-popup` directory.** The `mcp_log` popup test records nothing, since recording there would have needed a `cfg!(unix)` site, so it needs no directory.
7. **Details the plan left open, each chosen here:**
   - A select with no row to show opens nothing.
   - A group with no label reads the whole list's box, so a click on it is refused or lands on a row rather than acting on the group.
   - A press that starts on a row starts no text-selection drag, and the release chooses the row under it.
   - A close clears the typed text, so a Space right after it opens the list instead of joining old text.
   - The row height is the select's font on one line, which gives the stated limit that a larger authored font clips.

## Readings against the forecasts (`evidence/counts.md`)

- **As forecast:** `stand_keyboard` 21 → 23 · `stand_act_select` 10 → 10 · `STATE_ROWS` 18 → 20 · a11y leg 14 · 114 · 0 · `run stand` 237 · 0 · 7 over 51 files · `stand_act_ids` 68 and 9 · the closed screen 1295 B (open 1303 B) · `cli_commands` 14 → 15.
- **Off the forecast:** `select_interaction` stays 18, forecast "more". The list's rows went into `select_popup` instead.
- **Not settled:** the `Focussed node` line count. It read 0 lines through the sink on the workspace build with no base reading to compare, so it shows neither moved nor unmoved. `stand_keyboard`'s three-way focus read holds the premise instead.

## Controls

Hand mutations, restored and sha-compared each time, red before green: no paint call 5 red · no list hit 10 red · no anchor re-read 1 red. **The fourth, every click reading as landing on a row, survived its first firing.** In the select's own box the chosen option is the only one showing, so choosing it and closing read alike. An added row with a disabled chosen option then read it red.

## What stays unproven

- **CI on the fix:** the first run's other sixteen jobs were green on `8cd9c1286`; the fix's own run is entries 29 to 31, read in `evidence/ci.md`.
- **A real macOS build's header** is read by that job alone. This host reproduces the red and the fix through `font-embolden`, which sets the same flag.
- **The windowed list** is read, not run: the smoke proves the boot alone (entry 27, `service.name=seven_guis`). A whole-screen shot through the binary was read by eye and looked as intended.
- **What an assistive technology reads** is unmeasured, as stated.
- **No cold-agent run:** the new `covered` clause and the list could move `wrong_calls`, and `v030-15` owns the next run.
- **`stand_keyboard`'s closed-arrow test** passes untouched, but its Enter and Space rows now open and then choose. It still reads true; it no longer reads what its row order suggests.

## For the wrap

- **The operator's addition on the card** (inputs#I7): the residual and deferred lines for the five limits and the `press` line are the wrap's, at its P5.
- **Expected amendments** stand as the plan lists them. `.claude/rules/a11y.md` still says no popup is painted, and the wrap owns that text.

## Gate guards

The plan holds no `gate.py guard` entry, so there is no pass line to copy. Entry 1 is the plan's own preservation guard, a `git diff --quiet`, and read green.

## Process hygiene

| Process | Started by | Final state |
|---|---|---|
| `cargo metadata` (pid 94460, ppid 2929132) | not this run's tree; unattributed, its arguments a workspace read | left running: not this run's to stop |
| `escher-session`, `seven_guis_native` | this run's gates and hand runs | terminated (census lists none) |

`gate.py census` read `census: 1 running`, the row above. The session state directories under `target/tmp` were ended by `stop`.

## Decisions & corrections

- `recurrence-despite-learning`: the Bash guard refused one `cat` heredoc with a file target; the script went to a scratchpad file and ran by path (`.claude/rules/host-linux.md`, Transports).
- A correction of my own, recorded because it nearly stood: I took the row-versus-box point test for a general hazard. The surviving control showed it decides only for a disabled chosen option.
- `recurrence-despite-learning`: a controlled-select fixture was first written uncontrolled, and its choice reverted on re-render, the limit `docs/capabilities.md` states. The fixture now writes `selected` back.
- `harness.center_of` on an inline element reads no box, the deferred learning in the handoff: `select_popup` reads client rects through its own `client_point`.
