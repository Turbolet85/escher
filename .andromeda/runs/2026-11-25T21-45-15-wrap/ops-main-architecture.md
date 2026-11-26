@row_set n=1 row=decision/canvas-reading-diffable cell=rule seen=1 detector=none
value: A canvas reading is deterministic and diffable by line: two reads give the same bytes, a changed command reads as one line removed and one added, both closing on one node tag where a virtual node groups it, and a layer's filter is written as a flag only.
basis: fct-lscnn6ng
because: Each canvas line now closes on the index of the virtual node grouping it, so a changed command's removed and added lines name their node by one shared tag, which the diffable rule must state.
