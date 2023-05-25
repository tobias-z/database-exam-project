package dk.groupa.datatransfomer.Repository.MSSQL;

import dk.groupa.datatransfomer.Model.MSSQL.UserMS;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface UserRepositoryMS extends JpaRepository<UserMS, Long> {

    @Override
    @Query(value = "SELECT * FROM dbo.[user]", nativeQuery = true)
    List<UserMS> findAll();
}
