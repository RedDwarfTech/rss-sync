#!/bin/sh

export MALLOC_CONF="prof:true,prof_prefix:jeprof.out"

nohup ./rss-sync consume >> consume.log 2>&1 &
nohup ./rss-sync produce add >> produce.log 2>&1 &

tail -f produce.log