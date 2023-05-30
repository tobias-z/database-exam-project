package dk.groupa.dataimporter.Model;

import com.opencsv.bean.CsvBindByName;
import jakarta.persistence.*;
import java.util.Objects;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@AllArgsConstructor
@NoArgsConstructor
@Getter
@Setter
@Entity
public class Book {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @CsvBindByName(column = "Book_Title")
    private String title;

    @CsvBindByName(column = "Book_Description")
    private String description;

    @CsvBindByName(column = "Author_Name")
    private String author_name;

    @CsvBindByName(column = "Edition_Language")
    private String language;

    @CsvBindByName(column = "Rating_score")
    private Double rating_score;

    @CsvBindByName(column = "Rating_votes")
    private Integer rating_votes;

    @CsvBindByName(column = "Review_number")
    private Integer review_number;

    @CsvBindByName(column = "Year_published")
    private Integer year_published;

    private Integer available = 5;

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        Book book = (Book) o;
        return title.equals(book.title) && author_name.equals(book.author_name) && year_published.equals(book.year_published);
    }

    @Override
    public int hashCode() {
        return Objects.hash(title, author_name, year_published);
    }

    // This custom setter is made in order to get rid of any new lines that might be in the description to easier pass it on to the database.
    public void setDescription(String description) {
        // this.description = description.replaceAll("\n", " ");
        this.description = description;
    }

    @Override
    public String toString() {
        return "Book{" +
                "id=" + id +
                ", title='" + title + '\'' +
                ", description='" + description + '\'' +
                ", author_name='" + author_name + '\'' +
                ", language='" + language + '\'' +
                ", rating_score=" + rating_score +
                ", rating_votes=" + rating_votes +
                ", review_number=" + review_number +
                ", year_published=" + year_published +
                ", available=" + available +
                '}';
    }
}
