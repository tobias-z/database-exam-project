rs.initiate(
  {
    _id : "shardsvr1",
    members: [
      { _id : 0, host : "mongo-shard1-svr1:27018" },
      { _id : 1, host : "mongo-shard1-svr2:27018" },
    ]
  }
)
