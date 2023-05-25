package dk.groupa.datatransfomer.Service;

import dk.groupa.datatransfomer.Model.MSSQL.LoanMS;
import dk.groupa.datatransfomer.Repository.MSSQL.LoanRepositoryMS;
import dk.groupa.datatransfomer.Repository.Neo4J.UserRepositoryNeo4j;
import jakarta.transaction.Transactional;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@Transactional
public class LoanService {
    @Autowired
    LoanRepositoryMS loanRepositoryMS;

    @Autowired
    UserRepositoryNeo4j userRepositoryNeo4j;

    public List<LoanMS> findAllLoans() {
        return loanRepositoryMS.findAll();
    }

    public void createRelations() {
        findAllLoans().forEach(_loan -> userRepositoryNeo4j.createUserLoanToBookRelations(_loan.getUser_id(), _loan.getBook_id()));
    }
}
