@answer n=1 entity=established-decisions#11.16 v=no
why: the AGENTS.md out-of-tree rule; the chunk adds no instruction file and reaches no build input
@answer n=2 entity=established-decisions#12.1 v=yes
why: a driver click inside a just-scrolled box is swallowed by the overlay scrollbar thumb band until an advance passes its fade, so the probe's clicks on the served task must allow for it
@answer n=3 entity=established-decisions#13.1 v=no
why: table, fixed-layout, replaced-element, damage and paint-order rules; the probe reads no geometry and no named stand task reaches these layout paths
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element aspect-ratio rule; no img, svg or video in the probed tasks and the probe reads no geometry
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification; no link of the UI graph reads pseudo-element layout
@answer n=6 entity=established-decisions#14.3 v=no
why: IME enablement for a text input focused at editor construction; the headless probe enables no IME and no mount-focus path is named in scope
@answer n=7 entity=established-decisions#15.1 v=yes
why: removed-node mapping clearing (element_id_mapping and node_id_mapping on bridge removal) decides whether element_id still resolves after a re-render, which links (1) and (2) read
@answer n=8 entity=established-decisions#16.1 v=no
why: inert-subtree hit, focus and accessibility rules; no probed task names an inert node and the snapshot reads name no inert subtree
@answer n=9 entity=established-decisions#16.2 v=no
why: focus fixup on inert nodes without blur events; no inert node is in the probed tasks and no focus-loss path is named in scope
@answer n=10 entity=established-decisions#16.3 v=no
why: :focus-within and focus redraw bookkeeping; the probe's four links name no focus move or changed-set read
@answer n=11 entity=established-decisions#16.4 v=no
why: the select CHECKED state and the DOM select model; the task's not_needed item 1 states the probe reads no select
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup recording and group label rules belong to the select model, which the probe reads no part of (not_needed item 1)
@answer n=13 entity=established-decisions#16.6 v=no
why: open select list group header paint; the select popup is not_needed and the probe reads no select
@answer n=14 entity=established-decisions#16.7 v=no
why: default-sheet layout rules for optgroup and multiple rows; select layout the probe does not read
@answer n=15 entity=established-decisions#16.8 v=no
why: multiple select size hint; select layout the probe does not read
@answer n=16 entity=established-decisions#17.1 v=yes
why: a hit test skips a node dropped since the last resolve, and the probe's driver clicks on re-rendered nodes for link (2) resolve through that hit test
@answer n=17 entity=established-decisions#17.2 v=no
why: :open state on details and dialog; no stand task uses details or dialog and the probe reads no such state
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing rules; Dioxus builds the DOM through mutations and no link of the UI graph reads parsed HTML
@answer n=19 entity=established-decisions#19.1 v=yes
why: the windowed shell's default ControlFlow::Wait decides when the Poll after a window event runs, the loop the read-only dioxus-native launch path (item 10) covers
@answer n=20 entity=established-decisions#20.1 v=yes
why: redraw requests travel through the windowed shell's event loop, the loop item 10 reads and the windowed seven_guis smoke boot the probe reads
@answer n=21 entity=established-decisions#21.1 v=no
why: paint layers, culling and password masking; painting and renderer backends are not_needed
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element border box and culling on the scaled size; painting is not_needed
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript on Boa and external script fetching; the JavaScript path is not_needed and the probe reads a Dioxus app
@answer n=24 entity=established-decisions#24.1 v=yes
why: DioxusDocument::new base_url and the ua_stylesheets default-sheet rule are the bridge and the pinned stand config the probe reads, with the MemoryHistory and NoOpDocument eval rules beside them
@answer n=25 entity=established-decisions#25.1 v=yes
why: the MountedData hazard itself, a key-handler task that writes a signal first polled inside render_immediate, which the probe measures through poll(None)
