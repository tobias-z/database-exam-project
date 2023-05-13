package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.repository.LoanRepository;
import org.springframework.stereotype.Service;

@Service
public class LoanService {
    private final LoanRepository loanRepository;
    private final BookService bookService;

    public LoanService(LoanRepository loanRepository, BookService bookService) {
        this.loanRepository = loanRepository;
        this.bookService = bookService;
    }

    public Loan BorrowBook(int bookId, int userId) {
        return loanRepository.borrowBook(userId, bookId);
    }

    public Loan ReturnBook(int bookId, int userId) {
        return loanRepository.returnBook(userId, bookId);
    }
}