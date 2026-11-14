@row_set n=1 row=decision/file-urls-only-for-file-documents cell=rule seen=1 detector=D-tests-coverage
value: A `file:` URL is loaded only for a `file:` document: a script's `fetch()`, module `import` or `<script src>` is refused with a `FetchError` otherwise, and an engine sub-resource load (stylesheet, `@import`, font, image, iframe) through blitz-net reads no local file for a non-`file:` document.
basis: fct-acglc6pk, fct-z4iggkni, fct-oaky27yp
because: blitz-net now refuses a `file:` read for any engine sub-resource request whose loading document is not a `file:` document, so the rule naming script loads alone is incomplete.
@row_set n=2 row=decision/png-decoding-in-checks cell=rule seen=1 detector=none
value: A blitz-tests check decodes or encodes a PNG through the png dev-dependency; seven_guis, which has none, reads it with a hand base64 reader and the PNG header.
basis: fct-w5mamvxv
because: a blitz-tests check now also encodes a PNG through the png dev-dependency, so the row stating its role as decoding alone is incomplete.
@row_add n=1 row=decision/sub-resource-requests-carry-their-document area=integration-test-strategy fact=F1 detector=D-tests-coverage
name: Sub-resource requests carry their loading document
rule: Every engine sub-resource request (linked stylesheet, its reload, `@import`, `@font-face`, CSS image, img, iframe) carries its loading document's URL, checked in both layout modes; a navigation or an embedder's top-level load carries none.
guard: every_engine_request_carries_its_document_as_initiator
basis: fct-z4iggkni, fct-a62c3pvf
because: the chunk stamps every engine sub-resource request with its document and pins it with an integration check in both layout modes, a rule no row of the area states.
@row_add n=2 row=decision/blitz-dom-requests-through-stamping-constructor area=ci-integration fact=F2 detector=none
name: blitz-dom builds requests through one stamping constructor
rule: blitz-dom builds a network request only through its one stamping constructor, enforced by a crate-level clippy disallowed-methods config and one expect, so the clippy leg fails on any other construction.
why: Upstream's blitz-dom has no such clippy config, a new merge surface.
guard: clippy leg
basis: fct-vvhzkhrr, fct-7joq2qn6, fct-di5ejpmu
because: a new crate-level clippy config makes the clippy leg guard blitz-dom's request construction, a gate the CI rows do not state.
@row_add n=3 row=name/other/local-files-for-local-documents area=test-data-fixtures fact=F3 detector=none
class: other
name: local_files_for_local_documents under CARGO_TARGET_TMPDIR
basis: fct-ro7lwrlf, fct-oaky27yp
because: the new blitz-tests target occupies a test-only directory of its name under CARGO_TARGET_TMPDIR, an occupied name no row registers.
@row_add n=4 row=decision/real-provider-inside-stand-in area=mocking-stubbing-discipline fact=F4 detector=D-tests-framework
name: Real network provider inside a hand-written stand-in
rule: A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, with blitz-net and tokio taken as blitz-tests dev-dependencies for it.
guard: a_remote_page_reads_no_local_file_and_a_file_page_reads_them_all
basis: fct-w5mamvxv, fct-c75k7rws
because: the chunk's witness puts blitz-net's real provider behind a hand-written stand-in on its own runtime, a stubbing convention the discipline rows do not state.
