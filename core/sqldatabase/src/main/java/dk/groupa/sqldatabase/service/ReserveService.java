package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.BorrowQueue;
import dk.groupa.sqldatabase.entity.WaitingBorrow;
import dk.groupa.sqldatabase.repository.ReserveRepository;
import org.springframework.stereotype.Service;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.PriorityBlockingQueue;

@Service
public class ReserveService {
    private final ReserveRepository reserveRepository;
    private static final ConcurrentHashMap<Integer, PriorityBlockingQueue<WaitingBorrow>> reserveQueue = new ConcurrentHashMap<>();


    public ReserveService(ReserveRepository reserveRepository) {
        this.reserveRepository = reserveRepository;
    }

    public PriorityBlockingQueue<WaitingBorrow> Push(int userId, int bookId) {
        BorrowQueue borrowQueue = reserveRepository.reserveBook(userId, bookId);
        WaitingBorrow waitingBorrow = new WaitingBorrow(borrowQueue.getId(), borrowQueue.isSubscribed(), borrowQueue.getEnqueued_at(), borrowQueue.getUser_id());
        PriorityBlockingQueue<WaitingBorrow> prioQue = new PriorityBlockingQueue<>();

        if (reserveQueue.containsKey(bookId)) {
            prioQue = reserveQueue.get(bookId);
            prioQue.put(waitingBorrow);
            reserveQueue.replace(bookId, prioQue);
        } else {
            prioQue.put(waitingBorrow);
            reserveQueue.put(bookId, prioQue);
        }
        return prioQue;
    }

//    public Loan Pop(int bookId) {
//
//    }

//    public int NumberInQue(int bookId, int userId) {
//
//    }
}