@answer n=1 entity=threat-model-summary#1.1 v=no
why: remote web content (document loader, net fetches, parser, iframes) is dropped by the chunk's not-needed item 2, since the probe fetches nothing remote
@answer n=2 entity=threat-model-summary#2.1 v=no
why: script execution in blitz-vibey-script is dropped by the not-needed item 3, since the probe runs Dioxus apps, not document scripts
@answer n=3 entity=threat-model-summary#3.1 v=yes
why: the served task runs through the driver session, so the 0700 state directory, 0600 socket, path parameter and masked-value rule bind (needs items 3, 4 and 9)
@answer n=4 entity=threat-model-summary#4.1 v=no
why: the MCP stdio carrier is neither run nor touched by the probe, the MCP live trial is dropped by the not-needed item 7, and the probe adds no carrier
@answer n=5 entity=threat-model-summary#5.1 v=no
why: the committed capability statement in docs/capabilities.md is not edited or re-read by the chunk, and its pin reads no crossing the probe changes
