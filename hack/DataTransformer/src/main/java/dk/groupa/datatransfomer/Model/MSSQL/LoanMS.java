package dk.groupa.datatransfomer.Model.MSSQL;

import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Date;

@AllArgsConstructor
@NoArgsConstructor
@Getter
@Setter
@Entity(name = "loans")
public class LoanMS {
    @Id
    private Long id;
    private Long user_id;
    private Long book_id;
    private Date borrowed_at;
    private Date due_date;
    private Date returned_at;
}
