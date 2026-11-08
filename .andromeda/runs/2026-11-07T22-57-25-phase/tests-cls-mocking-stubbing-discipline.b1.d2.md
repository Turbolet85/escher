@answer n=1 entity=mocking-stubbing-discipline#1.1 v=no
why: blitz-dom's own mutator and document test fakes; the chunk changes no blitz-dom engine code
@answer n=2 entity=mocking-stubbing-discipline#1.2 v=no
why: interaction_state_teardown's own RecordingShell; the scope names no IME or shell behaviour
@answer n=3 entity=mocking-stubbing-discipline#1.3 v=no
why: select_popup's own CountingShell; the chunk touches no select popup
@answer n=4 entity=mocking-stubbing-discipline#1.4 v=no
why: blitz-shell's test-local event-loop wake counter; the chunk touches no blitz-shell code
@answer n=5 entity=mocking-stubbing-discipline#1.5 v=no
why: link_rel_attribute's own recording net provider; the scope names no link or stylesheet check
@answer n=6 entity=mocking-stubbing-discipline#1.6 v=no
why: render_blocking_stylesheet's file-private manual net provider; the scope names no such check
@answer n=7 entity=mocking-stubbing-discipline#1.7 v=yes
why: fixtures/mod.rs is a shared helper file the scope names, and HeldNet's answer and is_noop contract is what a new held-load stand check reads
@answer n=8 entity=mocking-stubbing-discipline#1.8 v=no
why: stand_load_wait's in-file LateNet; the chunk uses no LateNet and the scope names no load-wait behaviour
@answer n=9 entity=mocking-stubbing-discipline#1.9 v=yes
why: boot_load and stand::options are named scope, and LoadNet is the net_provider boot_load sets while every other boot keeps None
@answer n=10 entity=mocking-stubbing-discipline#1.10 v=no
why: blitz-test-harness settle unit tests' own Scripted and Held doubles; the chunk changes no settle behaviour
@answer n=11 entity=mocking-stubbing-discipline#1.11 v=no
why: blitz-net's unit-test handler; blitz-net is out of the chunk's scope
@answer n=12 entity=mocking-stubbing-discipline#1.12 v=no
why: WPT runner's local-directory net provider; WPT is out of the chunk's scope
@answer n=13 entity=mocking-stubbing-discipline#1.13 v=no
why: blitz-vibey-script's test script fetcher; that crate is out of the chunk's scope
@answer n=14 entity=mocking-stubbing-discipline#1.14 v=no
why: WPT's dummy navigation provider; WPT is out of the chunk's scope
@answer n=15 entity=mocking-stubbing-discipline#1.15 v=no
why: blitz-traits' no-op providers are named only as existing; the chunk's default config uses them without a call or value it must act on
@answer n=16 entity=mocking-stubbing-discipline#1.16 v=no
why: the harness's event-name recorder; the chunk's graph reads handlers but dispatches no event through it
@answer n=17 entity=mocking-stubbing-discipline#1.17 v=no
why: scrollbar_drag's own no-op event handler; the scope names no scrollbar drag check
@answer n=18 entity=mocking-stubbing-discipline#1.18 v=no
why: custom_widget_layout's own probe widgets; the chunk touches no widget layout
@answer n=19 entity=mocking-stubbing-discipline#1.19 v=no
why: paint_tree_bench's null painter for timings; benchmarks are out of the chunk's scope
@answer n=20 entity=mocking-stubbing-discipline#1.20 v=no
why: WPT's virtual-time timers; WPT is out of the chunk's scope and the chunk moves no clock
@answer n=21 entity=mocking-stubbing-discipline#1.21 v=yes
why: boot_timer's TimerTicks in every headless-stand boot is named scope, and the stand's signals move onto the registry under it
@answer n=22 entity=mocking-stubbing-discipline#1.22 v=no
why: debug_timer's dummy timer for instrumentation; the chunk adds no timer instrumentation and touches no debug_timer code
@answer n=23 entity=mocking-stubbing-discipline#1.23 v=no
why: WPT's JS harness replacements; WPT is out of the chunk's scope
@answer n=24 entity=mocking-stubbing-discipline#1.24 v=no
why: apps/browser's capturing publish closure; the chunk touches no browser crate
@answer n=25 entity=mocking-stubbing-discipline#1.25 v=no
why: apps/browser's no-op notify closure; the chunk touches no browser crate
