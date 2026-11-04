@answer n=1 entity=standard-contracts#1.1 v=yes
why: the Document trait poll that the harness drives through poll(None) and the windowed shell poll that takes the changed set are both named by the scope
@answer n=2 entity=standard-contracts#2.1 v=yes
why: the harness's shared VirtualClock, which only tick and advance_time move, is named by the scope and is the clock a document holds
@answer n=3 entity=standard-contracts#2.2 v=no
why: the select value reader is select-model surface and the probe reads no select
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open select list state is select-model surface and the probe reads no select
@answer n=5 entity=standard-contracts#2.4 v=no
why: the live select list reader and its reconcile reads are select-model surface the probe does not reach
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call for the select list is a select-model path the probe does not reach
@answer n=7 entity=standard-contracts#2.6 v=no
why: the select value and select_of readers, clear_select_typeahead and is_select belong to the select model and the driver's locate, neither in scope
@answer n=8 entity=standard-contracts#2.7 v=no
why: settling a multiple select's set on removal is a select-model write the probe does not run
@answer n=9 entity=standard-contracts#2.8 v=no
why: a failed stylesheet fetch no longer blocking render concerns a resource the offline stand boot requests, and the scope names no stylesheet fetch
@answer n=10 entity=standard-contracts#3.1 v=yes
why: the windowed shell's View::poll takes the changed set and refreshes the platform tree on change, the windowed poll path the scope reads under the accessibility feature it names
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single select's expanded write and aria-expanded read are select-model behaviour the probe does not touch
@answer n=12 entity=standard-contracts#3.3 v=no
why: the focus field of the accesskit tree is read by the platform adapter, while the probe reads only the snapshot
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checkbox toggled state is written into the accesskit tree, which the probe does not read, and the probe runs no checkbox
@answer n=14 entity=standard-contracts#3.5 v=no
why: resolved width and height read unrounded layout, geometry the UI graph does not read
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on attribute and child-list changes is engine behaviour the chunk writes nothing into, and no scope-named call carries it
@answer n=16 entity=standard-contracts#4.1 v=yes
why: the listener and event-driver path, with the Dioxus activation click queued so app handlers run before the default action and the bridge's oninput value carried unchanged, is what the listener-map reads and event-order needs rest on
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions are select-model behaviour the probe does not reach
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label click that focuses a bound single select is a select path the probe does not reach
@answer n=19 entity=standard-contracts#4.4 v=no
why: change dispatch on checkbox and radio clicks, and the scope names no checkable control in the lean or served stand tasks
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scrolling, fragment navigation, selection and tree readers are excluded because no link of the UI graph reads geometry
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is a geometry reading the probe does not read
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is a scrolling geometry reader the probe does not read
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is a driver lint reader the probe does not call
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk skipping dropped nodes is a hit-testing contract the scope excludes
@answer n=25 entity=standard-contracts#6.5 v=no
why: a non-invertible transform taking no hit is a hit-testing contract the scope excludes
