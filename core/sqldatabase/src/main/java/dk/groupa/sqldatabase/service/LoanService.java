package dk.groupa.sqldatabase.service;

import dk.groupa.sqldatabase.entity.Loan;
import dk.groupa.sqldatabase.repository.LoanRepository;
import org.springframework.stereotype.Service;

@Service
public class LoanService {
    private final LoanRepository loanRepository;
    private final ReserveService reserveService;

    public LoanService(LoanRepository loanRepository, ReserveService reserveService) {
        this.loanRepository = loanRepository;
        this.reserveService = reserveService;
    }

    public Loan BorrowBook(Long userId, Long bookId) {
        return loanRepository.borrowBook(userId, bookId);
    }

    public Loan ReturnBook(Long userId, Long bookId) {
        Loan loan = loanRepository.returnBook(userId, bookId);
        reserveService.Pop(bookId);
        return loan;
    }
}