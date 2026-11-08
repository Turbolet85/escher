@answer n=1 entity=a11y-strategy#1.1 v=no
why: the shell's poll-time refresh of the platform tree; the chunk reads the snapshot on demand and drives no shell poll or adapter.
@answer n=2 entity=a11y-strategy#1.2 v=no
why: the changed-set and settle rules; the chunk reads no has_changes, drains nothing, and its replayed answer does not read the set.
@answer n=3 entity=a11y-strategy#1.3 v=no
why: unhandled AccessKit events in blitz-shell; the chunk touches no shell event.
@answer n=4 entity=a11y-strategy#1.4 v=no
why: window focus and bounds forwarded to the adapter; the chunk builds no adapter.
@answer n=5 entity=a11y-strategy#2.1 v=no
why: HTML-AAM role mapping in the tree builder, which the chunk neither builds nor pins a role value of.
@answer n=6 entity=a11y-strategy#2.2 v=no
why: the html and body Role::Unknown assertion is the tree builder's role mapping, which the chunk does not touch.
@answer n=7 entity=a11y-strategy#2.3 v=no
why: ARIA state derivation in the tree builder; the replay compares two sessions under the same builder and pins no state value.
@answer n=8 entity=a11y-strategy#2.4 v=no
why: native checkbox and radio toggled derivation in the tree builder; the replay pins no toggled value.
@answer n=9 entity=a11y-strategy#2.5 v=yes
why: states that the snapshot's enabled reads the disabled attribute and not aria-disabled, the input of the enabled value the driver's disabled refusal reads; the label and description reads are not the chunk's.
@answer n=10 entity=a11y-strategy#3.1 v=no
why: platform module selection in accesskit_xplat; the chunk builds no adapter.
@answer n=11 entity=a11y-strategy#3.2 v=no
why: an accesskit_xplat upstream warning about its approach; the chunk adds no platform code.
@answer n=12 entity=a11y-strategy#3.3 v=no
why: empty or no-op platform adapter methods; the chunk builds no adapter.
@answer n=13 entity=a11y-strategy#3.4 v=no
why: the Android adapter's Rect import and parameters; the chunk builds no adapter.
@answer n=14 entity=a11y-strategy#3.5 v=no
why: the Android adapter's surface-view field; the chunk builds no adapter.
@answer n=15 entity=a11y-strategy#4.1 v=yes
why: the accessibility feature gates DioxusDocument::graph and actionable_nodes, which the coverage read and replay build on, while signal_registry stays ungated; the chunk's code must honour that gating.
