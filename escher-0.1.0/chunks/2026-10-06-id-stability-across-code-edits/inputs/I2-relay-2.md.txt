Three plan forks put to the founder at this phase's P4, after research, with the founder's answers verbatim.

1. Question: "Research found that every lean task's root sits directly under keyed containers that belong to the shell, not to the task (`main` > `task-shell` > `task-body` > the task component). Should a keyed ancestor outside an element's own component anchor its path?"
   Options offered: "Inside one component (Recommended)" — a component's root keeps starting at its component names, as today; the anchor applies only to unkeyed elements under a keyed element of the same component. · "Across components" — every unkeyed path starts at the nearest keyed DOM ancestor, whatever component owns it.
   Answer: "Inside one component (Recommended)"

2. Question: "The ruling defines an element an agent can act on as "focusable or role-bearing". CRUD's person row is a `div` with a click handler: it is not focusable and reads a generic role, so those two readers miss it, yet the stand's own CRUD check clicks it. Should carrying an event listener also count?"
   Options offered: "Add listener-bearing (Recommended)" — actionable = focusable, or an interactive role, or carries an event listener; each row gains an author key built from the person's model id; no focusability or tabindex change is made to the row. · "Focusable or role only" — the ruling as worded; no stand element gains a key.
   Answer: "Add listener-bearing (Recommended)"

3. Question: "How should an anchored path be spelled? It must never equal an author key, a component path or a document path. A bare `key/segment` can: a key spelled like a component name would read that component's path."
   Options offered: "#key/segment (Recommended)" · "key/segment, bare"
   Answer (the founder's own text): "Neither as written: spell an anchored path key//segment (a double slash after the anchor key, no leading sigil). A leading # starts a comment in bash and zsh, so an unquoted id would silently cut the command line, and intent finding 7 wants stand flows scriptable in shell alone. key//segment is collision-free by construction too: an author key holds no slash, a component path never has an empty segment, a document path starts with a slash. If research finds a case where // is not collision-free, tell me before planning around it."
