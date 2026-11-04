@answer n=1 entity=existing-scopes#1.1 v=no
why: accesskit_xplat is the platform accessibility adapter, not_needed by the task (item 12)
@answer n=2 entity=existing-scopes#1.2 v=no
why: debug_timer's real timer is read by none of the four links or the stand
@answer n=3 entity=existing-scopes#1.3 v=no
why: blitz-traits is a module list only; the event and shell rules it carries are stated by blitz-shell and dioxus-native-dom
@answer n=4 entity=existing-scopes#1.4 v=no
why: blitz-dom's module list states no rule; the MountedData hazard sits in dioxus_document.rs
@answer n=5 entity=existing-scopes#1.5 v=no
why: CSSOM and getComputedStyle are read by no link of the UI graph
@answer n=6 entity=existing-scopes#1.6 v=no
why: form owner reset, iframe loading and selector queries are not read by any link or stand task
@answer n=7 entity=existing-scopes#1.7 v=yes
why: hover, focus and active state set by the harness's pumped pointer input route the key events the hazard runs through; hit testing and geometry parts are excluded by item 14
@answer n=8 entity=existing-scopes#1.8 v=no
why: the AccessKit tree built here is not the dioxus snapshot the probe reads; taffy debug printing is read by no check
@answer n=9 entity=existing-scopes#1.9 v=no
why: the events entry lists submodule names only and states no dispatch or default-action rule
@answer n=10 entity=existing-scopes#1.10 v=no
why: scrolling and the visible-region reader are excluded by item 14, and selection for non-input elements is outside the scope
@answer n=11 entity=existing-scopes#1.11 v=no
why: Stylo styling, Device and conversions are read by no link; the stand sets no style_threading (item 11)
@answer n=12 entity=existing-scopes#1.12 v=no
why: versioned node storage and traversal are described by module name only, with no rule the probe honours
@answer n=13 entity=existing-scopes#1.13 v=no
why: no link of the UI graph reads geometry, so no reading is touched by the layout modes and layout dispatch is not reached
@answer n=14 entity=existing-scopes#1.14 v=no
why: attributes and element data are listed by name only; select and select_list are not_needed (item 1)
@answer n=15 entity=existing-scopes#1.15 v=no
why: HTML parsing is not_needed (item 9) and the stand's pages are not parsed from HTML
@answer n=16 entity=existing-scopes#1.16 v=no
why: networking is not_needed (item 6); the offline boot is the stand's own obligation and this entity states no rule for it
@answer n=17 entity=existing-scopes#1.17 v=no
why: painting and renderer backends are not_needed (item 5)
@answer n=18 entity=existing-scopes#1.18 v=no
why: page JavaScript on Boa is not_needed (item 2); the probe reads a Dioxus app
@answer n=19 entity=existing-scopes#1.19 v=yes
why: the windowed shell loop, with every window event followed by a Poll (item 10), is read in blitz-shell's event and window modules
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate only re-exports modules and states no rule the probe honours
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy's layout conversion is touched by no reading, since no link reads geometry
@answer n=22 entity=existing-scopes#1.22 v=yes
why: the dioxus-native launch path (dioxus_application, dioxus_renderer, event_handlers) is read for the windowed poll lines (item 10) and depends on dioxus-native-dom
@answer n=23 entity=existing-scopes#1.23 v=yes
why: the DioxusDocument bridge, its listener reads, element_id tiers, the accessibility-gated snapshot read read-only (item 17) and keyboard_compat are the probe's links (1) and (2) surface
@answer n=24 entity=existing-scopes#1.24 v=yes
why: the Harness and its input pumping and settle modules are the probe's instrument, with poll(None) and the shared VirtualClock (item 7)
@answer n=25 entity=existing-scopes#1.25 v=yes
why: escher-telemetry's field scrub and the rule that no snapshot or content field is logged govern any probe output (item 15)
