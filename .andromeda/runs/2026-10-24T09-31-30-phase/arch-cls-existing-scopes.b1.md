# Classified — arch · existing-scopes.b1

## Answers
existing-scopes#1.1 · yes · its Unix implementation forbids neither or both of async-io and tokio, and the tokio update reaches it indirectly through accesskit_unix
existing-scopes#1.2 · no · the take names no phase-timing site or its enable feature, so the chunk does not reach it
existing-scopes#1.3 · no · upstream's delta edits none of its modules, and the Not-needed list excludes the provider traits it holds (NetProvider and the rest)
existing-scopes#1.4 · yes · upstream's engine delta lands in its modules (the layout split, ElementData class atoms, Node out-of-flow-ness, config.rs's serialised traversal, the dropped-node hit test and the document clock)
existing-scopes#1.5 · yes · the mutation, query and CSSOM need binds through the Stylo 0.23 move, and getComputedStyle, CSS.supports and CSS.registerProperty live in these files
existing-scopes#1.6 · yes · query selectors, closest, matches and getElementById are the query contract that the mutation, query and CSSOM need binds through the Stylo 0.23 move
existing-scopes#1.7 · yes · document.rs is a named merge surface for the document clock re-read and the password mask, and it holds the CSSOM View geometry that the scrolling and input need binds
existing-scopes#1.8 · yes · accessibility.rs is named, since build_accessibility_tree and its state reads bind through the Stylo 0.23 move touching that file
existing-scopes#1.9 · yes · events/pointer.rs is upstream's named edit site, and the keyboard activation, range input and select events bind through it and the files around it
existing-scopes#1.10 · yes · scrolling.rs is a named merge surface, where scroll_into_view and visible_region bind through the scrolling conflict and the document clock re-read
existing-scopes#1.11 · yes · the Stylo pin move to git 0.23-pre and the style-threading ground (the serialised Parallel traversal, Stylo's global rayon pool) reach its styling participation and Device management
existing-scopes#1.12 · yes · tree.rs is the versioned node storage whose dropped ids stop resolving, which upstream's dropped-node hit-test skip and stale-node-mapping test reach
existing-scopes#1.13 · yes · layout is where the replaced-element aspect-ratio height, the inline layout module split, Node out-of-flow-ness, LayoutPassState and the layout/construct.rs conflict land
existing-scopes#1.14 · yes · ElementData holds the class atoms, is_disabled and the select's CHECKED state, and node/range.rs and node/select.rs carry the range and select models the events and DOM-semantics needs name
existing-scopes#1.15 · no · the Not-needed list excludes the HTML crate and its HtmlDocument, which upstream's delta does not reach, and its html5ever pin is carried by the pin need rather than by this parsing scope
existing-scopes#1.16 · no · the Not-needed list excludes the networking crates and blitz-net's Provider, and the tokio update reaches the graph through accesskit_unix rather than here
existing-scopes#1.17 · yes · render.rs is the named merge surface for the password input painted masked in every document, and the stand's screenshot pixels move with the Taffy or Stylo pins through the paint modules
existing-scopes#1.18 · no · the Not-needed list excludes the JavaScript runtime and ScriptDocument, since upstream's delta changes no script behaviour, and the document clock the take re-reads is blitz-dom's rather than this crate's clock module
existing-scopes#1.19 · yes · the tokio update reaches blitz-shell through accesskit_xplat, the indirect path the shell-and-windowing need names
existing-scopes#1.20 · no · the take adds no crate and names no change to the six engine crates the umbrella re-exports
existing-scopes#1.21 · yes · the Stylo pin move to git 0.23-pre that the take could make reaches its Stylo-to-Taffy conversion, and the stand's bounds move with the Taffy or Stylo pins through it
existing-scopes#1.22 · yes · the re-sync of packages/dioxus-native from the Dioxus repo changes its launch, launch_cfg, launch_cfg_with_props and Config builders
existing-scopes#1.23 · yes · the Dioxus 0.8 sync replaces dioxus_document.rs, events.rs, lib.rs and mutation_writer.rs, adds mutation_buffer.rs and deletes keyboard_compat.rs, which this entity's module list names
existing-scopes#1.24 · yes · the screenshot and contrast modules sit under the stand's pixel, bounds and computed-style checks that a Taffy or Stylo move can move
existing-scopes#1.25 · yes · the telemetry sink's target allowlist and format module are re-checked against upstream's new println! and env-read sites in examples/multi_doc.rs
