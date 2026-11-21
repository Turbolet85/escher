The operator's answer to the P4 technical fork, by question dialog:

Q: Where should the new checks for a sized single select (a single select with size above 1 shown as a list box, no popup) go?
A: «Extend existing (Recommended)» — add the engine checks to select_interaction.rs, the driver checks to stand_act_select.rs, and one no-popup row to select_popup.rs; no leg membership, pin or capabilities.md count moves.
