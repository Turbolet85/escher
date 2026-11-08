@answer n=1 entity=authentication-authorization#1.1 v=no
why: a survey of auth terms that found only a commented-out token name in tests/stylo_usage.rs; it states no rule the read's units must meet
@answer n=2 entity=authentication-authorization#1.2 v=no
why: a survey of auth and session terms over the s04 files; it states no rule over the read's units
@answer n=3 entity=authentication-authorization#1.3 v=no
why: a survey of credential and permission terms over the s05 files; it states no rule over the read's units
@answer n=4 entity=authentication-authorization#1.4 v=no
why: a survey of auth and session terms over the listed s06 files; it states no rule the chunk must honour
@answer n=5 entity=authentication-authorization#1.5 v=no
why: a survey of auth and permission terms over the s07 files; it states no rule over the read's units
@answer n=6 entity=authentication-authorization#1.6 v=no
why: a survey whose only hits are html5ever TokenizerOpts and a commented DOMTokenList; it states no rule for the read
@answer n=7 entity=authentication-authorization#1.7 v=no
why: a survey of token and secret terms over the s09 files; it states no rule over the read's units
@answer n=8 entity=authentication-authorization#1.8 v=no
why: a survey whose only hits are a URL component and an author stylesheets comment; it states no rule the read must meet
@answer n=9 entity=authentication-authorization#1.9 v=no
why: a survey whose only match is the word authors in the listed s11 files; it states no rule for the read
@answer n=10 entity=authentication-authorization#1.10 v=no
why: a survey of token and cookie terms over the s12 files; it states no rule over the read's units
@answer n=11 entity=authentication-authorization#1.11 v=no
why: a survey of authorization and cookie terms over the listed s13 files; it states no rule the read must meet
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store in blitz-net; the read loads nothing and fetches no content, so the net provider's cookies are not reached
@answer n=13 entity=authentication-authorization#2.2 v=no
why: the wpt-post-results.yml token grants; the WPT post-results workflow is not-needed scope and the chunk does not edit it
@answer n=14 entity=authentication-authorization#2.3 v=no
why: the wpt.yml token grants; the WPT workflow is outside the chunk's files and scope
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish-browser publish job and its Signed Builds environment; the upstream-only publish workflow is not-needed scope
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml's read-only GITHUB_TOKEN; the chunk edits no CI job or ci.yml and the scope names none
@answer n=17 entity=authentication-authorization#2.6 v=yes
why: the session socket's filesystem-only access and its 0700 state dir and 0600 socket gate the read's new verb on that socket, which the scope names as the session socket and its carriers
@answer n=18 entity=authentication-authorization#2.7 v=yes
why: the MCP stdio model, with no handshake and no bind, is the carrier of the read's new MCP tool, which the scope names under the MCP listing and results; the chunk must add no listener
