package dk.groupa.datatransfomer.Repository.MSSQL;

import dk.groupa.datatransfomer.Model.MSSQL.LastCronMS;
import jakarta.transaction.Transactional;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

@Repository
public interface LastCronRepositoryMS extends JpaRepository<LastCronMS, Long> {

    @Modifying
    @Transactional
    @Query(value = "UPDATE last_cron SET last_cron.last_run = getdate() WHERE last_cron.id = (SELECT TOP 1 last_cron.id FROM last_cron)", nativeQuery = true)
    void updateCronRunTime();


}
