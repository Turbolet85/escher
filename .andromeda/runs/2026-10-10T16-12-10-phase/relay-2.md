The operator's answers at phase P4, first question round (2026-10-10), as given in the question dialog.

Question: On which elements does the accessibility builder read a state attribute (aria-selected, aria-pressed, aria-invalid)?
Answer: Every element, by value (Recommended)

Question: Does this chunk read aria-checked? (AccessKit holds ONE property, toggled, for both aria-pressed and aria-checked, 'interpreted based on the node's role'.)
Answer: Not in this chunk (Recommended)

Question: What do the tri-state and token values read as in a snapshot — aria-pressed="mixed", aria-invalid="grammar" / "spelling"? (No stand screen shows any of them; each is a fixture case.)
Answer: Boolean model (Recommended)

Question: Which roles do the CRUD rows, their list and the flight mode buttons take? (Making the mode a radio group or tabs would read checked or selected, not pressed — that changes what v010-17 says and is the founder's, so it is not offered here.)
Answer: option + listbox, toggle buttons (Recommended)

Notes: Q3: the limit (mixed reads not pressed; grammar and spelling read invalid) is said in the text an agent is served — the schema help of the state — not only in a rustdoc line, and a fixture check pins each reading. Q2: the ARIA checkbox and switch reading gets its CARRY as you propose.
