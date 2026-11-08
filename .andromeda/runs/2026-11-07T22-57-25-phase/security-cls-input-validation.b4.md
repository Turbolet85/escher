@answer n=1 entity=input-validation#1.76 v=no
why: the checkbox and radio change emission is engine behaviour the chunk neither edits nor reads; the graph reads the vdom, not engine event dispatch
@answer n=2 entity=input-validation#1.77 v=yes
why: the select's snapshot value is stated unmasked and the select's options and choice are carried state the graph reads, so the mask rule over graph values rules over it
@answer n=3 entity=input-validation#1.78 v=no
why: the open select list field is engine state the chunk does not read or edit
@answer n=4 entity=input-validation#1.79 v=no
why: an open select list's key handling is engine input behaviour the chunk does not touch
@answer n=5 entity=input-validation#1.80 v=no
why: pointer handling in and around an open select list is engine input behaviour the chunk does not touch
@answer n=6 entity=input-validation#1.81 v=no
why: hostile markup box finiteness and panic-freedom under the list is an upstream blitz-dom layout rule, outside the chunk's named panic paths
@answer n=7 entity=input-validation#1.82 v=no
why: a multiple select's keys and clicks are engine input behaviour the chunk does not touch; its carried set is covered by 1.84
@answer n=8 entity=input-validation#1.83 v=no
why: optgroup role, label and the owner_select walk are accessibility and form rules the chunk neither reads nor edits
@answer n=9 entity=input-validation#1.84 v=yes
why: a multiple select's chosen set is stated as the snapshot's value and as state the graph carries, so the masking and value-carrier rules over graph values rule over it
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type verb on a select is an existing verb the chunk does not change, and its help text is fixed text it leaves alone
@answer n=11 entity=input-validation#1.86 v=yes
why: the snapshot builder the scope names reads closed_select_of, which decides the select value the snapshot carries, and the stale id reads none through get_node
@answer n=12 entity=input-validation#1.87 v=no
why: the select list's public readers and the row label are engine surface the chunk neither calls nor reads
@answer n=13 entity=input-validation#1.88 v=yes
why: the password and file input value mask on snapshot, Snapshot::to_text, Snapshot::diff and every carrier, with kept checks holding the mask, is named by the scope's masking and carrier items
@answer n=14 entity=input-validation#1.89 v=no
why: aspect-ratio layout values are not read or carried by the chunk
@answer n=15 entity=input-validation#1.90 v=no
why: inline-box height layout clamps are not read or carried by the chunk
@answer n=16 entity=input-validation#1.91 v=no
why: the text_extent layout reader is not called or edited by the chunk
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region is an engine geometry reader the chunk does not call or edit
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling before paint is a paint rule the chunk does not touch
@answer n=19 entity=input-validation#1.94 v=no
why: select list box and row paint is a paint rule the chunk does not touch
@answer n=20 entity=input-validation#1.95 v=no
why: non-invertible transform hit testing is an engine hit-test rule the chunk does not touch
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamping is a paint rule the chunk does not touch
@answer n=22 entity=input-validation#1.97 v=no
why: border radius scaling is a paint rule the chunk does not touch
@answer n=23 entity=input-validation#1.98 v=no
why: type=hidden paint suppression is a paint rule that states no value the graph carries
@answer n=24 entity=input-validation#1.99 v=no
why: scroll target clamping is an engine scrolling rule the chunk does not touch
@answer n=25 entity=input-validation#1.100 v=no
why: an open select list's own scroll is an engine scrolling rule the chunk does not touch
