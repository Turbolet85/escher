@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=56
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=15 class=omission
leaf-text: refused before a disk read with `FileForbidden` (fixed text, no URL or path) — and where it reads
source: dec-gsml2o7x «A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, script fetch, module import and script src fail, blitz-net refuses engine loads before a disk»
replace-with: refused before a disk read with `FileForbidden` (fixed text, no URL or path), and the browser app names no `file:` favicon and drops a `file:` navigation silently for any other document, remote pages included — and where it reads
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=23 class=omission
leaf-text: it receives no `Session`, so a refused call changes nothing;
source: dec-oostyleo «A fixture state name is compared whole against the app's closed table of named states before any session starts; any other name is refused as an unknown state and nothing is booted.»
replace-with: it receives no `Session`, so a refused call changes nothing; since 2026-11-14-start-in-a-state a fixture state name is compared whole against the app's closed table of named states before any session starts, any other name refused as an unknown state with nothing booted, and a reach error is one of a closed set (unknown state, refused at a step with its index and closed cause, not settled at a step with the busy reading), each a fixed text holding nothing a caller supplied;
