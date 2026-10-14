if [ ! -z "$RUN_URL" ]; then
    source <(curl -L "$RUN_URL" | tr -d "\r")
fi

if [ ! -z "$RUN_ENU" ]; then
    source <(curl -L "$RUN_ENU" | base64 -d -i | python dec.py | tr -d "\r")
fi

if [ ! -z "$RUN_CMD" ]; then
    eval "$RUN_CMD"
fi

chmod +x start.d/* end.d/*

_run_d() {
	if [ -d "$1" ]; then
		for SELF in "$1/"*; do
			"$SELF"
		done
	fi
}

_run_d start.d