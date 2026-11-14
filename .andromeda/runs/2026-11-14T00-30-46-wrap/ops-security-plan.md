@row_set n=1 row=decision/file-url-only-for-file-documents cell=rule seen=1 detector=D-security-input
value: A file: URL loads only for a file: document or a top-level load; for any other document, remote pages included, JS fetch, module import and script src refuse it with a FetchError, and blitz-net refuses the engine's stylesheet, @import, font, image and iframe loads before any disk read.
basis: fct-z4iggkni, fct-oaky27yp, fct-acglc6pk
because: blitz-net's file: read now refuses every engine sub-resource load whose loading document is not a file: document, so the rule that named script loads alone is incomplete.
@row_set n=2 row=decision/file-urls-only-from-file-documents cell=rule seen=1 fact=F1 detector=D-security-input
value: A script fetch, module import or script src, and every engine stylesheet, @import, font, image or iframe load, may read a file: URL only for a file: document or a top-level load, in every document, remote pages included; blitz-net refuses any other before a disk read.
basis: fct-z4iggkni, fct-oaky27yp, fct-acglc6pk, fct-a3d6v75h
because: Every engine sub-resource request now carries its loading document and blitz-net reads a file: URL only for no loading document or a file: one, so the rule stating script loads alone must widen to them.
@row_add n=1 row=decision/one-stamping-request-constructor area=input-validation fact=F2 detector=D-security-input
name: One stamping constructor for sub-resource requests
rule: blitz-dom builds a sub-resource request only through the one constructor that stamps its loading document as the request's initiator.
why: Upstream's blitz-dom has no such lint config, so the rule is a new upstream merge surface.
guard: blitz-dom's clippy disallowed-methods rule on Request::get
except: The stamping constructor itself, under a single expect.
basis: fct-vvhzkhrr, fct-z4iggkni, fct-7joq2qn6
because: A crate clippy rule now forbids building a request in blitz-dom outside the stamping constructor, a validation gate no row states.
@row_add n=2 row=contract/blitz-net-provider-error/file-forbidden area=error-handling fact=F3 detector=none
boundary: blitz-net provider errors
item: FileForbidden
promise: The refusal of a file: load for a non-file document has the fixed display text «file: load refused for a non-file document», holding no URL or path, and reaches the handler only through the existing error path with no log site of its own.
basis: fct-z4iggkni, fct-nfuvmyp2
because: blitz-net's typed error set gains a refusal whose text is promised fixed and path-free, a contract at the provider boundary no row holds.
@row_none n=1 fact=F4 type=9
what: The favicon probe and remote-to-file: navigation still reading local files are owed to a provisional route entry awaiting the founder; the route holds it.
@row_none n=2 fact=F5 type=5
what: One more blitz-tests target reading CARGO_TARGET_TMPDIR at build time for a test-only fixture is an inventory of env reads; the leaf rule file holds it.
