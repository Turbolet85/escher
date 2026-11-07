@answer n=1 entity=threat-model-summary#1.1 v=no
why: remote content fetching, the parsers, iframe and import limits and the select-list paint are not reached, since the buffered render fetches nothing and changes no parser or snapshot guard
@answer n=2 entity=threat-model-summary#2.1 v=no
why: script execution in blitz-vibey-script is not reached, since the Dioxus bridge runs no document scripts
@answer n=3 entity=threat-model-summary#3.1 v=yes
why: the driver's answers carry the stable element id and the masked password and file-input values with no NodeId crossing, and the scope names these values as staying identical
@answer n=4 entity=threat-model-summary#4.1 v=yes
why: MCP results carry ids, accessible names, masked control values, snapshot text and diffs, which the scope names as staying identical through the replayed render
@answer n=5 entity=threat-model-summary#5.1 v=yes
why: the stated carried set is the same ids, names, masked values, snapshot text and diffs that the scope requires to stay identical, and the chunk must add no carrier
