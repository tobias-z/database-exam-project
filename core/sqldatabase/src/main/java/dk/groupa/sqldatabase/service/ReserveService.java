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
    private static final ConcurrentHashMap<Integer, PriorityBlockingQueue<WaitingBorrow>> reserveQueue = new ConcurrentHashMap<>();


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

    public Loan UpdateBorrowQueueDB(int borrowQueueId, int userId, int bookId) {
        Loan loan = reserveRepository.borrowReserve(borrowQueueId, userId, bookId);
        return loan;
    }

    public WaitingBorrow Push(int userId, int bookId) {
        BorrowQueue borrowQueue = reserveRepository.reserveBook(userId, bookId);
        return PushToPrioQue(bookId, borrowQueue);
    }

    private static WaitingBorrow PushToPrioQue(int bookId, BorrowQueue borrowQueue) {
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

    public WaitingBorrow Pop(int bookId) {
        WaitingBorrow waitingBorrow;
        PriorityBlockingQueue<WaitingBorrow> prioQue = reserveQueue.get(bookId);
        if (prioQue == null) {
             return null;
        }
        waitingBorrow = prioQue.remove();
        UpdateBorrowQueueDB(waitingBorrow.getBorrowQueueId().intValue(), waitingBorrow.getUserId(), bookId);
        return waitingBorrow ;
    }

//    public int NumberInQue(int bookId, int userId) {
//
//    }
}