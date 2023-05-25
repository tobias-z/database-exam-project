package dk.groupa.datatransfomer.Model.Neo4J;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import org.springframework.data.neo4j.core.schema.Id;
import org.springframework.data.neo4j.core.schema.Node;

@Node
@AllArgsConstructor
@NoArgsConstructor
@Getter
@Setter
public class Book {
    @Id
    private Long id;
    private String title;
    private String author;
    private Double avgRating;
    private Integer yearPublished;
}
