d=$1; shift
cg=/sys/fs/cgroup$(sed -n 's/^0:://p' /proc/self/cgroup)
sed -n 's/^anon //p' "$cg/memory.stat" > "$d/peak.samples"
( while :; do sed -n 's/^anon //p' "$cg/memory.stat"; sleep 0.2; done >> "$d/peak.samples" ) &
s=$!
"$@" > "$d/peak.out" 2>&1; rc=$?
kill $s
echo "rc=$rc anon_peak=$(sort -n "$d/peak.samples" | tail -1)"
