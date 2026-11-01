# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#4.1 · no · the BBC fixture's reduced-motion queries live in examples/assets, which no reached file touches
visual-design-verification#5.1 · no · the session adds no reduced-motion or colour-scheme rule to any reached file, and default.css loses only the picture rule
visual-design-verification#5.2 · no · stand_act_popup.rs is not reached and settle_waiting is unedited, so the no-advance reading holds
visual-design-verification#6.1 · no · the 0.35 rule lives in apps/browser/assets/browser.css, outside the reached set, and a disabled write still restyles the button itself
visual-design-verification#6.2 · yes · stale · stylo.rs is a merge surface and the cited line 513 sits below the changed block at 231-238, so its digits may move
visual-design-verification#6.3 · yes · stale · select_popup.rs lost the row-3 skip above line 513, so the citation 624-647 moves
visual-design-verification#6.4 · no · wpt/runner/src/main.rs is not reached, and the timeout-quarantine.txt merge does not touch status-word printing
visual-design-verification#6.5 · no · select_popup_painted.rs is unedited and the embolden mention is no-change, the claim holding on both builds
visual-design-verification#6.6 · no · examples/form.rs is not reached
visual-design-verification#7.1 · no · blitz-traits shell.rs is not reached
visual-design-verification#7.2 · no · stylo_device.rs is not reached
visual-design-verification#7.3 · no · stylo_device.rs is not reached
visual-design-verification#7.4 · no · blitz-shell window.rs is not reached
visual-design-verification#7.5 · no · apps/readme CSS is not reached
visual-design-verification#7.6 · no · examples/preact is not reached
visual-design-verification#7.7 · yes · stale · default.css is a merge surface and upstream's picture rule left the sheet, so digits past it, including the 1122-1133 span, move
visual-design-verification#7.8 · no · events/pointer.rs is not reached
visual-design-verification#8.1 · yes · stale · node.rs is a merge surface and the cited 1450-1459 sit past the changed block at 1382-1384, and restyle narrowing reaches the hidden and inert claim
visual-design-verification#8.2 · yes · stale · node.rs is a merge surface and the cited 1461-1469 sit past the changed block at 1382-1384
visual-design-verification#8.3 · yes · stale · the rule lives in node.rs, a merge surface, and the cited 1485-1490 sit past the changed block at 1382-1384
visual-design-verification#8.4 · yes · stale · node.rs is a merge surface and the cited 1471-1475 and 1681-1689 sit past the changed block at 1382-1384
visual-design-verification#8.5 · no · scrollbar.rs and scrollbars.rs are not reached
visual-design-verification#8.6 · no · pointer.rs and stand_cells_served.rs are not reached, and settle_waiting is unedited so the advance reading holds
visual-design-verification#8.7 · yes · stale · select_popup.rs lost the row-3 skip above line 513, so the citation 673-712 moves, while the list paint holds no hunk
visual-design-verification#8.8 · no · packages/blitz-traits devtools.rs is not reached
