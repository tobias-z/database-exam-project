package com.groupawesome.apigraphdb.repository;

import com.groupawesome.apigraphdb.model.Author;
import org.springframework.data.neo4j.repository.Neo4jRepository;
import org.springframework.data.neo4j.repository.query.Query;
import org.springframework.data.repository.query.Param;

import java.util.Collection;

public interface AuthorRepository extends Neo4jRepository<Author, String> {

    @Query("MATCH (b:Book)-[:WRITTEN_BY]->(a:Author)\n" +
            "RETURN a, collect(b) as books")
    Collection<Author> getAllAuthors();

    @Query("MATCH (a:Author)<-[w:WRITTEN_BY]-(b:Book)\n" +
            "WHERE a.name = $name \n" +
            "WITH a, w, collect(b) AS books\n" +
            "RETURN a{.name, books: books}")
    Collection<Author> getAuthorByName(@Param("name") String name);
}
