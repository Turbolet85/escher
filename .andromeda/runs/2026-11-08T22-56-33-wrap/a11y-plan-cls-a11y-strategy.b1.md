@answer n=1 entity=a11y-strategy#1.1 v=no
why: the reached record and graph reader leave the blitz-shell refresh path, the tree builder reads and the DioxusDocument stable-id override unchanged
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the reached record writes a Rust-side field and the new graph reads are read-only, so the changed-set marking, the settle and the fresh-boot claims hold
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the session touches no blitz-shell event handling, so the unhandled AccessibilityDeactivated and ActionRequested TODOs stand
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding in blitz-shell is outside the session's reach and unchanged
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role derivation is unchanged and accessibility_roles.rs is not in the session's change
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the semantic-page role assertion and its test file are unchanged, so only html and body still map to Unknown
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the tree builder's ARIA state reads are unchanged, so the cited state mappings and their tests stand
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio write is unchanged and its pinned test in accessibility_states.rs is not reached
@answer n=9 entity=a11y-strategy#2.5 v=no
why: the six upstream reads are unchanged and the new stand target asserts keyboard activation and handler coverage, so no stand file states any of the six
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module selection lives in accesskit_xplat, which the session does not reach
@answer n=11 entity=a11y-strategy#3.2 v=no
why: the accesskit_xplat warning lives in a crate the session does not reach
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the adapter set_focus and set_window_bounds bodies in accesskit_xplat are unchanged by the session
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter Rect import and parameters in accesskit_xplat are unchanged by the session
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android surface view field read in accesskit_xplat is unchanged by the session
@answer n=15 entity=a11y-strategy#4.1 v=yes mode=stale
why: the session adds an ungated reached record and reader and moves the lib.rs re-exports from 35 to 36, so the cited lib.rs and dioxus_document.rs lines can have moved although the gating claim holds
