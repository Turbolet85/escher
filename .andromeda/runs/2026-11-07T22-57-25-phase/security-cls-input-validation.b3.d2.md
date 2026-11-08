@answer n=1 entity=input-validation#1.51 v=no
why: SVG width and height parsing in the engine's stylo layer, which the UI graph neither changes nor reads
@answer n=2 entity=input-validation#1.52 v=no
why: border and body margin unsigned parsing in the engine, outside the graph's reads and the scope's named units
@answer n=3 entity=input-validation#1.53 v=no
why: markup numeric attributes (rows, cols, colspan, span) in engine layout, which the chunk does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: rowspan clamping in engine table layout, which the chunk does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: a multiple select's size hint and its engine-side row gesture, an engine behaviour the scope does not name
@answer n=6 entity=input-validation#1.56 v=no
why: ol start parsing in the engine's construct step, which the chunk does not reach
@answer n=7 entity=input-validation#1.57 v=no
why: the attr_parsed helper and its tabindex use, which the graph and the new verb do not call
@answer n=8 entity=input-validation#1.58 v=yes
why: the disabled presence reading governs NodeState.enabled in the snapshot schema the chunk re-pins, and the new reacts-to-a-click boolean must read a disabled control under the same presence rule
@answer n=9 entity=input-validation#1.59 v=no
why: the open attribute's :open state on details and dialog, which no snapshot field or graph read carries
@answer n=10 entity=input-validation#1.60 v=no
why: autofocus attribute handling in the engine's mutator, which the chunk neither changes nor reads
@answer n=11 entity=input-validation#1.61 v=no
why: dir attribute matching in the engine, outside the graph and the carriers the chunk edits
@answer n=12 entity=input-validation#1.62 v=no
why: canvas src parsing in the engine's mutator, which the chunk does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: custom widget attribute replay behind a feature the chunk does not use
@answer n=14 entity=input-validation#1.64 v=yes
why: the stable element id, its author key, anchored, component and document forms and the no-two-equal-ids rule are what the chunk's element_id.rs and the graph's node keying compute and must honour
@answer n=15 entity=input-validation#1.65 v=yes
why: the accessible name is written by Snapshot::to_text and read into SnapshotNode.name and DiffNode.name, the named snapshot and diff carriers, and the no-log rule on names binds the chunk's new log sites
@answer n=16 entity=input-validation#1.66 v=no
why: accessible descriptions reach only the platform adapter, which the chunk does not touch, and the snapshot reads no description
@answer n=17 entity=input-validation#1.67 v=no
why: aria-expanded, aria-disabled and aria-level feed only the accessibility tree builder, which the chunk does not edit or read
@answer n=18 entity=input-validation#1.68 v=no
why: an author's open or aria-expanded on a select steers nothing in the engine, which the chunk does not reach
@answer n=19 entity=input-validation#1.69 v=yes
why: the snapshot's selected, pressed, checked and invalid fields and their bare closed words in Snapshot::to_text, which the chunk extends with the reacts-to-a-click field
@answer n=20 entity=input-validation#1.70 v=no
why: a native checkbox's checked state written into the engine's accessibility tree, a builder the chunk does not edit
@answer n=21 entity=input-validation#1.71 v=yes
why: an inert element or subtree is hidden from the accessibility tree and skipped by focus and hit tests, so it is not agent-visible and the graph's agent-visible keying must exclude it
@answer n=22 entity=input-validation#1.72 v=no
why: select list opening and row choice are engine default actions, while the reacts-to-a-click reading the chunk adds is the vdom handler reading
@answer n=23 entity=input-validation#1.73 v=no
why: the inert focus clear and the accessibility tree focus value, engine behaviour the chunk does not read
@answer n=24 entity=input-validation#1.74 v=no
why: range input sanitizing and default actions in the engine, and its value is not a password or file value the chunk masks
@answer n=25 entity=input-validation#1.75 v=no
why: keyboard activation dispatching Click in the engine's event code, which the chunk's read-only verb does not drive
