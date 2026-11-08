@answer n=1 entity=existing-scopes#1.1 v=no
why: accesskit_xplat is outside every file the change reached
@answer n=2 entity=existing-scopes#1.2 v=no
why: debug_timer is outside every file the change reached
@answer n=3 entity=existing-scopes#1.3 v=no
why: blitz-traits is outside every file the change reached and no engine file changed
@answer n=4 entity=existing-scopes#1.4 v=no
why: no blitz-dom file is in the change, and the task says no engine file changed and can_paint_text is only read
@answer n=5 entity=existing-scopes#1.5 v=no
why: cssom.rs and resolved_style.rs are not reached by the change
@answer n=6 entity=existing-scopes#1.6 v=no
why: form.rs, iframe.rs and query_selector.rs are not reached by the change
@answer n=7 entity=existing-scopes#1.7 v=no
why: the actions read consults hit testing, focus and the select list but moves none of them
@answer n=8 entity=existing-scopes#1.8 v=no
why: blitz-dom accessibility tree building is not reached, the change's accessibility work sits in dioxus-native-dom
@answer n=9 entity=existing-scopes#1.9 v=no
why: no blitz-dom event module is reached, and the actions read dispatches no event
@answer n=10 entity=existing-scopes#1.10 v=no
why: the actions read scrolls nothing and selects nothing
@answer n=11 entity=existing-scopes#1.11 v=no
why: no Stylo integration file is reached by the change
@answer n=12 entity=existing-scopes#1.12 v=no
why: no tree or traversal file is reached by the change
@answer n=13 entity=existing-scopes#1.13 v=no
why: no layout file is reached and the change moves no layout reading
@answer n=14 entity=existing-scopes#1.14 v=no
why: no node file is reached, and an open select list stays open after the actions read
@answer n=15 entity=existing-scopes#1.15 v=no
why: blitz-html is outside every file the change reached
@answer n=16 entity=existing-scopes#1.16 v=no
why: blitz-net is outside every file the change reached
@answer n=17 entity=existing-scopes#1.17 v=no
why: no blitz-paint file is reached, and screenshot changes only the driver's check, not painting
@answer n=18 entity=existing-scopes#1.18 v=no
why: blitz-vibey-script is outside every file the change reached
@answer n=19 entity=existing-scopes#1.19 v=no
why: blitz-shell is outside every file the change reached
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate's re-exports are not reached by the change
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy is outside every file the change reached
@answer n=22 entity=existing-scopes#1.22 v=no
why: dioxus-native's module list and its dependency on dioxus-native-dom are unchanged, as no manifest moved
@answer n=23 entity=existing-scopes#1.23 v=yes mode=add-to
why: the crate root re-exports ActionableNode beside UnkeyedActionable and actionable_nodes reads the one actionable rule in Readings (F5, F6)
@answer n=24 entity=existing-scopes#1.24 v=no
why: the harness screenshot and contrast modules are not reached, the change's screenshot check sits in the driver's executor
@answer n=25 entity=existing-scopes#1.25 v=no
why: the telemetry formatter and span fields are not reached, and the actions span's fields are unchanged
