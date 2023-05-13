package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.repository.ReserveRepository;
import org.springframework.stereotype.Service;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.PriorityBlockingQueue;

@Service
public class ReserveService {
    private final ReserveRepository repository;
    private final ConcurrentHashMap<Integer, PriorityBlockingQueue<Integer>> reserveQue;

    public ReserveService(ReserveRepository repository, ConcurrentHashMap<Integer, PriorityBlockingQueue<Integer>> reserveQue) {
        this.repository = repository;
        this.reserveQue = reserveQue;
    }

    public Loan Push(int bookId, int userId) {
        reserveQue.put(bookId, userId);

    }

    public Loan Pop(int bookId) {

    }

    public int NumberInQue(int bookId, int userId) {

    }
}
