package dk.groupa.sqldatabase.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Date;

@Entity
@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class WaitingBorrow implements Comparable<WaitingBorrow>{
    @Id
    private Long borrowQueueId;
    @Column(name = "is_subscribed")
    private boolean isSubscribed;
    @Column(name = "enqueued_at")
    private Date enqueuedAt;
    @Column(name = "user_id")
    private int userId;

    @Override
    public int compareTo(WaitingBorrow o) {
        if (this.isSubscribed && !o.isSubscribed) {
            return 1;
            }

        return 0;
    }
}