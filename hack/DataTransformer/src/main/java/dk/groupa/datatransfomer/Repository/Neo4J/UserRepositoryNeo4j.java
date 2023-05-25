package dk.groupa.datatransfomer.Repository.Neo4J;

import dk.groupa.datatransfomer.Model.Neo4J.User;
import org.springframework.data.neo4j.repository.Neo4jRepository;
import org.springframework.data.neo4j.repository.query.Query;
import org.springframework.data.repository.query.Param;

public interface UserRepositoryNeo4j extends Neo4jRepository<User, Long> {

    @Query("MATCH (u:User {id: $userId}), (b:Book {id: $bookId}) MERGE (u) - [:BORROWED] -> (b)")
    void createUserLoanToBookRelations(@Param("userId") Long userId, @Param("bookId") Long bookId);
}
