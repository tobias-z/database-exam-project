package dk.groupa.sqldatabase.repository;

import dk.groupa.sqldatabase.entity.Book;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface BookRepository extends JpaRepository<Book, Long> {

    @Query(value = "SELECT available from book where id = :book_id", nativeQuery = true)
    Integer available(@Param("book_id") int book_id);
}