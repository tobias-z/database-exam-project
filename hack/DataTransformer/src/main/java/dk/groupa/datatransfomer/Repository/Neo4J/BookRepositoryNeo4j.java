package dk.groupa.datatransfomer.Repository.Neo4J;

import dk.groupa.datatransfomer.Model.Neo4J.Book;
import org.springframework.data.neo4j.repository.Neo4jRepository;

public interface BookRepositoryNeo4j extends Neo4jRepository<Book, Long> {
}
