# Classified — a11y · visual-design-verification.b2

## Answers
visual-design-verification#6.3 · no · the WPT runner's status-word line in wpt/runner; no file the chunk reaches holds that rule
visual-design-verification#6.4 · no · a comment in examples/form.rs; the chunk edits no example markup
visual-design-verification#7.1 · no · viewport colour scheme and zoom are shell state the scope never names; the list's colours are the P4 fork
visual-design-verification#7.2 · no · stylist rebuild on scheme change in stylo_device.rs, not reached and not named by the scope
visual-design-verification#7.3 · no · scale-driven text shaping invalidation in stylo_device.rs, not reached and not named by the scope
visual-design-verification#7.4 · no · keyboard zoom steps in blitz-shell window.rs, not reached; the select's key rule names no zoom key
visual-design-verification#7.5 · no · rdme's prefers-color-scheme stylesheets in apps/readme, not reached
visual-design-verification#7.6 · no · TodoMVC and reference colour-scheme declarations in examples/preact, not reached
visual-design-verification#7.7 · no · un-inverting images and video under inverted-colors in default.css; the list paints no image, and default.css is reached whole with no rule over the list
visual-design-verification#7.8 · no · touch panning in pointer.rs lines 161-207 and 250-263 is a unit the change does not edit (it edits only the click on a select and on an option), a re-check
visual-design-verification#8.1 · yes · an option under a select is excepted from display:none in the tree and snapshot, including through an optgroup, and the list must not change which options the tree holds (select.rs, accessibility.rs)
visual-design-verification#8.2 · yes · visibility:hidden options are never hit targets, which the chunk's option hits and the rule that a closed select's option takes no click depend on
visual-design-verification#8.3 · no · the zero-determinant transform hit rule in node.rs is not named by the scope; a re-check the hit path owes
visual-design-verification#8.4 · no · pointer-events transparency in node.rs is not named by the scope; a re-check the hit path owes
visual-design-verification#8.5 · no · overlay scrollbar suppression in scrollbar.rs, not reached, and no list scrolling is named by the scope
visual-design-verification#8.6 · yes · the option click the chunk edits in pointer.rs falls under the rule that a pointerdown in a shown scrollbar band dispatches nothing, wherever an open list scrolls
visual-design-verification#8.7 · no · devtools layout outlines and highlight overlay settings in blitz-traits, not reached
