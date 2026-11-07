@answer n=1 entity=existing-scopes#1.1 v=no
why: the accesskit_xplat adapter is not_needed (task item 10) and the chunk does not reach it
@answer n=2 entity=existing-scopes#1.2 v=no
why: debug_timer is a feature-gated timer the chunk does not reach
@answer n=3 entity=existing-scopes#1.3 v=no
why: blitz-traits only lists its modules here and states no redraw behaviour
@answer n=4 entity=existing-scopes#1.4 v=no
why: the blitz-dom entity only lists the module names, including mutator, and states no flush or redraw rule
@answer n=5 entity=existing-scopes#1.5 v=no
why: CSSOM rule editing and getComputedStyle are not on the bridge's render path
@answer n=6 entity=existing-scopes#1.6 v=no
why: forms, iframes and query selectors are not named by the chunk's scope
@answer n=7 entity=existing-scopes#1.7 v=yes
why: it names blitz-dom focus, the engine focus change that set_focus reaches and the inverted pins assert (task item 5)
@answer n=8 entity=existing-scopes#1.8 v=no
why: blitz-dom accessibility tree building is not what the bridge's snapshot calls; the snapshot is the bridge's own code
@answer n=9 entity=existing-scopes#1.9 v=yes
why: it names the events focus submodule, which carries the blur, focusout, focus and focusin change that set_focus reaches
@answer n=10 entity=existing-scopes#1.10 v=yes
why: it names programmatic and into-view scrolling, the carriers of MountedData scroll_to, scroll and get_scroll_offset (task item 3 and the scope)
@answer n=11 entity=existing-scopes#1.11 v=no
why: Stylo integration is not reached by the render path changes
@answer n=12 entity=existing-scopes#1.12 v=yes
why: versioned node storage is what keeps a removed node from resolving, which the replayed buffer must keep (task item 3)
@answer n=13 entity=existing-scopes#1.13 v=yes
why: it names damage, the incremental layout invalidation that the incremental and non-incremental identity re-measured by incremental_oracle reads (task item 9)
@answer n=14 entity=existing-scopes#1.14 v=yes
why: it names the node tree and attributes that the replayed mutations write and remove through MutationWriter
@answer n=15 entity=existing-scopes#1.15 v=no
why: HTML parsing through blitz-html is not_needed (task item 11)
@answer n=16 entity=existing-scopes#1.16 v=no
why: blitz-net networking is not_needed (task item 4)
@answer n=17 entity=existing-scopes#1.17 v=no
why: blitz-paint painting and renderer backends are not_needed (task item 5)
@answer n=18 entity=existing-scopes#1.18 v=no
why: blitz-vibey-script runs no page script in a Dioxus document and is not_needed (task item 3)
@answer n=19 entity=existing-scopes#1.19 v=yes
why: it states that a redraw request reaches the receiver and wakes the loop once, the redraw behaviour the replay must keep (task item 4)
@answer n=20 entity=existing-scopes#1.20 v=no
why: the blitz crate only re-exports the other crates and the chunk does not touch it
@answer n=21 entity=existing-scopes#1.21 v=no
why: stylo_taffy style conversion is not named by the chunk's scope
@answer n=22 entity=existing-scopes#1.22 v=yes
why: the windowed dioxus-native launch is read per task item 14, and the crate depends on dioxus-native-dom, where the buffered render lands
@answer n=23 entity=existing-scopes#1.23 v=yes
why: the bridge crate's lib.rs module list gets mutation_buffer, and dioxus_document.rs and mutation_writer.rs are edited
@answer n=24 entity=existing-scopes#1.24 v=no
why: the blitz-test-harness modules, including screenshot and contrast, are only listed and state no rule the chunk's tests must honour
@answer n=25 entity=existing-scopes#1.25 v=yes
why: any trace the buffer module keeps passes through this formatter's target allowlist and field scrub (task items 10 and 11)
