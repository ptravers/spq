#!/bin/sh

set -e

cmd="$1"

sleep 3

>&2 echo "SPQ is up"
>&2 echo "Executing command $cmd"

exec $cmd
