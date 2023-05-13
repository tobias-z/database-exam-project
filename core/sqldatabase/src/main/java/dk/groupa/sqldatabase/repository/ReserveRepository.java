package dk.groupa.sqldatabase.repository;

import dk.groupa.sqldatabase.entity.Loan;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

@Repository
public interface ReserveRepository extends JpaRepository<Loan, Long> {

    @Query(value = "EXEC sp_ReserveBook @user_id = :user_id, @book_id = :book_id", nativeQuery = true)
    Loan reserveBook(@Param("user_id") int user_id, @Param("book_id") int book_id);
}
