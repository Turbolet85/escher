# Classified — security-plan · data-protection.b1

## Answers
data-protection#1.1 · yes · stale · the session adds a row's own `bounds` to the snapshot and diff, which a recording of an open list holds at rest, and the claim's reading list names no bounds
data-protection#2.1 · no · the Counter scripts open no list and take no shot, and the snapshot and diff gain `bounds` only on a list row (`NODE_FIELDS` stays thirteen), so the committed checks' recorded bytes do not move
