@answer n=1 entity=authentication-authorization#1.1 v=no
why: absence search over the s03 style files, which the chunk does not reach; it states a fact of the tree, not a rule the new code must meet
@answer n=2 entity=authentication-authorization#1.2 v=no
why: absence search for auth terms over the s04 files, which the chunk does not reach; no rule the new code must meet
@answer n=3 entity=authentication-authorization#1.3 v=no
why: absence search over the s05 files, which the chunk does not reach; no rule the new code must meet
@answer n=4 entity=authentication-authorization#1.4 v=no
why: absence search over the listed s06 files, which the chunk does not reach; no rule the new code must meet
@answer n=5 entity=authentication-authorization#1.5 v=no
why: absence search over the s07 files, which the chunk does not reach; no rule the new code must meet
@answer n=6 entity=authentication-authorization#1.6 v=no
why: absence search over the listed s08 files, which the chunk does not reach; the html5ever tokenizer match is not an auth rule
@answer n=7 entity=authentication-authorization#1.7 v=no
why: absence search over the s09 files, which the chunk does not reach; the password term is the masking control-value rule (needs n=8), not an auth rule
@answer n=8 entity=authentication-authorization#1.8 v=no
why: absence search over the s10 files, which the chunk does not reach; the URL component and author-stylesheet matches are not auth rules
@answer n=9 entity=authentication-authorization#1.9 v=no
why: absence search over the listed s11 files, which the chunk does not reach; only the authors match, no rule the new code must meet
@answer n=10 entity=authentication-authorization#1.10 v=no
why: absence search over the s12 files, which the chunk does not reach; the password term is the masking control-value rule (needs n=8), not an auth rule
@answer n=11 entity=authentication-authorization#1.11 v=no
why: absence search over the listed s13 files, which the chunk does not reach; the password term is the masking control-value rule (needs n=8), not an auth rule
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store is in blitz-net, which the chunk does not reach, and the chunk makes no network request (not_needed n=5)
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml is a WPT workflow the chunk does not touch (not_needed n=6)
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is a WPT workflow the chunk does not touch (not_needed n=6)
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish job and the Signed Builds environment sit in publish-browser.yml, which the chunk does not touch (not_needed n=6)
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml's read-only workflow permission is not reached, and the chunk adds no CI job; any new CI cargo command is covered by the --locked rule of standing n=4
@answer n=17 entity=authentication-authorization#2.6 v=yes
why: the new coverage verb is carried over the driver session socket, so the 0600 socket and the 0700 state-dir refusal are the access the verb inherits (scope names the session socket carrier, needs n=2)
@answer n=18 entity=authentication-authorization#2.7 v=yes
why: the new MCP coverage tool is served in mcp.rs, and its stdio trust statement (no peer check, answers only the spawning client) governs who reaches the tool (scope names the MCP carrier, needs n=14)
