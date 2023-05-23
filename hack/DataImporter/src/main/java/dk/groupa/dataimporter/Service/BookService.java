package dk.groupa.dataimporter.Service;

import dk.groupa.dataimporter.Model.Book;
import dk.groupa.dataimporter.Repository.BookRepository;
import dk.groupa.dataimporter.utils.CloseApplication;
import dk.groupa.dataimporter.utils.CsvParser;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class BookService {
    @Autowired
    CsvParser csvParser;

    @Autowired
    CloseApplication closeApplication;

    @Autowired
    BookRepository bookRepository;

    @Autowired
    AuthorService authorService;

    @Value("${csv-location}")
    private String csvLocation;

    public void parseCsvFiles() {
        List<Book> bookList = csvParser.fillBooksAndReturn(csvLocation);

        long startTime = System.currentTimeMillis();
        authorService.saveAuthors(bookList);
        bookRepository.saveAll(bookList);
        long endTime = System.currentTimeMillis();

        System.out.println("That took " + (endTime - startTime) + " milliseconds");
    }
}