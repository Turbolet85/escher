@answer n=1 entity=a11y-strategy#1.1 v=no
why: the session changed no blitz-shell or blitz-dom code, and the refresh path this entity describes lives there
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set contract lives in blitz-dom, which the session did not change
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the unhandled adapter events are in blitz-shell, which the session did not touch
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding lives in blitz-shell, which the session did not touch
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role mapping is in blitz-dom, which the session did not change
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the Unknown-role assertion concerns the engine's role mapping, which the session did not change
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the ARIA state builder in blitz-dom is untouched by the session
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio write in blitz-dom is untouched by the session
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's six reads in blitz-dom's accessibility builder are untouched by the session
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the accesskit_xplat platform module is outside everything the session changed
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat crate docs are outside everything the session changed
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the adapter set_focus and set_window_bounds bodies are outside everything the session changed
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter imports are outside everything the session changed
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android surface-view field read is outside everything the session changed
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=stale
why: the session changed dioxus-native-dom's crate root lib.rs, which this entity cites by line range, and added actionable_nodes beside the gated actionable-key check the entity lists
