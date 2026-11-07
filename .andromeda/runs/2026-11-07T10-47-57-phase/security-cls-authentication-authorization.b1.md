@answer n=1 entity=authentication-authorization#1.1 v=no
why: the search records no auth term in the s03 files beyond a commented-out local; the task marks authentication not needed and the chunk adds none
@answer n=2 entity=authentication-authorization#1.2 v=no
why: the search records no auth, login or session code in the s04 files; the task marks authentication not needed and the chunk adds none
@answer n=3 entity=authentication-authorization#1.3 v=no
why: the search records no credential or permission code in the s05 files; the task marks authentication not needed and the chunk adds none
@answer n=4 entity=authentication-authorization#1.4 v=no
why: the search records no auth, session or credential code in the listed s06 files; the task marks authentication not needed and the chunk adds none
@answer n=5 entity=authentication-authorization#1.5 v=no
why: the search records no auth, session, login or permission code in the s07 files; the task marks authentication not needed and the chunk adds none
@answer n=6 entity=authentication-authorization#1.6 v=no
why: the search records only html5ever tokenizer options and a commented DOMTokenList in the s08 files; the task marks authentication not needed and the chunk adds none
@answer n=7 entity=authentication-authorization#1.7 v=no
why: the search records no token, password, secret or cookie code in the s09 files; the task marks authentication not needed and the chunk adds none
@answer n=8 entity=authentication-authorization#1.8 v=no
why: the search records only a hyperlink URL component and an author-stylesheet comment in the s10 files; the task marks authentication not needed and the chunk adds none
@answer n=9 entity=authentication-authorization#1.9 v=no
why: the search records only the word authors in the listed s11 files; the task marks authentication not needed and the chunk adds none
@answer n=10 entity=authentication-authorization#1.10 v=no
why: the search records no token, password, secret or cookie code in the s12 files; the task marks authentication not needed and the chunk adds none
@answer n=11 entity=authentication-authorization#1.11 v=no
why: the search records no authorization, cookie or secret code in the listed s13 files; the task marks authentication not needed and the chunk adds none
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store lives in blitz-net, which the chunk does not reach because it fetches nothing (task not-needed item on remote fetching)
@answer n=13 entity=authentication-authorization#2.2 v=no
why: the wpt-post-results workflow's token grants are CI permissions the chunk neither edits nor reaches (task not-needed item on CI workflow security)
@answer n=14 entity=authentication-authorization#2.3 v=no
why: the wpt.yml token grants are CI permissions the chunk neither edits nor reaches (task not-needed item on CI workflow security)
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish job's repository guard and Signed Builds environment are CI permissions the chunk neither edits nor reaches (task not-needed item on CI workflow security)
@answer n=16 entity=authentication-authorization#2.5 v=no
why: the ci.yml read-only workflow token is an existing CI permission the chunk does not edit, and the chunk's cargo runs add no grant
@answer n=17 entity=authentication-authorization#2.6 v=no
why: the driver session socket's file-mode access check is in escher-driver, which the chunk neither touches nor reaches (task not-needed item on the driver socket)
@answer n=18 entity=authentication-authorization#2.7 v=no
why: the driver MCP stdio crossing is in escher-driver, which the chunk neither touches nor reaches (task not-needed item on the MCP stdio server)
