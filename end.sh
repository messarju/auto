shopt -s nullglob
_run_d() {
	if [ -d "$1" ]; then
		for SELF in "$1/"*; do
			if [ -e "$SELF" ]; then
				"$SELF"
			fi
		done
	fi
}

_run_d end.d
python3 .github/workflows/relup.py