package dk.groupa.sqldatabase.entity;

import jakarta.persistence.Column;
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
    @Column(name = "user_id")
    private int userId;
    @Column(name = "book_id")
    private int bookId;
    @Column(name = "enqueued_at")
    private Date enqueuedAt;
    @Column(name = "is_subscribed")
    private boolean isSubscribed;
}