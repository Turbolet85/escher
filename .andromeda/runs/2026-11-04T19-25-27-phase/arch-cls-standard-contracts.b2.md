@answer n=1 entity=standard-contracts#6.6 v=no
why: style resolve context is engine style resolution, which none of the four links reads
@answer n=2 entity=standard-contracts#7.1 v=no
why: layout pass state and Taffy traits are layout internals, and no link of the UI graph reads geometry
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking-context arm is paint and style behaviour the probe does not read
@answer n=4 entity=standard-contracts#9.1 v=no
why: node kinds, the range input and the Widget trait are not matched on by the four links
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled is the pointer gate a click on a disabled button meets before any handler, so the reacts-to-a-click reading must honour it
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled is the select list's reading of a disabled row, and the probe reads no select
@answer n=7 entity=standard-contracts#10.3 v=no
why: the select module's rules and typeahead are select behaviour, and the probe reads no select
@answer n=8 entity=standard-contracts#10.4 v=no
why: select cursor, gestures and apply_select_choice are select internals the probe does not touch
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and its colour constants belong to the open select list, which the probe does not read
@answer n=10 entity=standard-contracts#10.6 v=no
why: the fresh-open guard against a focus restyle concerns the select popup, which the probe does not read
@answer n=11 entity=standard-contracts#10.7 v=no
why: the select list's row cap and placement are select popup rules, and the probe reads no select
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node out_of_flow is a layout flag, and no link of the UI graph reads geometry
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children flag and paints_own_box are layout and paint rules the links do not read
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData classes are Stylo class matching, while link (1) reads the data-dioxus-id attribute, not classes
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state sets the open state of details and dialog, which no link reads
@answer n=16 entity=standard-contracts#11.1 v=yes
why: Viewport carries the stand's pinned 800x600 Light viewport, and NetProvider is_noop carries the offline boots the scope names
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-net's Provider, ProviderError and the HTML constructors are networking and parsing that the offline stand does not run and the probe does not call
@answer n=18 entity=standard-contracts#13.1 v=yes
why: the shell bootstrap and the BlitzShellEvent Poll variant are the windowed shell loop and launch path the probe reads at #10
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list paints the open select list, which the probe does not read
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are text painting the probe does not reach
@answer n=21 entity=standard-contracts#13.4 v=no
why: mix-blend-mode and isolation painting is paint the four links do not read
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness is an upstream-sync carry about paint tests, with no bearing on the probe's reads
@answer n=23 entity=standard-contracts#14.1 v=yes
why: BlitzShellProvider's request_redraw sends through the windowed shell's event loop, the loop the probe reads at #10
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the harness's poll(None) and the input helpers that settle nothing are the pumping the probe's hazard was measured through at #7
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the harness's VirtualClock, which only tick and advance_time move, is the shared clock the probe's #7 reading depends on
