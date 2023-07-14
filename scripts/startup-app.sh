#!/bin/sh

<<<<<<< HEAD
nohup ./rss-sync consume >> consume.log 2>&1 </dev/null &
nohup ./rss-sync produce add >> produce.log 2>&1 </dev/null &
=======
nohup ./rss-sync consume >> consume.log 2>&1 &
nohup ./rss-sync produce add >> produce.log 2>&1 &

tail -f produce.log
>>>>>>> 970c5ef12c8220e42619f3532cc5ee0df8f163d7
