# design · standing conventions

- S1 · each new escher-session argument or command answer follows the command table, the help self-description and the MCP role's result conventions · surface
  contract/escher-session-commands/actions-arguments · escher-session commands · actions arguments — actions takes no argument; an argument is refused malformed.
  contract/escher-session-commands/coverage-arguments · escher-session commands · coverage arguments — coverage's one shape is coverage <app> <check>..., an app the binary boots then one or more check paths, with no --session and no --record; any other shape is a usage ending, status 2.
  contract/escher-session-commands/graph-arguments · escher-session commands · graph arguments — graph takes no argument; an argument is refused malformed with nothing run.
  contract/escher-session-mcp/other-results · escher-session mcp role · other tool results — Any other tool result's text is the command's JSON line.
  contract/escher-session-mcp/screenshot-result · escher-session mcp role · screenshot tool result — A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
- S2 · each new usage or session error a new argument can raise carries its exit status from the schema's table and its stderr line shape · surface-cli
- S3 · each new run-start configuration a test or harness passes is applied under the one virtual clock, never the host's time · motion-calibrated-to-expression-level-unrecorded · motion
