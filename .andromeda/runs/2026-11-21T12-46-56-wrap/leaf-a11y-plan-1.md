@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=46
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=24 class=retired
leaf-text: the fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys while the tree's focus names the window, a named SC 2.4.3 gap, with no panic, the limit of reading inertness alone ratified
source: dec-fptknbzy «A focused node gives focus up with no DOM blur or focusout, in every document: at the flush when it becomes inert, and after each layout when it no longer shows; a display: contents control and one beside»
replace-with: and a focused node that no longer shows gives focus up after each layout, no DOM `blur` or `focusout` firing, while a `display: contents` control and one beside a hidden sibling keep focus (ratified, the Tab walk's skip of unshown nodes remedying the Tab trap the post-layout fixup alone made; a control whose own `:focus-within` rule hides its container still takes Tab and loses focus at the next layout, a stated limit)
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=23 class=omission
leaf-text: Focus: Tab / Shift+Tab via `focus_next_node` / `focus_prev_node`;
source: dec-2klbzoi5 «Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events, in every document, skipping a node that does not show: out of the document, under display: none»
replace-with: Focus: Tab / Shift+Tab via `focus_next_node` / `focus_prev_node`, in every document skipping a node that does not show (out of the document, under `display: none`, or not `visibility: visible`);
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=26 class=omission
leaf-text: a focus move, a checked change, a rewritten name
source: dec-3xjtbjfc «The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too»
replace-with: a focus move (a focus lost after layout too), a checked change, a rewritten name
