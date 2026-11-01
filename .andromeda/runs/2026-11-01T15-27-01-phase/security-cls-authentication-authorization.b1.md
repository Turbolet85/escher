# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · absence over the s03 files, the only hit a commented local in tests/stylo_usage.rs, which the chunk does not reach; the Not-needed line drops application auth records
authentication-authorization#1.2 · no · absence over the s04 files of auth, login and session words, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#1.3 · no · absence over the s05 files, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#1.4 · no · absence over the listed s06 files, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#1.5 · no · absence over the s07 files, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#1.6 · no · absence over the listed s08 files, only html5ever TokenizerOpts and a commented DOMTokenList matched; no auth rule the chunk must honour
authentication-authorization#1.7 · no · absence over the listed s09 files, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#1.8 · no · absence over the s10 files, matches only a hyperlink URL component and a comment; the Not-needed line drops application auth records
authentication-authorization#1.9 · no · absence over the listed s11 files, only `authors` matched; the Not-needed line drops application auth records
authentication-authorization#1.10 · no · absence over the s12 files, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#1.11 · no · absence over the listed s13 files, none in a reached file; the Not-needed line drops application auth records
authentication-authorization#2.1 · no · the reqwest cookie store lives in packages/blitz-net, which is not reached and not named by the scope; the chunk adds no cookie or session use
authentication-authorization#2.2 · no · wpt-post-results.yml's token permissions; the file is not reached and the WPT runner is out of scope
authentication-authorization#2.3 · no · wpt.yml's token permissions; the file is not reached and the WPT runner is out of scope
authentication-authorization#2.4 · no · the publish job's repository guard and Signed Builds environment live in publish-browser.yml, which is not reached; the Not-needed line drops the publish job
authentication-authorization#2.5 · yes · ci.yml's workflow-level contents: read with no job-level grant, asserted by test_ci_workflows.py; the free-disk edit inside matrix_test must keep that job's token read-only and the asserted shape
authentication-authorization#2.6 · no · the driver session socket's filesystem modes and refusals; escher-driver is not edited, the chunk adds no listener, and the Not-needed line drops these modes
authentication-authorization#2.7 · no · the driver MCP server's stdio access; escher-driver is not edited and the chunk adds no verb or exit, both Not-needed
