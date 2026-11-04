@answer n=1 entity=existing-scopes#1.1 v=no
why: the AccessKit platform adapters are excluded by the task's not-needed list.
@answer n=2 entity=existing-scopes#1.2 v=no
why: the probe reads through VirtualClock, not the debug timer, and no link reads it.
@answer n=3 entity=existing-scopes#1.3 v=no
why: a module listing of the traits crate that the probe's code does not name and no rule holds over.
@answer n=4 entity=existing-scopes#1.4 v=no
why: a module list only; the hazard's document RefCell sits in dioxus-native-dom, and the root states no rule the probe must honour.
@answer n=5 entity=existing-scopes#1.5 v=no
why: CSSOM and getComputedStyle are excluded by the task's CSS not-needed item.
@answer n=6 entity=existing-scopes#1.6 v=no
why: the probe finds nodes through data-dioxus-id, not selectors, forms or iframes.
@answer n=7 entity=existing-scopes#1.7 v=no
why: hit testing, hover, focus and geometry are excluded because no link of the UI graph reads geometry.
@answer n=8 entity=existing-scopes#1.8 v=no
why: accessibility tree building feeds the AccessKit adapter, which is not needed, and taffy debug printing is read by no link.
@answer n=9 entity=existing-scopes#1.9 v=yes
why: the pumped input and the handler-before-default-action ordering of need 12 run through this driver's pointer, keyboard and focus submodules, which links (2) and (3) depend on.
@answer n=10 entity=existing-scopes#1.10 v=no
why: scrolling and visible-region contracts are excluded, and selection of non-input elements is read by no link.
@answer n=11 entity=existing-scopes#1.11 v=no
why: the entity names Servo styling and Device management but states no viewport or colour value that the pinned Light viewport needs from it.
@answer n=12 entity=existing-scopes#1.12 v=no
why: versioned node storage, traversal and URL handling are read by no link the probe reads.
@answer n=13 entity=existing-scopes#1.13 v=no
why: the probe reads no geometry, so the layout dispatch and the two layout modes reach no verdict's reading.
@answer n=14 entity=existing-scopes#1.14 v=yes
why: element_id reads data-dioxus-id from the node's attributes and walks the node tree for its anchored and component paths.
@answer n=15 entity=existing-scopes#1.15 v=no
why: HTML parsing is not needed, since the Dioxus app builds its DOM through the mutation writer and not from HTML.
@answer n=16 entity=existing-scopes#1.16 v=no
why: networking and outbound hosts are not needed, and the entity holds no offline-boot rule.
@answer n=17 entity=existing-scopes#1.17 v=no
why: painting and renderer backends are not needed, and screenshots are read by no link.
@answer n=18 entity=existing-scopes#1.18 v=no
why: JavaScript on Boa is not needed, since the probe reads a Dioxus app and not page scripts.
@answer n=19 entity=existing-scopes#1.19 v=yes
why: the shell loop runs every window event followed by a Poll, which the probe reads from the application and event modules under need 10.
@answer n=20 entity=existing-scopes#1.20 v=no
why: a re-export crate only; the chunk reaches blitz-dom and the harness by their own crates and the entity states no rule.
@answer n=21 entity=existing-scopes#1.21 v=no
why: the stylo-to-taffy conversions sit under layout, which no link reads.
@answer n=22 entity=existing-scopes#1.22 v=yes
why: the launch path (dioxus_application and dioxus_renderer) and the contexts module hold the root contexts that links (3) and (4) read, and the entity names the dependency on dioxus-native-dom.
@answer n=23 entity=existing-scopes#1.23 v=yes
why: the DioxusDocument bridge, the element_id tiers, events.rs and the accessibility-gated snapshot modules are where links (1) and (2) are probed and where the MountedData hazard sits.
@answer n=24 entity=existing-scopes#1.24 v=yes
why: the Harness with its input, settle and poll(None) pump is what the probe drives and what the hazard is measured through.
@answer n=25 entity=existing-scopes#1.25 v=yes
why: the target allowlist and field scrub bind any probe output under need 15, and the chained panic hook is where the RefCell panic surfaces.
