package dk.groupa.sqldatabase.repository;

import dk.groupa.sqldatabase.entity.BorrowQueue;
import dk.groupa.sqldatabase.service.ReserveService;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface ReserveRepository extends JpaRepository<BorrowQueue, Long> {

    @Query(value = "EXEC sp_ReserveBook @user_id = :user_id, @book_id = :book_id", nativeQuery = true)
    BorrowQueue reserveBook(@Param("user_id") int user_id, @Param("book_id") int book_id);

    @Query(value = "SELECT * FROM borrow_queue", nativeQuery = true)
    List<BorrowQueue> getQueues();


}
