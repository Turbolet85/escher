@answer n=1 entity=threat-model-summary#1.1 v=no
why: its iframe, @import, stale-response and never-styled-snapshot rules sit in blitz-dom code the chunk neither edits nor calls, and the bridge fetches nothing
@answer n=2 entity=threat-model-summary#2.1 v=no
why: script execution in blitz-vibey-script, which the Dioxus bridge does not run, so the chunk neither reaches nor needs it
@answer n=3 entity=threat-model-summary#3.1 v=yes
why: the driver's answers carry the masked password and file-input values and the snapshot text and diff, which the scope keeps identical through the buffered replay
@answer n=4 entity=threat-model-summary#4.1 v=yes
why: MCP results carry those same masked values, snapshot text, diffs and masked screenshot image, which the scope names as staying identical
@answer n=5 entity=threat-model-summary#5.1 v=yes
why: the committed carried-set statement names the masked values and snapshot text and diffs the scope keeps identical, and it adds no carrier, as the scope requires
