_run_d() {
	if [ -d "$1" ]; then
		for SELF in "$1/"*; do
			"$SELF"
		done
	fi
}

_run_d end.d
python3 .github/workflows/relup.py