#!/usr/bin/env bash

set -u

set -e

set -x

nohup ./rss-sync consume > consume.log &!

nohup ./rss-sync produce add > produce.log &!
