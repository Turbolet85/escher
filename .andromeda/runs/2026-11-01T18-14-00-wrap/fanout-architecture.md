proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Network ports and listeners
    change: "The line reads no served network port, TCP or UDP, beside three test-only binds no binary serves and no caller reaches, so no crossing: blitz-net's two `TcpListener` binds on `127.0.0.1:0` in its `#[cfg(test)] mod tests` and escher-driver's unit test's `UnixListener`; one served local listener, the driver session's Unix-domain socket as now; the `escher-session mcp` role as now; still no second served listener."
    sidecar: "§Occupied Resources: blitz-net's two test-only TCP loopback binds registered, no served port, no crossing; test-only binds one to three."
    rationale: "Report Changes, Symbols / APIs, the ports bullet: two new test-only binds, TCP on loopback, «no network port» no longer true as worded, and the session rule owes their registration as test-only in arch §Occupied Resources; Expected amendments carries it; the body's flat «no network port, TCP or UDP» and «no second listener» are unregistered against them."
    basis: "packages/blitz-net/src/lib.rs:526"
  - detector: D-arch-resources
    severity: warning
    section: §Inherited Defaults → API style
    change: "The clause reads no served network (TCP or UDP) listener, blitz-net's two test-only loopback binds being no crossing, and one local Unix-domain socket, the driver session's, as now."
    sidecar: "§Inherited Defaults, API style: «no network listener» narrowed to no served one, matching §Occupied Resources."
    rationale: "The same claim as the primary, restated in the API style default: a flat no-network-listener, which the two `TcpListener` binds in blitz-net's test module make untrue as worded (report Changes, Symbols / APIs, the ports bullet)."
    basis: "packages/blitz-net/src/lib.rs:534"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → JavaScript (blitz-vibey-script)
    change: "The list reads: `__blitz_` globals carry messages and synchronous fetches, and `__blitz_parse_search_params` serves the bootstrap alone, deleted from `globalThis` before any page script; a JS bootstrap defines history, fetch, DOMException, the CSSOM classes and `URLSearchParams` with `URL.prototype.searchParams`, `href` and `search` live both ways."
    sidecar: "§Standard Contracts, JavaScript: the `URLSearchParams` global, `URL.prototype.searchParams` and the deleted native registered."
    rationale: "Report Changes, Symbols / APIs, `91ef65e21`: a new global class and prototype property on a page's script surface and a new `__blitz_` native; Counts, `url-params`, marks `standard-contracts.md:240` changed where it lists globals, and the body's own list of what the bootstrap defines stands without them."
    basis: "packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4"
  - detector: D-arch-decisions
    severity: warning
    section: §Project Intent → Upstream sync
    change: "The line reads upstream `main` last read at `e42ee1f25`, thirty-six of the fifty-seven commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and twenty-one left out; the merge base and the pins Stylo 0.22.0, Taffy 0.15 and Dioxus 0.7.3 stand as now."
    sidecar: "§Project Intent, Upstream sync: read `ebacd68bd` to `e42ee1f25`, twenty-four of forty to thirty-six of fifty-seven, sixteen to twenty-one left out."
    rationale: "Report Counts, `sync-line`, marks `architecture.md:291` changed three times; Cross-project claims measure `upstream/main` at `e42ee1f25`, merge base `23b9b26a9`, 57 past it, 36 taken, 21 left out; Expected amendments carries it; the standing line contradicts the measured take."
    basis: ".andromeda/architecture.md:291"
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions
    change: "The select list's sentence reads its merge surface is the sixteen files of the select-popup record, a line owned by «Upstream sync ahead of determinism and exploration», no longer by Epoch 3's Upstream sync ahead of the UI graph."
    sidecar: "§Established Decisions: the select list's merge surface re-owned to «Upstream sync ahead of determinism and exploration»."
    rationale: "This chunk is the named owner and has landed: report Counts, the owner rows under Sweep reach, mark `architecture.md:62` re-pointed to the owner the card names, and inputs#I6's merge-surfaces row names a `CARRY:` on «Upstream sync ahead of determinism and exploration»."
    basis: ".andromeda/architecture.md:62"
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions
    change: "The sentence on the eight direct engine changes reads the merge surface of those eight is a line owned by «Upstream sync ahead of determinism and exploration», `hit_inner` a textual conflict with upstream `0980cce5f` as now."
    sidecar: "§Established Decisions: the eight engine changes' merge surface re-owned with the select list's."
    rationale: "The same owner claim worded «Epoch 3's sync», the entry this chunk discharged: the detail's matching owner rows are in the report's Sweep reach list, and inputs#I6's merge-surfaces row moves the surfaces to the next sync's entry; `0980cce5f` stays among the left-out sixteen on that card, so the conflict clause stands."
    basis: ".andromeda/architecture.md:60"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: escalate
    section: §Established Decisions
    change: "A new sentence beside the select list's: this take's twelve upstream commits are engine behaviour changes for every document, ratified for their take; the key-press panic's fix, `BaseDocument::deallocate_anonymous_block` clearing `layout_parent` on each child still naming the block, is one more direct engine change for every document, ratified, flagged upstreamable with nothing sent upstream, its merge surface a line owned by «Upstream sync ahead of determinism and exploration»."
    sidecar: "§Established Decisions: the twelve behaviour changes and the panic's fix recorded as ratified; the founder's words on the answer card and in this entry, the body taking only the bare word."
    rationale: "Report Expected amendments, architecture §Established Decisions, carries it and marks it P2's halt, the answer card; For P2's halt says a master body takes none of his words, only the bare `ratified`; the body lists every other direct engine change for every document, and these stand on no sentence; no boundary widening read in the twelve, per the same bullet."
    basis: "packages/blitz-dom/src/document.rs:1493-1503"
  - detector: D-new-detail
    severity: warning
    section: §Infrastructure Patterns → Runtime threading and event flow
    file: new
    near: "§Infrastructure Patterns, the Runtime threading and event flow bullet, its clause on fetches through `NetProvider::fetch` with a `ResourceHandler`"
    change: "A failed fetch reaches the handler through `NetHandler::error`, a public default that does nothing, blitz-net's error arm calling it before the waker, and `ResourceHandler<StylesheetHandler>::error` responds `Err`, so the request leaves `pending_critical_resources` and a failed, empty or non-CSS stylesheet stops blocking render (packages/blitz-traits/src/net.rs:36-38; packages/blitz-net/src/lib.rs:313; packages/blitz-net/src/lib.rs:316; packages/blitz-dom/src/net.rs:144-146)"
    sidecar: "§Infrastructure Patterns, Runtime threading and event flow: the failed fetch's path through `NetHandler::error` added."
    rationale: "Report Changes, Symbols / APIs, `f6edb9701`: a new failure path on the fetch flow this bullet states and on the critical-resource gate of its `resolve` order; §Infrastructure Patterns has no category file, so this detail is the detector's to propose."
    basis: "packages/blitz-net/src/lib.rs:313"
note: "D-retired found no standing passage in the body for any of the six Retired items, each stands only in detail or other masters, so the cascade's; .andromeda/architecture.md:163 is listed changed by Counts contrast-none only where a line states its `None` conditions and it states none, so no proposal and the fact stays with .andromeda/architecture-detail/standard-contracts.md:131; the body's driver-session «no TCP or UDP port» is scoped to the session's crossing and stands."
