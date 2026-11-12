@row_add n=1 row=contract/command-span/cause area=span-trace-coverage rec=2,1
boundary: The driver's command span
item: cause
promise: Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
@row_add n=2 row=contract/command-span/busy area=span-trace-coverage rec=2,1
boundary: The driver's command span
item: busy
promise: One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
@row_add n=3 row=contract/tool-call-span/tool area=span-trace-coverage rec=3,1
boundary: The MCP tool_call span
item: tool
promise: The verb table's word for the tool, one of its fourteen; unrecorded for an unknown tool.
@row_add n=4 row=contract/tool-call-span/ending area=span-trace-coverage rec=3,1
boundary: The MCP tool_call span
item: ending
promise: One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).
@row_add n=5 row=contract/coverage-line/shape area=pii-scrubbing-compliance rec=5,4
boundary: The handler coverage line (escher-session coverage)
item: shape
promise: Written by the crate's hand writer as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, a command line answer never logged or fielded.
@row_add n=6 row=contract/coverage-line/owner area=pii-scrubbing-compliance rec=5,4
boundary: The handler coverage line (escher-session coverage)
item: owner
promise: `owner` is left out when absent.
