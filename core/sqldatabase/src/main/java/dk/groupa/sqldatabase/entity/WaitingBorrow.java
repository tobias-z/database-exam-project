package dk.groupa.sqldatabase.entity;

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
public class WaitingBorrow {
    @Id
    private Long borrowQueueId;
    private boolean isSubscribed;
    private Date enqueued_at;
    private int user_id;
}