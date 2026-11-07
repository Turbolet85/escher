@answer n=1 entity=authentication-authorization#1.1 v=no
why: the session adds no token, secret or password code (untouched item 2), so this absence search still holds on the changed files
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the session adds no auth, login or session code (untouched item 2), so this absence search still holds on the changed files
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the session adds no credential or permission code (untouched item 2), so this absence search still holds on the changed files
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the session adds no auth, session or credential code (untouched item 2), so this absence search still holds on the changed files
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the session adds no auth, session, login or permission code (untouched item 2), so this absence search still holds on the changed files
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the session adds no password, secret, token or authorization code (untouched item 2), so this absence search still holds on the changed files
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the session adds no token, password, secret or cookie code (untouched item 2), so this absence search still holds on the changed files
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the session adds no auth, token, secret or cookie code (untouched item 2), so this absence search still holds on the changed files
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the session adds no token, password, secret or cookie code (untouched item 2), so this absence search still holds on the changed files
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the session adds no token, password, secret or cookie code (untouched item 2), so this absence search still holds on the changed files
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the session adds no cookie, token, password or secret code (untouched item 2), so this absence search still holds on the changed files
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the cookie store sits in packages/blitz-net, which the session does not reach
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml is not among the session's files and no CI step or leg moved (untouched item 7), so its grants stand
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is not among the session's files and no CI step or leg moved (untouched item 7), so its grants stand
@answer n=15 entity=authentication-authorization#2.4 v=no
why: publish-browser.yml is not among the session's files and no CI step or leg moved (untouched item 7), so the publish guard stands
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml's read-only workflow token is not among the session's files and no CI step or leg moved (untouched item 7)
@answer n=17 entity=authentication-authorization#2.6 v=no
why: escher-driver's host.rs is not among the session's files and the driver is not reached (untouched item 4)
@answer n=18 entity=authentication-authorization#2.7 v=no
why: escher-driver's mcp.rs is not among the session's files and the driver is not reached (untouched item 4)
