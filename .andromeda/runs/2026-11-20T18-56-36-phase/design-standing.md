# design · standing conventions

- S1 · each new stand check of a driver verb runs the verb through every carrier it rides — session socket, command line, MCP — and reads its answer as an agent would · surface
- S2 · each new usage error or refusal takes its exit status from the schema's table and its message on stderr in the stated error shape · surface · surface-cli
- S3 · each new command line output line is agent-read, plain and without colour · surface-cli
  contract/escher-session-commands/screenshot · escher-session commands · screenshot [--id] — screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
