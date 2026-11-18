@row_add n=1 row=decision/release-requirements-witnessed-in-stand-repeat area=standard-contracts fact=F1 detector=none
name: Release requirements witnessed inside stand-repeat
rule: Every release requirement in a table of the stand requirement sweep names a witness test that the stand-repeat leg's arm runs, or is listed as having no screen witness; witness names are pinned to the current test functions, so a renamed test turns the sweep red.
guard: stand_requirement_sweep
basis: fct-dxmm2tfo, fct-dkn5ym6w, fct-efv5honp
because: The chunk adds the 0.3.0 table and makes the sweep read the stand-repeat arm itself with witness names pinned to current functions, a gate future release work must satisfy that no row of architecture states.
@row_set n=1 row=decision/masked-input-values cell=guard seen=1 fact=F2 detector=none
value: stand_masks_on_new_answers
basis: fct-cicl2hg4
because: The new stand check proves masked password and file values stay masked on the graph, actions and layout answers, kept checks and their replay, explored paths' checks and the MCP role, so it is the test that holds the masking decision.
