# Database Exam Project

## Table of Contents

- [Introduction](#introduction)
- [Usage](#usage)
  - [Container Interaction](#container-interaction)
  - [Application Interaction](#application-interaction)
  - [Postman](#postman)

## Introduction

Something

## Usage

To run the environment, all you need to do is run the start script in the root directory.

```bash
# -e to expose the databases for you to see whats in them
sh start.sh -e
```

This will run an alpine container which is also running docker. This container will be the one serving all our applications.

### Container Interaction

If you want to run command to see what's happening with the services running, this has to be done through the root app. You have two options:

- Run commands from your root terminal
```bash
docker exec youbook docker ps
```
- Exec into youbook and perform your actions from in there
```bash
docker exec -it youbook sh
```

### Application Interaction

Everything is exposed through a nginx proxy.

Applications are available through: `localhost:80/<service>/<uri-inside-service>`
Databases are available through: `localhost:{27017,7687,1433}` for either MongoDB, Neo4j or MSSQL

List of available services:

- logs (view for querying logs)
- log-api (API the logging view queries)

### Postman

To ease the process of executing a process, a postman collection has been made, and can be found [here](./documents/postman/youbook.postman_collection.json)
