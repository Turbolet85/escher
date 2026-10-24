# Classified — a11y-plan · visual-design-verification.b2

## Answers
visual-design-verification#7.4 · no · the session does not touch blitz-shell's keyboard zoom in window.rs; its windowed change is the launch's style threading
visual-design-verification#7.5 · no · the session does not touch apps/readme's prefers-color-scheme CSS; its apps change is apps/browser's allocator
visual-design-verification#7.6 · no · the session does not touch examples/preact's color-scheme pages; its only new example is examples/multi_doc.rs
visual-design-verification#7.7 · no · the session does not touch default.css's inverted-colors rule and changes no image or video style
visual-design-verification#7.8 · no · the session does not touch pointer.rs's touch-action rule, and its hit change skips only nodes dropped since the last resolve
visual-design-verification#8.1 · yes · add-to · hit_inner now skips nodes dropped since the last resolve in its loops, a hit-order fact this group states, and the session's node.rs inserts before line 1424 moved node.rs:1424-1433 too
visual-design-verification#8.2 · yes · stale · the session's node.rs inserts before line 1435 moved node.rs:1435-1443, the hit visibility test
visual-design-verification#8.3 · yes · stale · the session's node.rs inserts before line 1650 moved node.rs:1445-1449 and node.rs:1647-1655
visual-design-verification#8.4 · no · the session does not touch scrollbar.rs or its test scrollbars.rs
visual-design-verification#8.5 · no · the session does not touch blitz-traits devtools.rs
