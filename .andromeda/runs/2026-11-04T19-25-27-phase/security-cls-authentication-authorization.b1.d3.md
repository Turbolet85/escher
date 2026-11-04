@answer n=1 entity=authentication-authorization#1.1 v=no
why: absence scan over the s03 files states no rule; the only hits sit in tests/stylo_usage.rs, which the chunk does not reach
@answer n=2 entity=authentication-authorization#1.2 v=no
why: absence scan over the s04 files states no rule the probe's code must honour
@answer n=3 entity=authentication-authorization#1.3 v=no
why: absence scan over the s05 files states no rule the probe's code must honour
@answer n=4 entity=authentication-authorization#1.4 v=no
why: absence scan over the listed s06 files states no rule the probe's code must honour
@answer n=5 entity=authentication-authorization#1.5 v=no
why: absence scan over the s07 files states no rule the probe's code must honour
@answer n=6 entity=authentication-authorization#1.6 v=no
why: absence scan over the listed s08 files states no rule; the matches are an html5ever tokenizer option and a comment
@answer n=7 entity=authentication-authorization#1.7 v=no
why: absence scan over the listed s09 files states no rule the probe's code must honour
@answer n=8 entity=authentication-authorization#1.8 v=no
why: absence scan over the s10 files states no rule; the matches are a URL component and a stylesheet comment
@answer n=9 entity=authentication-authorization#1.9 v=no
why: absence scan over the listed s11 files states no rule; the only match is the word authors
@answer n=10 entity=authentication-authorization#1.10 v=no
why: absence scan over the s12 files states no rule the probe's code must honour
@answer n=11 entity=authentication-authorization#1.11 v=no
why: absence scan over the listed s13 files states no rule the probe's code must honour
@answer n=12 entity=authentication-authorization#2.1 v=no
why: the reqwest cookie store is the net provider's, and the probe fetches nothing remote (not_needed 2)
@answer n=13 entity=authentication-authorization#2.2 v=no
why: wpt-post-results CI token grants are CI token scoping, which this chunk does not change (not_needed 12) and the probe adds no workflow
@answer n=14 entity=authentication-authorization#2.3 v=no
why: wpt.yml CI token grants are CI token scoping, which this chunk does not change (not_needed 12) and the probe adds no workflow
@answer n=15 entity=authentication-authorization#2.4 v=no
why: the publish-browser repository guard and Signed Builds environment are out of scope (not_needed 4)
@answer n=16 entity=authentication-authorization#2.5 v=no
why: ci.yml read-only workflow token is CI token scoping, which this chunk does not change (not_needed 12) and the probe adds no CI job or grant
@answer n=17 entity=authentication-authorization#2.6 v=yes
why: scope item 9 names the driver-session path for the served task, and the state directory 0700 and socket 0600 modes this entity states bind that path, a fork the scope leaves open
@answer n=18 entity=authentication-authorization#2.7 v=no
why: the stdio MCP server is not named by the scope, the chunk adds no verb, and the MCP live trial is out of scope (not_needed 7)
