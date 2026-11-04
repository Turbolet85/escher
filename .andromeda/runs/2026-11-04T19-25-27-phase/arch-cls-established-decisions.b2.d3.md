@answer n=1 entity=established-decisions#11.16 v=no
why: the upstream AGENTS.md kept out of the tree; no probe surface reads an instruction file
@answer n=2 entity=established-decisions#12.1 v=no
why: overlay scrollbar hazard after a scroll; scrolling contracts are not needed and the probe drives no scroll
@answer n=3 entity=established-decisions#13.1 v=no
why: HTML table, replaced, damage and paint-tree layout rules; no link of the UI graph reads geometry or layout
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element natural height from aspect-ratio; the probe reads no layout
@answer n=5 entity=established-decisions#14.2 v=no standing=S1
why: the both-layout-modes test convention that S1 lists; the pseudo-element flow rule itself is not read by the probe
@answer n=6 entity=established-decisions#14.3 v=no
why: IME enablement for an already-focused text input; no probe check reads it, and the dioxus_document.rs re-read covers only the MountedData hazard
@answer n=7 entity=established-decisions#15.1 v=yes
why: the bridge clears element_id_mapping and node_id_mapping for a removed node, which the probe's element_id tiers for links one and two must honour; the disabled, lang and dir clauses are not read
@answer n=8 entity=established-decisions#16.1 v=no
why: inert honouring for focus, hit tests and the accessibility tree; the stand has no inert subtree and the probe reads no hit or focus
@answer n=9 entity=established-decisions#16.2 v=no
why: HTML focus fixup under an inert box without events; the probe reads no focus
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within chain and redraw on focus moves; no probe link reads focus
@answer n=11 entity=established-decisions#16.4 v=no
why: select choice, options and their accessibility writes; the probe reads no select
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup recording and group labels; the probe reads no select
@answer n=13 entity=established-decisions#16.6 v=no
why: optgroup header paint in an open select list; the probe reads no select and no paint
@answer n=14 entity=established-decisions#16.7 v=no
why: default sheet rules for optgroup and multiple selects; no probe read reaches them
@answer n=15 entity=established-decisions#16.8 v=no
why: multiple select size hint and its height; no probe read reaches it
@answer n=16 entity=established-decisions#17.1 v=no
why: hit test and paint walk skipping dropped nodes; hit testing is not needed, the probe reads listeners and element ids, not hits
@answer n=17 entity=established-decisions#17.2 v=no
why: details and dialog open state; the stand has neither element
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing rules of HtmlDocument; the probe boots a Dioxus document, which parses no HTML
@answer n=19 entity=established-decisions#19.1 v=no
why: windowed shell visibility, drop order and control flow; the probe is headless and reads only the poll lines of the windowed path
@answer n=20 entity=established-decisions#20.1 v=no
why: windowed redraw request channel; the probe reads no redraw request
@answer n=21 entity=established-decisions#21.1 v=no
why: painting layers, culling and masks; painting is not needed and no link reads paint
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element border box and culling on the scaled size; no link reads paint or geometry
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript runtime and script fetch rules; the probe reads a Dioxus app, not page scripts
@answer n=24 entity=established-decisions#24.1 v=yes
why: DioxusDocument adds no default user-agent sheet and uses a config's ua_stylesheets as given, a rule the stand's boot of the Dioxus document must honour
@answer n=25 entity=established-decisions#25.1 v=yes
why: the MountedData hazard at events.rs:162 under render_immediate, the chunk's measured hazard reading named in its needs
