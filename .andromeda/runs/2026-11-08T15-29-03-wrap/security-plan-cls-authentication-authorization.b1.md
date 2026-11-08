@answer n=1 entity=authentication-authorization#1.1 v=no
why: the session adds no token, secret, cookie, authorization or password-handling term, its only password word being a measured fixture name
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the session adds Session::actions, a method on the existing Session type, and no login or auth term
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the session adds no login, credential, permission or authorization term to any file
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the session adds Session::actions, a method on the existing Session type, and no authentication, login or credential term
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the session adds only the Session::actions method, with no auth, login or permission term
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the session adds no password, secret, token, authorization or cookie term beyond a measured fixture name
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the session adds no token, password, secret, api key, authorization or cookie term beyond a measured fixture name
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the session adds no auth, token, secret, password, cookie or authorization term beyond a measured fixture name
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the session adds no auth term, its new code using only the Session::actions and actions vocabulary
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the session adds no token, password, secret, api key, authorization or cookie term beyond a measured fixture name
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the session adds no authorization, cookie, x-api-key, password, secret or token term beyond a measured fixture name
@answer n=12 entity=authentication-authorization#2.1 v=no
why: blitz-net and its cookie store are not among the files the session reached, and no dependency or feature changed
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml is not reached, and no CI workflow permission changed
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is not reached, and no CI step or permission changed
@answer n=15 entity=authentication-authorization#2.4 v=no
why: publish-browser.yml is not reached, and the publish job and its signing environment are unchanged
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml and test_ci_workflows.py are not reached, and no CI workflow or its permissions changed
@answer n=17 entity=authentication-authorization#2.6 v=no
why: host.rs is not among the reached files, and the socket's filesystem access and absent handshake are unchanged
@answer n=18 entity=authentication-authorization#2.7 v=yes mode=stale
why: mcp.rs is reached with the actions tool and answer, so the cited lines 44-112 may have moved though the stdio trust boundary holds
