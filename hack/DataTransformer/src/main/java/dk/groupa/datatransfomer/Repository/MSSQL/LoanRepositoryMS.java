package dk.groupa.datatransfomer.Repository.MSSQL;

import dk.groupa.datatransfomer.Model.MSSQL.LoanMS;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface LoanRepositoryMS extends JpaRepository<LoanMS, Long> {
}
