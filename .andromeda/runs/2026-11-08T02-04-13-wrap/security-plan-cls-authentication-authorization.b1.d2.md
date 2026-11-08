@answer n=1 entity=authentication-authorization#1.1 v=no
why: the session adds no token, secret or password code, so the absence search for them still holds on the reached files
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the session adds no auth, login or session-credential code, so the absence search still holds on the reached files
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the session adds no login, credential, permission or authentication code, so the absence search still holds
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the session adds no authentication, authorization, login or credential code, so the absence search still holds
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the session adds no auth, session, login or permission code, so the absence search still holds
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the session adds no password, secret, token, cookie or authorization code, so the absence search still holds
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the session adds no token, password, secret, api key, authorization or cookie code, so the absence search still holds
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the session adds no auth, token, secret, password or cookie code, so the absence search still holds
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the session adds no token, password, secret, authorization or cookie code, so the absence search still holds
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the session adds no token, password, secret, api key, authorization or cookie code, so the absence search still holds
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the session adds no authorization, cookie, api key, password, secret or token code, so the absence search still holds
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the blitz-net cookie store is untouched, since blitz-net and the dependency manifests did not change
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml is untouched, since no CI workflow changed
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is untouched, since no CI workflow changed
@answer n=15 entity=authentication-authorization#2.4 v=no
why: publish-browser.yml is untouched, since no CI workflow changed
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml and test_ci_workflows.py are untouched, since no CI workflow changed and neither file is reached
@answer n=17 entity=authentication-authorization#2.6 v=yes mode=add-to
why: the driver session socket now carries the UI graph (ids, names, flags, booleans) while its filesystem-only access stands, so the row must carry the graph crossing
@answer n=18 entity=authentication-authorization#2.7 v=yes mode=add-to
why: mcp.rs is reached and the graph tool rides the same stdio server, so the row must carry the graph crossing ratified by P4 fork 1 beside the 2026-10-10 crossing
