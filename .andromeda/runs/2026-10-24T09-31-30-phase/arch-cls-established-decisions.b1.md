# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · the coupled html5ever, skrifa and svgtypes pins, the registry taffy and parley versions and the exact winit beta are the Dependency pinning Need
established-decisions#2.1 · no · build profiles are Not needed, since upstream's delta changes none of them
established-decisions#3.1 · yes · the one workspace version and the in-repo `=` pins are the Release versioning Need, which the Dioxus package sync and the manifest merge touch
established-decisions#4.1 · no · renderer backends are Not needed, since upstream's delta changes none of them and the chunk changes no raster path
established-decisions#5.1 · yes · the default feature lists of dioxus-native(-dom) and the blitz crates are re-synced by the Dioxus package sync and the manifest merge, and the stand's checks build on them
established-decisions#6.1 · yes · the incremental-layout default sits in config.rs, a file the serialised Parallel traversal and the clock edit both touch, and the one incremental pipeline Need binds it
established-decisions#7.1 · no · resource loading (render-blocking stylesheets, sub-document and import caps, srcdoc, image cache) is named by no Need, and the mutator.rs conflict is the changed-set insert, not these lines
established-decisions#8.1 · yes · the Needs bind the CSS approximations list through the new text-indent and word-break keywords
established-decisions#9.1 · yes · the Needs bind the unsupported-features list through the new text-indent and word-break keywords
established-decisions#10.1 · yes · scroll_into_view, visible_region, the clipping hit stop, absolute_position and the range keys are the Scrolling and input Need, bound through the scrolling.rs conflict and node/node.rs
established-decisions#11.1 · yes · the inert and form-control disabled reads, the transform-aware client rect and the password paint are named Needs, the caret click riding the pointer.rs conflict
established-decisions#11.2 · yes · the document clock is the Needs' DocumentConfig.clock, re-read on config.rs, document.rs, scrolling.rs and events/pointer.rs
established-decisions#11.3 · yes · the select model is the events Need (a select's input then change) and the DOM semantics Need (the choice held as CHECKED in node/select.rs)
established-decisions#12.1 · yes · the replaced-element and out-of-flow rules and the layout/construct.rs conflict are the Layout Need
established-decisions#13.1 · yes · disabled read on form controls alone through ElementData::is_disabled and the ElementData accessors are the DOM semantics Need, with the class atoms landing there
established-decisions#14.1 · yes · inert read by presence on an element or an ancestor is the inert Need under DOM semantics, governing the Tab walk, hit test and accessibility tree
established-decisions#14.2 · yes · a focus move marking only its two nodes is the changed-set rule that binds through mutator.rs, the predicted conflict holding the changed-set insert
established-decisions#14.3 · yes · a select's choice held as CHECKED, with node/select.rs and the mutator's select records, is the DOM semantics Need
established-decisions#15.1 · yes · the UA stylesheet default that DocumentConfig::default() supplies in config.rs binds the merge of that Default impl, though the HTML parsing rules are Not needed
established-decisions#16.1 · yes · accesskit_xplat forbidding neither or both of async-io and tokio on Unix is the shell-and-windowing Need, bound through the tokio lock update
established-decisions#17.1 · yes · the password input painted masked in every document is the Painting Need, merging through blitz-paint render.rs and blitz-dom document.rs
established-decisions#18.1 · no · the JavaScript runtime decisions and contract are Not needed, since upstream's delta changes no script behaviour
