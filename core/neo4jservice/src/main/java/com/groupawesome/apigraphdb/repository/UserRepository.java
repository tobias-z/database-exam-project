package com.groupawesome.apigraphdb.repository;

import com.groupawesome.apigraphdb.model.User;
import org.springframework.data.neo4j.repository.Neo4jRepository;
import org.springframework.data.neo4j.repository.query.Query;

import java.util.Collection;

public interface UserRepository extends Neo4jRepository<User, Long> {

    @Query("MATCH (u:User)-[r:RATED]->(b:Book) return u,r,b")
    Collection<User> getAllUsers();
}
