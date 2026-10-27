# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · no manifest, lock file or pin moved, as the report's Untouched list says of dec-dependency-pinning
established-decisions#2.1 · no · the session changed no manifest, so Cargo.toml's named build profiles stand
established-decisions#3.1 · no · no manifest or workflow changed, so the workspace version and publish-browser.yml stand
established-decisions#4.1 · no · the renderer sources, the examples and the wpt, rdme and dioxus-native feature wiring are unchanged
established-decisions#5.1 · no · the default feature lists sit in package manifests the session did not change
established-decisions#6.1 · no · the document defaults sit in blitz-dom and blitz-vibey-script sources the session did not change (Untouched: document core)
established-decisions#7.1 · no · resource loading sits in blitz-dom sources and their tests, none changed (Untouched: networking crates)
established-decisions#8.1 · no · the CSS approximations sit in blitz-dom and stylo_taffy sources the session did not change
established-decisions#9.1 · no · the unsupported-feature rules and rotate_z_axis.rs sit in engine sources the session did not change
established-decisions#10.1 · no · the scrolling and input rules sit in blitz-dom sources the session did not change (Untouched: scrolling, events)
established-decisions#11.1 · no · the seven direct engine changes sit in blitz-dom sources the session did not change (Untouched: events, scrolling, nodes and widgets)
established-decisions#11.2 · no · the document clock sits in blitz-dom's clock, config and scrolling sources, none changed (Untouched: document core)
established-decisions#11.3 · no · the select model sits in node/select.rs and the event sources, none changed (Untouched: events)
established-decisions#12.1 · no · the overlay scrollbar hazard is an engine reading in events/pointer.rs, which the session did not change (Untouched: events)
established-decisions#13.1 · no · the layout rules sit in blitz-dom's layout sources and tests, none changed (Untouched: layout)
established-decisions#14.1 · no · the replaced-element aspect rule sits in layout/replaced.rs, which the session did not change (Untouched: layout)
established-decisions#15.1 · no · the disabled reading sits in node/element.rs and the event sources, none changed (Untouched: nodes and widgets)
established-decisions#16.1 · no · inert handling sits in node/node.rs and accessibility.rs, which the session did not change (Untouched: nodes and widgets)
established-decisions#16.2 · no · focus handling sits in document.rs and mutator.rs, neither changed (Untouched: document core)
established-decisions#16.3 · no · the select choice rules sit in node/select.rs and the UA stylesheet, neither changed (Untouched: nodes and widgets)
established-decisions#17.1 · no · the hit test and the paint-tree walk sit in blitz-dom sources the session did not change (Untouched: nodes and widgets)
established-decisions#18.1 · no · HTML parsing sits in blitz-html and blitz-dom's config, both unchanged (Untouched: HTML crates)
established-decisions#19.1 · no · the shell and windowing sources are unchanged (Untouched: shell)
established-decisions#20.1 · no · the painting rules and the password mask sit in blitz-paint sources the session did not change (Untouched: paint)
