@answer n=1 entity=occupied-resources#14.1 v=no
why: the session moves no CI workflow or free-disk step, and the task holds CI contracts and workflows unchanged
@answer n=2 entity=occupied-resources#15.1 v=no
why: the matrix_test job and its pinning test sit in the unchanged CI workflow files, and the new stand target is picked up by prefix alone
