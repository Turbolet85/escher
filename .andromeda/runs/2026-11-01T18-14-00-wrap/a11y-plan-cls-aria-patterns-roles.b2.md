# Classified — a11y-plan · aria-patterns-roles.b2

## Answers
aria-patterns-roles#4.8 · no · the graphite fixture at examples/assets/graphite.html is not a reached file and the ARIA builder holds no hunk
aria-patterns-roles#4.9 · no · the servo.html nav and burger markup sits in an unreached fixture, and the builder's ARIA reads are unchanged
aria-patterns-roles#4.10 · no · the servo.html download-link role sits in an unreached fixture, and the builder's ARIA reads are unchanged
aria-patterns-roles#4.11 · no · svg.html is not a reached file, and its aria-hidden and focusable markup is unchanged
aria-patterns-roles#5.1 · no · its search excludes default.css, the only reached asset in scope, and examples/ and app markup have no diff
aria-patterns-roles#5.2 · no · the report shows no ARIA, accesskit or role text added, with accessibility.rs and its inventories holding no hunk
aria-patterns-roles#5.3 · yes · stale · the paint-tree slice holds reached merge surfaces (render.rs, layers.rs, blend.rs) that render_element's new blend layers moved, so the sole match `role_unchanged` is not shown unchanged
aria-patterns-roles#5.4 · no · the report shows no accesskit or ARIA text added, and accesskit_xplat and the accesskit census are untouched
aria-patterns-roles#5.5 · no · the report shows no `aria-` text added, and the accessibility inventories and builder hold no hunk
aria-patterns-roles#5.6 · no · the report shows no `aria` or role text added, and the builder's ARIA reads are unchanged
aria-patterns-roles#5.7 · no · the report shows no aria, role or accesskit text added, and the accessibility builder holds no hunk
aria-patterns-roles#5.8 · no · the report shows no `aria-` or role text added, and the accessibility builder holds no hunk
