package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.BorrowQueue;
import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.entity.WaitingBorrow;
import dk.groupa.sqldatabase.repository.ReserveRepository;
import jakarta.annotation.PostConstruct;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.PriorityBlockingQueue;

@Service
public class ReserveService {
    private final ReserveRepository reserveRepository;
    private static final ConcurrentHashMap<Long, PriorityBlockingQueue<WaitingBorrow>> reserveQueue = new ConcurrentHashMap<>();


    public ReserveService(ReserveRepository reserveRepository) {
        this.reserveRepository = reserveRepository;
    }
    @PostConstruct
    public void InitQueues() {
        List<BorrowQueue> borrowQueueList = reserveRepository.getQueues();
        for (BorrowQueue b: borrowQueueList) {
            PushToPrioQue(b.getUserId(), b);
        }
    }

    public Loan UpdateBorrowQueueDB(Long borrowQueueId, Long userId, Long bookId) {
        return reserveRepository.borrowReserve(borrowQueueId, userId, bookId);
    }

    public WaitingBorrow Push(Long userId, Long bookId) {
        BorrowQueue borrowQueue = reserveRepository.reserveBook(userId, bookId);
        return PushToPrioQue(bookId, borrowQueue);
    }

    private static WaitingBorrow PushToPrioQue(Long bookId, BorrowQueue borrowQueue) {
        WaitingBorrow waitingBorrow = new WaitingBorrow(borrowQueue.getId(), borrowQueue.isSubscribed(), borrowQueue.getEnqueuedAt(), borrowQueue.getUserId());
        PriorityBlockingQueue<WaitingBorrow> prioQue = new PriorityBlockingQueue<>();

        if (reserveQueue.containsKey(bookId)) {
            prioQue = reserveQueue.get(bookId);
            prioQue.put(waitingBorrow);
        } else {
            prioQue.put(waitingBorrow);
            reserveQueue.put(bookId, prioQue);
        }
        return waitingBorrow;
    }

    public WaitingBorrow Pop(Long bookId) {
        WaitingBorrow waitingBorrow;
        PriorityBlockingQueue<WaitingBorrow> prioQue = reserveQueue.get(bookId);
        if (prioQue == null) {
             return null;
        }
        waitingBorrow = prioQue.remove();
        UpdateBorrowQueueDB(waitingBorrow.getBorrowQueueId(), waitingBorrow.getUserId(), bookId);
        return waitingBorrow ;
    }

    public int NumberInQue(Long userId, Long bookId) {
        PriorityBlockingQueue<WaitingBorrow> prioQue = reserveQueue.get(bookId);
        int number = 0;
        for (WaitingBorrow w: prioQue) {
            number++;
            if (w.getUserId() == userId) {
                return number;
            }
        }
        return 0;
    }
}