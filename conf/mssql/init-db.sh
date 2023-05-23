#!/usr/bin/env bash

/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P thisIsSuperStrong1234 -d master -i /create-database.sql
