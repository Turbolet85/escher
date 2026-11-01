# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · manifests, lock and deny.toml are byte-identical and no crate was added or removed, so the pinning rules and their lock record stand
established-decisions#2.1 · no · the profiles live in Cargo.toml, unreached and byte-identical, so the profile list and its measurements stand
established-decisions#3.1 · no · the workspace version and release manifests are unreached and no release tool or crate changed
established-decisions#4.1 · no · the renderer selection lives in examples, wpt, rdme and dioxus-native files, none reached, and the feature manifests are unchanged
established-decisions#5.1 · no · the default feature lists live in unreached byte-identical manifests, and line 32 is named as not moving
established-decisions#6.1 · yes · add-to · F6 `init_stylo_prefs`, behind a `Once` and called from `UserAgentStylesheet::parse` and `BaseDocument::new`, joins the Stylo prefs list; document.rs and stylo.rs are merge surfaces, so the cited digits move too
established-decisions#7.1 · yes · add-to · F7 joins the render-blocking rule: a failed, empty or non-CSS head stylesheet stops blocking, and a bad `href` starts no load; mutator.rs, net.rs and render_blocking_stylesheet.rs are reached
established-decisions#8.1 · yes · stale · document.rs:3524-3529 and resolved_style.rs:774-797 are merge surfaces whose line citations move, and the CSSOM geometry claim is not shown to hold on the changed file
established-decisions#9.1 · yes · add-to · F5 `background-clip: text` is unsupported (`BackgroundClip::Text` is gecko-only on Stylo 0.22, `ac6f9f582` left out) joins the unsupported list; stylo.rs and construct.rs are merge surfaces
established-decisions#10.1 · yes · add-to · F1 the twelve upstream commits take a new record beside the `ebacd68bd` take record at lines 67 and 69; document.rs and render.rs are merge surfaces
established-decisions#11.1 · yes · stale · its merge-surface owner line (Epoch 3's «Upstream sync ahead of the UI graph») is re-pointed to the entry Upstream sync ahead of determinism and exploration
established-decisions#11.2 · yes · add-to · F2 the key-press panic's fix in `deallocate_anonymous_block` is a ninth ratified direct engine change beside the eight, and the owner line re-points to the next sync's entry
established-decisions#11.3 · yes · stale · the document-clock merge surface, which includes document.rs and events/pointer.rs, has its owner line re-pointed off the UI graph sync
established-decisions#11.4 · yes · stale · the select model's merge surface, which includes document.rs and mutator.rs, has its owner line re-pointed off the UI graph sync and its keyboard and pointer citations move
established-decisions#11.5 · yes · stale · cites document.rs:822-855, render.rs:248-249 and select_popup.rs:202-230, all reached; select_popup.rs's row-3 skip is retired
established-decisions#11.6 · yes · stale · cites document.rs:892-918 and select_popup.rs:401-516, reached files whose line digits move with the change
established-decisions#11.7 · yes · stale · cites select_popup.rs:249-388, a reached test file whose row-3 skip is retired, so the disabled-row claim is not shown to hold
established-decisions#11.8 · yes · stale · the rejected list's colours in default.css and hit_inner in document.rs sit in merge surfaces, the claim not shown to hold on the changed files
established-decisions#11.9 · yes · stale · the lint's contrast reading in contrast.rs is reached and the snapshot reader's signature changed, so the highlight-limit claim is not shown to hold
established-decisions#11.10 · yes · stale · the select list's sixteen-file merge surface includes reached merge surfaces, and its owner line is re-pointed off the UI graph sync
established-decisions#11.11 · no · the multiple-select key handling lives in keyboard.rs, which is not reached, and no cited file is reached
established-decisions#11.12 · no · the multiple-select pointer handling lives in pointer.rs, which is not reached, and no cited file is reached
established-decisions#11.13 · yes · add-to · F1 the twelve upstream behaviour changes ratified as engine changes for every document join the list of upstream's behaviour changes not counted as escher's direct changes
established-decisions#12.1 · yes · stale · cites document.rs:2436-2438, a reached merge surface whose line digits move, so the overlay-scrollbar hazard claim is not shown to hold
established-decisions#13.1 · yes · add-to · F9 `table_needs_rebuild` takes `ALL_DAMAGE` when a table box's position, padding or border changes, joining the damage rules; damage.rs and construct.rs are merge surfaces
