@row_set n=1 row=decision/count-and-rename-sweeps cell=rule seen=2 fact=F3 detector=none
value: A change of the driver's verb count or of a CI leg's file count sweeps the tests, examples and CI-script tests for pins of that count held as a literal, and a test rename sweeps the old name held as a string literal, such as a requirement's witness.
basis: fct-5wqynsxm
because: A CI-script test pinned the a11y leg's file count as a literal that moved with the leg and that a sweep over the leg script's names missed, so the sweep rule must reach a leg's file count too.
@row_retire n=1 row=decision/key-after-dropped-anonymous-block seen=1 heir=decision/key-after-lost-box-acts-after-refocus detector=none
why: A focused control whose anonymous block was dropped no longer keeps focus: the post-layout fixup gives focus up, and the key acts only after a re-focus.
basis: fct-4mnmyqn6, fct-mfew6peh
because: The post-layout focus fixup makes the row's «focus stays and the key still acts» false, in both its rule and its why, so the row is replaced by one stating the re-focus.
@row_add n=1 row=decision/key-after-lost-box-acts-after-refocus area=test-scope-summary detector=none
name: Key press on a boxless control acts after a re-focus
rule: A focused control whose anonymous block was dropped or whose box was lost gives focus up after the next layout; a key press on it does not panic, and the key acts on the control, a select's choice moving, once it is re-focused, in both layout modes.
guard: key_press_after_container_lost_its_box
basis: fct-4mnmyqn6, fct-mfew6peh
because: The post-layout focus fixup gives up focus on a control that no longer shows, and the guarding test now reaches the key through a re-focus.
@row_add n=2 row=decision/focus-test-pins-where-focus-lands area=unit-test-strategy fact=F1 detector=none
name: Focus tests pin where focus lands
rule: A focus test pins the node focus lands on, never only that a hidden control is not focused.
why: Once the post-layout focus fixup runs, an assertion that a hidden control is not focused holds with or without the change under test.
basis: fct-a6wxrphr
because: The new post-layout fixup takes focus off a hidden control at the next pump, so a not-focused assertion no longer discriminates and focus tests must pin the landing node.
@row_add n=3 row=decision/render-wait-by-stored-waker area=e2e-test-strategy fact=F2 detector=none
name: Render waits leave the waker to the producer
rule: A test task that waits for a later render on the Dioxus bridge leaves its waker for the producer to wake and never wakes its own waker while it waits.
why: A task that wakes its own waker while it waits livelocks inside the buffered render, a limit asserted by no check.
basis: fct-a4d7c4g3, fct-mzyqxmkh, fct-kabdmiwn
because: A self-waking wait was measured to hang inside the buffered render in both layout modes and was replaced by a stored-waker wait, a convention future tests on the bridge must follow.
@row_add n=4 row=decision/script-document-tests-seeded area=test-data-fixtures fact=F4 detector=none
name: Script-document random seeded from building code
rule: A script-document test that reads page Math.random seeds the document from its building code and pins the sequence that seed reads; an unseeded document reads the host's random, never asserted as a value.
guard: seeded_random
basis: fct-4mnmyqn6, fct-xvmt5uxa
because: The script document gained a seed builder for page Math.random, and its tests pin one sequence per seed while the unseeded case reads the host.
