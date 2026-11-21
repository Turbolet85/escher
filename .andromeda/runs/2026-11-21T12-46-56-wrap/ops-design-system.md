@row_retire n=1 row=decision/boxless-focused-select-keeps-focus seen=1 heir=decision/unshown-focused-control-gives-up-focus detector=none
why: A focused select whose container lost its box no longer keeps focus: it gives focus up after the next layout and takes a key only once re-focused.
basis: fct-mfew6peh
because: The post-layout focus fixup makes the row's rule false, and its key names the very behaviour that no longer holds, so the row gives way to the general rule.
@row_add n=1 row=decision/unshown-focused-control-gives-up-focus area=surface fact=F1 detector=none
name: Focused control that stops showing gives focus up
rule: After each layout, a focused control that no longer shows (out of the document, under display: none, or not visibility: visible) gives focus up silently, with no blur or focusout, is marked changed, and takes a key only once re-focused.
why: Retires the SC 2.4.3 gap of a hidden focused control keeping focus and taking keys.
guard: focus_fixup_unshown and key_press_after_container_lost_its_box
except: A display: contents control and a control beside a hidden sibling keep focus.
basis: fct-xvmt5uxa
because: The chunk adds the post-layout giving-up of focus by a control that does not show, a standing focus behaviour future surface work must respect.
@row_add n=2 row=decision/tab-walk-skips-unshown area=surface fact=F2 detector=none
name: Tab and Shift+Tab skip what does not show
rule: Tab and Shift+Tab skip a node that does not show, on every document.
why: Remedies the Tab trap the post-layout focus fixup alone made of a hidden focusable.
guard: tab_and_shift_tab_skip_a_control_that_does_not_show in focus_fixup_unshown
except: A control whose own :focus-within rule hides its container still takes Tab and loses focus at the next layout.
basis: fct-k5yrahcq
because: The Tab walk now filters on showing for every document, a ruled engine change no design-system row states yet.
@row_set n=1 row=contract/mounteddata-during-render/document-state cell=promise seen=1 detector=none
value: A MountedData call made during a render acts on the document as the previous render left it; a task awaiting the handle of a node its handler shows focuses it once mounted, and a call in the render that removes its target leaves focus off it.
basis: fct-4mnmyqn6
because: The chunk re-states the during-render remnant to the reachable case, so the promise about a node created in the same render no longer reads as the whole limit.
@row_set n=2 row=decision/buffered-render-replay cell=except seen=1 fact=F3 detector=none
value: A task that wakes its own waker while it waits during a render livelocks inside that render, so a task waiting for a later render leaves its waker for the producer.
basis: fct-a4d7c4g3
because: The busy-yield livelock is a stated limit of the buffered render for tasks polled during it, and the row naming polled tasks states none.
