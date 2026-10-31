# a11y-plan · picked accounting · group 3 of 5

## Picked
keyboard-navigation#3.5 · proposed · the claim holds because `press_multiple_select_key` keeps its semantics, and F11's `size`-gated row into view belongs beside it
keyboard-navigation#3.6 · not touched · the report keeps a `multiple` select's row semantics, and its new text still lists Ctrl+A among the select model's limits
keyboard-navigation#3.7 · not touched · the report keeps `press_multiple_select_key` and `click_select_row` semantics, and `node/select.rs` is not reached
keyboard-navigation#3.8 · not touched · the report's `select-model` control holds the select's options in tree order and its one write path, and `node/select.rs` and `form.rs` are not reached
keyboard-navigation#3.9 · not touched · `mutator.rs` is not reached and the report's `select-model` control moves nothing of a `multiple` write
keyboard-navigation#3.10 · proposed · `type` now focuses any select with no click, firing no DOM focus events on a single select, and `focus_select` closes any list
keyboard-navigation#3.11 · not touched · the Ctrl limit on `press` stands, the report's inputs#I7 and addendum holding its residual at `.andromeda/residuals.md:43`, and the help restatement moves only digits here
keyboard-navigation#3.12 · proposed · its «takes no activation key» on `select#flight-type` is the retired limit, since Enter and Space now open the list, and F2 to F6, F9 and F10 belong beside it
keyboard-navigation#3.14 · not touched · `keyboard.rs` is reached, but the report's Keys bullet changes only Tab and the select arms, so the copy rule holds
keyboard-navigation#3.15 · not touched · `keyboard.rs` is reached, but the report's Keys bullet changes only Tab and the select arms, so implicit submission holds
keyboard-navigation#3.16 · proposed · a click on a label bound to a single select now focuses it, opens nothing and no longer re-dispatches, and F8 belongs beside it
keyboard-navigation#4.1 · not touched · `blitz-vibey-script` is not reached and the `focus()`, `blur()` and `activeElement` claim holds, F7 sitting with the harness entity that pins it
keyboard-navigation#7.1 · not touched · `stand_snapshot_state.rs` is reached, but its additions at 121-123 and 131 leave the Tab check's claim standing and only digits may move
keyboard-navigation#7.2 · proposed · the existing walk claims hold, and F7's focus and Tab walk with a list open, pinned by the two appended `stand_keyboard` tests, belongs beside them
keyboard-navigation#7.6 · proposed · «Enter and Space opening nothing on a single select» is retired and renamed in `select_interaction`, and F12's new checks belong beside it
keyboard-navigation#7.7 · proposed · `stand_keyboard` moves 21 to 23 tests, and the closed-arrow test's Enter and Space rows now open and then choose
keyboard-navigation#8.3 · not touched · a bounded s07 search record the report neither reruns nor names, so the select list's engine code does not move it
