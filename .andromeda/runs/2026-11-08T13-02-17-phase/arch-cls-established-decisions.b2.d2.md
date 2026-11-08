@answer n=1 entity=established-decisions#11.16 v=no
why: the upstream AGENTS.md stays out of the tree and no chunk code or test reads it.
@answer n=2 entity=established-decisions#12.1 v=yes
why: the covered refusal's hit path (hit_with_scrollbar) answers a just-scrolled overlay scrollbar band before the node beneath it.
@answer n=3 entity=established-decisions#13.1 v=no
why: layout construction internals the read only reaches through visible_region; the scope names no layout rule and the chunk changes no layout.
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element aspect-ratio sizing; the chunk changes no layout and names no replaced element.
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification; pseudo boxes are not actionable listed nodes and the scope names none.
@answer n=6 entity=established-decisions#14.3 v=no
why: IME enabling for a focused text input on mount; the read types and focuses nothing.
@answer n=7 entity=established-decisions#15.1 v=yes
why: the disabled reading through ElementData::is_disabled, the snapshot's clickable reading and unkeyed_actionable's focusability are the refusal semantics the scope names.
@answer n=8 entity=established-decisions#16.1 v=yes
why: inert elements answer no hit on the shared refusal path and DioxusDocument::unkeyed_actionable skips them, so they bound which nodes the read lists.
@answer n=9 entity=established-decisions#16.2 v=no
why: focus fixup on inert subtrees; the read moves no focus and the scope names no focus rule.
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within bookkeeping and redraw requests; the read moves no focus and requests no frame.
@answer n=11 entity=established-decisions#16.4 v=yes
why: the select's option and selectedness rules are the owner-served case the scope names, a select serving its options.
@answer n=12 entity=established-decisions#16.5 v=yes
why: optgroup children count as the select's options and the group's Role::Group label is the owner-served group the scope names.
@answer n=13 entity=established-decisions#16.6 v=no
why: header painting in an open select list; painting is out of scope and a group header chooses nothing the read lists.
@answer n=14 entity=established-decisions#16.7 v=yes
why: the default sheet's display rules for grouped and multiple select options decide which owner-served options are laid out, so the visible_region reading runs through them.
@answer n=15 entity=established-decisions#16.8 v=yes
why: a multiple select's size hint and overflow-y auto clip its option rows, which the off-screen refusal reads.
@answer n=16 entity=established-decisions#17.1 v=yes
why: a hit test skips a node dropped since the last resolve, so a dropped node is neither listed nor refused as covered.
@answer n=17 entity=established-decisions#17.2 v=no
why: :open matching on details and dialog; the read lists no details or dialog verb and the scope names none.
@answer n=18 entity=established-decisions#18.1 v=yes
why: scrollbar thumb hit-testing is feature-gated, which decides whether the covered reading's hit answers a scrollbar band.
@answer n=19 entity=established-decisions#19.1 v=no
why: window creation and the windowed shell; the read runs headless through the driver session.
@answer n=20 entity=established-decisions#20.1 v=no
why: redraw requests through the windowed shell's event loop; the read requests no redraw.
@answer n=21 entity=established-decisions#21.1 v=no
why: layer limits, culling and the password mask are painting, which the task excludes and the read never paints.
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element's scaled border box and paint culling; the read paints nothing and takes no shot.
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript on Boa and its fetch and timers; the read runs no script.
@answer n=24 entity=established-decisions#24.1 v=no
why: the Dioxus document's base URL, default sheet wiring, hot reload and history; the chunk adds no stylesheet and names none of these.
@answer n=25 entity=established-decisions#25.1 v=no
why: the buffered render serves MountedData writes the read never makes, and its merge surface is owned by a residual rather than a predicted merge surface of this chunk.
