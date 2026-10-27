# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · yes · records no live auth, password, secret, token, cookie or authorization code in the 32 s03 files (only a commented-out token variable in a test sketch), the witness for the no-other-authentication claim the README re-asserts for the socket and stdio (task Needs)
authentication-authorization#1.2 · yes · records no auth, login or session code in the 86 s04 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.3 · yes · records no login, credential, permission, authentication or authorization code in the 15 s05 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.4 · yes · records no authentication, authorization, login, session or credential code in the 17 listed s06 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.5 · yes · records no auth, session, login or permission code in the 8 s07 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.6 · yes · records no password, secret, token, authorization or x-api-key code in the 16 listed s08 files (only an html5ever tokenizer option and a commented DOMTokenList matched), the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.7 · yes · records no token, password, secret, api_key, authorization or cookie code in the 32 listed s09 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.8 · yes · records no auth, token, secret, password, cookie or authorization code in the 32 s10 files (only a URL component and an author-stylesheets comment matched), the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.9 · yes · records no token, password, secret, authorization, cookie, x-api-key or auth code in the 21 listed s11 files (only the word authors matched), the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.10 · yes · records no token, password, secret, api key, authorization or cookie code in the 61 s12 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#1.11 · yes · records no authorization, cookie, x-api-key, password, secret or token code in the 12 listed s13 files, the same witness for the re-asserted no-other-authentication claim
authentication-authorization#2.1 · no · the reqwest cookie store behind feature cookies is blitz-net's outbound request behaviour, which the Not-needed list leaves untouched and no Needs line names
authentication-authorization#2.2 · no · the wpt-post-results job grants are CI workflow permissions, the Not-needed list drops the CI post-results guard, and the chunk edits no workflow
authentication-authorization#2.3 · no · the wpt.yml job grants are CI workflow permissions that the chunk neither edits nor re-reads (no workflow change)
authentication-authorization#2.4 · no · the publish job's repository guard and Signed Builds environment are CI access the Not-needed list drops (publish-browser signing material and environment), with no workflow change
authentication-authorization#2.5 · no · ci.yml's workflow-level read-only token, asserted by test_ci_workflows.py, is CI configuration the chunk neither edits nor re-reads
authentication-authorization#2.6 · yes · the driver socket's filesystem-only access (state directory 0700, socket 0600, no handshake, token or peer check, and no check that the answering pid is its spawned child) is what the Needs require the README to describe, with no stronger claim
authentication-authorization#2.7 · yes · the MCP stdio boundary (a child of the spawning client, no handshake, token, secret or peer check, no listener, port or socket) is what the README's driver paragraph must describe
