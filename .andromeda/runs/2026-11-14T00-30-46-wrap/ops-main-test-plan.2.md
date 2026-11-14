@row_set n=1 row=decision/file-urls-only-for-file-documents cell=guard seen=2 detector=none
value: remote_document_cannot_load_file_urls, file_document_can_load_sibling_file_urls, local_files_for_local_documents
basis: fct-acglc6pk, fct-nfuvmyp2
because: the rule now covers the engine's sub-resource loads through blitz-net, and the new blitz-tests target that holds that half in both layout modes guards it beside the script-load checks.
