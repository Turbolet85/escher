# Answer card — Q1, the SVG image fetch (boundary widening by upstream merge, rul-72xhfjoz)

## The founder's answers it rests on
- inputs#I5 §2 — upstream's 7b8712cba, external images for SVG `href` on every document: «Взять (Recommended)» (relayed verbatim by the overseer, 2026-11-17).
- inputs#I8 §2 — the SVG fetch's measured scope, `<image>` and `<feImage>` `href`, `<use>` not fetched: «Да, в силе (Recommended)» (relayed verbatim by the overseer, 2026-11-17).

## The escalated operations lying wholly inside them
- ops-architecture.md · decision/sub-resource-requests-carry-initiator — area established-decisions · fit: fits type 1 — the stamped sub-resource kinds name an SVG image or feImage href, a use href never fetched.
- ops-security-plan.md · decision/svg-image-href-fetch — area input-validation · fit: fits type 1 — an SVG image or feImage href fetched as an image sub-resource through the stamping constructor, under the file-URL refusal; a use href never fetched; the widening, ratified.
- ops-test-plan.md · decision/file-urls-only-for-file-documents — area of its row · fit: fits type 1 — the `file:` rule's refused sub-resource loads name the SVG image or feImage href.

## Named beside the card, not asked
- ops-architecture.md · decision/svg-embedded-images-by-context — fit: not_master type 4 (how the parse passes the image cache by caller; the code holds it): rejected.

## The answer
- operator, 2026-11-17: «Confirm all three (Recommended)» — the confirm that each listed operation lies inside the founder's answers; the ratification stays his (inputs#I5 §2, inputs#I8 §2).
