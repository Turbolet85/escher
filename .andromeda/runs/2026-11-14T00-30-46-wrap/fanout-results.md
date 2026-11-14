# Fan-out results — 2026-11-13-local-files-for-local-documents

architecture: add 7 · set 4 · retire 0 · none 1 · ops-architecture.md (re-spawned once, `propose-r1`: the first file answered F5 twice, `[EVERY_TASK_FACT]`; its raw twin `.raw-ops-architecture.md`)
security-plan: add 2 · set 2 · retire 0 · none 2 · ops-security-plan.md
test-plan: add 4 · set 2 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 0 · set 1 · retire 0 · none 1 · ops-obs-plan.md
design-system: none · ops-design-system.md — reason: no new UI, every new surface reads tokens n/a, no row shown false
layout-templates: none · ops-layout-templates.md — reason: no surface, region, stand page or boot changed
a11y-plan: none · ops-a11y-plan.md — reason: no interactive element, no a11y schema or log field moved

Main's own: ops-main-test-plan.2.md — set 1 (check 5: the plan's expected entry «its guards gain local_files_for_local_documents», which the proposer's set of the rule left out).

Check 5, the plan's expected entries with no operation of their own:
- architecture · decision/non-exhaustive-request-types — `Request` stays `#[non_exhaustive]` (report, Symbols / APIs), so the row holds; the new field is registered by `contract/blitz-traits-net/request-initiator`.
- architecture · contract/blitz-net-provider/fetch and contract/html-net/provider — the report reads both promises holding (the refusal takes the existing `error` path; the typed `ProviderError` set gains a variant). No operation; the variant is registered in security-plan's `contract/blitz-net-provider-error/file-forbidden`.
- architecture · contract/script/document-may-load and contract/blitz-vibey-script-api/document-may-load — both promises hold through the re-export (`blitz_vibey_script::document_may_load` resolves; report Outcome). The body's new home is `contract/blitz-traits-net/document-may-load`. Only the rows' `verified` evidence coordinates are stale, and no operation sets them. The proposed `contract/blitz-traits-net/document-may-load` read `not_master` (the body's module is the code's to hold), so the entry rests on the two standing rows.
- architecture §Occupied Resources — `name/other/cargo-target-tmpdir` registers the env name (fits). The witness's own directory read `not_master` in test-plan (an inventory), so the occupied name is the env var alone.
- security-plan · environment values — no master site (report: pattern `tmpdir` refused). The env-read list stands in the leaf `.claude/rules/security.md`, which the cascade owns; the proposer's `@row_none` F5 agrees.
- security-plan · the «no path restriction, size cap or timeout» sub-clause — no master site (report). It is the cascade's, in the leaves `.claude/rules/security.md` and `.claude/docs/security-summary.md`.
- test-plan · the blitz-tests counts — no master site (report). Superseded by the measured 171 → 172.
- obs-plan · the new target's silence row — held by `decision/checks-silent` (`dec-4n2hhddr`), which covers every check and its shared modules. Entry 7's census reads 0.

Check 6, Spec claims disproved:
- The plan's predicted counts (161 → 162) — no master site. Routed to curation, P3 (a predicted count in a plan is unanchored).
- The directive's master (security-plan) — superseded by `inputs#I5` (the operator, 2026-11-14): the clause lands on architecture `decision/file-script-load-refused`, by ops-architecture.md's set n=2. DISPOSED.

## ops-architecture.md
decision/url-scheme-routing · apply — check 5: the plan's expected entry; the row's unconditional disk read is false now that blitz-net refuses a `file:` read for a non-`file:` loading document (report, Symbols / APIs)
decision/file-script-load-refused · apply — check 5: the plan's expected entry, the clause «so a remote page cannot read local files» owed by I1 (the founder, 2026-11-12, «Как в коде + работа») and placed on this architecture row by the operator's corrected directive, inputs#I5
decision/file-url-load-rule · apply — check 5: the plan's expected entry; the rule named script loads alone, and blitz-net applies the same predicate to every engine request
decision/builder-method-naming · apply — check 1, accurate this-chunk addition: the new consuming builder `Request::initiator` is not named `with_…` (report, Symbols / APIs), so the convention needs its exception
decision/blitz-dom-stamped-requests · apply — check 5: the plan's expected entry «architecture · conventions (a new row)»
decision/sub-resource-requests-carry-initiator · apply — check 1, accurate this-chunk addition: the seven stamped paths (report, Symbols / APIs; Outcome)
contract/blitz-traits-net/request-initiator · apply — check 1, accurate this-chunk addition: a new public field and builder (report, Symbols / APIs)
contract/blitz-traits-net/document-may-load · reject — check 7: not_master type 4 — the promise states only which module holds the predicate's body and that another crate re-exports it, a placement the code holds
contract/stylesheet-handler/document-url · apply — check 1, accurate this-chunk addition: a public field on a public struct (report, Symbols / APIs)
name/other/cargo-target-tmpdir · apply — check 5: the plan's expected entry on §Occupied Resources, registered test-only (report, Counts)
decision/blitz-dom-clippy-config · reject — check 7: not_master type 6 — it retells that the config file exists and is read by the clippy leg; what it guarantees stands in decision/blitz-dom-stamped-requests

## ops-security-plan.md
decision/file-url-only-for-file-documents · apply — check 5: the plan's expected entry, the rule widened from script loads to the engine's sub-resource fetches
decision/file-urls-only-from-file-documents · apply — check 5: the plan's expected entry, the twin row
decision/one-stamping-request-constructor · apply — check 1, accurate this-chunk addition: the construction guard is the validation gate that makes every sub-resource request carry its document (report, Schema / config; Outcome)
contract/blitz-net-provider-error/file-forbidden · apply — check 1, accurate this-chunk addition: the fixed, path-free `FileForbidden` text (report, Symbols / APIs; Coverage)

## ops-test-plan.md
decision/file-urls-only-for-file-documents · apply — check 5: the plan's expected entry
decision/png-decoding-in-checks · apply — check 1, accurate this-chunk addition: a check now encodes a PNG too (report, Dependencies)
decision/sub-resource-requests-carry-their-document · reject — check 7: not_master type 4 — it retells how the initiator is stamped on each path with no reason, which the code holds (architecture's decision/sub-resource-requests-carry-initiator states the decision)
decision/blitz-dom-requests-through-stamping-constructor · apply — check 1, accurate this-chunk addition: the clippy leg now guards construction (report, Harness / gate surface)
name/other/local-files-for-local-documents · reject — check 7: not_master type 5 — an inventory of test files and CARGO_TARGET_TMPDIR readers, not a name the register reserves
decision/real-provider-inside-stand-in · apply — check 1, accurate this-chunk addition: the witness's ReadThrough stand-in (report, Dependencies; Decisions & corrections)

## ops-obs-plan.md
decision/no-file-loads-from-non-file-documents · apply — check 5: the plan's expected entry, extended to the engine's sub-resource refusals with no log site of their own

## ops-main-test-plan.2.md
decision/file-urls-only-for-file-documents · apply — check 5: the plan's expected entry «its guards gain local_files_for_local_documents»; lands after ops-test-plan.md's set of the rule
