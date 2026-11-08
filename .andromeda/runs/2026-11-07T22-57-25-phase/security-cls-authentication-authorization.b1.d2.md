@answer n=1 entity=authentication-authorization#1.1 v=no
why: the scan over s03 finds only a commented-out local named token in a stylo test, so no auth rule the chunk must meet.
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the auth, login and session scan over s04 finds nothing, so no session or login rule reaches the chunk's files.
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the credential and permission scan over s05 finds nothing the chunk's graph or verb must honour.
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the authentication, session and credential scan over the listed s06 files finds nothing the chunk reaches.
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the auth, session and permission scan over s07 finds nothing, and the chunk changes no CSS, SVG or font handling.
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the scan over s08 matches only html5ever TokenizerOpts and a commented DOMTokenList, and form submission is not_needed.
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the scan over s09 finds no password or secret, and the password value masking the chunk adds is a value rule under n=4, not an authentication rule.
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the scan over s10 matches only a hyperlink URL component and an author stylesheet comment, so no auth rule binds.
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the scan over the listed s11 files matches only authors, so no auth rule binds.
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the scan over the s12 files finds no token, password, secret or cookie rule the chunk must meet.
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the scan over the listed s13 files finds no credential rule the chunk must meet.
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store belongs to blitz-net's net provider, which the chunk does not touch and which the scope marks not_needed.
@answer n=13 entity=authentication-authorization#2.2 v=no
why: the wpt-post-results.yml token grants belong to a CI workflow the scope marks not_needed.
@answer n=14 entity=authentication-authorization#2.3 v=no
why: the wpt.yml token grants belong to the WPT workflow the scope marks not_needed.
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish-browser job and its Signed Builds environment are named not_needed by the scope.
@answer n=16 entity=authentication-authorization#2.5 v=yes
why: the scope names the cargo-deny audit leg under --locked, and this rule fixes every ci.yml job's GITHUB_TOKEN as read-only, so that leg must keep the grant.
@answer n=17 entity=authentication-authorization#2.6 v=yes
why: the new verb's answer travels on the driver session socket, and the scope's n=15 trust boundary names its owner-only, no-authentication access.
@answer n=18 entity=authentication-authorization#2.7 v=yes
why: the new verb's MCP tool is served on the stdio server, and the scope's n=15 trust boundary names its no-handshake, process-stdio access.
