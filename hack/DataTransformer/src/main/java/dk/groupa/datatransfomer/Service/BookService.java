package dk.groupa.datatransfomer.Service;

import dk.groupa.datatransfomer.Model.Neo4J.Book;
import dk.groupa.datatransfomer.Model.MSSQL.BookMS;
import dk.groupa.datatransfomer.Repository.MSSQL.BookRepositoryMS;
import dk.groupa.datatransfomer.Repository.Neo4J.BookRepositoryNeo4j;
import dk.groupa.datatransfomer.utils.Neo4jParser;
import jakarta.transaction.Transactional;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@Transactional
public class BookService {
    @Autowired
    BookRepositoryMS bookRepositoryMS;

    @Autowired
    BookRepositoryNeo4j bookRepositoryNeo4j;

    @Autowired
    Neo4jParser neo4jParser;

    public void updateBookInformation (List<BookMS> bookMSList) {
        List<Book> books = neo4jParser.parseBookFromMSToNeo4j(bookMSList);

        saveToNeo4j(books);
    }

    public void saveToNeo4j (List<Book> books) {
        bookRepositoryNeo4j.saveAll(books);
    }
}
