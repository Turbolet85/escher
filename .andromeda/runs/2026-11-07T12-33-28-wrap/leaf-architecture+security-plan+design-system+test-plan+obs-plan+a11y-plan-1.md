@leaf_group group=architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan-1
@leaf_read leaf=CLAUDE.md lines=153
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=33 class=omission
leaf-text: headless Dioxus renderer on blitz (`DioxusDocument`);
source: fct-zgbhkh7f «the render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow»
replace-with: headless Dioxus renderer on blitz (`DioxusDocument`), whose `initial_build` and `poll` render into the crate-private `MutationBuffer` with the document unborrowed, then replay it through one `MutationWriter` under one borrow, so code the vdom runs during a render (a component body, a task it polls) calls `MountedData`'s handles with no `RefCell already borrowed` panic, acting on the document as the previous render left it (`set_focus_to` returning `false` for a node created in that same render) — a direct engine change for every Dioxus document, ratified;
