The operator's answers to the three technical forks of P4, given in this session's question dialog, 2026-10-10. Each question as asked, the option chosen, and the notes typed with the answers, verbatim.

Q1. "Where does the bounds fix sit in the engine?" — answer: "The shared reader (Recommended)".
The option as put: Fix physical_unrounded_geometry in both bodies so it returns the box's own position, as its doc says. Three callers that use a box as the origin of its content take the offset themselves, and visible_region drops its add-back. Five sites in upstream-owned code plus ours; leaves no compensation standing.

Q2. "Is a conformance (WPT) reading taken before and after the fixes?" — answer: "No WPT reading".
The option as put: Proof is our checks plus upstream's own tests in the workspace (inline_fragment_rects, scrollbars, scrollbar_drag, paint_order, the script DOM tests), under both builds. The plan states the conformance reading as not taken. No checkout exists on this host and fork CI runs none.

Q3. "The founder's two forks are in the file. How does the plan proceed?" — answer: "Write the plan now on the narrowest options".
The option as put: Scroll containers only, and the second reader left with a route owner: both stay inside the ruling he already gave. The plan says so; a wider answer from him later is a revision of the plan.

Notes typed with the answers, verbatim:
Q2: state in the plan that no conformance reading is taken and why; first read (no clone, no build) whether the wpt runner can run the cssom-view tests at all without a script engine, and put that one fact in the plan — if it can, cheaply, say so at P5. Q3: I bring his two forks to him by dialog now; a wider answer is a plan revision..
