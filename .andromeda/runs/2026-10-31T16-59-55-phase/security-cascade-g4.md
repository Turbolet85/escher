# security · cascade read · group 4 of 6

## Carried
- (.andromeda/security-plan-detail/input-validation.md:281) The edited `lint`, reading options with boxes of their own, must take this reader's `None` for a stale option or one with no inline layout data as a skip, stay its one non-test caller unless a change names another, and keep its findings within what the carriers already ratify.
  > Since 2026-10-19-visual-lint a read-only reader on every document, remote pages included: an id that does not resolve, a node that is no inline root or one with no inline layout data reads `None` through `get_node`, never an index; it mutates nothing and has no log site, print, env read or panic path; it returns geometry only, and its one non-test caller is `Session::lint`, in process and through the `lint` verb's arm (packages/blitz-dom/src/text_extent.rs:18-54)
  (.andromeda/security-plan.md:141)
  > `text_extent` is a read-only reader returning geometry only.
  (.andromeda/security-plan.md:65)
  > bounded replies carry ids, names, control values (a password's or file input's masked), snapshot text, diffs, screenshots and a lint's findings, the state readings as closed words and booleans, never an attribute's text, and no `NodeId`.
  (.andromeda/security-plan.md:67)
  > Driver session socket (escher-driver) — the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
- (.andromeda/security-plan-detail/input-validation.md:284) The edited `execute` and `lint` must read an open list's option boxes through this reader, with `None` on a non-finite edge handled as no visible region and no panic, and an act on an option must pass the driver's closed `validate` path by its stable id alone.
  > On every document, remote pages included: `visible_region` reads each clipping box through its transforms, by the private `padding_box_in_view` over `transformed_bounds`; with no transform on the chain it reads the padding box in the client rect's frame, and a mapped box with a non-finite edge answers `None`; exact under a translate and a scale, under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show; its callers are `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`
  (.andromeda/security-plan.md:77)
  > Driver command schema (escher-driver) — `validate` checks every call against a closed `'static` table of twelve verbs and seven bounded argument kinds, returning a typed `Command` or a `Refusal`, the first failure in a fixed order; nothing public runs a `Command` that did not come from `validate`, and the socket, the command line and MCP all pass through it; an `id` naming no element, or an element that cannot take the call, is refused with nothing run.
  (.andromeda/security-plan.md:79)
  > Driver command schema (escher-driver) — no `unwrap`, `expect` or panic on anything a call supplies, no clock read by the schema or its validation, no `NodeId` kept between calls
  (.andromeda/security-plan.md:137)
  > Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
  (.andromeda/security-plan.md:73)
  > Recorded check (escher-driver `Check::from_bytes`) — a kept check's bytes are untrusted: a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused `malformed` holding nothing of the file, with no panic path; every replayed step still passes `validate` inside `Session::run`.
- (.andromeda/security-plan-detail/input-validation.md:287) The overlay's painted list, for any option count or a select near a viewport edge, must pass this cull and never paint non-finite or out-of-range geometry.
  > Geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419)
  (.andromeda/security-plan.md:143)
  > Paint values — non-finite geometry is culled, gradient overflow clamped, oversized radii scaled down, and hidden inputs not painted.
- (.andromeda/security-plan-detail/input-validation.md:290) The overlay's new hit path must take no hit for a select under a transform whose determinant is 0 or not finite, as the tree's own hit test answers for that subtree.
  > On every document, remote pages included: `Node::hit_inner` answers `None` for the whole subtree of a transform whose determinant is 0 or not finite, so it takes no hit; the added lines hold no log site, print, env read or panic path
- (.andromeda/security-plan-detail/input-validation.md:293) `blitz-paint` is reached whole, so any gradient a list's row or box paints from page styles must keep this clamp and its unit tests.
  > Percentage overflow clamped to the f32 range, with unit tests for both extremes (packages/blitz-paint/src/gradient.rs:497-504; packages/blitz-paint/src/gradient.rs:521-535)
- (.andromeda/security-plan-detail/input-validation.md:296) The list's box and rows must scale oversized radii through this rule, and the new engine defaults in `assets/default.css` must pass the CSS parsing rule like any other declaration.
  > Oversized radii are scaled down uniformly so adjacent radii do not overlap (packages/blitz-paint/src/kurbo_css/css_box.rs:57-75)
  (.andromeda/security-plan.md:125)
  > CSS declarations — validated by parsing: invalid declarations are ignored, and unsupported property names or invalid values refused.
