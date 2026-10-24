# Classified — security · input-validation.b4

## Answers
input-validation#1.65 · yes · the single-choice select's arrows, Home and End, typeahead and choice channels, which the new flight-booker select row and stand_act_select joining the a11y leg both assert
input-validation#1.66 · yes · the snapshot's fixed mask for a password's or file input's value, which every value assertion the chunk adds must keep masked
input-validation#1.67 · no · aspect-ratio coercion is replaced-element layout, which no Tab, activation or contrast row reads and layout/replaced.rs is not reached
input-validation#1.68 · no · the inline-box height clamp is line-breaker layout, which no Tab, activation or contrast row reads
input-validation#1.69 · no · text_extent's one non-test caller is Session::lint, and the chunk edits no lint or screenshot check
input-validation#1.70 · no · geometry culling is a paint-only rule, and the chunk paints nothing since the contrast reader reads computed styles
input-validation#1.71 · no · gradient length clamping is a paint-only rule, and the contrast reader already reads None for any background image
input-validation#1.72 · no · border-radius scaling is a paint-only rule, and the chunk reads no box geometry
input-validation#1.73 · no · the hidden-input rule only skips painting type=hidden inputs, and no focus order or contrast row reads paint
input-validation#1.74 · no · the scroll-offset clamp is scrolling.rs's, and the chunk asserts focus, activation, select keys and contrast, never a scroll offset
input-validation#1.75 · no · method and enctype set a submission's transport and encoding, which no Needs item names, while the select row asserts what leaves rather than how
input-validation#1.76 · yes · the select row's form-submission assertion needs the select to be a submitted control, which this rule decides by excluding disabled controls
input-validation#1.77 · no · CRLF normalisation changes only line breaks inside a submitted value, and the chunk's submitted values are a select option value and single-line text fields
input-validation#1.78 · no · the accept attribute is a file-picker rule, and no Needs item opens a file dialog
input-validation#1.79 · no · NativeFormData::valid always returning true is a standing gap, and the chunk's rows read submitted values and the dispatched click, not form validity
input-validation#1.80 · no · a data-dioxus-id parse failure needs a malformed attribute that the renderer never writes, and the chunk addresses controls by author key
input-validation#1.81 · no · dangerous_inner_html is a raw-HTML path no Needs item names, and the chunk asserts no inner HTML
input-validation#1.82 · no · an ignored unsupported attribute value type moves no Tab, activation or contrast reading the chunk asserts
input-validation#1.83 · yes · a falsy stand value clears disabled, inert or selected instead of writing it, which the enabled-state Tab sequences and the presence-read disabled and inert gates rest on
input-validation#1.84 · no · the Response status RangeError is a JS API rule, and the stand runs no document script
input-validation#1.85 · no · the fetch method TypeError is a JS API rule, and the stand runs no script and fetches nothing offline
input-validation#1.86 · no · the JS selector SyntaxError is blitz-vibey-script's, separate from the Rust harness selector Result in the Needs, and no JS selector is written
input-validation#1.87 · no · the scroll-option TypeError is a JS API rule, and the stand runs no document script
input-validation#1.88 · no · width and height reflection is a JS DOM rule, and the stand runs no document script
input-validation#1.89 · no · the timer delay rule is the JS runtime's, and the stand's Timer steps by the Rust timer_step, not a script timer
