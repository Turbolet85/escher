@row_set n=1 row=decision/upstream-sync-dependency-set cell=rule seen=3 detector=D-security-deps
value: Each upstream sync takes upstream's dependency set whole or, by cherry-pick, a coupled registry set upstream moved, packages entering with it, or a lock-only semver-compatible update; git content only by Taffy's own pin; its lock read name by name, the audit leg green after it.
basis: fct-xfukctcw, fct-vaq3umam
because: The cherry-picked take brought the AnyRender 0.15 set from the registry with seven packages entering the lock, and the founder amended the rule in its standing form, so «enters no package» no longer holds.
@row_set n=2 row=decision/programmatic-scroll-clamped cell=rule seen=1 fact=F2 detector=none
value: Scroll targets and each node's scroll offset at final layout are clamped to the scroll range, the viewport scroll and any scroll animation on a device change; a removed subtree's scroll state is reset, and a programmatic scroll to a missing node does nothing.
basis: fct-y4zyhjnd, fct-ti3zu7r4
because: The take clamps every node's scroll offset at final layout, the viewport scroll and scroll animation on device change and resets a removed subtree's scroll state, a ratified engine change the row's narrower clamp says less of.
@row_retire n=1 row=decision/free-disk-only-on-stand-repeat seen=1 heir=decision/free-disk-on-test-and-stand-repeat detector=none
why: The default-features test job now frees runner disk too, and the guard the row names was renamed.
basis: fct-w2ipnyht, fct-wewkknlb
because: The free-disk step moved from one job to two, so a row keyed and ruled on the stand-repeat job alone is false and is replaced by its heir.
@row_add n=1 row=decision/free-disk-on-test-and-stand-repeat area=dependency-security fact=F3 detector=none
name: Runner disk freed for the test and stand-repeat jobs
rule: Only the default-features test job and the Stand checks repeated job free runner disk, with the free-disk-space action pinned to a commit; the test job reads df -h before and after its test leg into the step log only, with no upload.
guard: test_free_disk_only_on_test_and_stand_repeat, test_test_job_reads_its_disk
basis: fct-w2ipnyht, fct-wczmmtsv
because: The default-features test job gained the SHA-pinned free-disk step and two df -h readings kept in the step log, replacing the stand-repeat-only rule.
@row_add n=2 row=decision/svg-image-href-fetch area=input-validation fact=F1 detector=D-security-input
name: SVG image href fetched as a sub-resource
rule: An SVG image or feImage element's href, the unprefixed href else xlink:href, is fetched as an external image sub-resource through the stamping constructor, carrying its document as initiator and under the file-URL refusal; a use element's href is never fetched.
why: A boundary widening by upstream merge, ratified.
guard: every_engine_request_carries_its_document_as_initiator
basis: fct-y4zyhjnd, fct-ubjduri7, fct-thrm5ppp, fct-vaq3umam
because: The merge added a remote fetch of SVG image and feImage hrefs, validated by the stamp and the file-URL refusal, and no standing row names this widened input surface.
