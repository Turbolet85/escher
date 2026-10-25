# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · Cargo.toml, the lockfile and deny.toml are untouched and no reached file is a manifest
established-decisions#2.1 · no · the root manifest's profile sections are untouched and no reached file is a manifest
established-decisions#3.1 · no · the release workflow and the workspace version are untouched, and the CI workflows are unchanged
established-decisions#4.1 · no · the renderer examples, wpt, rdme and dioxus-native files it cites are not reached and no engine crate changed
established-decisions#5.1 · no · the default feature lists in the cited manifests are untouched and no engine crate under packages changed
established-decisions#6.1 · no · the cited defaults and the examples' ColorScheme::Light lines are in files not reached; boot_load changes only the harness's net provider
established-decisions#7.1 · no · the render-blocking and link-loading paths in blitz-dom and blitz-net are unchanged, and the load page's fetch is answered by the stand's own LoadNet
established-decisions#8.1 · no · the approximations live in stylo_taffy and blitz-dom files the session does not reach
established-decisions#9.1 · no · the unsupported-feature list and its cited test live in files the session does not reach
established-decisions#10.1 · yes · add-to · F8: right after a scroll a main-button pointerdown in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing, a pointer left on the thumb keeping the bar shown
established-decisions#11.1 · no · the session reaches no blitz-dom hit, inert, disabled, password-caret or client-rect file, so the seven engine changes stand as stated
established-decisions#11.2 · yes · add-to · F8, its clock half: on the VirtualClock the shown overlay bar stays shown until an advance passes its fade
established-decisions#11.3 · no · the select model's engine files are not reached, and the flight booker's select is a stand change this claim does not name
established-decisions#12.1 · no · the layout files and tests it cites are not reached and no layout path changed
established-decisions#13.1 · no · replaced.rs is not reached and the aspect-ratio rule is unchanged
established-decisions#14.1 · yes · stale · the session re-pinned tests/blitz-tests/tests/stand_keyboard.rs (activation table 20 → 16 rows, tests 14 → 15), so its citation at :677-718 has moved; the engine's disabled rule holds
established-decisions#15.1 · no · inert_subtree.rs and the node and accessibility files it cites are not reached and no inert path changed
established-decisions#15.2 · no · the focus-within code in document.rs and mutator.rs is not reached, and the Tab-order change is a stand matter this claim does not name
established-decisions#15.3 · no · the select CHECKED and selected rules are in engine files the session does not reach, and the claim names no stand file
established-decisions#16.1 · no · the dropped-node hit skip lives in engine files the session does not reach
established-decisions#17.1 · no · html_sink.rs and the feature-gate files it cites are not reached, and the session changes no parser or feature
established-decisions#18.1 · no · the shell and windowing files it cites are not reached and no shell path changed
established-decisions#19.1 · no · the paint files it cites are not reached and the painting path is unchanged
