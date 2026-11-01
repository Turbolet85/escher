# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · the manifests, lock and deny.toml are byte-identical and no crate was added or removed, so the pins stand
established-decisions#2.1 · no · Cargo.toml's profiles are untouched and the manifests are byte-identical
established-decisions#3.1 · no · release versioning lives in Cargo.toml and publish-browser.yml, neither reached, and the merge base is unmoved
established-decisions#4.1 · no · none of the renderer-backend files (examples, wpt runner, apps/readme, dioxus-native renderer, harness screenshot.rs) is reached
established-decisions#5.1 · no · manifests are byte-identical, the default feature lists are unchanged, and the font-embolden feature itself stands, the new leg only calling it
established-decisions#6.1 · yes · add-to · F6: the Stylo prefs move into a once-guarded init_stylo_prefs called from UserAgentStylesheet::parse and BaseDocument::new, and document.rs, a merge surface, moves the cited lines
established-decisions#7.1 · yes · add-to · F7: a failed, empty or non-CSS head stylesheet stops blocking render and a bad href starts no load, through f6edb9701, in reached mutator.rs and net.rs
established-decisions#8.1 · yes · add-to · F8: `<picture>` is no longer display: contents, its upstream rule leaving default.css, and resolved_style.rs and document.rs, merge surfaces, move the cited lines
established-decisions#9.1 · yes · add-to · F5: background-clip: text stays unsupported, ac6f9f582 tried and left out on Stylo 0.22, and stylo.rs and construct.rs, merge surfaces, move the cited lines
established-decisions#10.1 · yes · add-to · F1: the twelve upstream behaviour changes of this take are ratified as engine changes for every document, each by its commit, a new sentence beside the upstream history at lines 67 and 69, and document.rs and node.rs move the cited lines
established-decisions#11.1 · yes · stale · the owner of the merge surface at line 75 re-points off Epoch 3's Upstream sync ahead of the UI graph to the entry Upstream sync ahead of determinism and exploration
established-decisions#11.2 · yes · add-to · F2: the key-press panic's fix is a ratified, flagged-upstreamable direct engine change for every document with nothing sent, and the owner at line 76 re-points to the entry Upstream sync ahead of determinism and exploration
established-decisions#11.3 · yes · stale · the owner at line 77 re-points off Epoch 3's Upstream sync ahead of the UI graph, and document.rs, a merge surface, sits in its merge surface
established-decisions#11.4 · yes · stale · the owner at line 79 re-points off Epoch 3's Upstream sync ahead of the UI graph, and document.rs, mutator.rs, node/element.rs and default.css, all merge surfaces, sit in its merge surface
established-decisions#11.5 · yes · stale · document.rs and render.rs, merge surfaces, and tests/blitz-tests/tests/select_popup.rs, reached, move the cited lines
established-decisions#11.6 · yes · stale · document.rs, a merge surface, and tests/blitz-tests/tests/select_popup.rs, reached, move the cited lines
established-decisions#11.7 · yes · stale · tests/blitz-tests/tests/select_popup.rs, reached, moves the cited lines
established-decisions#11.8 · no · default.css, where the rejected list's colours would land, moves only by the picture rule's removal (F8) with every select rule staying, and no list, :open or overlay hit was added
established-decisions#11.9 · no · docs/capabilities.md is unreached, the lint's contrast reads the same past a blend, and the list still paints last
established-decisions#11.10 · yes · stale · the owner at line 85 re-points off Epoch 3's Upstream sync ahead of the UI graph, and the sixteen-file merge surface count may move
established-decisions#11.11 · no · press_multiple_select_key sits in keyboard.rs, unreached, and no key or select rule moved
established-decisions#11.12 · no · click_select_row sits in pointer.rs, unreached, and no pointer rule moved
established-decisions#11.13 · yes · add-to · F1: the twelve upstream behaviour changes of this take are ratified as engine changes for every document, each by its commit, beside the six upstream behaviour changes recorded here
established-decisions#12.1 · yes · stale · document.rs, a merge surface, moves the cited lines 2436 to 2438
established-decisions#13.1 · yes · add-to · F9: compute_layout_damage's table_needs_rebuild takes ALL_DAMAGE when a table box's position, padding or border changes, beside the damage bits, and construct.rs and damage.rs, merge surfaces, move the cited lines
