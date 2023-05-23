package dk.groupa.dataimporter.Service;

import dk.groupa.dataimporter.Model.Author;
import dk.groupa.dataimporter.Model.Book;
import dk.groupa.dataimporter.Repository.AuthorRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.stream.Collectors;

@Service
public class AuthorService {
    @Autowired
    AuthorRepository authorRepository;

    private List<Author> mapToAuthor(List<Book> bookList) {
        return bookList.stream().map(book -> book.getAuthor_name()).map(Author::new).toList();
    }

    public void saveAuthors (List<Book> bookList) {
        List<Author> authors = mapToAuthor(bookList);
        authorRepository.saveAll(authors);
    }
}
