#!/bin/sh -e

CURDIR=$(cd $(dirname $0); pwd)

exec $CURDIR/back-end

