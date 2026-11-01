proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: '## Occupied Resources'
    change: 'Two word edits, the line staying under 2 000 chars: «no network port — no TCP or UDP port;» becomes «no served network port — no TCP or UDP port a binary serves;», and «(beside it, since 2026-10-14-quality-gates, one test-only bind:» becomes «(beside it, three test-only binds: blitz-net''s two below and, since 2026-10-14-quality-gates, this one:».'
    sidecar: 'occupied-resources res-network-ports-and-listeners: one test-only bind becomes three, and no network port reads as no served port.'
    rationale: 'Report Counts · test-bind marks occupied-resources.md:8 changed, one to three, and Symbols says the two new binds are TCP on loopback, so an unqualified «no TCP or UDP port» no longer holds of the tree; the served claim stands, no new served port.'
    basis: packages/blitz-net/src/lib.rs:526
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 11
    section: '## Occupied Resources'
    change: 'New row after the entity: - **Test-only TCP binds:** since 2026-11-01-upstream-sync-ui-graph `blitz-net`''s `#[cfg(test)] mod tests` binds `std::net::TcpListener::bind("127.0.0.1:0")` twice, one bind in each of two tests, test-only, served by no binary and reached by no caller, so no crossing and no served port (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)'
    sidecar: 'occupied-resources res-network-ports-and-listeners: registers blitz-net''s two test-only TCP loopback binds.'
    rationale: 'Report Symbols: the session rule of 2026-10-15 owes the registration as test-only in arch §Occupied Resources; New text places the two binds in delivers_network_and_bad_url_errors and http_error_body_is_not_delivered_as_css.'
    basis: packages/blitz-net/src/lib.rs:534
    fact: F25
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 205
    section: '## Occupied Resources'
    change: '«no network port, TCP or UDP; one local listener» becomes «no served network port, TCP or UDP, blitz-net''s tests alone binding TCP on loopback, served by no binary and reached by no caller; one local listener»; the rest of the line stands.'
    sidecar: 'architecture Occupied Resources broad line: no network port reads as no served port, the two test-only binds named.'
    rationale: 'The broad line of res-network-ports-and-listeners; report Symbols: no new served port, two new test-only TCP binds on loopback.'
    dependent-of: D-arch-resources
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 78
    section: '## Occupied Resources'
    change: 'New row after this one: - The `Once` `INIT` in blitz-dom''s `pub(crate) fn init_stylo_prefs`, process-wide since 2026-11-01-upstream-sync-ui-graph: the first call, from `UserAgentStylesheet::parse` or `BaseDocument::new`, sets Stylo''s prefs under their Stylo 0.22 names before the user-agent sheet is parsed, and a later call does nothing; no log site (packages/blitz-dom/src/document.rs:87-102; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)'
    sidecar: 'occupied-resources res-process-wide-state-and-threads: registers the process-wide Once of init_stylo_prefs.'
    rationale: 'Report Symbols, 7a3dd8918: init_stylo_prefs behind a Once, called from both sites; New text lists INIT.call_once in document.rs. The category registers process-wide statics of this kind, and no occupied-resources row names it.'
    basis: packages/blitz-dom/src/document.rs:87-102
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 110
    section: '## Occupied Resources'
    change: 'The row now reads: - The `matrix_test` job holds no `jlumbroso/free-disk-space` step: its step, skipped on every matrix row, is removed since 2026-11-01-upstream-sync-ui-graph, `Test (android)` reading no such step in CI#41333172440, so `Stand checks repeated` alone runs the action, pinned by `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/ci.md)'
    sidecar: 'occupied-resources res-ci-infrastructure: matrix_test''s free-disk step retired, the pin that holds it out named.'
    rationale: 'Report Retired (2) and Counts · free-disk: occupied-resources.md:110 states the matrix_test step, changed, removed; Harness names the new pin.'
    basis: .github/scripts/test_ci_workflows.py:110-115
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 33
    section: '## Cross-cutting Patterns'
    change: '«`mounted` listeners fire after `initial_build` and each `poll`» gains, after its citations and before the next `;`: «, skipping an element removed since its event was queued, read through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)».'
    sidecar: 'cross-cutting-patterns crosscut-event-processing: a mounted event skips an element removed since it was queued.'
    rationale: 'Report Symbols, 6ef4b532d: flush_queued_mounted_events skips an element removed since its event was queued, in the very function this clause states. The row''s claim is otherwise unmoved.'
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:235-239
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 42
    section: '## Cross-cutting Patterns'
    change: 'Append to the end of this line: «; since 2026-11-01-upstream-sync-ui-graph `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replace the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion, the changed-set insert in `remove_and_drop_all_children` staying first, and `append_text_to_node` also damages the parent (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807; packages/blitz-dom/src/mutator.rs:257-259; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)».'
    sidecar: 'cross-cutting-patterns crosscut-invalidation-and-state-integrity: narrowed restyle reach and text-append damage to the parent, the changed-set insert still first.'
    rationale: 'Report Symbols, fe33b778b and f76777766. Counts leaves the row''s other claims standing: cross-cutting-patterns.md:39 a true claim sharing the anon-block token and cross-cutting-patterns.md:40 naming snapshot_node_and, another symbol.'
    basis: packages/blitz-dom/src/mutator.rs:689-735
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 9
    section: '## Project Intent'
    change: 'Four edits: (a) «last read at `ebacd68bd`, twenty-four of its commits taken» becomes «last read at `e42ee1f25`, 57 commits past the merge base, thirty-six of them taken»; (b) after «`efdc5472a` (CSS properties as `CSSStyleDeclaration` accessors)» insert «, then twelve together (`45c333f16`): `fe33b778b` (#1276, restyle narrowing) · `8b1a0a3ce` (#1312, table damage) · `dee854c76` (#1317, `<picture>`) · `f6edb9701` (#1318, failed stylesheets) · `6fca6308d` (#1316, blend and isolation) · `7a3dd8918` (#1310, prefs before the UA sheet) · `91ef65e21` (#1314, `URLSearchParams`) · `a26c6ddeb` (#1322, `CustomWidget` attributes) · `f76777766` (#1319, inline decorations) · `6ef4b532d` (#1305, mounted events) · `ae774d7f4` (#1339, computed sizes) · `e42ee1f25` (#1342, culling bounds, a port)»; (c) «sixteen left out, each with its measured reason, among them» becomes «twenty-one left out, each with its measured reason, `22f0cd000` · `ac6f9f582` · `e5929f9e2` · `0674e8efa` · `784a567e4` joining the sixteen, among them»; (d) the closing measure gains «and escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md». The merge base and the waiting Stylo, Taffy and Dioxus clauses stand. Split the record at «then eleven together» and at «twenty-one left out», each continuation opening with one space, so no line passes 2 000 chars.'
    sidecar: 'project-intent intent: upstream read at e42ee1f25, thirty-six of 57 taken, twenty-one left out, the twelve named.'
    rationale: 'Report Counts · sync-line and left-out mark project-intent.md:9 changed, and Cross-project gives the 57, 36 and 21; inputs#I6''s first row names the five added to the sixteen. Report Lines over 2 000 chars lists this line.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 291
    section: '## Project Intent'
    change: 'Edits: «last read at `ebacd68bd`, twenty-four of the forty commits» becomes «last read at `e42ee1f25`, thirty-six of the 57 commits», «sixteen left out» becomes «twenty-one left out», and the measure reads «as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/lock.md»; the rest of the line stands.'
    sidecar: 'architecture Project Intent broad line: upstream read at e42ee1f25, thirty-six of 57 taken, twenty-one left out.'
    rationale: 'Report Counts · sync-line and left-out mark architecture.md:291 changed, three tokens and the left-out class; Expected amendments carries it.'
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/project-intent.md
    near: 9
    section: '## Project Intent'
    change: 'New line after the Upstream sync record: Upstream''s own tree panics the same way as the key press this fork fixed, at `packages/blitz-dom/src/node/node.rs:1740:27` at `e42ee1f25`, focused by `Tab` (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/panic-reading.md)'
    sidecar: 'project-intent intent: upstream''s own tree panics the same way at e42ee1f25.'
    rationale: 'Report Symbols and Cross-project: upstream''s tree panics the same way, focused by Tab, its first probe by a click having been vacuous; nothing sent upstream.'
    fact: F3
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 314
    section: '## Inherited Defaults'
    change: '«no network (TCP or UDP) listener» becomes «no served network (TCP or UDP) listener»; the rest of the line stands.'
    sidecar: 'architecture Inherited Defaults API style: no network listener reads as no served one, two tests binding TCP on loopback.'
    rationale: 'Report Symbols: two new test-only TcpListener binds and no new served port, so the unqualified wording no longer holds of the tree.'
    basis: packages/blitz-net/src/lib.rs:526
    dependent-of: D-arch-resources
note: 'inherited-defaults'' API-style row was not picked, though body line 314 states its wording and is proposed here, so its detail line still owes «no served»; the same bind count on CLAUDE.md and api-security.md:8 belongs to other owners.'
