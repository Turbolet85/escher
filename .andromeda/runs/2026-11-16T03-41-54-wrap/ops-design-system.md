@row_add n=1 row=decision/layout-read-in-process-only area=surface-cli fact=F1 detector=none
name: Layout read in process only
rule: The layout read, which says why an element sits where it does at its size, is reached in process alone: no escher-session command, MCP tool, socket call or kept check carries it.
state: provisional
basis: fct-x3q3dkxj, fct-5qpmkurf, fct-u7f3beyp
because: The chunk built the layout read in process and kept every carrier of the CLI and MCP surface from it as a provisional decision, which no surface-cli row yet states.
@row_add n=2 row=contract/layout-read/answer area=surface fact=F2 detector=none
boundary: The layout read
item: answer
promise: The layout read answers an element by its stable id with a placing word, its shifts and a size cause per axis, all closed words and numbers, never a NodeId; an unread id is refused not-found and a deleted element's id stale.
basis: fct-x3q3dkxj, fct-5qpmkurf
because: A new reader of the driver promises its answer's shape and refusals, and no surface row states it.
@row_add n=3 row=contract/layout-read/closed-select-option area=surface fact=F2 detector=none
boundary: The layout read
item: an option of a closed select
promise: An option of a closed select reads the placing word Select at its select's box.
basis: fct-x3q3dkxj, fct-gy4gjjmp
because: The read gives a closed select's hidden options their select's box as a stated exception, and no surface row says so.
