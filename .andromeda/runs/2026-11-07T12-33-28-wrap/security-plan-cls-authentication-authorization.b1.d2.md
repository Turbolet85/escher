@answer n=1 entity=authentication-authorization#1.1 v=no
why: the session adds no auth, token, secret or password code to the reached files (untouched n=2), and the commented token hits sit in tests/stylo_usage.rs, which the change does not reach
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the session adds no auth, login or session code to the reached files (untouched n=2), so the absence search still holds on the changed files
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the session adds no login, credential, permission or authentication code to the reached files (untouched n=2), so the absence search still holds
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the session adds no authentication, authorization, login, session or credential code to the reached files (untouched n=2), so the absence search still holds
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the session adds no auth, session, login or permission code to the reached files (untouched n=2), so the absence search still holds
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the session adds no token, password, secret or authorization code to the reached files (untouched n=2); the html5ever TokenizerOpts hit is untouched by the change
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the session adds no token, password, secret, api_key, authorization or cookie code to the reached files (untouched n=2), so the absence search still holds
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the session adds no auth, token, secret, password, cookie or authorization code to the reached files (untouched n=2); the matches are a hyperlink URL and a comment
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the session adds no token, password, secret, authorization, cookie, x-api-key or auth code to the reached files (untouched n=2), so the absence search still holds
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the session adds no token, password, secret, api key, authorization or cookie code to the reached files (untouched n=2), so the absence search still holds
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the session adds no authorization, cookie, x-api-key, password, secret or token code to the reached files (untouched n=2), so the absence search still holds
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store lives in packages/blitz-net, which the change does not reach, and no dependency or feature moved (untouched n=6, n=3)
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml is not reached and no CI workflow, permission or CI step moved (untouched n=7)
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is not reached and no CI workflow, permission or CI step moved (untouched n=7)
@answer n=15 entity=authentication-authorization#2.4 v=no
why: publish-browser.yml is not reached and no CI workflow, environment or CI step moved (untouched n=7)
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml and test_ci_workflows.py are not reached and no CI step or permission moved (untouched n=7)
@answer n=17 entity=authentication-authorization#2.6 v=no
why: escher-driver host.rs is not reached, and the driver's session socket and its widening rule stay untouched (untouched n=4)
@answer n=18 entity=authentication-authorization#2.7 v=no
why: escher-driver mcp.rs is not reached, and the driver and its MCP server stay untouched (untouched n=4)
