package com.groupawesome.apigraphdb.repository;

import com.groupawesome.apigraphdb.model.Author;
import org.springframework.data.neo4j.repository.Neo4jRepository;
import org.springframework.data.neo4j.repository.query.Query;

import java.util.Collection;

public interface AuthorRepository extends Neo4jRepository<Author, Long> {

    @Query("MATCH (a:Author)-[w:WROTE]->(b:Book) return a,w,b")
    Collection<Author> getAllAuthors();
}
