package dk.groupa.datatransfomer.Repository.MSSQL;

import dk.groupa.datatransfomer.Model.MSSQL.BookMS;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface BookRepositoryMS extends JpaRepository<BookMS, Long> {
    @Query(value = "SELECT id, title, author_name, rating_score, year_published FROM book WHERE last_updated > :date", nativeQuery = true)
    List<BookMS> getChangesSinceLastCron(@Param("date") String date);
}
