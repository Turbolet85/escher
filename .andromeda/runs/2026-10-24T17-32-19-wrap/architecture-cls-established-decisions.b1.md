# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · no manifest, lockfile or pin moved; the reached files are CI scripts and test files, not the Cargo.toml lines this row cites
established-decisions#2.1 · no · the dev profile in Cargo.toml is unchanged and the session adds no test target or build setting
established-decisions#3.1 · no · the release versioning in publish-browser.yml and Cargo.toml is unchanged; no release path is reached
established-decisions#4.1 · no · no renderer, example or harness raster source changed; the reached files are test and CI files only
established-decisions#5.1 · no · no crate's default feature list changed; Cargo.toml is not reached
established-decisions#6.1 · no · the document defaults in blitz-dom's config and document sources are unchanged; none is reached
established-decisions#7.1 · no · resource loading in blitz-dom is unchanged and the link and render-blocking tests this row cites are not reached
established-decisions#8.1 · no · the CSS approximations in blitz-dom and stylo_taffy are unchanged; no engine source is reached
established-decisions#9.1 · no · the unsupported-feature claims in blitz-dom and stylo_taffy are unchanged; no engine source is reached
established-decisions#10.1 · no · the scrolling and input engine sources this row cites are unchanged; the session edits no engine source
established-decisions#11.1 · no · the seven engine changes this row lists are unchanged, and no engine source is reached
established-decisions#11.2 · no · the document clock and its constants are unchanged; no engine source is reached
established-decisions#11.3 · no · the select model is unchanged; its engine sources are not reached and the row names no runner
established-decisions#12.1 · no · the layout rules in blitz-dom's layout modules are unchanged; no layout source is reached
established-decisions#13.1 · no · the replaced-element aspect-ratio rule in blitz-dom is unchanged; no layout source is reached
established-decisions#14.1 · no · the lang, dir and disabled rules are unchanged; the focusability and keyboard tests this row cites are not reached
established-decisions#15.1 · no · inert's engine rules are unchanged and inert_subtree::removing_inert_restores_focus_hit_and_tree still pins the limit; the session only adds a stand-repeat runner
established-decisions#15.2 · no · focus-within handling in blitz-dom's document and mutator sources is unchanged; no engine source is reached
established-decisions#15.3 · no · the select choice rules in blitz-dom's select, mutator and form sources are unchanged; none is reached
established-decisions#16.1 · no · the dropped-node hit skip in blitz-dom is unchanged; no engine source is reached
established-decisions#17.1 · no · the HTML parsing settings in blitz-html and blitz-dom config are unchanged; no parser source is reached
established-decisions#18.1 · no · the shell and windowing workarounds in blitz-shell and blitz-traits are unchanged; no shell source is reached
established-decisions#19.1 · no · the painting rules and the password mask in blitz-paint are unchanged; the password caret test added to stand-repeat changes no paint rule
