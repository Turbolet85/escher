@answer n=1 entity=authentication-authorization#1.1 v=no
why: absence scan of password, token, secret, cookie and authorization words over the s03 set; the chunk adds no credential or auth code, so the absence holds, a re-check every change owes
@answer n=2 entity=authentication-authorization#1.2 v=no
why: absence scan of auth, login and session words over the s04 set; the chunk adds no login or auth code to it, a re-check every change owes
@answer n=3 entity=authentication-authorization#1.3 v=no
why: absence scan of login, credential, permission and authenticate words over the s05 set; the chunk adds no such code, a re-check every change owes
@answer n=4 entity=authentication-authorization#1.4 v=no
why: absence scan of auth, session, login and credential words over the listed s06 files; the chunk adds no such code to them, a re-check every change owes
@answer n=5 entity=authentication-authorization#1.5 v=no
why: absence scan of auth, session, login and permission words over the s07 set; the chunk adds none, a re-check every change owes
@answer n=6 entity=authentication-authorization#1.6 v=no
why: absence scan over the listed s08 files, where only an html5ever tokenizer option and a commented DOMTokenList matched; the chunk adds no such code, a re-check every change owes
@answer n=7 entity=authentication-authorization#1.7 v=no
why: absence scan of token, password, secret, api key and cookie words over the s09 set; the chunk adds no such code, a re-check every change owes
@answer n=8 entity=authentication-authorization#1.8 v=no
why: absence scan of auth, token, secret, password and cookie words over the s10 set; the matches are a URL component and a comment, and the chunk adds no such code, a re-check every change owes
@answer n=9 entity=authentication-authorization#1.9 v=no
why: absence scan over the listed s11 files, where only the word authors matched; the chunk adds no auth or secret code, a re-check every change owes
@answer n=10 entity=authentication-authorization#1.10 v=no
why: absence scan of token, password, secret, api key and cookie words over the s12 set; the chunk adds no such code, a re-check every change owes
@answer n=11 entity=authentication-authorization#1.11 v=no
why: absence scan over the listed s13 files; the chunk adds no authorization, cookie or secret code to them, a re-check every change owes
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store sits in blitz-net, which the chunk neither reaches nor uses, and the chunk makes no network request, so the rule does not bind
@answer n=13 entity=authentication-authorization#2.2 v=no
why: the wpt-post-results.yml token grants are on a WPT workflow the chunk does not touch, so the rule does not bind
@answer n=14 entity=authentication-authorization#2.3 v=no
why: the wpt.yml token grants are on a WPT workflow the chunk does not touch, so the rule does not bind
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish job and the Signed Builds environment are on publish-browser.yml, which the chunk does not touch, so the rule does not bind
@answer n=16 entity=authentication-authorization#2.5 v=no
why: the read-only workflow-level grant in ci.yml and its test_ci_workflows.py assertion; the chunk adds no CI job or grant and reaches neither ci.yml nor that test, so the rule does not bind
@answer n=17 entity=authentication-authorization#2.6 v=yes
why: the session socket's filesystem access model (0700 state directory, 0600 socket, no peer check) is the carrier of the new verb the scope names as a fork, so the rule binds over the coverage call
@answer n=18 entity=authentication-authorization#2.7 v=yes
why: the stdio MCP server's access model (no peer check, only the spawning client) is the carrier of the coverage tool, whose answer the scope sends over MCP in mcp.rs and mcp/listing.rs, so the rule binds
