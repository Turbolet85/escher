@row_add n=1 row=decision/cells-current-cell-selected area=surface-desktop-native fact=F1 detector=none
name: Cells current cell states selected
rule: The Cells task's current data cell alone states aria-selected true, set by the stand as an attribute, and an arrow key moves it to the next cell.
guard: stand_cells_served
basis: fct-voh3czmv, fct-gvhc5qds, fct-73upmmhj
because: The chunk made the Cells task's current data cell alone state aria-selected true by attribute, moved by an arrow key, and no row of the desktop-native surface says so.
