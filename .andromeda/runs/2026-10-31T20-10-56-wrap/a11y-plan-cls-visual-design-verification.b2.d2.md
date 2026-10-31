# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#6.3 · yes · add-to · F20 and F21: the open list's highlight and disabled-row colours are engine constants, measured as painted by select_popup_painted, beside the state-colour rule
visual-design-verification#6.4 · no · the form example's accent-colour comment sits in examples/form.rs, which the session does not reach
visual-design-verification#7.1 · no · the viewport's colour-scheme and zoom fields sit in blitz-traits shell.rs, which the session does not reach
visual-design-verification#7.2 · no · the colour-scheme device rebuild sits in stylo_device.rs, which the session does not reach
visual-design-verification#7.3 · no · the hidpi and zoom scale tracking sits in stylo_device.rs, which the session does not reach
visual-design-verification#7.4 · no · the keyboard zoom steps sit in blitz-shell window.rs, which the session does not reach
visual-design-verification#7.5 · no · rdme's prefers-color-scheme CSS sits in apps/readme, which the session does not reach
visual-design-verification#7.6 · no · the colour-scheme declarations sit in examples/preact pages, which the session does not reach
visual-design-verification#7.7 · yes · stale · default.css is a reached file and the session's inserts there may move the citation at 1126-1137, while the un-inverting rule holds
visual-design-verification#7.8 · yes · stale · pointer.rs is a reached file and its pointer edits may have moved the citations at 161-207 and 254-267, while the touch-action rule holds
visual-design-verification#8.1 · yes · stale · accessibility.rs is a reached file and the insert at 238-242 moves the citation at 281-301, while 19-28 holds above it
visual-design-verification#8.2 · no · the visibility rule sits in node/node.rs, which is not reached and whose hit_inner is unedited; an open list needs a visible select
visual-design-verification#8.3 · no · the non-invertible transform rule sits in node/node.rs, which is not reached and whose hit_inner is unedited
visual-design-verification#8.4 · no · the pointer-events rule sits in node/node.rs, which is not reached and whose hit_inner is unedited
visual-design-verification#8.5 · no · the scrollbar-width rule sits in scrollbar.rs, which is not reached
visual-design-verification#8.6 · yes · add-to · F24: the open list has no fade and needs no advance, set beside the band's advance rule, whose thumb hazard the report says stands as written
visual-design-verification#8.7 · no · the devtools overlay settings sit in blitz-traits devtools.rs, which is not reached
