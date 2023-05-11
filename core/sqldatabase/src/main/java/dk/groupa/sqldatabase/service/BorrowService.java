package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.repository.BorrowRepository;
import org.springframework.stereotype.Service;

@Service
public class BorrowService {
    private final BorrowRepository borrowRepository;
    private final BookService bookService;

    public BorrowService(BorrowRepository borrowRepository, BookService bookService) {
        this.borrowRepository = borrowRepository;
        this.bookService = bookService;
    }

    public Loan BorrowBook(int bookId, int userId) {
        return borrowRepository.borrowBook(userId, bookId);
    }
}