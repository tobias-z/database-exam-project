package dk.groupa.dataimporter.utils;

import com.opencsv.bean.CsvToBeanBuilder;
import dk.groupa.dataimporter.Model.Book;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;

import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

@Component
public class CsvParser {
    @Autowired
    CloseApplication closeApplication;

    private List<Path> findAllCsvFilesInPath(String path) {
        List<Path> result = new ArrayList<>();

        try {
            Path filePath = Paths.get(path);
            String fileExtension = "csv";

            if(!Files.isDirectory(filePath)) {
                throw new IllegalArgumentException("Path must be a directory.");
            }

            try(Stream<Path> walk = Files.walk(filePath)) {
                result = walk
                        .filter(p -> !Files.isDirectory(p))
                        .filter(f -> f.getFileName().toString().endsWith(fileExtension))
                        .collect(Collectors.toList());
            }
            return result;
        } catch (IOException e)  {
            System.out.println(e);
            closeApplication.error("findAllCsvFilesInPath", "There was an IOException error.");
        }

        return result;
    }

    private List<Book> parseCsvToObject (List<Path> csvFiles) {
        List<Book> bookList = new ArrayList<>();

        try {
            // if the list is empty that must mean there are no CSV files present to parse.
            if(csvFiles.isEmpty()) {
                System.out.format("Seems like the path that was given to %s, did not have any csv files to parse. Will exit now. With error code: %s", "parseCsvToObject", "10");
                closeApplication.error("parseCsvToObject", "The csv files list is empty.");
            }

            for(Path csvFile : csvFiles) {
                bookList.addAll(new CsvToBeanBuilder(new FileReader(csvFile.toFile())).withType(Book.class).build().parse());
            }

            return bookList;
        } catch (IOException e) {
            System.out.println(e);
            closeApplication.error("parseCsvToObject", "There was an IOException error.");
        }
        return bookList;
    }

    public List<Book> fillBooksAndReturn(String pathToCsvFile) {
        List<Path> csvPaths;
        List<Book> bookList = new ArrayList<>();

        csvPaths = findAllCsvFilesInPath(pathToCsvFile);

        // Don't need to go any further trying to parse anything size we did not find any csv files in the specific location.
        if(csvPaths.size() == 0) {
            closeApplication.error("fillBooksAndReturn", "csvPaths array size is apparently 0.");
        }

        bookList = parseCsvToObject(csvPaths);

        // Don't need to go any further trying to parse anything size we did not find any csv files in the specific location.
        if(bookList.size() == 0) {
            closeApplication.error("fillBooksAndReturn", "bookList array size is apparently 0.");
        }

        return bookList;
    }
}
