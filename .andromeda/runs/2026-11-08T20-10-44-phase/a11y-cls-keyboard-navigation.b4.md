@answer n=1 entity=keyboard-navigation#5.7 v=no
why: Cells commit on Enter and blur is the example app's own key behaviour, which the chunk does not touch
@answer n=2 entity=keyboard-navigation#5.8 v=no
why: Cells grid Tab stop, arrows and scroll are app focus order and scrolling outside the graph, listener, actions and snapshot reads the chunk makes
@answer n=3 entity=keyboard-navigation#5.9 v=no
why: Cells editor open and commit is app key handling, and the chunk adds no key behaviour
@answer n=4 entity=keyboard-navigation#5.10 v=no
why: CRUD list keys, option roles and the unread aria-activedescendant are app key handling and tree-builder ARIA derivation the chunk leaves alone
@answer n=5 entity=keyboard-navigation#5.11 v=no
why: TodoMVC autofocus and Enter-to-add are the Preact example's own behaviour, not named by the scope
@answer n=6 entity=keyboard-navigation#5.12 v=no
why: Android Activity focus requests are platform code outside the chunk
@answer n=7 entity=keyboard-navigation#6.1 v=no
why: docs.rs toolbar tabindex is focusability, which the chunk changes none of
@answer n=8 entity=keyboard-navigation#6.2 v=no
why: google role=button on non-button elements is role derivation in the tree builder, which the chunk does not read
@answer n=9 entity=keyboard-navigation#6.3 v=no
why: jsaction keydown handlers sit outside the vdom walk the listener rule reads
@answer n=10 entity=keyboard-navigation#6.4 v=no
why: a focus-visible outline is paint, and the chunk paints nothing
@answer n=11 entity=keyboard-navigation#6.5 v=no
why: the graphite focus border is paint, and the chunk paints nothing
@answer n=12 entity=keyboard-navigation#6.6 v=no
why: the servo focus ring box-shadow is paint, and the chunk paints nothing
@answer n=13 entity=keyboard-navigation#6.7 v=no
why: servo headings with tabindex -1 are focusability, which the chunk changes none of
@answer n=14 entity=keyboard-navigation#6.8 v=no
why: label for/id and nesting association is label naming, which the chunk does not touch
@answer n=15 entity=keyboard-navigation#7.1 v=yes
why: the focused control's snapshot node reading focused, and its agreement with the tree's focus, is the focus reading the replayed actions answer is compared under (scope item 5)
@answer n=16 entity=keyboard-navigation#7.2 v=yes
why: the snapshot's focused node agreeing with the tree's focus at each walk step is the focus reading the replay compares under (scope item 5), while the Tab order walk itself is focus order the chunk does not change
@answer n=17 entity=keyboard-navigation#7.3 v=no
why: the shared-module location of the LOAD and CELLS tab order tables is focus order the chunk does not change, and it states no role or name table
@answer n=18 entity=keyboard-navigation#7.4 v=yes
why: a disabled range taking no click is the outcome the driver's refusal of a click on snapshot enabled Some(false) must produce, and the Slider role and name read after an arrow step are the snapshot model's role and name
@answer n=19 entity=keyboard-navigation#7.5 v=no
why: input and change event firing on typing and range keys is engine event dispatch, not the vdom listener registration the coverage reads
@answer n=20 entity=keyboard-navigation#7.6 v=yes
why: a disabled select taking no click is the outcome the driver's click refusal on enabled Some(false) must produce, while its key and typeahead parts are engine key handling
@answer n=21 entity=keyboard-navigation#7.7 v=no standing=S2
why: the entity states that the select popup checks run in both layout modes, the stand-check convention S2 names
@answer n=22 entity=keyboard-navigation#7.8 v=no
why: the flight type select arrows and its aria-label name are select key behaviour and label naming, which the chunk does not touch
@answer n=23 entity=keyboard-navigation#7.9 v=no
why: a key press after a dropped anonymous block is engine key handling, and the chunk adds no leg or test prefix
@answer n=24 entity=keyboard-navigation#8.1 v=no
why: tabindex absence in the s06 files is focusability, which the chunk does not read
@answer n=25 entity=keyboard-navigation#8.2 v=no
why: tabindex absence in the s10 files is focusability, which the chunk does not read
