@answer n=1 entity=authentication-authorization#1.1 v=no
why: the new test file adds no auth code, token, handshake or peer check (untouched n=2), so this absence search still holds on the changed set
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the new test file adds no auth, login or session code (untouched n=2), so this absence search still holds on the changed set
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the new test file adds no credential or permission code (untouched n=2), so this absence search still holds on the changed set
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the new test file adds no auth, session or credential code (untouched n=2), so this absence search still holds on the changed set
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the new test file adds no auth, session or permission code (untouched n=2), so this absence search still holds on the changed set
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the new test file adds no token, password, secret or authorization code (untouched n=2), so this absence search still holds on the changed set
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the new test file adds no token, secret or cookie code (untouched n=2), so this absence search still holds on the changed set
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the new test file adds no auth, token or cookie code (untouched n=2), so this absence search still holds on the changed set
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the new test file adds no token, cookie or authorization code (untouched n=2), so this absence search still holds on the changed set
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the new test file adds no token, secret or cookie code (untouched n=2), so this absence search still holds on the changed set
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the new test file adds no authorization, cookie or token code (untouched n=2), so this absence search still holds on the changed set
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the lock change moves only pulldown-cmark, not reqwest, and the cookie store is set in packages/blitz-net, which the change does not reach
@answer n=13 entity=authentication-authorization#2.2 v=no
why: the CI workflow files are outside the reached set (Cargo.lock, deny.toml, the new test file), and no workflow permission moved (untouched n=11)
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml is outside the reached set (Cargo.lock, deny.toml, the new test file), and no workflow permission moved (untouched n=11)
@answer n=15 entity=authentication-authorization#2.4 v=no
why: publish-browser.yml is outside the reached set (Cargo.lock, deny.toml, the new test file), and no CI tool or environment moved (untouched n=11)
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml and test_ci_workflows.py are outside the reached set, and the ci.yml permission grant does not move with the lock or deny change
@answer n=17 entity=authentication-authorization#2.6 v=no
why: the driver session socket code in packages/escher-driver/src/host.rs is not reached, and the session touches no driver code (untouched n=3)
@answer n=18 entity=authentication-authorization#2.7 v=no
why: the driver MCP server in packages/escher-driver/src/mcp.rs is not reached, and the session touches no driver code (untouched n=3)
