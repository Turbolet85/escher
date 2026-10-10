Answers to the three technical forks asked at P4 of 2026-10-10-driver-cli, given through this session's question dialog, 2026-10-10. Each question and each chosen option's label are verbatim; the note is the answerer's own text, verbatim.

1. Question: "Technical fork 1 of 3 (operator) — how is the JSON on stdout written? escher-driver has three dependencies and no serialization crate today, and architecture's [Driver session] decision records that set as locked."
   Options offered: "By hand (Recommended)" · "serde_json dependency"
   Answer: "By hand (Recommended)"
   Note: "For this chunk the writer is output-only over fixed shapes. Pin on "MCP surface" that the question re-opens there, where a JSON parser is needed anyway: say then what keeping the hand writer costs against serde_json."

2. Question: "Technical fork 2 of 3 (operator) — what bounds an id in the session's record? Today it holds at most 4096 ids but stores each whole, whatever its length, and an id longer than 1024 bytes can be named by no call (CARRY from refusal-detection)."
   Options offered: "Skip over-long ids (Recommended)" · "A byte budget"
   Answer: "Skip over-long ids (Recommended)"

3. Question: "Technical fork 3 of 3 (operator) — how long does a client wait for a command's answer? The lifecycle's reply is read under a 2 s bound today; a command's answer would inherit it unless the plan states another."
   Options offered: "30 s for a command (Recommended)" · "Keep 2 s for everything"
   Answer: "30 s for a command (Recommended)"
