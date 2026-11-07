@answer n=1 entity=depth-strategy#6.2 v=no
why: the absence of shadow or elevation tokens in the s08 files is a fact the buffered render never reads or changes, and the task marks depth values not needed
@answer n=2 entity=depth-strategy#6.3 v=no
why: the chunk's files (dioxus_document.rs, the mutation buffer, the mutation writer, the runtime probe) are outside the s10 slice, and no shadow or stacking behaviour moves through the render path
@answer n=3 entity=depth-strategy#6.4 v=no
why: the s11 search matched only a touch-point radius, and the chunk changes no radius, shadow or z-index, so nothing here binds the mutation-apply change
