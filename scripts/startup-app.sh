#!/bin/sh

nohup ./rss-sync consume >> consume.log </dev/null &
nohup ./rss-sync produce add >> produce.log </dev/null &