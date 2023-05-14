package dk.groupa.sqldatabase.entity;

import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.*;

import java.util.Date;

@Entity
@Table(name = "borrow_queue")
@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class BorrowQueue {
    @Id
    private Long id;
    private int user_id;
    private int book_id;
    private Date enqueued_at;
    private boolean isSubscribed;
}
