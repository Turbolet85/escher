@row_set n=1 row=decision/stand-boots-offline cell=except seen=1 detector=none
value: The stand load page's boot, with its self-answering LoadNet stand-in answering one fixed sheet after a delay, and the error-text check's failing page, whose FailingNet stand-in fails its fetch.
basis: fct-xm65aaqt, fct-wnvlf276, fct-7ba2mhup
because: The chunk adds a stand boot holding a failing network stand-in, so a load page is no longer the one stand boot that keeps a network provider.
@row_set n=2 row=decision/stand-load-page-fixed-answer cell=rule seen=1 detector=none
value: The stand load page's checks rely on its stand-in network answering the fixed URL with fixed bytes after a fixed delay, every other stand boot having no network provider but the error-text check's failing page, whose stand-in fails its fetch.
basis: fct-xm65aaqt, fct-wnvlf276, fct-7ba2mhup
because: The chunk adds a second stand boot with a network stand-in, so «every other stand boot having no network provider» no longer holds as written.
@row_add n=1 row=decision/doc-leg-gates-public-doc-edits area=9-ci-integration fact=F1 detector=none
name: Doc leg gates a public doc comment edit
rule: A chunk that edits a public doc comment lists the doc leg in its local gate block.
why: The fast leg runs no rustdoc, so a broken or private intra-doc link otherwise reads red only in CI's Documentation job.
basis: fct-v5o2v3e5, fct-valsxynb
because: The chunk's public doc comments linked a crate-private item, green on every local gate and red only in CI's Documentation job, and no test-plan row yet states that the doc leg gates such an edit.
