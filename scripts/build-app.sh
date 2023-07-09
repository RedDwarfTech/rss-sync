#!/usr/bin/env bash

set -u

set -e

set -x

docker build -f Dockerfile.static-link -t reddwarf-pro/rss-sync:1.0.0 .