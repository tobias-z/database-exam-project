package dk.groupa.datatransfomer.Service;

import com.fasterxml.jackson.annotation.JacksonAnnotationsInside;
import dk.groupa.datatransfomer.Model.MSSQL.AuthorMS;
import dk.groupa.datatransfomer.Model.MSSQL.BookMS;
import dk.groupa.datatransfomer.Repository.MSSQL.BookRepositoryMS;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.util.Pair;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.stream.Collectors;

@Service
public class UpdateService {
    @Autowired
    BookRepositoryMS bookRepositoryMS;

    @Autowired
    BookService bookService;

    @Autowired
    AuthorService authorService;

    @Autowired
    UserService userService;

    @Autowired
    LastCronService lastCronService;

    @Value("${cron-job}")
    private String cronJob;

    public Pair<List<BookMS>, List<AuthorMS>> getUpdateOrInsertedBooksSinceLastCron() {
        // Need to fetch that row from "last_cron" to get the last time data was updated.

        List<BookMS> books = bookRepositoryMS.getChangesSinceLastCron(String.valueOf(lastCronService.findAll().get(0).getLast_run()));
        List<AuthorMS> authors = books.stream().map(_book -> new AuthorMS(_book.getAuthor())).collect(Collectors.toList());

        return Pair.of(books, authors);
    }

    @Scheduled(cron = "0 */5 * * * *")
    public void updateNeo4jData() {
        Pair<List<BookMS>, List<AuthorMS>> newestData = getUpdateOrInsertedBooksSinceLastCron();

        // Merge all new book data into neo4j
        bookService.updateBookInformation(newestData.getFirst());

        // Merge all new author into neo4j
        authorService.updateAuthorInformation(newestData.getSecond());

        // Merge all new user data into neo4j
        userService.updateUserInformation();

        lastCronService.updateLastRun();
    }
}
