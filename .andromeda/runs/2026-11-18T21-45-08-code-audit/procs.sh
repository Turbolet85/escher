n=0
if [ "$(uname -s)" = Linux ] && [ -e /proc/self/exe ]; then
  for p in /proc/[0-9]*; do
    i=${p#/proc/}
    for k in exe cwd; do
      t=$(readlink "$p/$k" 2>/dev/null) || continue
      for d in "$@"; do
        r=$(realpath -m "$d")
        case "$t" in
          "$r"/*|"$r"|"$r (deleted)")
            a=$(ps -o etime=,pcpu= -p "$i") || continue 3
            echo "$i · $(echo $a | sed 's/ / · /')% · $k $t"
            n=$((n + 1)); continue 3;;
        esac
      done
    done
  done
  echo "processes under the scratch dirs: $n"
else
  echo "processes: not read on this host ($(uname -s): no /proc)"
fi
