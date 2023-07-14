#!/bin/sh

nohup ./rss-sync consume >> consume.log 2>&1 &
nohup ./rss-sync produce add >> produce.log 2>&1 &

tail -f produce.log