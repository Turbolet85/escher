# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · a blitz-dom unit-test fake that counts redraw requests; the statement claims no redraw behaviour and the chunk reaches no such test
mocking-stubbing-discipline#1.2 · no · an interaction-state test's IME recorder; the statement claims no IME behaviour
mocking-stubbing-discipline#1.3 · no · a link-rel test's URL recorder; no statement claim rests on link loading
mocking-stubbing-discipline#1.4 · no · upstream's file-private manual net in a render-blocking test; the statement claims no render-blocking behaviour
mocking-stubbing-discipline#1.5 · yes · HeldNet is the held stand-in that stand_settle reads NotSettled Loads through, and the never-answered arm of the load wait uses it
mocking-stubbing-discipline#1.6 · yes · LateNet is stand_load_wait's self-answering stand-in, the witness that a load in flight is answered inside LOAD_WAIT
mocking-stubbing-discipline#1.7 · no · settle.rs unit-test stand-ins; the settle claims are witnessed by stand_settle and stand_load_wait, not by unit tests
mocking-stubbing-discipline#1.8 · no · the WPT runner's local-file net; WPT is outside the statement's capabilities
mocking-stubbing-discipline#1.9 · no · a blitz-vibey-script test fetcher; script execution is outside the statement's capabilities
mocking-stubbing-discipline#1.10 · no · the WPT runner's dummy navigation provider; WPT is outside the statement's capabilities
mocking-stubbing-discipline#1.11 · no · blitz-traits' default no-op providers; the statement names no provider default and cites no check built on one
mocking-stubbing-discipline#1.12 · yes · it records each dispatched DOM event name, the instrument for the select and range one-input and one-change counts and for keyboard activation's dispatched Click
mocking-stubbing-discipline#1.13 · no · a scrollbar-drag test's no-op handler; the statement claims no scrollbar drag
mocking-stubbing-discipline#1.14 · no · a custom-widget layout test's probe widgets; the statement claims no custom widget
mocking-stubbing-discipline#1.15 · no · a paint-timing bench's null renderer; the statement claims no paint timing and the screenshot path paints through VelloCpuImageRenderer
mocking-stubbing-discipline#1.16 · no · the WPT runner's virtual timers; WPT is outside the statement's capabilities
mocking-stubbing-discipline#1.17 · yes · TimerTicks is the witness for the virtual clock: advance moves the clock by exactly ms, advanced_ms is that move, and timer_step delivers whole ticks
mocking-stubbing-discipline#1.18 · no · debug_timer's zero-cost dummy when off; phase timing is not a statement claim
mocking-stubbing-discipline#1.19 · no · the WPT runner's replaced JS harness; WPT is outside the statement's capabilities
mocking-stubbing-discipline#1.20 · no · an apps/browser worker test's capturing publish closure; apps/browser is outside the statement
mocking-stubbing-discipline#1.21 · no · an apps/browser no-op notify closure; apps/browser is outside the statement
mocking-stubbing-discipline#1.22 · no · the WPT-diff PR script's dry-run stub in its own CI-script test; the README edit reaches neither the script nor its test
mocking-stubbing-discipline#2.1 · no · a blitz-dom hover-test skip precondition; unit-test conventions are not needed and no claim rests on hover tests
mocking-stubbing-discipline#3.1 · no · an absence of mocking libraries in CI and build config; no claim or README line rests on it
mocking-stubbing-discipline#3.2 · no · an absence of mocking libraries in the apps; apps/browser is outside the statement
