Question: Control V2 (the lint's `flatten` skip of a virtual node) reads DOES NOT HOLD, and no widening can turn it red: a virtual node's lint entry has no element and no visible part, so `overlaps` and `readings` already pass it by. The plan's acceptance «every control V1–V8 reads red» is therefore unmet for V2 by construction, and the research premise behind it is false. Every gate is green and V1 and V3–V8 hold. How should this run close?

Answer: Green, drop the skip

Notes: A guard no input can turn red is not kept. Drop the inert skip; report V2 as having no guard to control and the research premise as disproved. Keep (or add, if absent) the behaviour check that the lint answer on the declaring fixture names no virtual id — the fact F9 A wants is then pinned by a reading, not by dead code; say in the report which test holds it.
