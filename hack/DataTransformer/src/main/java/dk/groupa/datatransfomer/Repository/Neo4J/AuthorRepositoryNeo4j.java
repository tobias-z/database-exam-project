package dk.groupa.datatransfomer.Repository.Neo4J;

import dk.groupa.datatransfomer.Model.Neo4J.Author;
import org.springframework.data.neo4j.repository.Neo4jRepository;
import org.springframework.data.neo4j.repository.query.Query;
import org.springframework.data.repository.query.Param;

public interface AuthorRepositoryNeo4j extends Neo4jRepository<Author, String> {

    @Query("MATCH (a:Author {name: $author}), (b:Book {author: a.name}) MERGE (b) - [:WRITTEN_BY] -> (a)")
    void createRelations(@Param("author") String author);
}
