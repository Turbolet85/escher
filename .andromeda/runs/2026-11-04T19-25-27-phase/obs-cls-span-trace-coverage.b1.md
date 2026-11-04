@answer n=1 entity=span-trace-coverage#1.1 v=yes
why: the probe may be kept under examples/seven_guis, adding a file to the 32-file examples census that must keep reading no span (task_item 6 and 11)
@answer n=2 entity=span-trace-coverage#1.2 v=no standing=S3
why: the apps slice no-span census, which the chunk adds no file to and does not enter
@answer n=3 entity=span-trace-coverage#1.3 v=no standing=S3
why: the s05 files no-span census, which the chunk does not enter
@answer n=4 entity=span-trace-coverage#1.4 v=no standing=S3
why: the s06 listed files no-span census, which the chunk adds no file to
@answer n=5 entity=span-trace-coverage#1.5 v=no standing=S3
why: the layout slice no-span census, which the chunk adds no file to; both layout modes are stand runs, not layout code
@answer n=6 entity=span-trace-coverage#1.6 v=yes
why: the census binds where the probe reads node ids through dioxus-native-dom (task_item 5), which no span may enter; the listed files are not named, so it is carried
@answer n=7 entity=span-trace-coverage#1.7 v=no standing=S3
why: the s09 listed files no-span census, which the chunk does not enter
@answer n=8 entity=span-trace-coverage#1.8 v=no standing=S3
why: the blitz-vibey-script and blitz slice, which the chunk reads only through dioxus-native-dom and adds no file to
@answer n=9 entity=span-trace-coverage#1.9 v=no standing=S3
why: the s11 listed files no-span census, which the chunk does not enter
@answer n=10 entity=span-trace-coverage#1.10 v=yes
why: the probe adds a file to the test crate slice, whose 61-file no-tracing census must record it (no subscriber, no env read, no print)
@answer n=11 entity=span-trace-coverage#1.11 v=no standing=S3
why: the wpt runner slice is outside the chunk's scope (not_needed) and is not entered
@answer n=12 entity=span-trace-coverage#2.1 v=yes
why: the scope names the driver command span with its eight fields, and the probe must add no field to it (task_item 6)
@answer n=13 entity=span-trace-coverage#2.2 v=yes
why: host_log's fourth test counts the hosted command span lines and reads 0 needles under both builds, which the scope requires to keep passing (task_item 10)
@answer n=14 entity=span-trace-coverage#2.3 v=yes
why: host_log's and mcp_log's select-list tests read 0 needles at trace, both named by the scope (task_item 10)
@answer n=15 entity=span-trace-coverage#2.4 v=yes
why: the upstream take adds no span, and host_log and mcp_log read 0 needles at trace under both builds, which the scope names (task_item 10)
@answer n=16 entity=span-trace-coverage#3.1 v=yes
why: the scope names the tool_call span with its tool and ending fields, which the probe must not add to (task_item 6)
@answer n=17 entity=span-trace-coverage#3.2 v=yes
why: the cold-agent pipe reads tool_call's two fields, which the scope protects and the probe must leave as they are (task_item 6)
