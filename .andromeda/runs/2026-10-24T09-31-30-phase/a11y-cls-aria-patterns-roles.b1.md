# Classified — a11y · aria-patterns-roles.b1

## Answers
aria-patterns-roles#1.1 · yes · the builder's HTML-element-to-role mapping for nav, main, aside, footer, article, blockquote and figure, re-proven by accessibility_roles
aria-patterns-roles#1.2 · yes · the builder's header and section role mapping, re-proven by accessibility_roles
aria-patterns-roles#2.1 · yes · role derivation from the role attribute, a named builder need re-proven by accessibility_roles
aria-patterns-roles#2.2 · yes · the four ARIA state reads (aria-selected, aria-pressed, aria-checked by role, aria-invalid) and the role switch mapping, pinned by accessibility_states
aria-patterns-roles#2.3 · yes · the button and submit-input role mapping, re-proven by accessibility_roles
aria-patterns-roles#2.4 · yes · the anchor rule (a link only with href, else generic), re-proven by accessibility_roles
aria-patterns-roles#2.5 · yes · the select mapping to ComboBox, or ListBox when multiple, re-proven by accessibility_roles
aria-patterns-roles#2.6 · yes · the list, table, row, cell and header role mapping, re-proven by accessibility_roles
aria-patterns-roles#2.7 · yes · the input-by-type and textarea role mapping (range as Slider, password as PasswordInput), re-proven by accessibility_roles
aria-patterns-roles#2.8 · yes · the div, h2 and p role mapping (GenericContainer, Heading, Paragraph), re-proven by accessibility_roles
aria-patterns-roles#2.9 · yes · the hidden-content exclusion and the select option's display none exception, re-proven by accessibility_hidden and inert_subtree
aria-patterns-roles#3.1 · no · apps/browser's clickable div controls, an upstream-only surface outside escher's a11y surface that the chunk does not reach
aria-patterns-roles#3.2 · no · apps/browser's logo, favicon and icon-button alt text, an upstream-only surface outside escher's a11y surface that the chunk does not reach
aria-patterns-roles#3.3 · no · apps/readme's focus-state CSS for role=button and role=tabpanel, which the chunk does not reach and whose focus-ring styling is not needed
aria-patterns-roles#4.1 · no · an examples/assets fixture's markup, which the sync leaves unchanged and whose role and aria-hidden reads bind through #2.1 and #2.9
aria-patterns-roles#4.2 · no · an examples/assets fixture's contentinfo role, which the sync leaves unchanged and whose role mapping binds through #2.1
aria-patterns-roles#4.3 · no · an examples/assets fixture's combobox attributes, which the sync leaves unchanged and which the chunk's reads do not name
aria-patterns-roles#4.4 · no · an examples/assets fixture's listbox, option and menu roles, which the sync leaves unchanged and whose role mapping binds through #2.1
aria-patterns-roles#4.5 · no · an examples/assets fixture's landmark roles, which the sync leaves unchanged and whose role mapping binds through #2.1
aria-patterns-roles#4.6 · no · an examples/assets fixture's script-flipped aria-expanded, which the sync leaves unchanged and whose click flip is script behaviour the chunk does not touch
aria-patterns-roles#4.7 · no · an examples/assets fixture's aria-label on nav and sections, since text alternatives in examples/assets fixtures are not needed
aria-patterns-roles#4.8 · no · an examples/assets fixture's carousel aria-labels, since text alternatives in examples/assets fixtures are not needed
aria-patterns-roles#4.9 · no · an examples/assets fixture's nav, burger link and hidden bars, which the sync leaves unchanged and whose role and aria-hidden reads bind through #2.1 and #2.9
aria-patterns-roles#4.10 · no · an examples/assets fixture's role=button download links, which the sync leaves unchanged and whose role mapping binds through #2.1
aria-patterns-roles#4.11 · no · an examples/assets fixture's aria-hidden and focusable=false icons, which the sync leaves unchanged and whose focusable attribute the focusability rule does not read
