package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.BorrowQueue;
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
    public void initQueues() {
        List<BorrowQueue> borrowQueueList = reserveRepository.getQueues();
        for (BorrowQueue b: borrowQueueList) {
            PushToPrioQue(b.getUserId(), b);
        }
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
            //reserveQueue.replace(bookId, prioQue);
        } else {
            prioQue.put(waitingBorrow);
            reserveQueue.put(bookId, prioQue);
        }
        return waitingBorrow;
    }

    public WaitingBorrow Pop(int bookId) {
        //TODO: DB call
        WaitingBorrow waitingBorrow;
        PriorityBlockingQueue<WaitingBorrow> prioQue = reserveQueue.get(bookId);
        System.out.println(prioQue);
        if (prioQue == null) {
             return null;
        }
        waitingBorrow = prioQue.remove();
        System.out.println(prioQue);
        return waitingBorrow ;
    }

//    public int NumberInQue(int bookId, int userId) {
//
//    }
}