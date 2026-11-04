@answer n=1 entity=threat-model-summary#1.1 v=no
why: the session adds a test target and moves only pulldown-cmark in Cargo.lock, so the remote-content paths in packages/blitz-net and blitz-html and the reqwest claim still hold
@answer n=2 entity=threat-model-summary#2.1 v=no
why: the script-execution paths in packages/blitz-vibey-script and the non-default javascript feature are unchanged, and the lock change moves no JS runtime dependency
@answer n=3 entity=threat-model-summary#3.1 v=no
why: the session socket, serve and host code and the calls and answers wire are unchanged, since no driver code moved
@answer n=4 entity=threat-model-summary#4.1 v=no
why: the MCP stdio server in escher-driver and its result carriers are unchanged, since no driver code moved and the lock change reaches only rdme and the browser capture feature
@answer n=5 entity=threat-model-summary#5.1 v=no
why: docs/capabilities.md is not among the reached files and the crossing it states is unchanged
