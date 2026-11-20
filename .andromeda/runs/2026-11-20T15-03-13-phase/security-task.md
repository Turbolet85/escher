@task_prose part=chunk
Five select list and selection debts, each one first written as a founder fork into one founder-forks.md in the run dir, then fixed or kept as a residual by his word.
1. Highlighted-row contrast: an open select list's highlighted row, and a multiple select's chosen row, made readable by the visual lint's contrast defect.
2. A larger authored font: the open list's row height made to fit a select's font larger than one line, so rows no longer clip.
3. The highlight's reading: the highlighted row of an open list stated in the snapshot, diff, MCP result, kept check and accessibility tree, or kept unread; a single select with size above 1 rendered as a list box, or kept as a closed dropdown; Cells' data cells given a selected reading and grid role, or kept as they are.
Each fix is proven headless on the stand or a fixture in both layout modes; each kept debt is re-stated as a residual with the founder's reason.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the open select list's engine state — its rows, row height from the select's font, highlighted row and scroll offset — edits: the row height computation
@task_file n=2 path=packages/blitz-paint/src/render/select_list.rs mark=edits
what: the select list painter that paints the open list over the page with the engine's own colours and the highlight — edits: the row painting at the row height
@task_item n=1 part=needs
The open select list's highlighted row and a multiple select's chosen row: their text and background colours are the engine's own select list colours, and any contrast reading of them by the visual lint's contrast defect is a lint finding that must stay closed content, holding no row label or page text.
@task_item n=2 part=needs
Lint findings cross every ratified carrier: a new contrast finding on a select list row leaves by those carriers alone and adds no new finding kind field or open value without the founder's word.
@task_item n=3 part=needs
The open select list's row height from the select's authored font: a larger font must not clip, and the row count stays capped at twenty rows, the list stays painted over the page and its rows scroll into view by the list's own offset; hostile markup under an open list (huge fonts, many rows) must still lay out and paint finitely.
@task_item n=4 part=needs
The highlighted row's reading: any reading added to the snapshot, diff, MCP result, kept check or accessibility tree crosses the driver's carriers; a closed-value field inside a ratified carrier is no widening, while a new key or open value is a boundary widening that is the founder's fork.
@task_item n=5 part=needs
The select list crossing today is limited to bounds and pixels, the open list shot carries labels as pixels, and row labels are never logged: a highlight reading must not carry a row label as text on any carrier or into any log unless the founder widens the crossing.
@task_item n=6 part=needs
Snapshot and diff state words and the three state readings per control: a highlighted or selected reading on an open list row or a Cells cell reuses the closed selected and unselected words and the diff's selected key, read from engine state only, and the value mask still holds on every carrier.
@task_item n=7 part=needs
Accessibility adapter crossing: a highlighted row or a selected cell reaching the accessibility tree crosses only as its content classes allow, the tree is refreshed on the change, focus names a built node, and accessible names stay out of logs.
@task_item n=8 part=needs
Which select opens a list: today size is read on a multiple select alone and a single select with size above 1 renders as a closed dropdown; rendering it as a list box changes the open gate, the single-select selectedness, its keys and clicks, where its choice leaves and the select size hint cap, all through the one select choice write path.
@task_item n=9 part=needs
Cells' data cells, keyed by author keys cell-{A..Z}{0..25} and the edit input cell-input: a selected reading and grid role, if added, must follow the Cells grid keyboard model and the stable element id's author-key derivation, and state selected by the same attribute reading the flight booker and CRUD use.
@task_item n=10 part=needs
Every fix that changes engine behaviour reaches every document, remote pages included: an engine change for every document is itself the founder's fork, and the security tier's rule on boundary widening governs where a reading leaves on a carrier.
@task_item n=11 part=needs
No new carrier, verb, exit, refusal cause, session error, log field or span is added by this chunk; one only by the founder's word.
@task_item n=12 part=needs
Founder forks are written into one file in the run dir and the phase stops until his relay returns: technical forks that turn nothing of what the product is are the operator's, by dialog.
@task_standing n=1
convention: each new headless test or fixture proving a fix runs on the stand or a committed fixture, reads no clock beyond the virtual clock, and holds no secret or credential in its evidence
home: input-validation
@task_standing n=2
convention: each new check or kept check written for a fix stays owner-only and holds only the closed content the kept check classes allow
home: data-protection
@task_item n=1 part=not_needed
Authentication and authorization, CI tokens and publish secrets: the chunk adds no credential path and no CI job.
@task_item n=2 part=not_needed
Outbound HTTP, fetch, CORS, CSP and request size or timeout rows: the select list and Cells changes make no network request.
@task_item n=3 part=not_needed
The select list thumb's drag and the pointer gestures on it: that debt belongs to a later chunk.
@task_item n=4 part=not_needed
Range input, password painting and masking of input values beyond keeping the mask on every carrier: the chunk touches no range or password control.
@task_item n=5 part=not_needed
Session socket grammar, connection bounds, state directory and recording directory rows: the chunk adds no verb, argument or session file.
@task_item n=6 part=not_needed
Dependency pins, licence and audit gates' ignores, and secret management environment variables: the chunk adds no dependency and reads no environment.
@task_item n=7 part=not_needed
Browser history, cookies, HTTP cache and sqlite storage: the chunk stores nothing.
@task_item n=8 part=not_needed
WPT runner, quarantine and post-results workflow rows: the chunk's proofs are headless stand or fixture tests, not WPT.
