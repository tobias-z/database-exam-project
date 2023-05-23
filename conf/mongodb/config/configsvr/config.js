rs.initiate(
  {
    _id: "config-replicaset",
    configsvr: true,
    members: [
      { _id : 0, host : "mongo-config1:27017" },
    ]
  }
)
