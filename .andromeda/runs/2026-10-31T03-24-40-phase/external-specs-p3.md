# External specification text read at P3 (2026-10-31), as a fetch agent quoted it

Verbatim quotes, link markup and backticks removed by the agent. Pages read: https://html.spec.whatwg.org/multipage/interaction.html and https://html.spec.whatwg.org/multipage/webappapis.html (HTML Living Standard, "Last Updated 30 October 2026"), https://www.w3.org/TR/html-aam-1.0/ (W3C Working Draft, 26 October 2026). The synchronous-fixup target `infrastructure.html#node-remove-focus-fixup` was not fetched.

## Focus fixup — https://html.spec.whatwg.org/multipage/webappapis.html#event-loop-processing-model, the "update the rendering" steps, step 17

"For each doc of docs, if the focused area of doc is not a focusable area, then run the focusing steps for doc's viewport, and set doc's relevant global object's navigation API's focus changed during ongoing navigation to false."

Its note:

"For example, this might happen because an element has the hidden attribute added, causing it to stop being rendered. It might also happen to an input element when the element gets disabled."

"This will usually fire blur events, and possibly change events."

"In addition to this asynchronous fixup, if the focused area of the document is removed, there is a synchronous fixup. That one will not fire blur or change events."

## Inert — https://html.spec.whatwg.org/multipage/interaction.html

#inert-subtrees: "Inert nodes generally cannot be focused, and user agents do not expose the inert nodes to accessibility APIs or assistive technologies."

#the-inert-attribute: "The inert attribute is a boolean attribute that indicates, by its presence, that the element and all its flat tree descendants which don't otherwise escape inertness (such as modal dialogs) are to be made inert by the user agent."

#data-model, under "Elements that meet all the following criteria:": "the element is not inert;"

#focus-processing-model, key event routing: "If target node is not inert:"

No sentence the agent read states in words what happens to focus when a focused element becomes inert; the mechanism is step 17 above with the "not inert" criterion.

## Native checkedness — https://www.w3.org/TR/html-aam-1.0/

#el-input-checkbox: "checkbox role, with the aria-checked state set to "mixed" if the element's indeterminate IDL attribute is true, or "true" if the element's checkedness is true, or "false" otherwise"

#el-input-radio: "radio role, with the aria-checked state set to "true" if the element's checkedness is true, or "false" otherwise."
