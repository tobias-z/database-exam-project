package dk.groupa.datatransfomer.utils;

import dk.groupa.datatransfomer.Model.MSSQL.AuthorMS;
import dk.groupa.datatransfomer.Model.MSSQL.BookMS;
import dk.groupa.datatransfomer.Model.MSSQL.UserMS;
import dk.groupa.datatransfomer.Model.Neo4J.Author;
import dk.groupa.datatransfomer.Model.Neo4J.Book;
import dk.groupa.datatransfomer.Model.Neo4J.User;
import org.springframework.stereotype.Component;

import java.util.List;

@Component
public class Neo4jParser {
    public List<Book> parseBookFromMSToNeo4j(List<BookMS> bookMSList) {
        return bookMSList.stream().map(_book -> new Book(_book.getId(), _book.getTitle(), _book.getAuthor(), _book.getAvgRating(), _book.getYearPublished())).toList();
    }

    public List<Author> parseAuthorFromMSToNeo4j(List<AuthorMS> authorMSList) {
        return authorMSList.stream().distinct().map(_book -> new Author(_book.getName())).toList();
    }

    public List<User> parseUserFromMSToNeo4j(List<UserMS> userMSList) {
        return userMSList.stream().map(_user -> new User(_user.getId(), _user.getEmail(), _user.getRole())).toList();
    }
}
