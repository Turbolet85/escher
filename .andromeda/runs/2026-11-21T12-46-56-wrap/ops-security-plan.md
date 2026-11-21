@row_set n=1 row=decision/inert-focus-clear-silent cell=rule seen=1 fact=F3 detector=none
value: A focused node gives focus up with no DOM `blur` or `focusout` when it comes under `inert` and, after each layout, when it no longer shows; a `display: contents` control and one beside a hidden sibling keep focus.
basis: fct-xvmt5uxa, fct-4mnmyqn6, fct-mfew6peh
because: The chunk adds a second silent focus clear, the post-layout one for a control that does not show, so the row naming the silent clear must name both paths.
@row_set n=2 row=decision/programmatic-focus-events cell=except seen=1 detector=none
value: The inert fix-up's focus clear, the post-layout clear of a focused node that no longer shows, and the driver's select focus fire none.
basis: fct-xvmt5uxa, fct-mfew6peh
because: The post-layout focus clear queues no blur or focusout, so the exception list of the focus-events rule must name it.
@row_set n=3 row=decision/seeded-source-in-process-seed-only cell=rule seen=1 fact=F5 detector=none
value: The seeded source of randomness takes its seed only from in-process code: a root context the virtual DOM's builder gives else seed 0, or a script document's building code, whose page Math.random it replaces; outside any run it seeds from the host's random keys, reading no env var, file or carrier.
basis: fct-xvmt5uxa, fct-mc6o3rqy, fct-4mnmyqn6
because: A script document can now be seeded by its building code alone, its page Math.random replaced by a native seeded function, so the in-process-seed rule must cover that source.
@row_set n=4 row=decision/dropped-anonymous-block-clears-parent cell=rule seen=1 detector=none
value: Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a re-focused control whose block was dropped does not panic and still acts.
basis: fct-w5e2yko2, fct-4mnmyqn6
because: A control whose container lost its box now gives focus up at the next layout, so a key reaches it only after a re-focus and the row's «still acts» must say so.
@row_add n=1 row=decision/tab-walk-skips-unshown area=input-validation fact=F1 detector=none
name: Tab walk skips a node that does not show
rule: Tab and Shift+Tab move focus only to a focusable node that shows, skipping one under `display: none` on itself or an ancestor, with no computed style, or not computing `visibility: visible`.
why: An engine change for every document, ratified, to remedy the Tab trap a hidden focusable would make.
guard: tab_and_shift_tab_skip_a_control_that_does_not_show
except: A control whose own `:focus-within` rule hides its container still takes Tab and loses focus at the next layout.
basis: fct-k5yrahcq, fct-amaxcqzi, fct-xvmt5uxa
because: The Tab walk's filter moves from focusable alone to focusable and shows for every document, a decision no row of the area states.
@row_add n=2 row=decision/boundary-registry-pruned-before-render area=error-handling fact=F4 detector=none
name: Error-boundary registry pruned before each render
rule: The Dioxus bridge's error-boundary registry is pruned of dropped boundaries before each render, so it stays bounded instead of growing for the document's life; a boundary dropped in a step still reads until the next render.
guard: twenty_remounts_keep_the_registry_bounded
basis: fct-xvmt5uxa, fct-hhfijknc, fct-4mnmyqn6
because: The registry that grew for the document's life is now bounded by a prune, a resource bound no row of the area states.
@row_none n=1 fact=F2 type=2
what: The exception of the Tab-walk decision, carried in the except cell of decision/tab-walk-skips-unshown; the statement in docs/capabilities.md holds it as a limit.
