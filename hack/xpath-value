#!/usr/bin/env bash

xml=$1
path=$2
if [ -f "$xml" ]; then
  value=$(xpath -e "$path" "$xml" | perl -pe 's/^.+?\>//; s/\<.+?$//;')
  echo -n "$value"
else
  echo "Invalid xml file '$xml'!"
  exit 1;
fi
