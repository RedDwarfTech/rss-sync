#!/bin/sh

export MALLOC_CONF="prof:true,prof_prefix:jeprof.out"

nohup ./rss-sync >> rss-sync.log 2>&1 &

tail -f rss-sync.log