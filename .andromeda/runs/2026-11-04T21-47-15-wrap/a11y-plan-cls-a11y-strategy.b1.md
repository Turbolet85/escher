@answer n=1 entity=a11y-strategy#1.1 v=no
why: the session changed no blitz-shell or blitz-dom code, so the poll-time tree refresh and its changed-set drain stand as described
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set marks and their engine sites are in blitz-dom code the session did not change
@answer n=3 entity=a11y-strategy#1.3 v=no
why: the AccessibilityDeactivated and ActionRequested TODOs in blitz-shell application.rs are untouched by the session
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarding in blitz-shell is untouched, no shell file changed
@answer n=5 entity=a11y-strategy#2.1 v=no
why: role derivation is untouched and the session changed no builder or role-mapping code
@answer n=6 entity=a11y-strategy#2.2 v=no
why: role derivation is untouched and the session changed no builder code behind the Unknown-role rule
@answer n=7 entity=a11y-strategy#2.3 v=no
why: the builder's ARIA state reads are untouched, as the section's untouched item records, and no engine or builder code changed
@answer n=8 entity=a11y-strategy#2.4 v=no
why: the native checkbox and radio read in the builder is untouched, and no engine or builder code changed
@answer n=9 entity=a11y-strategy#2.5 v=no
why: upstream's six reads and the snapshot's description and enabled reads are untouched, NODE_FIELDS stays thirteen, and no builder code changed
@answer n=10 entity=a11y-strategy#3.1 v=no
why: the platform module under packages/accesskit_xplat is outside the session's change set, and the only lock move is pulldown-cmark
@answer n=11 entity=a11y-strategy#3.2 v=no
why: accesskit_xplat's lib.rs is outside the session's change set
@answer n=12 entity=a11y-strategy#3.3 v=no
why: the Android, Windows and macOS adapter files under packages/accesskit_xplat are outside the session's change set
@answer n=13 entity=a11y-strategy#3.4 v=no
why: android.rs is outside the session's change set, and the Cargo.lock move is pulldown-cmark alone
@answer n=14 entity=a11y-strategy#3.5 v=no
why: android.rs is outside the session's change set, and the Cargo.lock move is pulldown-cmark alone
@answer n=15 entity=a11y-strategy#4.1 v=no
why: the session changed no packages/ file and no Cargo.toml, so dioxus-native-dom's feature gates and the forwarding to blitz stand
