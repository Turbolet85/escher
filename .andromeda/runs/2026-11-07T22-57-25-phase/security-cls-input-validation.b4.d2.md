@answer n=1 entity=input-validation#1.76 v=no
why: the engine's checkbox and radio change dispatch is blitz-dom event behaviour the graph does not read, its handlers come from the vdom and no dispatch changes
@answer n=2 entity=input-validation#1.77 v=yes
why: a select's snapshot value (the chosen option's value, no mask applying) and its option tree nodes are carried by DioxusDocument::snapshot, the scope-named carrier the chunk extends, so the value masking decision binds
@answer n=3 entity=input-validation#1.78 v=no
why: the select list field and its reconcile and hover path are engine state that neither the graph read nor the new verb touches
@answer n=4 entity=input-validation#1.79 v=no
why: an open list's keys and Tab are keyboard input the chunk's read verb never drives
@answer n=5 entity=input-validation#1.80 v=no
why: a pointer press, drag and release in an open select list is input handling the chunk neither changes nor drives
@answer n=6 entity=input-validation#1.81 v=no
why: hostile size, option count and label geometry of an open list is layout the chunk neither lays out nor reads
@answer n=7 entity=input-validation#1.82 v=no
why: multiple-select keys and clicks are input handling the chunk neither changes nor drives
@answer n=8 entity=input-validation#1.83 v=yes
why: an optgroup takes Role::Group with its label as name in the accessibility tree, which decides the agent-visible nodes the graph keys on and the names the snapshot carries
@answer n=9 entity=input-validation#1.84 v=yes
why: a multiple select's snapshot value (its first chosen option in tree order) and its option tree nodes are carried by DioxusDocument::snapshot, which the chunk extends and the masking decision covers
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type on a select is an existing verb path the chunk does not change, and its help text is not one the chunk edits
@answer n=11 entity=input-validation#1.86 v=yes
why: the snapshot builder calls closed_select_of, which answers None for an open list, and a stale id must read none without a panic, the rule the graph reader owes under item 12
@answer n=12 entity=input-validation#1.87 v=no standing=S2
why: the select list read types and readers are not read by the chunk, and the label never logged clause is the log content convention each new log line keeps, fixed words, bools and counts only
@answer n=13 entity=input-validation#1.88 v=yes
why: a password or file input's value reads as MASKED_VALUE on the snapshot, its text, its diffs and every driver and MCP answer, and the scope requires the mask on every value the graph carries
@answer n=14 entity=input-validation#1.89 v=no
why: aspect-ratio clamps in layout are not read or changed by the chunk, which changes no engine layout
@answer n=15 entity=input-validation#1.90 v=no
why: inline-box height clamps in line layout are engine layout the chunk does not read
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent's callers are the lint reader and the lint verb, neither named nor changed by the chunk
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region's callers are in_view, crop and lint, none named or changed by the chunk
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling before paint is a paint rule the chunk does not touch
@answer n=19 entity=input-validation#1.94 v=no
why: the select list's painted box and rows are paint the chunk does not change
@answer n=20 entity=input-validation#1.95 v=no
why: hit testing under a non-invertible transform is an engine hit rule that the vdom-reading graph never runs
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamps are paint values the chunk does not change
@answer n=22 entity=input-validation#1.97 v=no
why: border-radius scaling is a paint value the chunk does not change
@answer n=23 entity=input-validation#1.98 v=no
why: hidden inputs are unpainted, a paint rule the chunk does not change
@answer n=24 entity=input-validation#1.99 v=no
why: scroll-target clamping and the no-op on a missing node are scroll behaviour the chunk does not drive
@answer n=25 entity=input-validation#1.100 v=no
why: an open select list's own scroll is scroll and layout behaviour the chunk does not drive
