package dk.groupa.dataimporter;

import dk.groupa.dataimporter.Service.BookService;
import dk.groupa.dataimporter.utils.CloseApplication;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;


@SpringBootApplication
public class DataImporterApplication implements CommandLineRunner {
    @Autowired
    CloseApplication closeApplication;

    @Autowired
    BookService bookService;

    public static void main(String[] args) {
        SpringApplication.run(DataImporterApplication.class, args);
    }

    @Override
    public void run(String... args) {
        bookService.parseCsvFiles();

        // Everything seems to have gone great since we reach this point. So lets close the application with the correct error code.
        closeApplication.exit(0);
    }
}
