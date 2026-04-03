[ ! -f /tmp/env.sh ] || . /tmp/env.sh  
[ -f $WHAT.sh ] && . $WHAT.sh
if [ -f $WHAT ]; then
    chmod +x $WHAT && exec ./$WHAT
fi