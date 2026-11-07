@answer n=1 entity=visual-design-verification#3.8 v=no
why: the 2px focus outline is blitz-dom default stylesheet paint, which the chunk neither changes nor asserts; its focus checks read engine focus, not the outline
@answer n=2 entity=visual-design-verification#3.9 v=no
why: the forced-colors focus outline is the google fixture's own paint; the chunk touches no fixture or style
@answer n=3 entity=visual-design-verification#3.10 v=no
why: the password caret and its scroll paint are blitz-paint output; the scope's focus moves name no caret or paint
@answer n=4 entity=visual-design-verification#4.1 v=no
why: the BBC fixture's reduced-motion queries; no fixture or motion path changes
@answer n=5 entity=visual-design-verification#5.1 v=no
why: the absence of motion and colour-scheme rules over the s02 files; the chunk adds no motion or stylesheet rule
@answer n=6 entity=visual-design-verification#5.2 v=no
why: motion on an open select list; the chunk changes no select and the pinned test is a select popup check outside the scope
@answer n=7 entity=visual-design-verification#6.1 v=no
why: disabled icon button opacity in apps/browser; the browser app is not touched
@answer n=8 entity=visual-design-verification#6.2 v=yes
why: the replay routes set-attribute and removal of the open attribute through MutationWriter, and open_state's mutator write and removal reads that path; whether open is among the 28 boolean attributes is left open by the scope, so this is a fork and binds
@answer n=9 entity=visual-design-verification#6.3 v=no
why: an open select's list is no :open state; the chunk changes no select or open-state code
@answer n=10 entity=visual-design-verification#6.4 v=no
why: the WPT runner's status word print; the wpt runner is not touched
@answer n=11 entity=visual-design-verification#6.5 v=no
why: the group header's synthesized bold in a select list is paint; no select paint changes
@answer n=12 entity=visual-design-verification#6.6 v=no
why: a comment in examples/form.rs; no example changes
@answer n=13 entity=visual-design-verification#7.1 v=no
why: the viewport's colour scheme and zoom fields in blitz-traits; the chunk changes no shell or viewport value
@answer n=14 entity=visual-design-verification#7.2 v=no
why: the colour-scheme device change in stylo_device; the chunk changes no device path
@answer n=15 entity=visual-design-verification#7.3 v=no
why: hidpi and zoom scale tracking in stylo_device; the scope does not name scale
@answer n=16 entity=visual-design-verification#7.4 v=no
why: keyboard page zoom in the blitz-shell window; the windowed launch renders through the buffer but the scope names no zoom key
@answer n=17 entity=visual-design-verification#7.5 v=no
why: rdme's prefers-color-scheme CSS in apps/readme; not touched
@answer n=18 entity=visual-design-verification#7.6 v=no
why: the TodoMVC and reference page colour-scheme declarations; no example page changes
@answer n=19 entity=visual-design-verification#7.7 v=no
why: the inverted-colors rule in the blitz-dom default stylesheet; no paint or stylesheet change
@answer n=20 entity=visual-design-verification#7.8 v=no
why: touch-action panning in the pointer events; the scope names no pointer path
@answer n=21 entity=visual-design-verification#8.1 v=yes
why: the scope names inert: the mutator flush's focus fixup for a node under inert and the tree the replayed buffer refreshes must keep the inert exclusion, and removing inert must restore the tree
@answer n=22 entity=visual-design-verification#8.2 v=no
why: visibility hidden and collapse hit-testing; the scope names no visibility or hit rule
@answer n=23 entity=visual-design-verification#8.3 v=no
why: the non-invertible transform hit rule; the scope does not name transforms or hit testing
@answer n=24 entity=visual-design-verification#8.4 v=no
why: the pointer-events none hit rule; the scope does not name pointer events
@answer n=25 entity=visual-design-verification#8.5 v=no
why: scrollbar-width none paint suppression; the scope's MountedData scroll calls do not name scrollbar paint
