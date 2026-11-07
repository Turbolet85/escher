@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's own ShellProvider redraw-counting fakes in mutator.rs and document.rs; blitz-dom is not reached and the scope names no shell call on the render path
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: RecordingShell in interaction_state_teardown.rs records set_ime_enabled; no IME call is on the chunk's render or poll path
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's CountingShell counts select-list redraws; the select popup is not a Dioxus document path the chunk reaches
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: CountedWakes in blitz-shell's event loop test; blitz-shell is not touched and the scope names no event-loop wake
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: RecordingNetProvider in link_rel_attribute records URLs; no stylesheet or link path is named by the scope
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet's file-private ManualNetProvider; its file is not reached and no scope item names it
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=yes
why: HeldNet is the load counter stand_settle reads as NotSettled busy Loads; the scope names settle's Loads reading, which the buffered poll must not move
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=yes
why: LateNet's in-flight-then-answered timing is what stand_load_wait re-asserts, and the stand-repeat leg with zero flakiness budget runs that check over the new render path
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: LoadNet does not override is_noop, so the harness counts the load; stand_load_page and the load-in-flight check, both re-asserted stand_* checks, read it through the buffered poll
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: settle.rs unit doubles (Scripted document, Held net) witness the settle loop's own rules; the chunk changes neither settle nor these doubles, and poll's answer is witnessed by stand_settle through DioxusDocument
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's test-local Handler over its fetch helper; blitz-net is not reached
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WptNetProvider serves WPT files; WPT runs on upstream only, not the Dioxus bridge
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: MapFetcher in blitz-vibey-script tests; ScriptDocument is not a Dioxus document
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: DummyNavigationProvider for WPT; WPT is not reached by the chunk
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: no-op net, navigation and shell providers; the scope names only the no-op HTML parser that DocumentConfig::default carries, and no re-asserted check reads a provider's output
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: RecordingHandler records dispatched DOM event names through dispatch_recorded; the scope names press, focused and mounted-event order but not this recorder
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: NoopEventHandler passed to EventDriver in scrollbar_drag; that file is not reached
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: Probe and SizedProbe widgets in custom_widget_layout; widget layout is blitz-dom's and unchanged
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: NullScenePainter in paint_tree_bench; no paint path changes
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT virtual time without the timer thread; WPT is not reached
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: TimerTicks is applied on the next pass, which is the buffered poll the chunk changes; pump is named by the scope, and stand_timer, stand_settle and stand_act_timer re-assert that delivery
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's zero-cost dummy timer; no timing instrumentation is added or changed
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: WPT JS harness replacement; WPT is not reached by the chunk
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: worker publication closure in apps/browser url_suggestions; the browser app is not reached
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: no-op notify closure for the nucleo driver in apps/browser; not reached
