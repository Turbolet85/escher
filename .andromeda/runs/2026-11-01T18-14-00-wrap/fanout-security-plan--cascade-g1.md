proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 11
    section: "## Threat Model Summary"
    change: "Remove line 11's last sentence whole, the one beginning «A remote page can also reach a key-press panic»; the line's first sentence, on a page's markup reaching the engine-painted select list, stays as it is."
    sidecar: "threat-model-summary: the boundary-remote-content group's key-press panic sentence retired, the panic fixed by `deallocate_anonymous_block` clearing `layout_parent`; retired together with Error Handling's panic row."
    rationale: "report.md:33's Retired (1) names the key-press panic as a limit in §Threat Model Summary; report.md:23 has `deallocate_anonymous_block` ending it for all three ways of losing the box; report.md:50 counts `threat-model-summary.md:11` ×3 changed, retired together with `error-handling.md:48`; report.md:139 reads the criterion met."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:11
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 19
    section: "## Threat Model Summary"
    change: "After line 19, as a continuation line of the boundary-script-execution group, add: « A page's script also reaches `URLSearchParams` and `URL.prototype.searchParams`, with `href` and `search` live both ways, through `url_search_params.js`, the first of ten bootstrap scripts: a realization on this registered crossing, no boundary widening; its native helper `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs, and its argument checks are held under Input Validation (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184).»"
    sidecar: "threat-model-summary: the boundary-script-execution group gains `URLSearchParams` and `URL.prototype.searchParams` as a realization on the registered script crossing, no boundary widening."
    rationale: "report.md:17 adds the global class, the live `URL.prototype.searchParams` and the native helper deleted before any page script; report.md:92 reads `URLSearchParams` a realization on the registered crossing with no widening; report.md:46 states this surface is unnamed in security-plan; report.md:102 records its validation present, so D-security-input finds no drift and this is an addition."
    basis: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4
note: No New fact is of this group's categories (F5, F6, F7 and F8 are input-validation's, F9, F10 and F11 error-handling's); a page's CSS reaching painted blend and isolation, on the remote-content vector, is left to F7 under Input Validation; D-security-auth and D-security-deps found no drift (report.md:28, report.md:30).
