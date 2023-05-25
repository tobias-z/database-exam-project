package dk.groupa.datatransfomer.Service;

import dk.groupa.datatransfomer.Model.MSSQL.LoanMS;
import dk.groupa.datatransfomer.Model.MSSQL.UserMS;
import dk.groupa.datatransfomer.Model.Neo4J.User;
import dk.groupa.datatransfomer.Repository.MSSQL.UserRepositoryMS;
import dk.groupa.datatransfomer.Repository.Neo4J.UserRepositoryNeo4j;
import dk.groupa.datatransfomer.utils.Neo4jParser;
import jakarta.transaction.Transactional;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@Transactional
public class UserService {
    @Autowired
    UserRepositoryMS userRepositoryMS;

    @Autowired
    UserRepositoryNeo4j userRepositoryNeo4j;

    @Autowired
    Neo4jParser neo4jParser;

    @Autowired
    LoanService loanService;

    public void updateUserInformation () {
        List<UserMS> userMSList = userRepositoryMS.findAll();
        List<User> users = neo4jParser.parseUserFromMSToNeo4j(userMSList);

        saveToNeo4j(users);
        loanService.createRelations();
    }

    public void saveToNeo4j (List<User> users) {
        userRepositoryNeo4j.saveAll(users);
    }
}
