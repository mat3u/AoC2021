#!/bin/bash

MAX_NUM=$(ls -Ad aoc* | egrep -o "\d+" | sort -n | tail -1)
NEXT="aoc$(($MAX_NUM+1))"

mkdir $NEXT
cd $NEXT

cargo init

COOKIE="session=53616c7465645f5fa4980e242b83bfff7fc632d95130f3c80421e752dbc577c264e0012daa6d6a8d5bdad3f4afb12ba2"
URL="https://adventofcode.com/2021/day/$NEXT/input"

curl "$URL" -H "$COOKIE" > input.txt
touch sample.txt

