@row_set n=1 row=decision/changed-set-marks cell=rule seen=3 detector=none
value: The engine's changed set marks a node's mutation, a focus or checked change (post-layout focus loss too), a text input, a select's new choice, a single select's list opening or closing, by scroll too, and its highlight moving rows; node creation, hover, active, other scrolls and layout alone do not.
basis: fct-xvmt5uxa, fct-mfew6peh
because: A focus given up by the post-layout fixup now marks its node in the changed set, so the row's flat «layout do not» no longer holds whole.
@row_set n=2 row=decision/tab-and-shift-tab-move-focus cell=rule seen=1 fact=F1 detector=none
value: Tab moves focus to the next focusable node and Shift+Tab to the previous one, dispatching focus events, in every document, skipping a node that does not show: out of the document, under display: none, or not visibility: visible.
basis: fct-amaxcqzi, fct-k5yrahcq, fct-pdqxu66m
because: The Tab walk's filter went from focusable alone to focusable and shown, an engine change for every document that was ratified, so the row must state the skip.
@row_retire n=1 row=decision/focus-fixup-reads-inert-alone seen=2 heir=decision/focus-fixup-gives-up-unshown-focus detector=none
why: The focus fixup no longer reads inertness alone: a focused control that no longer shows gives focus up after the next layout, so the stated SC 2.4.3 gap is closed.
basis: fct-mnilfhvh, fct-qgds5ekv, fct-mfew6peh
because: The report names this row's «reads is_inert() alone … keeps focus and takes keys» as disproved by the chunk's post-layout fixup, and the key itself names the retired limit.
@row_add n=1 row=decision/focus-fixup-gives-up-unshown-focus area=keyboard-navigation fact=F2 detector=none
name: Focus fixup gives up inert or unshown focus
rule: A focused node gives focus up with no DOM blur or focusout, in every document: at the flush when it becomes inert, and after each layout when it no longer shows; a display: contents control and one beside a hidden sibling keep focus.
why: Ratified, with the Tab walk's skip of unshown nodes remedying the Tab trap the post-layout fixup alone made.
guard: inert_subtree; focus_fixup_unshown; key_press_after_container_lost_its_box
basis: fct-xvmt5uxa, fct-4mnmyqn6, fct-mryduozh
because: The chunk adds the post-layout give-up of an unshown focused control beside the inert fixup, and this row is the heir of the retired inert-alone row.
@row_add n=2 row=decision/focus-within-hidden-holder-limit area=keyboard-navigation fact=F3 detector=none
name: Focus-within rule hiding its holder, a stated limit
rule: A control whose own :focus-within rule hides its container still takes Tab and loses focus at the next layout.
guard: focus_fixup_unshown
basis: fct-mryduozh, fct-k5yrahcq
because: The Tab walk's skip leaves one stated limit owned by no residual, which future keyboard work must take into account.
