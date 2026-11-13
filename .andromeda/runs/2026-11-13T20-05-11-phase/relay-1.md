Question (P4 fork, question dialog): How deep should `handle_click` (pointer.rs, reads 97) be split? Upstream's own `handle_click` reads 69 of that. On 2026-10-16 escher deliberately left it above 15 so upstream syncs would stay out of arms escher doesn't own. This route entry now asks for 'under or toward 15'.

Answer: Every arm (Recommended)

Notes: The operator's technical answer. The entry asks for under or toward 15, and forked code is escher's own: staying additive is a preference where it costs nothing, never a reason to leave a function at 69. The accepted price is a hand re-apply when a later sync edits one of those arms. Behaviour stays byte for byte: no arm changes what it does.
