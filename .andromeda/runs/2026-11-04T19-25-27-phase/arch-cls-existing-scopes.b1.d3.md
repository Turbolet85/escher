@answer n=1 entity=existing-scopes#1.1 v=no
why: the AccessKit platform adapter is not reached; the chunk reads the snapshot, not the adapter (not-needed 12)
@answer n=2 entity=existing-scopes#1.2 v=no
why: the feature-gated debug timer serves no link, reading or measurement the chunk names
@answer n=3 entity=existing-scopes#1.3 v=yes
why: DOM events reach the vdom through blitz-traits' events module and its node_id type, on the path link (2) reads
@answer n=4 entity=existing-scopes#1.4 v=yes
why: the Harness and the DioxusDocument wrap blitz-dom's document, node, events and tree modules
@answer n=5 entity=existing-scopes#1.5 v=no
why: no link reads computed or inline style, and the handler writes state, not CSS
@answer n=6 entity=existing-scopes#1.6 v=no
why: the probe finds nodes by data-dioxus-id, not by selectors, and forms and iframes are not reached
@answer n=7 entity=existing-scopes#1.7 v=no
why: hit testing and geometry are not needed (not-needed 14), the probe names no hover, focus or active state, and the select open list is not-needed 1
@answer n=8 entity=existing-scopes#1.8 v=no
why: blitz-dom's accessibility tree and taffy debug print are not what the snapshot reads, since the snapshot is dioxus-native-dom's crate-private model
@answer n=9 entity=existing-scopes#1.9 v=yes
why: the driver and pointer event dispatch carries the probe's clicks to the handlers whose listeners link (2) reads (item 12)
@answer n=10 entity=existing-scopes#1.10 v=no
why: scrolling, visible-region and selection contracts are not needed (not-needed 14)
@answer n=11 entity=existing-scopes#1.11 v=no
why: Servo styling and its Device carry no reading the probe makes; the pinned viewport is a stand setting
@answer n=12 entity=existing-scopes#1.12 v=yes
why: the element_id document-path tier reads node positions in the tree, which links (1) and (2) read
@answer n=13 entity=existing-scopes#1.13 v=yes
why: the writing-mode layout path is one of the layout modes the verdicts run in both (item 8), and display dispatch is what layout touches
@answer n=14 entity=existing-scopes#1.14 v=yes
why: attributes and element data carry the data-dioxus-id the bridge reads, and the node tree is what the probe reads; the select parts are not reached
@answer n=15 entity=existing-scopes#1.15 v=no
why: the Dioxus app builds its DOM through the mutation writer, never by HTML parsing
@answer n=16 entity=existing-scopes#1.16 v=no
why: networking is not needed (not-needed 6), and the offline boot makes no fetch call the chunk must write
@answer n=17 entity=existing-scopes#1.17 v=no
why: painting and renderer backends are not needed (not-needed 5), and the probe reads no paint output
@answer n=18 entity=existing-scopes#1.18 v=no
why: the probe reads a Dioxus app, not page scripts (not-needed 2)
@answer n=19 entity=existing-scopes#1.19 v=yes
why: the windowed shell's application, window and event modules are the loop item 10 reads, every window event followed by a Poll
@answer n=20 entity=existing-scopes#1.20 v=no
why: the umbrella only re-exports crates the probe reaches directly, and no rule in it binds
@answer n=21 entity=existing-scopes#1.21 v=yes
why: its writing_mode module is the taffy side of the writing-mode layout mode that both verdict runs cover
@answer n=22 entity=existing-scopes#1.22 v=yes
why: the launch path and root contexts (dioxus_application, dioxus_renderer, contexts) are the services items 10 and 13 read
@answer n=23 entity=existing-scopes#1.23 v=yes
why: DioxusDocument, element_id, events, the listener map and the accessibility-gated snapshot are what links (1) to (4) and the snapshot read use
@answer n=24 entity=existing-scopes#1.24 v=yes
why: Harness, input and settle carry the pumped input and the poll(None) the hazard was measured through
@answer n=25 entity=existing-scopes#1.25 v=yes
why: the event formatter's target allowlist and field scrub carry the field rules every probe output must hold (item 15)
