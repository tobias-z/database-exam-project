package dk.groupa.datatransfomer.Service;

import dk.groupa.datatransfomer.Model.MSSQL.AuthorMS;
import dk.groupa.datatransfomer.Model.Neo4J.Author;
import dk.groupa.datatransfomer.Repository.Neo4J.AuthorRepositoryNeo4j;
import dk.groupa.datatransfomer.utils.Neo4jParser;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class AuthorService {
    @Autowired
    AuthorRepositoryNeo4j authorRepositoryNeo4j;

    @Autowired
    Neo4jParser neo4jParser;

    public void updateAuthorInformation (List<AuthorMS> authorMSList) {
        List<Author> authors =  neo4jParser.parseAuthorFromMSToNeo4j(authorMSList);

        saveToNeo4j(authors);
        createRelations(authors);
    }

    public void saveToNeo4j(List<Author> authors) {
        authorRepositoryNeo4j.saveAll(authors);
    }

    public void createRelations(List<Author> authors) {
        authors.forEach(_author -> authorRepositoryNeo4j.createRelations(_author.getName()));
    }
}
