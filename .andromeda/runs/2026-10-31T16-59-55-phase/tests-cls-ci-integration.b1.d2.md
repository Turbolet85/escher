# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the a11y leg's fourteen named files include select_interaction, stand_act_select, stand_keyboard, stand_contrast and stand_accessibility_ids, which the chunk edits or restates, and the stand-repeat glob picks up any new stand_ file
ci-integration#2.1 · no · pseudo_element_flow_classification runs in the workspace test leg and agent-run run all with no stand_ prefix, and the chunk names no check over it
ci-integration#3.1 · no · the cargo cache keys of the test, a11y and stand-repeat jobs; the chunk re-reads only the free-disk step, not the cache
ci-integration#4.1 · yes · CI#41190771843's red on Stand checks repeated is the out-of-disk event the chunk's free-disk watch is kept on
ci-integration#5.1 · yes · the free-disk step's action SHA and inputs at ci.yml:311-316 are the step the chunk re-reads and watches
ci-integration#6.1 · yes · the step's cure is unproven, so the watch reads CI runs and cannot take the step as the cure
ci-integration#7.1 · no · the matrix job's free-disk step at ci.yml:392-394 is a different job's step, which the chunk does not re-read
ci-integration#8.1 · no · the apt-install bound of the package install steps is a separate step of the stand-repeat job; the chunk re-reads only the free-disk step
ci-integration#9.1 · yes · the chunk's bounds reads, overlay pixel checks and reduced-motion virtual-time checks run under both the workspace and the per-package builds, which differ in writing-mode and overlay scrollbars
ci-integration#10.1 · yes · the chunk's new unit and integration checks move these per-job result and passed counts, so the chunk's CI tallies must be recorded beside the stand counts
