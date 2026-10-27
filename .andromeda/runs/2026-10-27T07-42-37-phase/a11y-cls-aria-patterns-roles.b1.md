# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · the landmark role reads (nav, main, aside, footer, article, blockquote, figure) pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#1.2 · yes · the header and section role reads pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.1 · yes · the role attribute's role_from_name reads, among the builder's role reads the accessibility_* checks re-assert after the take (Needs: builder reads)
aria-patterns-roles#2.2 · yes · the aria-selected, aria-pressed, aria-invalid and aria-checked state reads pinned by accessibility_states.rs and named in the Needs, while the native checkbox and radio reading neither stays an engine debt this chunk does not fix
aria-patterns-roles#2.3 · yes · the button and submit role reads pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.4 · yes · the link role rule (an a is a Link only with href) pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.5 · yes · the select role rule (multiple gives ListBox, else ComboBox) pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.6 · yes · the list, table, row and cell role reads, th by scope included, pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.7 · yes · the input role reads by type (range to Slider, password to PasswordInput, checkbox to CheckBox and the rest) pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.8 · yes · the div, h2 and p role reads pinned by accessibility_roles.rs and re-asserted after the take (Needs: builder reads)
aria-patterns-roles#2.9 · yes · the hidden-content exclusion (aria-hidden, hidden, display none, visibility hidden, a select's option kept, children of a hidden element) that the Needs name under tree exclusion and accessibility_hidden.rs re-reads, which the 53e9d81c6 details :open change moves
aria-patterns-roles#3.1 · no · the clickable-div browser controls sit in apps/browser, which the take does not reach and no check the task names reads
aria-patterns-roles#3.2 · no · the browser's alt texts sit in apps/browser, which the take does not reach and no check the task names reads
aria-patterns-roles#3.3 · no · the rdme stylesheet's focus states sit in apps/readme, which the take does not reach, and the Not needed section drops focus-ring tokens
aria-patterns-roles#4.1 · no · the GitHub fixtures' aria-hidden, tooltip, img and aria-labelledby markup sits in examples/assets, which the take does not reach, no fixture is edited and no check the task names reads it
aria-patterns-roles#4.2 · no · the google fixture's contentinfo role (bottom_only.html) sits in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.3 · no · the google fixture's combobox textarea sits in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.4 · no · the google fixture's listbox, option and menu roles sit in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.5 · no · the google fixture's landmark regions sit in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.6 · no · the servo-new fixture's aria-expanded hamburger sits in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.7 · no · the servo-new fixture's aria-label nav and sections sit in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.8 · no · the graphite fixture's carousel buttons sit in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.9 · no · the servo.html fixture's navigation and burger markup sits in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.10 · no · the servo.html fixture's role=button download links sit in examples/assets, which the take does not reach and no check the task names reads
aria-patterns-roles#4.11 · no · the svg.html fixture's aria-hidden and focusable=false icons sit in examples/assets, which the take does not reach and no check the task names reads
