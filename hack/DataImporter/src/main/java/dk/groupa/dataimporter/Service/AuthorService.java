package dk.groupa.dataimporter.Service;

import dk.groupa.dataimporter.Model.Author;
import dk.groupa.dataimporter.Model.Book;
import dk.groupa.dataimporter.Repository.AuthorRepository;
import java.util.Set;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.stream.Collectors;

@Service
public class AuthorService {
    @Autowired
    AuthorRepository authorRepository;

    private List<Author> mapToAuthor(Set<Book> bookList) {
        return bookList.stream().map(book -> book.getAuthor_name()).distinct().map(Author::new).toList();
    }

    public void saveAuthors (Set<Book> bookList) {
        List<Author> authors = mapToAuthor(bookList);
        authorRepository.saveAll(authors);
    }
}
