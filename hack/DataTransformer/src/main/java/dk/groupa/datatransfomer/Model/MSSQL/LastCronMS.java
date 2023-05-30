package dk.groupa.datatransfomer.Model.MSSQL;

import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Date;

@Entity(name = "last_cron")
@AllArgsConstructor
@NoArgsConstructor
@Getter
@Setter
public class LastCronMS {
    @Id
    private Long id;
    private Date last_run;
}
