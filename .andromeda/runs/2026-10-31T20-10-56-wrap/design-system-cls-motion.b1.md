# Classified — design-system · motion.b1

## Answers
motion#1.1 · no · the searched apps/browser CSS, seven_guis src and todomvc.css are outside the reached files, and the session edited no styles or sources there
motion#1.2 · yes · add-to · F12, a wheel over the open list scrolls the list and a wheel elsewhere closes it then scrolls, joins this wheel-over-scroller line
motion#1.3 · no · the script-runtime and dioxus-native-dom scroll-behaviour paths (events.rs, vibey-script) are not among the reached files
motion#1.4 · yes · stale · stand_keyboard.rs is reached and gained checks (21 to 23), so its cited lines 293-335 may have moved and the report does not show them unmoved
motion#1.5 · yes · stale · a list row's scroll_into_view writes only the list's scroll_y and returns with no chain or viewport, scrolling.rs:669-672 changed inside the cited 655-738, and F12's key-brings-row-into-view-at-once belongs here too
motion#1.6 · yes · stale · scrolling.rs changed past line 669 and the cited fling lines 891-909 sit past it, so their numbers moved
motion#1.7 · yes · stale · document.rs gained lines above the cited 2341-2358 and render.rs changed past line 7, so the cited lines 728-733 and 755-758 moved
motion#1.8 · no · the report reads the thumb hazard as unmoved, the list answering first only for its own rows and no scrollbar answering with it, so a page scrollbar band still takes the drag as stated
motion#2.1 · no · stylo.rs changed only past line 1124, so the cited animation-state lines 112-130 are unmoved and the claim holds
motion#2.2 · yes · stale · document.rs gained lines above the cited 2631-2648, moving that range, while the list adds no animating member
motion#3.1 · yes · add-to · F11, the list has no motion and a screen read after a step holds the new state with no advance, joins this clock statement beside click, type and press
motion#4.1 · no · the load-wait claim cites escher-driver session.rs, which is not among the reached files
motion#4.2 · no · the requestAnimationFrame timer lives in blitz-vibey-script, which is not reached
motion#4.3 · no · the animation event data lines are in dioxus-native-dom events.rs, not reached, and the session brings no animation event
motion#5.1 · no · the BBC fixture lines are in examples/assets, which is not reached
motion#6.1 · no · the report reads the list as having no motion to reduce and its gate entry green, so the reduced-motion absence stands
motion#6.2 · no · the searched s07 slice files are not among the reached files; the list's no-motion fact is carried at motion#3.1
motion#7.1 · no · the cited examples/assets animation fixtures are not reached
motion#7.2 · no · the cited google.html fixture is not reached
motion#7.3 · no · the cited graphite, gosub and inline-flex-transform fixtures are not reached
motion#7.4 · no · the cited servo.css fixture is not reached
motion#7.5 · no · the cited examples/svg_native.rs and examples/transforms.rs are not reached
