#!/usr/bin/env bash

set -u

set -e

set -x

./rss-sync consume 

./rss-sync produce add 
