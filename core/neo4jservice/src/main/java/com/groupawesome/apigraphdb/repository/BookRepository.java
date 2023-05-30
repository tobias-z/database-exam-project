package com.groupawesome.apigraphdb.repository;

import com.groupawesome.apigraphdb.model.Book;
import org.springframework.data.neo4j.repository.Neo4jRepository;
import org.springframework.data.neo4j.repository.query.ExistsQuery;
import org.springframework.data.neo4j.repository.query.Query;
import org.springframework.data.repository.query.Param;

import java.util.Collection;

public interface BookRepository extends Neo4jRepository<Book, Long> {

    @Query("MATCH (b:Book)-[w:WRITTEN_BY]->(a:Author) return b,w,a")
    Collection<Book> getAllBooks();

    @Query("MATCH (b:Book)-[w:WRITTEN_BY]->(a:Author)\n" +
            "RETURN b\n" +
            "ORDER BY b.avgRating DESC\n" +
            "LIMIT $limitNum")
    Collection<Book> getTopRatedBooks(@Param("limitNum") Integer limitNum);
}
