@answer n=1 entity=standard-contracts#6.6 v=no
why: stylo style-data filtering and the resolve context are engine internals the four links never reach.
@answer n=2 entity=standard-contracts#7.1 v=no
why: crate-private layout pass traits and replaced and table APIs, and the probe reads no geometry (scope item 14).
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking arm is paint, outside the chunk.
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node-kind enums and the Widget and custom-widget rules, while the probe reads element ids and attributes only.
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled is the pointer and activation-key gate that decides whether a click or key press reaches a button's listener, which the click-reaction reading and the key-handler hazard run through.
@answer n=6 entity=standard-contracts#10.2 v=no
why: the crate-private disabled-row reading of the select list, and select is not_needed.
@answer n=7 entity=standard-contracts#10.3 v=no
why: the select model rules, and the probe reads no select.
@answer n=8 entity=standard-contracts#10.4 v=no
why: select cursor and gesture internals, and the probe reads no select.
@answer n=9 entity=standard-contracts#10.5 v=no
why: the select list view types and colours, and the select list is not_needed.
@answer n=10 entity=standard-contracts#10.6 v=no
why: the select popup's focus-and-open ordering, and the probe reads no select.
@answer n=11 entity=standard-contracts#10.7 v=no
why: select list placement and row cap, and the probe reads no select.
@answer n=12 entity=standard-contracts#10.8 v=no
why: the out-of-flow flag is read by layout, and no link reads geometry or layout.
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children box flag is a layout and paint rule, and no link reads either.
@answer n=14 entity=standard-contracts#10.10 v=no
why: class atoms read by Stylo selector matching, and the probe reads no classes.
@answer n=15 entity=standard-contracts#10.11 v=no
why: the details and dialog open state behind :open matching, and the probe reads no open-state element.
@answer n=16 entity=standard-contracts#11.1 v=yes
why: the offline headless stand boots with a NetProvider whose is_noop() reads true and sets the pinned Viewport (scope items 5), so the offline reading runs through these values.
@answer n=17 entity=standard-contracts#12.1 v=no
why: HTML parsing and blitz-net are not_needed (items 6 and 9), and the probe builds from the Dioxus vdom.
@answer n=18 entity=standard-contracts#13.1 v=yes
why: the BlitzShellEvent Poll variant and the dioxus-native shell bootstrap are the windowed loop the probe reads for the hazard's windowed path (scope item 10).
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list and select painting are not_needed.
@answer n=20 entity=standard-contracts#13.3 v=no
why: the stroke-text run paint is painting, which is not_needed.
@answer n=21 entity=standard-contracts#13.4 v=no
why: the mix-blend-mode and isolation paint layers are painting, which is not_needed.
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend frame-witness gap is a paint CARRY owned by upstream sync, and the probe merges nothing (scope item 10 not_needed).
@answer n=23 entity=standard-contracts#14.1 v=yes
why: the windowed shell's redraw requests travel through its event loop as RequestRedraw events, the path the windowed reading of the Poll lines runs through.
@answer n=24 entity=standard-contracts#15.1 v=yes
why: Harness, its pump, settle and input helpers and HarnessOptions (incremental, font_ctx) are the harness the probe pumps through without settling (scope item 7).
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the harness VirtualClock moved only by tick and advance_time is the clock the probe's poll(None) runs read (scope item 7).
