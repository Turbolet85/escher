# Classified — arch · stack-and-technologies.b1

## Answers
stack-and-technologies#1.1 · yes · the new Rust code and tests must build under edition 2024 and rust-version 1.91.0, so no API above that floor
stack-and-technologies#1.2 · no · the chunk adds no workspace crate, version pin or package metadata
stack-and-technologies#1.3 · yes · the new default colours and open-list rules ship in the UA sheet that Stylo's Stylesheet parses and Stylist matches, and a config's ua_stylesheets is used as given
stack-and-technologies#1.4 · no · html5ever and xml5ever parsing is unchanged; option and optgroup tags are read as the parser already built them
stack-and-technologies#1.5 · no · Taffy layout is consumed unchanged; the chunk reads resolved boxes and adds no layout mode
stack-and-technologies#1.6 · yes · option and group labels are painted, and text_contrast measures text, through parley and fontique FontContext at the default family and size
stack-and-technologies#1.7 · yes · the overlay is painted through PaintScene in paint_scene, every blitz-paint renderer, screenshot and screenshot_rect, under the layer cap
stack-and-technologies#1.8 · yes · the overlay's rects and default colours use peniko and color values, and hit testing inverts transforms with kurbo and euclid geometry to refuse a non-invertible one
stack-and-technologies#1.9 · yes · Enter, released Space, arrows and typeahead on a select arrive as keyboard-types 0.8 events in keyboard.rs and through the winit shell's redraw loop
stack-and-technologies#1.10 · yes · build_accessibility_tree writes the select's open state and the option and group names through accesskit's tree builder
stack-and-technologies#1.11 · no · the list loads nothing; reqwest, tokio and the HTTP stack are not reached
stack-and-technologies#1.12 · no · the chunk adds no script surface; Boa is not reached
stack-and-technologies#1.13 · yes · Dioxus event dispatch carries the input and change events, the controlled app's selected write, and the bridge's form data for a select
stack-and-technologies#1.14 · yes · the driver reads the two kept checks that stand_replay replays through serde_json, and png encodes the screenshot that screenshot_rect and the stand decode
stack-and-technologies#1.15 · no · the wasm canvas entry and browser-target builds are out of scope; the chunk adds no wasm surface
stack-and-technologies#1.16 · yes · the windowed path builds its documents Parallel, so the overlay and its state must hold under parallel-construct and the Stylo style pool
stack-and-technologies#1.17 · no · browser history persistence is not reached
stack-and-technologies#1.18 · no · browser app extras, the allocator, nucleo and the Android entry are not reached
stack-and-technologies#1.19 · no · the rdme markdown app is not reached
stack-and-technologies#1.20 · yes · the four stand tables each gain a row, the new stand checks sit in blitz-tests, and the stand's test dependencies serve them
stack-and-technologies#1.21 · no · no packaging moves; the Nix flake and the dev-host toolchain are unchanged
stack-and-technologies#1.22 · yes · the free-disk-space step of the Stand checks repeated job (ci.yml:311-316) is the watch the chunk re-reads
stack-and-technologies#1.23 · no · standing S5 · the fmt, clippy and rustdoc-warnings gates every change passes; the S9 coverage floor and stand-repeat budget are the same standing rule, and the chunk adds no dependency to audit
stack-and-technologies#2.1 · yes · the new functions in execute, apply_select_choice, the overlay and hit testing must stay under the cognitive-complexity ceiling of 15 on the dev-host reading
