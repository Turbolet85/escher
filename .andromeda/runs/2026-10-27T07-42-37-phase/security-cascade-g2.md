# security · cascade read · group 2 of 6

## Carried
- (.andromeda/security-plan-detail/input-validation.md:48) Where a take moves a kept answer and the two committed Counter checks are re-recorded by their script, each re-recorded check must still read through the strict reader (every record's `answer` a JSON object) and replay step for step under `stand_replay` in the fast leg, never loosened to pass.
  > `digest` 16 lowercase hex and, since 2026-10-19-recorded-check-replay, `answer` required and a JSON object (missing, a string, a number, an array or `null` refused; key order not enforced, read as a map); anything else refused `Cause::Malformed` with no fault, holding nothing of the file, with no panic path (40 refusal rows, 8 read rows and edge rows at both bounds); read by `serde_json`, which writes nothing, each `Call` built through `command::call_of`, the builder the MCP server shares, so a replayed step still passes `validate` inside `Session::run` — since 2026-10-19-recorded-check-replay `Check::replay` runs each kept step's call through `Session::run` alone, nothing on a session of another app, and stops at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding bools, counts and closed schema words only; the reader also reads the two committed checks under `tests/blitz-tests/checks/` (`stand_replay`)
  (.andromeda/security-plan.md:73)
  > Recorded check (escher-driver `Check::from_bytes`) — a kept check's bytes are untrusted: a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused `malformed` holding nothing of the file, with no panic path; every replayed step still passes `validate` inside `Session::run`.
- (.andromeda/security-plan-detail/input-validation.md:56) After conflict resolutions in `node/node.rs` (the hit stops, `ebacd68bd`'s unstyled hit) and `scrolling.rs` (`visible_region`, `0980cce5f`), the driver's refusals must keep their fixed order and their reading of the engine's hit and visible region, each refused call changing nothing, in both layout modes, as the re-run stand refusal checks assert.
  > a `click` or a `type` on an element that cannot take it is refused with nothing run as well, with one answer, the first that holds in a fixed order — `disabled` (the snapshot reads the element not enabled), then `off-screen` (the centre of its bounds lies outside the viewport, or outside the visible part of a scrolling box that holds it), then `covered` (the hit at that point never reaches it) — and each refused call leaves the snapshot and both focus readings as they were, in both layout modes; `scroll` applies none of the three and says `in_view` in its result instead of refusing;
  (.andromeda/security-plan.md:77)
  > an `id` naming no element, or an element that cannot take the call, is refused with nothing run.
- (.andromeda/security-plan-detail/input-validation.md:80) `events/pointer.rs` moves under the take, so the link-navigation path must still resolve an `href` against the document URL and leave an unparseable one un-navigated.
  > Resolved against the document URL; an unparseable href is not navigated (packages/blitz-dom/src/events/pointer.rs:884-902; packages/blitz-dom/src/url.rs:19-21)
  (.andromeda/security-plan.md:89)
  > Link `href` — resolved against the document URL; an unparseable href is not navigated.
- (.andromeda/security-plan-detail/input-validation.md:86) `scrolling.rs` is a predicted conflict surface and `0980cce5f` rewrites much of it, so the resolution must keep the fragment's lossy percent-decode before matching.
  > Percent-decoded lossily before matching (packages/blitz-dom/src/scrolling.rs:805-808)
  (.andromeda/security-plan.md:93)
  > URL fragment — percent-decoded lossily before matching.
- (.andromeda/security-plan-detail/input-validation.md:98) Where the take carries `cee6b8731`, which edits `fetch.rs` and the runtime's fetch and module paths, the script fetcher must still turn an invalid `data:` URL or non-UTF-8 script bytes into `FetchError::InvalidData`, the `file:` narrowing added beside it and never in its place.
  > Invalid `data:` URLs or non-UTF-8 script bytes produce `FetchError::InvalidData` (packages/blitz-vibey-script/src/fetch.rs:48-55)
  (.andromeda/security-plan.md:101)
  > Script fetch — invalid `data:` URLs or non-UTF-8 script bytes are `FetchError::InvalidData`.
- (.andromeda/security-plan-detail/input-validation.md:137) `layout/construct.rs` is a predicted conflict surface (`2963b65a2`, `7fc9ee3af`, `0980cce5f`), so the resolution must keep inline SVG source parsed through `parse_svg_image`, a parse error logged and the element left without image data, never a panic.
  > Parsed with `parse_svg_image`; a parse error is logged and the element left without image data (packages/blitz-dom/src/layout/construct.rs:461-481)
  (.andromeda/security-plan.md:123)
  > Inline SVG source — a parse error is logged and the element left without image data.
- (.andromeda/security-plan-detail/input-validation.md:140) Where the take carries `efdc5472a`, whose CSS property accessors give a script a new way to write declarations, and where it carries `06ecec886`, which moves `resolved_style.rs`, declarations must still be validated by parsing and an invalid one ignored per CSSOM.
  > Validated by parsing; invalid declarations are ignored per CSSOM (packages/blitz-dom/src/resolved_style.rs:162-182; packages/blitz-dom/src/resolved_style.rs:218-251; packages/blitz-dom/src/cssom.rs:716-757)
  (.andromeda/security-plan.md:125)
  > CSS declarations — validated by parsing: invalid declarations are ignored, and unsupported property names or invalid values refused.
- (.andromeda/security-plan-detail/input-validation.md:143) `node/element.rs` moves under the take (`53e9d81c6`, and `1c5265a8b` and `06ecec886` where taken), and the new accessors set style through it, so `set_style_property` / `remove_style_property` must still reject an unsupported property name or an invalid value by returning `false`.
  > Reject unsupported property names and invalid values by returning `false` (packages/blitz-dom/src/node/element.rs:797-813; packages/blitz-dom/src/node/element.rs:863-867)
  (.andromeda/security-plan.md:153)
  > JS API arguments — the script runtime's native APIs check or coerce their arguments, throwing a RangeError, TypeError, SyntaxError or JS error on a bad call or falling back to a default; `fetch()` takes GET or HEAD only.
- (.andromeda/security-plan-detail/input-validation.md:152) `document.rs` moves under the take (`0980cce5f`, and `06ecec886` and `3954ecb85` where taken), so a `<style>` element's text must still have its HTML entities decoded before parsing.
  > HTML entities decoded before parsing (packages/blitz-dom/src/document.rs:1297-1302)
  (.andromeda/security-plan.md:129)
  > `<style>` element — HTML entities are decoded before parsing.
- (.andromeda/security-plan-detail/input-validation.md:155) `stylo.rs` moves under the take (`53e9d81c6`'s `:open`, and `1c5265a8b` and `06ecec886` where taken), so a `bgcolor` attribute from any page must still pass only as `#` and 3 or 6 hex digits, anything else ignored.
  > Accepted only as `#` followed by 3 or 6 hex digits; anything else is ignored (packages/blitz-dom/src/stylo.rs:898-919; packages/blitz-dom/src/stylo.rs:1186-1193)
  (.andromeda/security-plan.md:131)
  > Legacy attributes — `bgcolor`, dimensions, SVG `width` / `height`, `border` and body margin pass closed parsers; anything else is ignored or rejected.
- (.andromeda/security-plan-detail/input-validation.md:158) In the same moved `stylo.rs`, legacy dimension attributes must still go through Stylo's HTML dimension-value parser, its nonzero variant kept for table width and cells — under a Stylo 0.23 take, that parser's own API included.
  > Go through Stylo's HTML dimension-value parser, with a nonzero variant for table width and cells (packages/blitz-dom/src/stylo.rs:921-950; packages/blitz-dom/src/stylo.rs:1060-1066)

## Bindings
- **Recorded check ↔ data protection**: a re-recorded committed Counter check must pass the strict reader and stay inside the committed checks' medium, with no typed text, host path, name or secret and no `screenshot` step (.andromeda/security-plan-detail/input-validation.md:48) ↔ (.andromeda/security-plan.md:187)
- **Driver refusals ↔ error handling**: `ebacd68bd` hit-tests an element with no computed styles, and the `covered` reading goes through that hit, so the hit path must not reach the `layout_style()` panic on such a node (.andromeda/security-plan-detail/input-validation.md:56) ↔ (.andromeda/security-plan.md:360)
- **Script fetch ↔ threat model**: the fetcher's typed invalid-data error and `cee6b8731`'s `file:` narrowing together keep the default fetcher at `file:` and `data:` alone (.andromeda/security-plan-detail/input-validation.md:98) ↔ (.andromeda/security-plan.md:13); its errors keep their typed `Display` (.andromeda/security-plan.md:336)
- **Inline SVG source ↔ logging**: the logged parse error stays behind blitz-dom's `tracing` feature with a no-op fallback once the conflict in `construct.rs` is resolved (.andromeda/security-plan-detail/input-validation.md:137) ↔ (.andromeda/security-plan.md:392)
- **CSS declarations ↔ error handling**: the new accessors validate through the parse and `set_style_property` paths, while `selectorText` and `cssText` assignment still never throws (.andromeda/security-plan-detail/input-validation.md:140) · (.andromeda/security-plan-detail/input-validation.md:143) ↔ (.andromeda/security-plan.md:335)
