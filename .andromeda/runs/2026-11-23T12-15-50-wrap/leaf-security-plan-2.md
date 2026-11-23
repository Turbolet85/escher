@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=32
last: - 2026-10-15: A listener or env read that only a test makes
@leaf_edit n=1 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: the value's check staying the widget's own — an engine change for every document behind the feature, ratified.
source: dec-3olmush5 «Behind the custom-widget feature, a custom widget is handed the document's own shared, mutable font context before each paint, and the base document exposes that context publicly»
replace-with: the value's check staying the widget's own — an engine change for every document behind the feature, ratified. Behind the same feature a custom widget is handed the document's own shared, mutable font context before each paint, the base document exposing it publicly, so app widget code holds a live handle to the engine's font context — no input crosses, the default ignores it, ratified.
