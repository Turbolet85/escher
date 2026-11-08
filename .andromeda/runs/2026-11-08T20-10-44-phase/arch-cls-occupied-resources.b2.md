@answer n=1 entity=occupied-resources#14.1 v=no
why: the free-disk step of the Stand checks repeated CI job; the chunk edits no workflow and adds no CI job or step, and its stand tests neither state nor must act on that disk fact
@answer n=2 entity=occupied-resources#15.1 v=no
why: the matrix_test job and the pin in test_ci_workflows.py are reached by no task file or scope item; the chunk adds no CI action step
