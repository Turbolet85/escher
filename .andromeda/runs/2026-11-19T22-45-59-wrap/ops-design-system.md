@row_add n=1 row=decision/inline-box-placeholder-font-fallback area=typography fact=F1 detector=none
name: Inline box placeholder font fallback
rule: An inline box inside text that no available font can shape is laid out with a private engine fallback family, the bullet font, registered in every font context a document uses, a given one included; where a font exists the layout does not move.
why: Fixes an upstream defect where the placeholder left such an inline box unplaced; a given font context never registers the bullet font, so the fallback is registered there too. Flagged upstreamable.
guard: an_inline_block_in_text_no_font_can_shape_is_placed_once_its_own_font_loads
basis: fct-xcqgt2ri, fct-iw37e7va, fct-douoqnv6, fct-rvkwe2xo
because: The chunk adds an engine fallback family for inline boxes in unshapeable text, a ruled difference from upstream that typography holds no row for.
@row_add n=2 row=name/other/blitz-inline-box-placeholder area=typography fact=F1 detector=none
class: other
name: blitz-inline-box-placeholder
basis: fct-xcqgt2ri
because: The chunk occupies a private engine font family name in every font context, which no row of typography registers.
@row_add n=3 row=decision/chrome-devtools-opt-in area=surface fact=F2 detector=none
name: Chrome devtools are opt-in
rule: The Chrome devtools, a CDP server listening on 127.0.0.1 with no authentication, the devtools frontend, the element picker and the open-devtools keyboard shortcuts, are built only under the opt-in devtools feature, which no default feature set turns on.
why: The listener checks no Origin and asks no authentication.
basis: fct-xcqgt2ri, fct-wakes6es
because: The merge brings the Chrome devtools surface, ruled opt-in and left out of every default feature set, which the surface holds no row for.
@row_add n=4 row=decision/skia-default-renderer area=surface fact=F3 detector=none
name: Skia as the default renderer
rule: The browser app and the WPT runner render with Skia by default.
basis: fct-wakes6es, fct-wogrtpli
because: The chunk makes Skia the default renderer of the browser app and the WPT runner, a technology chosen for a role that no row of the surface states.
@row_add n=5 row=decision/wheel-gesture-axis-lock area=motion fact=F4 detector=none
name: Wheel gesture locks to its dominant axis
rule: A wheel gesture scrolls along its dominant axis only, a small first movement not deciding the axis and each gesture deciding its own; wheel events outside a gesture scroll both axes, and overscroll-behavior contain stops scroll chaining per axis.
guard: wheel_axis_lock, overscroll_behavior
basis: fct-trpvqtky, fct-6fjqftwn, fct-6cioon5h, fct-6bxyx7n5, fct-aa4jbdn2, fct-a7ke2r6n
because: The merge brings wheel axis locking and per-axis scroll chaining, a scrolling behaviour the motion rows do not state.
@row_add n=6 row=decision/cancelled-pointer-events-keep-gestures area=surface fact=F5 detector=none
name: Cancelled pointer events keep click and drag
rule: A pointerup a handler cancels still clicks and still ends a scrollbar thumb drag; a cancelled pointerdown still counts towards a double click but selects no text.
guard: pointerup_cancelled, pointerdown_cancelled
basis: fct-ohaxmeez, fct-7kzh3vb4, fct-7jasdaqu, fct-7yl25uuk
because: The chunk takes the cancelled pointerup click and the cancelled pointerdown double click as ruled (W7), a gesture rule the surface holds no row for.
@row_add n=7 row=decision/visibility-collapse-hidden-from-a11y area=surface fact=F6 detector=none
name: visibility collapse hidden from the accessibility tree
rule: An element with visibility collapse is left out of the accessibility tree.
guard: excludes_button_with_visibility_collapse
basis: fct-ohaxmeez, fct-bhfcjucw
because: The chunk takes visibility collapse as hidden from the accessibility tree as ruled (W9), which no row of the surface states.
@row_add n=8 row=decision/programmatic-focus-fires-focus-events area=surface fact=F7 detector=none
name: Programmatic focus fires focus events
rule: A programmatic focus change fires blur, focusout, focus and focusin.
except: The inert fix-up's focus clear and the driver's select focus fire no focus event.
guard: programmatic_focus_changes_dispatch_focus_events
basis: fct-xcqgt2ri, fct-trpvqtky, fct-azzvypea
because: Programmatic focus now fires the four focus events with two named silent exceptions (W5), which no row of the surface states.
@row_add n=9 row=contract/dom-events/is-trusted area=surface fact=F7 detector=none
boundary: DOM events
item: isTrusted
promise: An event the user agent dispatches reads isTrusted true; an event a script dispatches reads isTrusted false.
basis: fct-trpvqtky, fct-ov42vbyr
because: Script-dispatched events now read isTrusted false (W6), a promise to page scripts that the surface holds no row for.
