#!/bin/sh

stop=0
until [ $stop -eq 1 ]
do
    sleep 2
    diesel migration run
    if [ $? -eq 0 ]; then
        stop=1
    fi
done