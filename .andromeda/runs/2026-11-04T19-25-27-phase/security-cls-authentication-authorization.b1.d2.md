@answer n=1 entity=authentication-authorization#1.1 v=no
why: absence search for auth terms in the s03 files; it states no rule the probe must meet, and the probe adds no credential or permission check
@answer n=2 entity=authentication-authorization#1.2 v=no
why: absence search for auth, login and session terms in the s04 files; it states no rule the probe must meet, and the driver session it uses is covered by #2.6
@answer n=3 entity=authentication-authorization#1.3 v=no
why: absence search for login, credential and permission terms in the s05 files; it states no rule the chunk must honour
@answer n=4 entity=authentication-authorization#1.4 v=no
why: absence search for authentication and session terms in the listed s06 files; it states no rule the chunk must honour
@answer n=5 entity=authentication-authorization#1.5 v=no
why: absence search for auth, session, login and permission terms in the s07 files; it states no rule the chunk must honour
@answer n=6 entity=authentication-authorization#1.6 v=no
why: absence search for password, token and authorization terms in the listed s08 files; it states no rule the chunk must honour
@answer n=7 entity=authentication-authorization#1.7 v=no
why: absence search for token, password and cookie terms in the s09 files; it states no rule the chunk must honour
@answer n=8 entity=authentication-authorization#1.8 v=no
why: absence search for auth and token terms in the s10 files; it states no rule the chunk must honour
@answer n=9 entity=authentication-authorization#1.9 v=no
why: absence search for token, password and cookie terms in the listed s11 files; it states no rule the chunk must honour
@answer n=10 entity=authentication-authorization#1.10 v=no
why: absence search for token, password and API-key terms in the s12 files; it states no rule the chunk must honour
@answer n=11 entity=authentication-authorization#1.11 v=no
why: absence search for authorization, cookie and token terms in the listed s13 files; it states no rule the chunk must honour
@answer n=12 entity=authentication-authorization#2.1 v=no
why: blitz-net's reqwest cookie store exists only under the cookies feature, and the probe fetches nothing remote (not_needed n=2) and names no cookie or feature read
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results.yml token grants belong to the WPT post-results workflow, which the scope names as not needed (not_needed n=4)
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml token grants are CI token scoping, which the scope names as not needed (not_needed n=12) and the chunk does not change
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish job and the Signed Builds environment are named as not needed (not_needed n=4)
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml's read-only workflow token is CI token scoping, which the scope names as not needed (not_needed n=12); the chunk adds no CI job or grant
@answer n=17 entity=authentication-authorization#2.6 v=yes
why: the driver session's state directory at 0700 with refusal when open, and its socket at 0600, are the rules the scope names for the served task through the driver session (task_item n=9), with the path as a parameter
@answer n=18 entity=authentication-authorization#2.7 v=no
why: the MCP stdio server is the MCP live trial, named as not needed (not_needed n=7); the probe adds no verb or carrier and runs no MCP session
