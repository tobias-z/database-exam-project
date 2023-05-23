#!/usr/bin/env bash

print_help() {
    echo "Usage: sh start.sh [OPTIONS]

Options:
    -v  A specific root-app version. Default is 'latest'. Example: -v develop-0.1.0-snapshot
    -e  Expose all the databases for you to access outside. This requires that the ports are available.
    -h  Show this help menu.
"
}

while getopts v:eh flag
do
    case "${flag}" in
        v)
            version=${OPTARG}
            ;;
        e)
            exposed_dbs="-p 27017:27017 -p 7687:7687 -p 1433:1433"
            ;;
        h)
            print_help
            exit 0
            ;;
    esac
done

if [ -z $version ]; then
    echo "No version provided. Using the latest."
    if docker image ls | grep "tobiaszimmer/root-app" | grep latest; then
        echo "Found old latest version. Deleting to ensure we use the latest."
        docker image rm tobiaszimmer/root-app:latest
        if [ $? -eq 1 ]; then
            echo "Unable to remove old latest version of 'tobiaszimmer/root-app:latest'. Perhaps you need to stop an already running one."
            exit 1
        fi
    fi
    version="latest"
fi

docker run -d --name youbook --privileged $exposed_dbs -p 80:80 tobiaszimmer/root-app:$version
