fct-cq2qgcv5@1 · A parse error in inline SVG source is logged and leaves the element without image data instead of failing the document.
fct-um6i723w@1 · Invalid CSS declarations are ignored and unsupported property names or invalid values refused, so no invalid declaration reaches style.
fct-uzjgvwv6@1 · An invalid selector list leaves its rule unchanged, and query parse errors are returned to the caller rather than panicking.
fct-5wlhxhdv@1 · Legacy presentational attributes pass closed parsers, and any other value is ignored or rejected.
fct-ht5tidu5@1 · `text_extent` is a read-only reader returning geometry only.
fct-bh2jhziy@1 · Scroll targets are clamped to the scroll range, and a programmatic scroll to a missing node does nothing.
fct-34fqtojx@1 · A file input's `accept` is not applied as a filter on the files chosen.
fct-66vufcs4@1 · Form data validity in dioxus-native always reports valid, so no form is ever held back as invalid.
fct-qk7naa6l@1 · Dioxus `dangerous_inner_html` is applied as inner HTML of the element, unsanitized.
fct-ydvrpbvz@1 · The test harness query helper panics on an invalid selector, while selector parsing itself returns a `Result`.
fct-b6vsgrbb@1 · The WPT runner runs only JavaScript script types and escapes its wrapper HTML.
fct-zwiprjdk@1 · The CI PR lookup takes only an open PR whose head sha equals the run's, and the comment splice replaces only when both markers exist in order.
fct-63f2oo75@1 · Request URLs and the cache directory path appear in the logs.
fct-ij663iad@1 · Clear history deletes every history row both in memory and on disk, and the HTTP cache can be cleared on demand.
fct-ecpelafu@1 · A failed fetch shows the user error.html with the error's Debug text in its error paragraph.
fct-fcagkjyj@1 · Error pages are flagged is_error and skip history recording and favicon probing.
fct-mcdiomir@1 · Persistence callers never receive a Result; errors are logged at warn and swallowed.
fct-svn2b5w4@1 · A failed stylesheet request leaves the pending critical resources, so it blocks no render.
fct-xkfthmc4@1 · NetHandler::error is a default that does nothing, so a handler sees no failure unless it overrides it.
fct-27ntkqqx@1 · Uncaught script errors are logged and not propagated, and a script that fails to resolve or fetch is skipped.
fct-hrr3a26v@1 · Resolve and hit testing may run with no DOM and return early with a warning, and a UI event on a document with no target is ignored.
fct-c4rr3jfv@1 · Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic.
fct-p2nck7ms@1 · Aborting the configured AbortSignal cancels every in-flight fetch tied to the document.
fct-3vrrikft@1 · Sub-document navigations route back to the parent document instead of navigating the host.
fct-upgu656r@1 · The select list's public readers and writer read a row with no styles as None and add no panic path.
fct-wogfa4ah@1 · The render-path recorder holds no unwrap, expect or panic and has no slicing panic path.
fct-64ubemwx@1 · A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row.
fct-5onuddg3@1 · A failed, empty or non-CSS stylesheet does not block render, and a net handler wrapper that does not forward error drops the failure silently.
fct-36ipyrv2@1 · An open select list's highlighted row is in no snapshot and no tree, and the lint does not read its contrast.
fct-dxkboam6@1 · Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused.
fct-cfr7fwzo@1 · The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session.
fct-lnusqzqb@1 · The :open pseudo-class matches a details or dialog by its open attribute's presence, kept current by attribute writes and removals.
fct-rc3jl2xp@1 · Handler coverage relies on the reached record holding the stable id and event of each node whose listeners held a dispatched event, deduplicated, so a handler whose run removes its element still counts.
fct-6dak5se6@1 · The unkeyed-actionable reading and the actionable-nodes reading both read one actionable rule: focusable, an interactive role, or current listeners holding an event other than `mounted`.
fct-pnrbnsjg@2 · Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open.
fct-exdlckxb@1 · Degenerate aspect ratios are discarded before layout uses them.
fct-o56d7tkg@1 · The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout.
fct-pwj6p2fo@1 · Geometry whose transformed bounds are non-finite or beyond the f32 range is culled before painting.
fct-gukbahaa@1 · Scroll targets are clamped to the scrollable range, and a programmatic scroll to a missing node does nothing.
fct-q43mkq7y@1 · Form submission excludes controls under a datalist, disabled controls, non-submitter buttons and unchecked checkboxes and radios.
fct-3atajnay@1 · The Dioxus event handler records a dispatch only where a signal registry is registered, and a page with none dispatches as before.
fct-et47i5mq@1 · Dioxus dangerous_inner_html is applied as inner HTML.
fct-wlz77kgt@1 · A falsy value of a listed boolean attribute clears the attribute rather than writing it.
fct-73s6fq52@1 · Script reads and writes CSS properties through named accessors on CSSStyleDeclaration, one per Stylo property name.
fct-62emqkgs@1 · Stored URLs are re-parsed on load and unparsable rows are skipped.
fct-txc2a4of@1 · Serialization escapes text and attribute values but writes text inside raw-text elements such as style and script unescaped.
fct-4ck7btcl@1 · A failed load's error text stops in the load response and the redacted Resource load failed line, and no carrier takes it.
fct-ksl3fslx@1 · A select list's option and group labels leave only as painted pixels and the snapshot's existing name, the list holding no log site, span or field.
fct-4cylhur3@1 · The render-path mutation buffer logs nothing itself and its replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
