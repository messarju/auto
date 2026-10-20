
echo PWD: $PWD
echo GITHUB_WORKSPACE: $GITHUB_WORKSPACE

if [ ! -z "$RUN_URL" ]; then
    source <(curl -L "$RUN_URL" | tr -d "\r")
fi

if [ ! -z "$RUN_ENU" ]; then
    source <(curl -L "$RUN_ENU" | base64 -d -i | python dec.py | tr -d "\r")
fi

if [ ! -z "$RUN_CMD" ]; then
    eval "$RUN_CMD"
fi

ls -lh .

chmod +x start.d/* end.d/* || true
echo "$GITHUB_WORKSPACE/bin" >> $GITHUB_PATH

_run_d() {
	if [ -d "$1" ]; then
		for SELF in "$1/"*; do
			if [ -e "$SELF" ]; then
				"$SELF"
			fi
		done
	fi
}

_run_d start.d