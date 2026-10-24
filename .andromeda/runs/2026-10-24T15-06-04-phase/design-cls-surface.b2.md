# Classified — design · surface.b2

## Answers
surface#14.2 · no · examples/screenshot.rs is upstream's example binary, reached by no file in the change, and the sweep's screenshot witness reads escher-session's and the harness's PNG, not this print
surface#15.1 · yes · escher-session's stdout is one JSON answer line with no colour or ESC byte and exit statuses 0 · 1 · 2 · 3, and its mcp role's stdout carries protocol lines only, which the cli witnesses and host_log and mcp_log's stdout-is-the-answer check rest on
surface#16.1 · no · WASM font registration is the web-spa surface the sweep does not need (headless native only), and neither examples/wasm_hello nor packages/dioxus-native is reached
surface#17.1 · no · wasm_hello's canvas size, 4px radius and outline-free tabindex are the web-spa surface the sweep does not need (headless native only), and examples/wasm_hello is reached by no file
