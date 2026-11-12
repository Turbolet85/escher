fct-xhpyuh5z@1 · A check reading has_changes() as "this step changed something" must drain the changed set first, since a harness settle does not drain it, a fresh harness document's set is non-empty and a pointer move alone marks nothing.
fct-sidyoogt@1 · A node's role is taken from its role attribute, else from the HTML element mapping, else Unknown.
fct-2bz7tofn@1 · Focus order relies on an element being focusable only when not inert, not a disabled form control and holding tabindex >= 0 or being a native focusable element, and a focused node coming under inert gives focus up with no blur or focusout.
fct-7hugl7cl@1 · A pointer click focuses a button, a button-type input or a select, while a click on any other element, such as a CRUD row or the list, clears focus.
fct-dzuhkwn2@1 · A closed single select opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root.
fct-pqyrk5rl@1 · Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events.
fct-ag43xb7g@1 · With a select's list open, Tab chooses the current row and closes the list before moving focus, in either direction.
fct-yxl65pxf@1 · Focus stays on a select while its list is open, and the Tab walk holds with the list open and after each close.
fct-is66hfzy@1 · Cached focusability is recomputed when tabindex, href or disabled is set or removed, so a roving tabindex works.
fct-jc7cdake@1 · The :focus-visible selector never matches, while :focus and :focus-within do.
fct-3rgd2unt@1 · Removing the focused node resets focus to the body and runs blur side effects.
fct-7uphfywk@1 · Clicking a checkbox or radio toggles it, dispatches input and moves focus to the control.
fct-o3zz2arg@1 · A page's script hears exactly one change per checkable click, from the engine, since the script runtime synthesizes none.
fct-y4ncelk5@1 · Each key change on a single select dispatches one input then one change carrying the select's value, a key that changes nothing firing none.
fct-k6bfqrau@1 · Every choice from an open list goes through one write path that fires one input then one change, and choosing the already chosen option fires nothing.
fct-lvplfrnk@1 · A disabled optgroup's rows read as disabled through one rule that the readings and the form's entries share.
fct-6opcw6lo@1 · The driver's type on a select focuses it with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits.
fct-gp3teo4p@1 · A click on a label bound to a single select focuses the select, opens nothing and re-dispatches nothing to it.
fct-55wog4z5@1 · A synthetic click is a primary mouse click at the node's center.
fct-v3jtrgvp@1 · A script's element.style read or write goes through the inline style, so display and visibility reach the tree's hidden-content exclusion.
fct-tc36t6wc@1 · The Android native view is focusable in touch mode and requests focus, so key presses reach the app.
fct-jn5hbj7a@1 · Text nodes become TextRun nodes carrying their text, and their parent is labelled by them, so an element's text reaches its accessible name.
fct-5vvi2itr@1 · Callers receive no initial tree synchronously; initial-tree requests, action requests and deactivation all reach one EventHandler.
fct-fod53y3h@1 · A text input already focused when its text editor is constructed has IME enabled with no fresh focus, in every document.
fct-ajar2qgv@1 · Author styles keyed on `:open` apply to an open details or dialog, `:open` reading the element's open state.
fct-jajzvsxm@1 · An open select's list is no `:open` state, so no author style keys on a list standing open.
fct-o4ijraeq@1 · The viewport carries a light/dark color scheme and a document zoom level, 1.0 unzoomed.
fct-zd6d5frj@1 · A color-scheme change on the viewport is a device change that rebuilds the stylist device.
fct-dwhu55j2@1 · A total scale change from hidpi or zoom invalidates text shaping.
fct-anrtdftj@1 · Right after a scroll, a driver click in the shown overlay scrollbar's band is taken as a thumb drag, so an advance past the fade is needed first.
