@account n=1 entity=keyboard-navigation#1.1 v=not_in_scope
why: focusability is a rule of a component the probe only uses, and the scope names no focusability reading, call or fork over it, a re-check every change owes
@account n=2 entity=keyboard-navigation#1.5 v=carried
why: the probe reaches a key handler on a node Tab-focused through the harness press, and the focus change that Tab dispatches is an input of the link (4) changed-set reading
@account n=3 entity=keyboard-navigation#1.10 v=not_in_scope
why: disabled-ness is not a reading of the probe, which writes no attribute and drives only enabled stand controls, a re-check every change owes
@account n=4 entity=keyboard-navigation#1.14 v=not_in_scope
why: the FOCUS and FOCUSRING element states are a focus-ring styling rule the probe neither reads nor paints, a re-check every change owes
@account n=5 entity=keyboard-navigation#2.2 v=carried
why: a click on a CRUD row, a handler-to-state surface the probe drives, clears focus, a focus change the link (4) changed-set reading takes as input
@account n=6 entity=keyboard-navigation#2.4 v=carried
why: the probe's click on a Counter or CRUD button focuses it and a click on a CRUD row clears focus, a focus change beside the handler's state write the probe reads
@account n=7 entity=keyboard-navigation#3.16 v=not_in_scope
why: the driver's press and a multiple select's set are no part of the probe, which drives keys through the harness press and reads no select list
@account n=8 entity=keyboard-navigation#3.17 v=carried
why: Enter and Space dispatching a Click whose Dioxus onclick runs before handle_click is the handler path the scope's links (2), (3) and (4) read on the Counter or CRUD task
@account n=9 entity=keyboard-navigation#3.22 v=carried
why: the Click Enter and Space dispatch is built from the node's synthetic click, the value the probe's handler path carries
@account n=10 entity=keyboard-navigation#3.24 v=carried
why: the windowed key dispatch is the route by which the hazard's key handler is reached on the windowed shell, the path the chunk re-reads and does not run
@account n=11 entity=keyboard-navigation#4.5 v=carried
why: a KeyDown reaching Dioxus as keyboard data through the bridge is the call the probe drives to reach the hazard's key handler and link (2)'s key listeners
@account n=12 entity=keyboard-navigation#4.7 v=carried
why: set_focus is a MountedData call in events.rs, the kind of call whose panic in a spawned task the chunk re-verifies at HEAD
@account n=13 entity=keyboard-navigation#4.8 v=carried
why: it records the MountedData hazard's mechanism and coordinates that the chunk re-verifies at HEAD and names in the founder's reading
@account n=14 entity=keyboard-navigation#5.10 v=carried
why: the CRUD list's keys and a row's click writing the selection and the two fields is the handler-to-state surface the scope's links (3) and (4) read
@account n=15 entity=keyboard-navigation#7.1 v=carried
why: stand_snapshot_state's key-press check is one the task names as already driving keys through the press and press_with the probe drives
@account n=16 entity=keyboard-navigation#7.2 v=carried
why: stand_keyboard is the check the task names as driving keys through press and press_with, its Enter and Space rows and CRUD list keys witnessing the handler paths the probe reads
@account n=17 entity=keyboard-navigation#7.9 v=not_in_scope
why: a key press after a container lost its box is a test of another job that the probe does not drive or re-read, a re-check every change owes
@account n=18 entity=visual-design-verification#5.2 v=not_in_scope
why: an open select list's motion is no part of the probe, which reads no select list
@account n=19 entity=visual-design-verification#8.1 v=not_in_scope
why: hidden-content exclusion from the tree is a rule of the tree builder the probe only reads through, and the scope names no hidden node, a re-check every change owes
@account n=20 entity=visual-design-verification#8.6 v=not_in_scope
why: the overlay scrollbar's band after a scroll governs a served Cells click the probe neither drives nor re-reads, a re-check every change owes