- (.andromeda/security-plan-detail/input-validation.md:299) `blitz-paint`'s `render.rs` is reached whole, so an input with type=hidden must stay unpainted beside the overlay's new branches.
  > Inputs with type=hidden are not painted (packages/blitz-paint/src/render.rs:306-310)
- (.andromeda/security-plan-detail/input-validation.md:302) A list of any option count and a driver's scroll to an option must keep targets clamped to the range, and a scroll to an option a page has removed must return without effect.
  > Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)
  (.andromeda/security-plan.md:145)
  > Scrolling — targets are clamped to the scroll range; a programmatic scroll to a missing node does nothing.
- (.andromeda/security-plan-detail/input-validation.md:314) `events/pointer.rs` is reached whole, so the file input's branch must keep this reading beside the new list branches, and the edited snapshot's option `bounds` must leave the file input's value masked.
  > The `accept` attribute is not applied as a filter (packages/blitz-dom/src/events/pointer.rs:955-957)
  (.andromeda/security-plan.md:147)
  > a file input's `accept` is not applied as a filter.
  (.andromeda/security-plan.md:139)
  > Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path; a password input paints a bullet per character, and a recorded masked `type` is kept as `MASKED_STAND_IN`; the DOM and form submission keep the value.
- (.andromeda/security-plan-detail/input-validation.md:329) The list's key and pointer code must read `disabled`, `inert`, `multiple` and `selected` by presence, as this removal makes true in a Dioxus document, and must read `aria-expanded`, outside the closed list and so still written holding `false`, by a closed parse and never by presence; a newly read `size` must parse with a default fallback, and a group's label must stay page text with no panic path.
  > Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written.
  (.andromeda/security-plan.md:135)
  > Markup attributes — read from parsed content, remote pages included, by in-process readers with no log site or panic path: `disabled` and `inert` by presence, ARIA states and closed values through closed parses carrying no attribute text, the range and select controls held within their rules, Enter activating buttons and links and Space buttons as a click, names and descriptions as page text; they leave escher only by the ratified carriers, the page's own form submission and events aside.
  (.andromeda/security-plan.md:133)
  > Markup numeric attributes — parsed with default fallbacks; `rowspan` clamped to 1..=65534, `<ol start>` parsed as `usize`.
- (.andromeda/security-plan-detail/input-validation.md:410) A choice from the list on the stand's `select#flight-type` must reach this closed match through `apply_select_choice`'s `input` value over its two options, and the stand tables that read this select move with any new row.
  > the flight booker's mode `select#flight-type` writes its `flight_type` signal by a closed match on the `input` value — `return` a return flight, any other value one-way — over its two options (`one-way`, `return`) (examples/seven_guis/src/tasks/flight_booker.rs:66-89)
  (.andromeda/security-plan.md:165)
  > Example apps — the 7GUIs inputs, the widget's `color` attribute and the TodoMVC input are validated or fall back.

## Bindings
- **Open list's option ids and scroll targets ↔ error handling's graceful degradation**: a select or option a page removes or changes while its list is open must resolve through `get` to nothing and be skipped, which the missing-node scroll rule and the readers' `None` answers rest on, (.andromeda/security-plan.md:346)
- **Overlay geometry and option boxes ↔ error handling's panic paths**: the overlay's paint, hit and key code must read options that hold no layout while closed without reaching `layout_style()`, `layout_data` or an inline root's panic at paint, (.andromeda/security-plan.md:360) (.andromeda/security-plan.md:364)
- **Driver's validate path and restated `covered` reading ↔ error handling's typed refusal**: the restated reading and schema text must leave `Refusal` one of eight causes with fixed text and no `String` field, so it holds nothing a call supplied, (.andromeda/security-plan.md:343)
- **Snapshot's option `bounds` and file input masking ↔ data protection's local user data and committed checks**: the snapshot's change must keep a file path out of every carrier, and any effect on a committed check's digest is read through the strict reader, (.andromeda/security-plan.md:195) (.andromeda/security-plan.md:187)
- **Overlay over page-authored options ↔ the threat model's remote-content vector**: the validation rules above hold on remote pages because the list takes input and paints pixels on every document, (.andromeda/security-plan.md:11)
