#!/usr/bin/env sh

docker-compose exec mongo-config1 sh -c "mongosh --port 27017 < /scripts/mongo-init.js"
docker-compose exec mongo-shard1-svr1 sh -c "mongosh --port 27018 < /scripts/mongo-init.js"
docker-compose exec mongo-shard2-svr1 sh -c "mongosh --port 27019 < /scripts/mongo-init.js"
sleep 10
docker-compose exec mongo-router-1 sh -c "mongosh < /scripts/mongo-init.js"
