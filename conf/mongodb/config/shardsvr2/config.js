rs.initiate(
  {
    _id : "shardsvr2",
    members: [
      { _id : 0, host : "mongo-shard2-svr1:27019" },
      { _id : 1, host : "mongo-shard2-svr2:27019" },
    ]
  }
)
