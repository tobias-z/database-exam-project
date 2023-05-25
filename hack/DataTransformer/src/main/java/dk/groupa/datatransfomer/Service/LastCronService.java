package dk.groupa.datatransfomer.Service;

import dk.groupa.datatransfomer.Model.MSSQL.LastCronMS;
import dk.groupa.datatransfomer.Repository.MSSQL.LastCronRepositoryMS;
import jakarta.transaction.Transactional;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@Transactional
public class LastCronService {
    @Autowired
    LastCronRepositoryMS lastCronRepositoryMS;

    public void updateLastRun() {
        //lastCronRepositoryMS.updateCronRunTime();
    }
    public List<LastCronMS> findAll () {
        return lastCronRepositoryMS.findAll();
    }
}
