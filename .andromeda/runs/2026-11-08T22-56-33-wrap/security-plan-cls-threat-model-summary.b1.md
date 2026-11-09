@answer n=1 entity=threat-model-summary#1.1 v=no
why: the remote-content entry points cite blitz and apps files the session does not reach, and it adds no remote input
@answer n=2 entity=threat-model-summary#2.1 v=no
why: the script-execution entry points cite blitz-vibey-script, browser and WPT files the session does not reach
@answer n=3 entity=threat-model-summary#3.1 v=yes mode=stale
why: it cites session_host.rs lines 36 to 58, a reached file in which the same binary's command line gains a coverage role, so those line citations can move
@answer n=4 entity=threat-model-summary#4.1 v=no
why: the graph carriers are unchanged, and the coverage line is a separate answer that carries no UI graph
@answer n=5 entity=threat-model-summary#5.1 v=no
why: the actions carriers are unchanged, and the coverage line is not an actions answer
@answer n=6 entity=threat-model-summary#6.1 v=no
why: its cited mcp.rs is not reached, and the MCP entry point and carried set are unchanged, the coverage role being CLI only
@answer n=7 entity=threat-model-summary#7.1 v=no
why: the MCP graph result is unchanged, as the session moves no MCP surface
@answer n=8 entity=threat-model-summary#8.1 v=no
why: the MCP actions result is unchanged, as the session moves no MCP surface
@answer n=9 entity=threat-model-summary#9.1 v=no
why: docs/capabilities.md is reached, but the MCP onward carried set it states still holds, the coverage role adding no MCP carrier
