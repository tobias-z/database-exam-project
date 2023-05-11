package dk.groupa.sqldatabase.repository;

import org.apache.catalina.User;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

@Repository
public interface BorrowRepository extends JpaRepository<User, Long> {
    @Query(value = "EXEC sp_BorrowBook @user_id = :user_id, @book_id = :book_id", nativeQuery = true)
    Loans borrowBook(@Param("user_id") int user_id, @Param("book_id") int book_id);

}
