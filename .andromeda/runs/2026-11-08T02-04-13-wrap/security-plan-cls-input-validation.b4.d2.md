@answer n=1 entity=input-validation#1.76 v=no
why: the checkbox and radio change event lives in blitz-dom pointer.rs, which the session did not change, and the dispatch change is inert without a registry
@answer n=2 entity=input-validation#1.77 v=yes mode=stale
why: the session reaches dioxus_document.rs, snapshot.rs and escher-driver schema.rs, so the cited lines 246-249, 257-281 and 420-425 may have moved
@answer n=3 entity=input-validation#1.78 v=no
why: the select list state lives in blitz-dom document.rs and pointer.rs, which the session did not reach
@answer n=4 entity=input-validation#1.79 v=no
why: the open list's key handling lives in blitz-dom keyboard.rs, which the session did not change
@answer n=5 entity=input-validation#1.80 v=no
why: pointer handling in and around an open list lives in unchanged blitz-dom files
@answer n=6 entity=input-validation#1.81 v=no
why: the hostile-markup claim rests on blitz-dom code and the select_popup test, none of which the session reached
@answer n=7 entity=input-validation#1.82 v=no
why: multiple-select keys and clicks live in blitz-dom keyboard.rs and pointer.rs, which the session did not change
@answer n=8 entity=input-validation#1.83 v=no
why: optgroup handling lives in unchanged blitz-dom node/select.rs and mutator.rs
@answer n=9 entity=input-validation#1.84 v=yes mode=stale
why: dioxus_document.rs is reached whole by the event-dispatch change, so the cited multiple_select_values lines 33-47 may have moved
@answer n=10 entity=input-validation#1.85 v=yes mode=stale
why: the graph verb reaches execute.rs and schema.rs, so the cited type-help and dispatch lines 242-249 and 571-577 may have moved
@answer n=11 entity=input-validation#1.86 v=yes mode=stale
why: snapshot.rs gained lines ahead of the cited 254-264 and dioxus-native-dom lib.rs is reached, so the citations shift
@answer n=12 entity=input-validation#1.87 v=no
why: the select list's public readers live in unchanged blitz-dom document.rs and lib.rs
@answer n=13 entity=input-validation#1.88 v=yes mode=stale
why: snapshot.rs, snapshot_diff.rs and snapshot_text.rs are reached and the masking row's cited lines moved, and the graph is a new carrier the row must cover
@answer n=14 entity=input-validation#1.89 v=no
why: aspect-ratio handling lives in unchanged blitz-dom layout code
@answer n=15 entity=input-validation#1.90 v=no
why: the inline-box height clamp lives in unchanged blitz-dom layout code
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent.rs is unchanged and the graph answer reads ids, names and flags, no geometry, so Session::lint stays the sole caller
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region lives in unchanged blitz-dom scrolling.rs and the graph answers no geometry, so its listed callers stand
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling lives in unchanged blitz-paint and blitz-dom layout code
@answer n=19 entity=input-validation#1.94 v=no
why: select list painting lives in unchanged blitz-paint code
@answer n=20 entity=input-validation#1.95 v=no
why: the non-invertible transform hit test lives in unchanged blitz-dom node.rs
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamps live in unchanged blitz-paint code
@answer n=22 entity=input-validation#1.97 v=no
why: border radius scaling lives in unchanged blitz-paint code
@answer n=23 entity=input-validation#1.98 v=no
why: the hidden-input paint skip lives in unchanged blitz-paint render.rs
@answer n=24 entity=input-validation#1.99 v=no
why: scroll target clamps live in unchanged blitz-dom scrolling.rs
@answer n=25 entity=input-validation#1.100 v=no
why: a select list's own scroll lives in unchanged blitz-dom scrolling.rs and node/select_list.rs
