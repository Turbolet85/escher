@answer n=1 entity=threat-model-summary#1.1 v=no
why: it cites only apps/browser and the blitz-net, blitz-html and blitz-dom files, none of which the session reaches, and the session brings no remote-content input
@answer n=2 entity=threat-model-summary#2.1 v=no
why: it cites blitz-vibey-script, apps/browser, the WPT runner and examples/preact, none reached, and the session's bridge event record is not the script fetch or event path
@answer n=3 entity=threat-model-summary#3.1 v=yes mode=stale
why: it cites examples/seven_guis/src/session_host.rs:36-58, a reached file whose binary gains the coverage role, so the cited host and start lines may have moved though the socket grammar, bounds and carriers hold
@answer n=4 entity=threat-model-summary#4.1 v=no
why: the graph answer's carriers hold unchanged, since the coverage line carries no graph content and the session moves no graph reading
@answer n=5 entity=threat-model-summary#5.1 v=no
why: the actions answer and its carriers hold unchanged, since the coverage line prints no actions content
@answer n=6 entity=threat-model-summary#6.1 v=no
why: the MCP stdio server keeps its grammar, bound and carriers per the untouched list, its mcp.rs is not reached, and the coverage role adds no MCP carrier
@answer n=7 entity=threat-model-summary#7.1 v=no
why: the graph result on MCP is unchanged, since the coverage line is a command-line stdout line and not an MCP result
@answer n=8 entity=threat-model-summary#8.1 v=no
why: the actions result on MCP and the socket content limits are unchanged, since the session adds no MCP or socket carrier
@answer n=9 entity=threat-model-summary#9.1 v=no
why: the reached docs/capabilities.md still states the MCP carried set truthfully, since the coverage role adds no MCP carrier
