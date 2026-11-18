@row_none n=1 fact=F1 type=4
relied: The accessibility tree's focus names the window when the focused node is no longer among the built nodes, which decision/accessibility-focus-built-nodes-only already states; the hidden-focus case is a named SC 2.4.3 limit the a11y-plan and its test hold, no new security rule.
@row_none n=2 fact=F2 type=4
relied: Removing a focused node clears the document's focused node, engine focus falling to the root html element and tree focus to the window, which the accessibility tree and the harness's focus readings rely on; the engine code and its stand_keyboard test hold it, stating no ratified reason.
