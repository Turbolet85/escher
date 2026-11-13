@row_set n=1 row=decision/click-focuses-buttons cell=rule seen=1 fact=F1 detector=none
value: A pointer click on a button of any type or an input of type submit, reset or button focuses it, submits the form owner when it is a submit button and ends the walk, in every document; a click on another non-interactive element clears focus, leaving the focused reading the root element, never none.
basis: fct-opk6fla4, fct-f72ihcen
because: The chunk met that a click no element takes clears focus and the focused reading then falls back to the root element, never none, so a check asserting no focus reads red on a correct engine; the row says only that focus clears and must state what the reading becomes.
