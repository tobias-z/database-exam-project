package dk.groupa.sqldatabase.repository;

import dk.groupa.sqldatabase.entity.Loan;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

@Repository
public interface LoanRepository extends JpaRepository<Loan, Long> {
    @Query(value = "EXEC sp_BorrowBook @user_id = :user_id, @book_id = :book_id", nativeQuery = true)
    Loan borrowBook(@Param("user_id") Long user_id, @Param("book_id") Long book_id);

    @Query(value = "EXEC sp_ReturnBook @user_id = :user_id, @book_id = :book_id", nativeQuery = true)
    Loan returnBook(@Param("user_id") Long user_id, @Param("book_id") Long book_id);
}