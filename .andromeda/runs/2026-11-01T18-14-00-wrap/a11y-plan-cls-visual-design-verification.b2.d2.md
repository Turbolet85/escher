# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#4.1 · no · the BBC fixture's media queries live in examples/assets/bbc_reduced.html, a file the session does not reach
visual-design-verification#5.1 · no · the absence search runs over the 21 s02 files, none of which the session reaches
visual-design-verification#5.2 · no · the open-list motion absence is pinned in stand_act_popup.rs, not reached, and the session adds no motion
visual-design-verification#6.1 · no · the 0.35 opacity lives in apps/browser/assets/browser.css, not reached
visual-design-verification#6.2 · yes · stale · stylo.rs is a merge surface and the cited stylo.rs:513 lies past the changed 231-238 block, so the line may have moved; the open attribute's restyle now rests on restyle_for_attribute_change
visual-design-verification#6.3 · yes · stale · select_popup.rs:624-647 sits past line 513, which the removed row-3 skip shifted
visual-design-verification#6.4 · no · the status-word claim lives in wpt/runner/src/main.rs, not reached
visual-design-verification#6.5 · no · the feature mention at line 67 is no-change per the report, and the pinned test lives in select_popup_painted.rs, not reached
visual-design-verification#6.6 · no · the comment lives in examples/form.rs, not reached
visual-design-verification#7.1 · no · the viewport scheme and zoom live in packages/blitz-traits/src/shell.rs, not reached
visual-design-verification#7.2 · no · the device-change tracking lives in packages/blitz-dom/src/stylo_device.rs, not reached
visual-design-verification#7.3 · no · the scale tracking lives in packages/blitz-dom/src/stylo_device.rs, not reached
visual-design-verification#7.4 · no · the zoom keys live in packages/blitz-shell/src/window.rs, not reached
visual-design-verification#7.5 · no · the rdme CSS lives in apps/readme, not reached
visual-design-verification#7.6 · no · the color-scheme declarations live in examples/preact, not reached
visual-design-verification#7.7 · yes · stale · packages/blitz-dom/assets/default.css is a merge surface, and the upstream picture rule left the sheet above the cited 1122-1133 block, so its digits move
visual-design-verification#7.8 · no · touch-action lives in packages/blitz-dom/src/events/pointer.rs, not reached
visual-design-verification#8.1 · yes · stale · packages/blitz-dom/src/node/node.rs is a merge surface with changed hunks above the cited 1450-1459, and hidden and inert taking effect now rests on the restyle narrowing
visual-design-verification#8.2 · yes · stale · packages/blitz-dom/src/node/node.rs is a merge surface and the cited 1461-1469 sits past the changed 1382-1384 hunk
visual-design-verification#8.3 · yes · stale · packages/blitz-dom/src/node/node.rs is a merge surface and the cited 1485-1490 sits past the changed 1382-1384 hunk
visual-design-verification#8.4 · yes · stale · packages/blitz-dom/src/node/node.rs is a merge surface and the cited 1471-1475 and 1681-1689 sit past the changed 1382-1384 hunk
visual-design-verification#8.5 · no · scrollbar-width lives in scrollbar.rs and scrollbars.rs, neither reached
visual-design-verification#8.6 · no · the scroll-band claim lives in pointer.rs and stand_cells_served.rs, neither reached
visual-design-verification#8.7 · yes · stale · the cited select_popup.rs:673-712 sits past line 513, which the removed row-3 skip shifted; the other two citations are in unreached files
visual-design-verification#8.8 · no · devtools overlays live in packages/blitz-traits/src/devtools.rs, not reached
