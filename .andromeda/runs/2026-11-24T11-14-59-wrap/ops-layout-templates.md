@row_add n=1 row=decision/desktop-window-os-drag-and-drop area=surface-desktop-native fact=F1 detector=D-layout-surface
name: Desktop window takes OS drag-and-drop
rule: The desktop window accepts OS drag-and-drop of text and files as dragenter, dragover, dragleave and drop events on the node under the drag, a dropped URI read as a file path only when it is a file URI.
why: Upstream's drop input taken as written, ratified.
basis: fct-xuofjrvy, fct-pr3l6fxd, fct-vixl64fe
because: The merge brings the desktop shell's OS drop source and the engine's drag events, so the desktop surface now takes drops and its rows must say how.
@row_add n=2 row=decision/custom-widget-accepts-drop-by-drop-effect area=surface-desktop-native fact=F1 detector=none
name: Custom widget accepts a drop by its drop effect
rule: A custom widget, such as the canvas widget, accepts a drop only by setting the dropEffect of the dragover event it receives; a widget that sets none refuses the drop.
why: Upstream's widget hook taken as written, ratified.
guard: widget_accepts_a_drop_by_setting_the_drop_effect
basis: fct-xuofjrvy, fct-vixl64fe
because: The merged widget hook makes dragover's dropEffect the one way a canvas-like widget on the desktop surface takes a drop, a rule future widgets must respect.
@row_none n=1 fact=F2 type=5
what: The root examples dnd and file_drop are an inventory of windowed example code, no command-line or product surface of layout-templates; their refused file-URL previews are a security-plan input rule, and the code holds the rest.
