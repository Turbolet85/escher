@answer n=1 entity=authentication-authorization#1.1 v=no
why: the session adds no token, secret, password, cookie or authorization wording, per the untouched authentication and secret facts.
@answer n=2 entity=authentication-authorization#1.2 v=yes mode=stale
why: the coverage role adds session wording (escher-session coverage, a fresh session per check) to reached files this auth|login|session search may span, so its no-hit reading and file count can have moved.
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the session adds no login, credential, permission or authentication wording and no credential source, per the untouched authentication and secret facts.
@answer n=4 entity=authentication-authorization#1.4 v=yes mode=stale
why: the coverage role's new session wording lands in reached files this session-term search may span, so its no-hit reading can have moved.
@answer n=5 entity=authentication-authorization#1.5 v=yes mode=stale
why: the coverage role's new session wording lands in reached files this auth|session|login|permission search may span, so its no-hit reading can have moved.
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the session adds no token, secret, password, api key or authorization wording, per the untouched no-token and no-secret facts.
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the session adds no token, password, secret, api key, authorization or cookie wording, per the untouched no-token and no-secret facts.
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the session adds no auth, token, secret, password, cookie or authorization wording, per the untouched authentication and secret facts.
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the session adds no token, password, secret, authorization, cookie, x-api-key or auth wording, per the untouched authentication and secret facts.
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the session adds no token, password, secret, api key, authorization or cookie wording, per the untouched authentication and secret facts.
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the session adds no authorization, cookie, x-api-key, password, secret or token wording, per the untouched authentication and secret facts.
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the cookie store lives in blitz-net, which is not among the files the session reaches.
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml is not among the reached files and no CI workflow changed, per the untouched dependency and CI facts.
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is not among the reached files and no CI workflow changed, per the untouched dependency and CI facts.
@answer n=15 entity=authentication-authorization#2.4 v=no
why: publish-browser.yml is not among the reached files and no CI workflow changed, per the untouched dependency and CI facts.
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml and test_ci_workflows.py are not among the reached files and no CI workflow changed, per the untouched dependency and CI facts.
@answer n=17 entity=authentication-authorization#2.6 v=no
why: host.rs is not reached and the coverage role binds no socket, state directory or permission, per the untouched driver-socket fact.
@answer n=18 entity=authentication-authorization#2.7 v=no
why: mcp.rs is not reached and the coverage role adds no MCP or stdio surface, per the untouched MCP fact.
