# Auth Service

This is a wrapper around the auth library.

It supports grpc requests to make auth requests.

## Development

To develop the app, you will need to run the docker-compose.yml in the root of the project.

```bash
docker compose up -d
```

and then initialize the sql database with test data. For this there is an `init-test-db.sql` in the root of the project.