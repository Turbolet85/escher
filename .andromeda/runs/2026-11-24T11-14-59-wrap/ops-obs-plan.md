@row_add n=1 row=decision/root-example-prints-not-telemetry area=log-coverage fact=F2 detector=D-obs-instrumentation
name: Root example prints are example stdout
rule: The println prints of the root examples are example stdout, not telemetry: they pass through no sink and are no log site.
basis: fct-pr3l6fxd (root examples, instrumentation n/a, stdout prints); fct-qbl64vse (the prints measured all in the dnd example); fct-pwiivh4c (obs-plan §Log Coverage carried)
because: The take brings root examples that print to stdout, and the plan's obs-plan amendment classes those prints as example stdout rather than uninstrumented telemetry, which no row yet says.
@row_add n=2 row=decision/dropped-data-never-logged area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
name: Dropped data never logged
rule: Dropped data (dropped file paths, dragged text and the contents of a file a Dioxus handler reads) is never logged: the OS drop source, the drag dispatch and the dropped-file reader write it to no log line, span or field.
basis: fct-pr3l6fxd (OS drop input and Dioxus dropped-file reader, PII n/a, nothing logged); fct-xuofjrvy (the drop source and the dropped-file reader)
because: The take adds a new class of user data, the dropped data, and no PII row yet says it never reaches a log.
@row_none n=1 fact=F1 type=7
what: The take adding no log site to the drop input path and the dropped-file reader is an absence inside single files; the code holds it.
