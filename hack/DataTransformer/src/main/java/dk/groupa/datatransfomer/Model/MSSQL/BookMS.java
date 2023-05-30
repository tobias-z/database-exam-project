package dk.groupa.datatransfomer.Model.MSSQL;

import jakarta.persistence.*;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import org.springframework.data.neo4j.core.schema.Node;

@AllArgsConstructor
@NoArgsConstructor
@Getter
@Setter
@Entity(name = "book")
public class BookMS {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(name = "title")
    private String title;

    @Column(name = "author_name")
    private String author;

    @Column(name = "rating_score")
    private Double avgRating;

    @Column(name = "year_published")
    private Integer yearPublished;

    @Override
    public String toString() {
        return "Book{" +
                "id=" + id +
                ", title='" + title + '\'' +
                ", author_name='" + author + '\'' +
                ", rating_score=" + avgRating +
                ", year_published=" + yearPublished +
                '}';
    }
}
