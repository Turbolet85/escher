Question (phase P4, the founder's fork, the MountedData panic): The founder's fork, the MountedData panic (Dioxus runtime probe chunk). Measured at HEAD from the code: DioxusDocument::poll holds the document's RefCell borrowed while Dioxus 0.7's render_immediate polls queued tasks. So a MountedData call (scroll_to, set_focus, get_client_rect…) made in a task spawned from an event handler panics «RefCell already borrowed» (events.rs:162). Any Dioxus app can hit it; the stand avoids it by calling handles synchronously. Upstream fixed it in 286874cc1, which renders into a mutation buffer with the document unborrowed and applies the buffer afterwards, but that commit also moves Dioxus to 0.8.0-alpha.3. Every fix is an engine change for every Dioxus document. Which way?

Options, in order: 1. Port, own chunk (Recommended) · 2. Port in this chunk · 3. Wait for Dioxus 0.8 · 4. Guard, no panic

Answer: "Port, own chunk (Recommended)"
Notes: The founder's choice, by dialog in the overseer session: «Порт, отдельный чанк (Рекомендую)».
