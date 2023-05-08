#!/usr/bin/env bash

version=$1

if [ -z $version ]; then
    echo "No version provided. Using the latest."
    if docker image ls | grep "tobiaszimmer/root-app" | grep latest; then
        echo "Found old latest version. Deleting to ensure we use the latest."
        docker rm tobiaszimmer/root-app:latest
        if [ $? -eq 1 ]; then
            echo "Unable to remove old latest version of 'tobiaszimmer/root-app:latest'. Perhaps you need to stop an already running one."
            exit 1
        fi
    fi
    version="latest"
fi

docker run \
    -d \
    --name youbook \
    --privileged \
    -p 80:80 \
    -p 27017:27017 \
    tobiaszimmer/root-app:$version
