@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=stale
leaf-text: any other scroll and layout marking none
source: dec-3xjtbjfc «The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too»
replace-with: any other scroll and layout alone marking none, a focus lost after a layout marking it
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=32 class=retired
leaf-text: `disabled` and `hidden` take no part in it — the fixup reads `is_inert()` alone, so a focused control that no longer shows, by `hidden`, `style="display:none"` or CSS alone, keeps focus and takes keys while the tree's focus names the window, a named SC 2.4.3 gap
source: dec-fptknbzy «A focused node gives focus up with no DOM blur or focusout, in every document: at the flush when it becomes inert, and after each layout when it no longer shows; a display: contents control and one beside a»
replace-with: `disabled` takes no part in it, and after each layout a focused node that no longer shows, by `hidden`, `style="display:none"` or CSS alone, gives focus up too, with no DOM `blur` or `focusout` — a `display: contents` control and one beside a hidden sibling keeping focus, and a control whose own `:focus-within` rule hides its container still taking Tab and losing focus at the next layout, a stated limit
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=33 class=omission
leaf-text: - Tab/Shift+Tab move through `focus_next_node` / `focus_prev_node`;
source: dec-2klbzoi5 «Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events, in every document, skipping a node that does not show: out of the document, under display: none, or»
replace-with: - Tab/Shift+Tab move through `focus_next_node` / `focus_prev_node`, in every document, skipping a node that does not show — out of the document, under `display: none`, or not `visibility: visible`;
