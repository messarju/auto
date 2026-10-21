if [ -f $WHAT ]; then
    [ -f /tmp/env.sh ] && . /tmp/env.sh  
    chmod +x ./$WHAT && exec ./$WHAT
elif [ -f $WHAT.sh ]; then
    [ -f /tmp/env.sh ] && . /tmp/env.sh  
    . $WHAT.sh
fi