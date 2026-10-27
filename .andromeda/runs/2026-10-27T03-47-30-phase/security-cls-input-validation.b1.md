# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the example binaries' argv handling is not needed, the chunk leaves the `screenshot` URL argument untouched
input-validation#1.2 · no · the example binaries' numeric argument fallbacks are not needed, the examples' argv is untouched
input-validation#1.3 · no · the screenshot example's output filename is not needed, the examples' argv is untouched
input-validation#1.4 · no · the `preact_script` path check is not needed, the examples' argv is untouched
input-validation#1.5 · no · `agent-run.sh`'s argument checks are untouched and the chunk names none of them
input-validation#1.6 · yes · the fixed STAND_PROMPT (six apps, one screenshot and one visual lint goal, no element id or app slug) and the `claude` precondition are what the 2026-10-25 run sent the model; its argv allowlist is not needed
input-validation#1.7 · no · `apt-install.sh`'s argument checks are untouched and the chunk names none of them
input-validation#1.8 · no · `coverage_floor.py`'s argument checks are untouched and the chunk names none of them
input-validation#1.9 · no · the agent-written check's bounds and replay are not needed (the script is neither changed nor re-run) and the restated run's crossing is outbound only
input-validation#1.10 · yes · the Unix-domain socket's owner-only, unix-only wire that the driver paragraph describes, and the ids, masked control values, snapshot text, diffs and lint findings its answers carry
input-validation#1.11 · yes · the state directory's `0700` mode, the `0600` socket file and the refusal of a group- or other-open directory, which the README's owner-only socket claim rests on
input-validation#1.12 · yes · the recording directory's `0700` mode, the `0600` `create_new` check files and the nothing-logged rule, which the kept-checks-at-rest claim rests on
input-validation#1.13 · yes · the replay half, `Check::replay` stopping at the first differing step with bools and counts only, is the "replayable check" the scope names; the strict reader stays untouched
input-validation#1.14 · no · the session label's bound is named neither by the driver paragraph nor by the statement's Needs, and no code changes
input-validation#1.15 · yes · the twelve verbs, seven argument kinds and eight refusal causes the statement's pin counts, plus the lint and load-wait rules the README names
input-validation#1.16 · yes · the `escher-session` command line's one JSON line on stdout, exit statuses 0 · 1 · 2 · 3, nothing the caller typed written back, and the `--record` refusal the Try-it section may show
input-validation#1.17 · yes · the MCP stdio server as a child of the spawning client with no listener, and its results' ids, names, masked values, snapshot text, diffs, screenshot block and lint findings
input-validation#1.18 · no · the reference browser's urlbar parsing is not reached, the browser app being untouched and unnamed by the scope
input-validation#1.19 · no · the external-navigation method and scheme rules are not named by the scope, and the chunk changes no navigation code
input-validation#1.20 · no · link `href` resolution is in-document navigation, untouched by the chunk
input-validation#1.21 · no · the document-URL parse error is a typed engine error, untouched (Not needed: engine panic paths and typed errors)
input-validation#1.22 · no · fragment decoding is in-document scrolling, untouched by the chunk
input-validation#1.23 · no · `file:` URL reads are remote-content and net-provider handling, which the chunk does not reach (Not needed)
input-validation#1.24 · no · `data:` URL decoding in the net provider is remote-content handling, which the chunk does not reach (Not needed)
input-validation#1.25 · no · the shell's `data:` provider is remote-content handling, which the chunk does not reach (Not needed)
