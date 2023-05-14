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
    private final ConcurrentHashMap<Integer, PriorityBlockingQueue<WaitingBorrow>> reserveQueue;


    public ReserveService(ReserveRepository reserveRepository, ConcurrentHashMap<Integer, PriorityBlockingQueue<WaitingBorrow>> reserveQueue) {
        this.reserveRepository = reserveRepository;
        this.reserveQueue = reserveQueue;
    }

    public PriorityBlockingQueue<WaitingBorrow> Push(int userId, int bookId) {
        BorrowQueue borrowQueue = reserveRepository.reserveBook(userId, bookId);

        WaitingBorrow waitingBorrow = new WaitingBorrow(borrowQueue.getId(), borrowQueue.isSubscribed(), borrowQueue.getEnqueued_at(), borrowQueue.getUser_id());

        reserveQueue.put(bookId, waitingBorrow);
    }

//    public Loan Pop(int bookId) {
//
//    }

//    public int NumberInQue(int bookId, int userId) {
//
//    }
}
