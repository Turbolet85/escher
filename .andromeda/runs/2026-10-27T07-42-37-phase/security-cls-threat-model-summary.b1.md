# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · yes · its never-styled guard (document.rs) binds the take's hit test of elements with no computed styles, which must avoid the same panic; its net, parser, iframe and @import rules sit in files the chunk does not reach
threat-model-summary#2.1 · yes · the default ScriptFetcher's file:-and-data:-only rule binds, as upstream's cee6b8731 narrows file: loads to file: documents and the runtime.rs split moves the fetch() base-URL lines it cites
threat-model-summary#3.1 · yes · its no-NodeId rule and its mask rule for passwords and file inputs bind the dioxus-native-dom id and mask port (7fc9ee3af, 286874cc1) and the pointer.rs file-input move; the socket's directory, modes and wire mechanics are Not-needed
threat-model-summary#4.1 · yes · its mask rule for passwords and file inputs and its no-NodeId rule bind the same id and mask port and file-input move, witnessed by stand_mcp_masked; its framing, bounds and cold-agent pipe are Not-needed
threat-model-summary#5.1 · yes · its carried set (ids, masked values, snapshot text and diffs, no description) binds the same id and mask port and file-input move as 4.1; docs/capabilities.md is not reached and its pin reads none of it
