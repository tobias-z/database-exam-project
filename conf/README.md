# Configuration

This directory contains the configuration of the production environment

## Usage

To create a service or database that should be run in production, do these steps:

- Create a directory named something that makes sense.
- Create a file called `docker-compose.yml` inside it which will include all services that this config should run.
- If desired, create a file called `config.yml` inside the same directory, which can contain various configuration properties. For a full list of these properties, please see section [Configuration Properties](#configuration-properties)

A common use case could be that you want to ensure that multiple services have been run before another pair of services should start.

An example of this, is the MongoDB setup. Here two directories have been created, one for the database itself, and one for the services. This allows us to use the `depends-on` property, in the `log-console` configuration.

# Configuration Properties

TODO: docs
