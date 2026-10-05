f=$(ls -t target/debug/deps/accessibility_roles-* | grep -v '\.d$' | head -1); s=$(stat -c %s "$f"); size -A "$f" | awk -v s="$s" '/^\.debug/{d+=$2} END{printf "debug %d of %d file bytes\n", d, s}'
