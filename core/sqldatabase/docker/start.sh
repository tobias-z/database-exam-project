#!/usr/bin/env bash
/opt/mssql/bin/sqlservr &
sh /run-initialization.sh
sleep infinity