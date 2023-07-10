#!/bin/sh

nohup ./rss-sync consume 2>&1 >> consume.log </dev/null &
nohup ./rss-sync produce add 2>&1 >> produce.log </dev/null &