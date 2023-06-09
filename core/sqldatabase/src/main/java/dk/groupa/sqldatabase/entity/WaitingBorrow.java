package dk.groupa.sqldatabase.entity;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import java.util.Date;

@Getter
@Setter
@AllArgsConstructor
@NoArgsConstructor
public class WaitingBorrow implements Comparable<WaitingBorrow>{
    private Long borrowQueueId;
    private boolean isSubscribed;
    private Date enqueuedAt;
    private Long userId;

    @Override
    public int compareTo(WaitingBorrow o) {
        if (this.isSubscribed && !o.isSubscribed) {
            return 1;
        } else if (!this.isSubscribed && o.isSubscribed) {
            return -1;
        } else {
            return this.enqueuedAt.compareTo(o.enqueuedAt);
        }
    }
}