#!/bin/bash
CURDIR=$(cd $(dirname $0); pwd)
PROJDIR=$(dirname $CURDIR)
SOLNAME=$(printf "s%04d" $1)

cd ${PROJDIR}/leetcode
cargo new $SOLNAME
