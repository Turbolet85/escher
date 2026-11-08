@answer n=1 entity=screen-reader-support#1.1 v=yes
why: the graph keys every agent-visible accesskit node, and a TextRun is one with no owner element, so its keying must hold
@answer n=2 entity=screen-reader-support#1.2 v=no
why: the aria-label naming rule is the tree's own; the chunk changes no name mapping and the graph carries no name
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the `<label>` association is a naming rule the chunk leaves as it is, and the graph reads no label
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: the select's bounds rule (client rect, and the open row's bounds from select_list_bounds first) is the snapshot bounds the graph must carry as built (item 10)
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the single select's tree expanded reading is no graph field and the chunk does not change it; upstream expanded reads are not needed
@answer n=6 entity=screen-reader-support#1.6 v=no
why: a stated limit on an open list's highlighted row, which the chunk neither reads nor changes
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is unmeasured and the windowed AT witness is not needed
@answer n=8 entity=screen-reader-support#1.8 v=yes
why: a multiple select's rows read their own client rects as bounds, which the graph's node bounds must match in both layout modes (item 10)
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values() is a handler's read of a multiple select's set, not a graph carrier or a snapshot reading the chunk changes
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: every element node carries its stable element id as author_id, the keying item 3 requires, and the tree the adapter gets must equal the harness tree node for node
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot model's text form, parts, diff, MCP results and node-key help are the carriers the new reacts-to-click field extends, and this entity states them
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: it states NODE_FIELDS as thirteen, which the new reacts-to-click boolean moves to fourteen
@answer n=13 entity=screen-reader-support#3.3 v=yes
why: an option label leaves an open list with no log site, the rule the obs log allowlist and redaction (item 7) must keep for graph nodes
@answer n=14 entity=screen-reader-support#3.4 v=no
why: the native checkbox or radio checked reading is a tree and snapshot reading the chunk does not change, and the checkable roles stay untouched
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and DioxusDocument::unkeyed_actionable, which the chunk fulfils while keeping the listener mark over-inclusive (item 2)
@answer n=16 entity=screen-reader-support#4.2 v=yes
why: states the reacts-to-click reading on Cells and Counter that the new snapshot field must carry, with no field yet added (items 2 and 8)
@answer n=17 entity=screen-reader-support#4.3 v=yes
why: the over-inclusive listener mark and the vdom dynamic_attrs walk, mounted included, are what link 1 reads exactly (item 2)
@answer n=18 entity=screen-reader-support#4.4 v=no
why: the cells-grid role is the not-needed Cells grid item (res-wvatzhpe), and the grid reacts to keydown, not click, so the new field and role derivation do not reach it
@answer n=19 entity=screen-reader-support#4.5 v=yes
why: the actionable-key check reads empty on the load page, a page whose elements take author ids and must still read no finding under the chunk's rule
@answer n=20 entity=screen-reader-support#5.1 v=yes
why: the CRUD list and row roles and names and the flight booker select's state reads are the stated roles and names of seven_guis tasks whose signals move onto the registry (item 12)
@answer n=21 entity=screen-reader-support#6.1 v=no
why: the element's HTML tag name is a tree fact the chunk's graph and checks neither read nor write
@answer n=22 entity=screen-reader-support#6.2 v=yes
why: the tree root Window's id u64::MAX and the focus fallback to it are ids the reserved virtual-node id form must collide with none of
@answer n=23 entity=screen-reader-support#6.3 v=no
why: a statement of the role test's purpose, with no rule the chunk must meet
@answer n=24 entity=screen-reader-support#7.1 v=no
why: the platform adapter's event routing, excluded as not needed (platform adapter item)
@answer n=25 entity=screen-reader-support#8.1 v=no
why: the AccessKit adapter's creation from the window handle, excluded as not needed (platform adapter item)
