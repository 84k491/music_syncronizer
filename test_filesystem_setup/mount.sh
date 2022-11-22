#!/bin/bash

for dir in $(cat $1)
do
    mount -t tmpfs tmpfs $dir -o size=1m
    if [ $? -eq 0 ]; then
        echo "Tmpfs mounted on "$dir" successfully"
    else
        echo "Mounting tmpfs on "$dir" failed!"
        exit 1
    fi
done
